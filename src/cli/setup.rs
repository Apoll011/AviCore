use crate::api::Api;
use crate::content::getters::get_from_settings;
use crate::content::lang::LanguageProvider;
use crate::content::skill::SkillProvider;
use crate::ui::{
    ask, ask_confirm, ask_number, main_progress_style, select_option, spinner_style, step, sub_step,
};
use crate::utils::config_dir;
use console::style;
use content_resolver::ResourceResolver;
use indicatif::ProgressBar;
use log::{error, info, warn};
use self_update::cargo_crate_version;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Setup {
    config_path: PathBuf,
    online: bool,
}

#[derive(Debug)]
pub enum SetupError {
    IoError(std::io::Error),
    NetworkError(String),
    ConfigError(String),
    #[allow(dead_code)]
    UpdateError(String),
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::IoError(e) => write!(f, "IO Error: {}", e),
            SetupError::NetworkError(e) => write!(f, "Network Error: {}", e),
            SetupError::ConfigError(e) => write!(f, "Configuration Error: {}", e),
            SetupError::UpdateError(e) => write!(f, "Update Error: {}", e),
        }
    }
}

impl std::error::Error for SetupError {}

impl From<std::io::Error> for SetupError {
    fn from(err: std::io::Error) -> Self {
        SetupError::IoError(err)
    }
}

impl Setup {
    pub fn new(dir: &Path) -> Self {
        Self {
            config_path: dir.to_path_buf(),
            online: false,
        }
    }

    pub async fn check(&mut self) -> Result<(), SetupError> {
        info!("Checking system...");
        info!("Config path set to: {}", self.config_path.display());

        if self.needs_setup() {
            self.setup().await?;
        }

        Ok(())
    }

    fn needs_setup(&self) -> bool {
        !self.config_path.exists()
    }

    async fn setup(&mut self) -> Result<(), SetupError> {
        sub_step(1, 6, "Initialization");

        // Setup mode selection
        let modes = vec![
            "Offline (Bare Minimum)",
            "Online (Download all the required resources)",
        ];
        let modes_v = ["offline", "online"];
        let mode = modes_v[select_option("How would you like to setup Avi", &modes)];

        self.online = mode == "online";

        // Environment Configuration
        sub_step(2, 6, "Environment Configuration");

        let langs = vec!["English", "Português"];
        let lang_v = ["en", "pt"];
        let lang = lang_v[select_option("Select the system language", &langs)];

        let devs = vec![
            "Listener (Only Listen to input)",
            "Speaker (Can speak sentences)",
            "Both (Listener and Speaker)",
            "None",
        ];
        let dev_v = ["listener", "speaker", "both", "none"];
        let dev = dev_v[select_option("Select Device Profile", &devs)];

        // Network Configuration
        sub_step(3, 6, "Network Configuration");

        let nets = vec!["Core (Central Hub)", "Node (Satellite)"];
        let nets_v = ["core", "node"];
        let net = nets_v[select_option("Select the network topology", &nets)];

        let gateway = ask_confirm("Allow device to act as Gateway");

        // NLU Configuration
        sub_step(4, 6, "NLU Configuration");

        let (nlu_host, nlu_port) = self.configure_nlu().await?;

        // Setting up configuration
        sub_step(5, 6, "Setting up");

        let pb = ProgressBar::new_spinner();
        pb.set_style(spinner_style());
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_message("Setting up config folder.");

        self.create_config_folder(
            lang.to_string(),
            dev.to_string(),
            net.to_string(),
            gateway,
            nlu_host,
            nlu_port,
        )?;

        pb.finish_with_message("Config folder created.");

        sub_step(6, 6, "Setup Complete");
        println!(
            "{} {}",
            style("✔").green().bold(),
            style("Initial setup completed successfully!").bright()
        );

        Ok(())
    }

    pub async fn post_runtime_init(&self) -> Result<(), SetupError> {
        self.online_setup(
            Arc::new(get_from_settings("lang_resolvers".to_string()).unwrap()),
            Arc::new(get_from_settings("skill_resolvers".to_string()).unwrap()),
        )
        .await
    }

    async fn configure_nlu(&self) -> Result<(String, u16), SetupError> {
        let mut nlu_host = "0.0.0.0".to_string();
        let mut nlu_port: u16 = 1178;

        if !self.has_nlu().await {
            println!(
                "{} {}",
                style("⚠").yellow().bold(),
                style("Natural Language Understanding (NLU) server not detected.").yellow()
            );

            let nlus = vec![
                "Install NLU server Locally",
                "Configure remote NLU connection",
                "Skip",
            ];
            let nlu_v = ["install", "config", "skip"];
            let nlu = nlu_v[select_option("NLU Setup Options", &nlus)];

            match nlu {
                "install" => {
                    self.install_nlu()?;
                }
                "config" => {
                    nlu_host = self.validate_host(&ask("Remote NLU host", &nlu_host))?;
                    nlu_port =
                        self.validate_port(ask_number("Remote NLU port", nlu_port.into()) as i64)?;
                }
                _ => {
                    warn!("NLU configuration skipped. You may need to configure it later.");
                }
            }
        } else {
            println!(
                "{} {}",
                style("✔").green().bold(),
                style("Natural Language Understanding (NLU) server detected... Skipping step.")
                    .bright()
            );
        }

        Ok((nlu_host, nlu_port))
    }

    fn validate_host(&self, host: &str) -> Result<String, SetupError> {
        if host.trim().is_empty() {
            return Err(SetupError::ConfigError("Host cannot be empty".to_string()));
        }

        // Basic validation - could be enhanced with regex
        if host.contains(' ') {
            return Err(SetupError::ConfigError(
                "Host cannot contain spaces".to_string(),
            ));
        }

        Ok(host.trim().to_string())
    }

    fn validate_port(&self, port: i64) -> Result<u16, SetupError> {
        if port < 1 || port > 65535 {
            return Err(SetupError::ConfigError(
                "Port must be between 1 and 65535".to_string(),
            ));
        }
        Ok(port as u16)
    }

    async fn online_setup(
        &self,
        skill_resolvers: Arc<ResourceResolver>,
        lang_resolvers: Arc<ResourceResolver>,
    ) -> Result<(), SetupError> {
        if !self.online {
            return Ok(());
        }

        sub_step(1, 5, "Downloading online resources");

        sub_step(2, 5, "Checking for updates");
        let setup_clone = self.clone();
        let _ = tokio::task::spawn_blocking(move || {
            if let Err(e) = setup_clone.update() {
                error!("Update failed: {}", e);
            }
        })
        .await;

        sub_step(3, 5, "Downloading skills");
        self.download_initial_skills(skill_resolvers)
            .await
            .map_err(|e| SetupError::NetworkError(e.to_string()))?;

        sub_step(4, 5, "Downloading Languages");
        self.download_languages(lang_resolvers)
            .await
            .map_err(|e| SetupError::NetworkError(e.to_string()))?;

        sub_step(5, 5, "Downloading Dashboard");
        self.dashboard();

        Ok(())
    }

    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("apoll011")
            .repo_name("aviCore")
            .bin_name("avicore")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;

        info!("Update status: `{}`!", status.version());
        Ok(())
    }

    async fn has_nlu(&self) -> bool {
        let api = Api::new();
        api.alive().await.is_ok()
    }

    pub async fn download_skill(
        &self,
        skill_id: String,
        pb: &ProgressBar,
        skill_provider: &SkillProvider,
    ) -> Result<(), Box<dyn std::error::Error>> {
        pb.set_message(format!("Downloading {}...", style(&skill_id).cyan()));

        let output_dir = config_dir().join("skills");

        let result = skill_provider
            .download_skill(&skill_id, &output_dir)
            .await?;

        info!(
            "Downloaded {} files ({} bytes) for skill: {}",
            result.files_written.len(),
            result.total_bytes,
            skill_id
        );

        pb.inc(1);
        Ok(())
    }

    async fn download_initial_skills(
        &self,
        resolver: Arc<ResourceResolver>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let provider = SkillProvider::new(resolver, "skills".to_string());

        let skills = provider.list_skills().await?;
        let pb = ProgressBar::new(skills.len() as u64);
        pb.set_style(main_progress_style());

        for skill in skills.iter() {
            if let Err(e) = self.download_skill(skill.id.clone(), &pb, &provider).await {
                error!("Failed to download skill {}: {}", skill.id, e);
                // Continue with other skills even if one fails
            }
        }

        pb.finish_with_message("All skills downloaded.");
        println!(
            "{} {}",
            style("✔").green().bold(),
            style("Skill synchronization complete.").bright()
        );

        Ok(())
    }
    async fn download_languages(
        &self,
        resolver: Arc<ResourceResolver>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let provider = LanguageProvider::new(resolver, "lang".to_string());
        let langs_list = provider.list_languages().await?;

        for lang in langs_list {
            let path = config_dir().join("lang").join(format!("{}.lang", lang));

            match File::create(&path) {
                Ok(mut file) => {
                    let content = provider.fetch_language(&lang).await?;
                    file.write_all(content.as_bytes())?;
                    info!("Downloaded language file: {}", lang);
                }
                Err(e) => {
                    error!("Failed to create language file {}: {}", lang, e);
                    return Err(Box::new(e));
                }
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn download_language(
        lang_id: String,
        resolver: Arc<ResourceResolver>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let provider = LanguageProvider::new(resolver, "lang".to_string());
        let path = config_dir().join("lang").join(format!("{}.lang", lang_id));

        let mut file = File::create(&path)?;
        let content = provider.fetch_language(&lang_id).await?;
        file.write_all(content.as_bytes())?;

        info!("Downloaded language: {}", lang_id);
        Ok(())
    }

    pub fn download_dashboard(&self) {
        let dashboard_assets = [
            "Index & Templates",
            "React Runtime",
            "Control Stylesheets",
            "Static Assets (Icons)",
            "Service Worker",
        ];

        step(3, 5, "Installing Web Dashboard");
        sub_step(1, 1, "Connecting to repository...");

        let pb = ProgressBar::new(dashboard_assets.len() as u64);
        pb.set_style(main_progress_style());

        for asset in &dashboard_assets {
            pb.set_message(format!("Unpacking {}...", style(asset).cyan()));
            // Simulate network/disk I/O
            thread::sleep(Duration::from_millis(600));
            pb.inc(1);
        }

        pb.finish_and_clear();
        println!(
            "{} {}",
            style("✔").green().bold(),
            style("Dashboard UI installed.").bright()
        );
    }

    fn dashboard(&self) {
        if !self.is_dashboard_available() {
            println!(
                "{} {}",
                style("⚠").yellow().bold(),
                style("Dashboard not found locally.").yellow()
            );
            if ask_confirm("Would you like to download the Avi Dashboard?") {
                self.download_dashboard();
            }
        }
    }

    pub fn is_dashboard_available(&self) -> bool {
        self.config_path.join("site").exists()
    }

    pub fn install_nlu(&self) -> Result<(), SetupError> {
        // TODO: Implement NLU installation
        warn!("NLU installation not yet implemented");
        Ok(())
    }

    pub fn create_config_folder(
        &self,
        language: String,
        device_type: String,
        net_topology: String,
        gateway: bool,
        nlu_host: String,
        nlu_port: u16,
    ) -> Result<(), SetupError> {
        info!("Creating configuration folders...");

        // Create necessary directories
        let folders = ["config", "lang", "skills"];
        fs::create_dir_all(&self.config_path)?;

        for folder in folders {
            let folder_path = self.config_path.join(folder);
            fs::create_dir_all(&folder_path)?;
            info!("Created directory: {}", folder_path.display());
        }

        // Create const.config
        self.create_const_config()?;

        // Create settings.config with user-provided values
        self.create_settings_config(
            language,
            device_type,
            net_topology,
            gateway,
            nlu_host,
            nlu_port,
        )?;

        // Create default language files only if offline mode
        if !self.online {
            self.create_default_language_files()?;
        }

        info!("Configuration folder created successfully");
        Ok(())
    }

    fn create_const_config(&self) -> Result<(), SetupError> {
        let const_content = include_str!("../../config/config/const.config");
        let const_path = self.config_path.join("config/const.config");

        let mut file = File::create(&const_path)?;
        file.write_all(const_content.as_bytes())?;

        info!("Created const.config");
        Ok(())
    }

    fn create_settings_config(
        &self,
        language: String,
        device_type: String,
        net_topology: String,
        gateway: bool,
        nlu_host: String,
        nlu_port: u16,
    ) -> Result<(), SetupError> {
        let api_url = format!("http://{}:{}", nlu_host, nlu_port);

        let settings_content = format!(
            r#"settings:
  api_url:
    value: {}
    vtype: io.ip
    description: IP address of the nlu api
    ui: text
    required: true
  watch_skill_dir:
    value: true
    vtype: boolean
    description: Watch Skill Dir for changes and reload the skills.
    ui: toggle
    group: Skills Watch Dir
  watch_dir_debounce_time:
    value: 1
    vtype: time.seconds
    description: Time for debouncing of the Directories Watchers
    ui: slider
    min: 1
    max: 30
    group: Skills Watch Dir
  dialogue_cap:
    value: {}
    vtype: enum
    enum_:
      - speaker
      - listener
      - none
      - both
    description: Select Dialogue mode
    ui: dropdown
  lang:
    value: {}
    vtype: enum
    enum_:
      - en
      - pt
    description: Language
    ui: dropdown
  device_type:
    value: {}
    vtype: enum
    enum_:
      - core
      - node
    description: The network topology of the device
    ui: dropdown
  can_gateway:
    value: {}
    vtype: boolean
    description: If the device can work as a gateway for embedded devices
    ui: toggle
  lang_resolvers:
    value:
      - git:apoll011@aviCore:master:./config
    vtype: list
    description: A list of all the lang resolvers
  skill_resolvers:
    value:
      - git:apoll011@aviCore:master:./config
    vtype: list
    description: A list of all the skill resolvers
"#,
            api_url,
            device_type, // dialogue_cap maps to device profile (listener/speaker/both/none)
            language,    // lang
            net_topology, // device_type (core/node)
            gateway      // can_gateway
        );

        let settings_path = self.config_path.join("config/settings.config");
        let mut file = File::create(&settings_path)?;
        file.write_all(settings_content.as_bytes())?;

        info!("Created settings.config with user preferences");
        Ok(())
    }

    fn create_default_language_files(&self) -> Result<(), SetupError> {
        let lang_files = [
            ("en.lang", include_str!("../../config/lang/en.lang")),
            ("pt.lang", include_str!("../../config/lang/pt.lang")),
        ];

        for (filename, content) in lang_files {
            let lang_path = self.config_path.join("lang").join(filename);
            let mut file = File::create(&lang_path)?;
            file.write_all(content.as_bytes())?;
            info!("Created default language file: {}", filename);
        }

        Ok(())
    }
}
