use crate::api::Api;
use crate::content::lang::LanguageProvider;
use crate::content::skill::SkillProvider;
use crate::ui::ask;
use crate::ui::ask_confirm;
use crate::ui::ask_number;
use crate::ui::info_line;
use crate::ui::main_progress_style;
use crate::ui::select_option;
use crate::ui::spinner_style;
use crate::ui::step;
use crate::ui::sub_step;
use crate::utils::config_dir;
use console::style;
use content_resolver::ResourceResolver;
use indicatif::ProgressBar;
use log::{error, info};
use reqwest::StatusCode;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub struct Setup {
    config_path: PathBuf,
    online: bool,
}

#[allow(dead_code, unused)]
impl Setup {
    pub fn new(dir: &Path) -> Self {
        Self {
            config_path: dir.to_path_buf(),
            online: false,
        }
    }

    pub async fn check(&mut self) {
        info!("Checking system...");
        info!("Config path set to: {}", self.config_path.display());
        if self.need_to() {
            self.setup().await;
        }
    }

    fn need_to(&self) -> bool {
        !Path::new(&self.config_path).exists()
    }

    async fn setup(&mut self) {
        sub_step(1, 6, "Initialization");

        let mut nlu_host = "0.0.0.0";
        let mut nlu_port = 1178;

        let modes = vec![
            "Offline (Bare Minimum)",
            "Online (Download all the required resources)",
        ];
        let modes_v = ["offline", "online"];
        let mode = modes_v[select_option("How would you like to setup Avi", &modes)];

        sub_step(2, 6, "Enviroment Configuration");

        let langs = vec!["English", "Português"];
        let lang_v = ["en", "pt"];
        let lang = lang_v[select_option("Select the system language", &langs)];

        let devs = vec![
            "Listenner (Only Listen to input)",
            "Speaker (Can speake sentences)",
            "Both (Listenner and Speaker)",
            "None",
        ];
        let dev_v = ["listenner", "speaker", "both", "none"];
        let dev = dev_v[select_option("Select Device Profile", &devs)];

        sub_step(3, 6, "Enviroment Configuration");

        let nets = vec!["Core (Central Hub)", "Node (Satelite)"];
        let nets_v = ["core", "node"];
        let net = nets_v[select_option("Select the system language", &nets)];

        let gateway = ask_confirm("Allow device to act as Gateway");

        sub_step(4, 6, "NLU COnfiguration");

        if (!self.has_nlu().await) {
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
            let nlu = nlu_v[select_option("Select the system language", &nlus)];

            if nlu.eq("install") {
                self.install_nlu();
            } else if nlu.eq("config") {
                let nlu_host = ask("Remote NLU host", nlu_host).as_str();
                let nlu_port = ask_number("Remote NLU host", nlu_port);
            }
        } else {
            pub fn success(msg: &str) {
                println!(
                    "{} {}",
                    style("✔").green().bold(),
                    style("Natural Language Understanding (NLU) server detected... Skipping step.")
                        .bright()
                );
            }
        }

        sub_step(5, 6, "Setting up");

        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(spinner_style());

        pb.enable_steady_tick(Duration::from_millis(120));

        let pb = pb.with_message("Setting up config folder.");

        self.create_config_folder(
            lang.to_string(),
            dev.to_string(),
            net.to_string(),
            gateway,
            nlu_host.to_string(),
            nlu_port,
        );
        pb.finish_with_message("Config folder created.");

        if (mode.eq("online")) {
            self.online = true;
        } else {
            println!(" {}", style("Skipping resource download.").bright())
        }
    }

    async fn online_setup(&self, skill_resolvers: Arc<ResourceResolver>) {
        if !self.online {
            return;
        }

        self.download_initial_skills(skill_resolvers).await;
        self.dashoard();
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
        pb.set_message(format!("Downloading {}...", style(skill_id.clone()).cyan()));

        let output_dir = config_dir().join("skills");

        let result = skill_provider
            .download_skill(&skill_id, &output_dir)
            .await?;

        println!(
            "Downloaded {} files ({} bytes)",
            result.files_written.len(),
            result.total_bytes
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

        for skill in skills.iter().map(|s| s.id.clone()) {
            self.download_skill(skill.to_string(), &pb, &provider).await;
        }

        pb.finish_with_message("All skills downloaded.");
        println!(
            "{} {}",
            style("✔").green().bold(),
            style("Skill synchronization complete.").bright()
        );

        Ok(())
    }

    pub async fn download_language(
        lang_id: String,
        resolver: Arc<ResourceResolver>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let provider = LanguageProvider::new(resolver, "lang".to_string());
        let path = config_dir().join("lang").join(format!("{}.lang", lang_id));
        let mut file = File::create(path)?;

        let content = provider.fetch_language(&lang_id).await?;

        file.write_all(content.as_bytes())?;

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

        pb.finish_and_clear(); // Clears the bar so it doesn't clutter the terminal
        println!(
            "{} {}",
            style("✔").green().bold(),
            style("Dashboard UI installed.").bright()
        );
    }

    fn dashoard(&self) {
        if !self.is_dashboard_avaliable() {
            println!(
                "{} {}",
                style("⚠").yellow().bold(),
                style("Dashboard not found locally.").yellow()
            );
            if ask_confirm("Would you like to download the Avicia Dashboard?") {
                self.download_dashboard();
            }
        } else {
            info_line("Dashboard", "Already installed");
            if ask_confirm("Check for updates?") {
                self.download_dashboard(); // Re-run download/update
            }
        }
    }

    pub fn is_dashboard_avaliable(&self) -> bool {
        Path::new("config/site").exists()
    }

    pub fn install_nlu(&self) {
        todo!()
    }

    pub fn create_config_folder(
        &self,
        language: String,
        device_type: String,
        net_topology: String,
        gateway: bool,
        nlu_host: String,
        nlu_port: usize,
    ) {
        //Create config, skills
        let folders = ["config", "lang", "skills"];

        let defaults = HashMap::from([
            (
                "config/const.config",
                include_str!("../config/config/const.config"),
            ),
            (
                "config/const.config",
                include_str!("../config/config/settings.config"),
            ),
            ("lang/en.lang", include_str!("../config/lang/en.lang")),
            ("lang/pt.lang", include_str!("../config/lang/pt.lang")),
        ]);

        let _ = fs::create_dir_all(&self.config_path);

        for folder in folders {
            let _ = fs::create_dir(self.config_path.join(folder));
        }

        for (path, content) in &defaults {
            let mut file = match File::create(self.config_path.join(path)) {
                Ok(f) => f,
                Err(e) => {
                    error!("Error criating file: {}", e);
                    continue;
                }
            };

            let _ = file.write_all(content.as_bytes());
        }
    }
}
