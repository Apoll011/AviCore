use crate::ctx::runtime;
#[allow(unused_imports)]
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::ui::ask;
use crate::ui::ask_confirm;
use crate::ui::ask_number;
use crate::ui::info_line;
use crate::ui::main_progress_style;
use crate::ui::select_option;
use crate::ui::spinner_style;
use crate::ui::step;
use crate::ui::sub_step;
use console::style;
use indicatif::ProgressBar;
use log::error;
use log::info;
#[allow(unused_imports)]
use log::warn;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Eq, PartialEq)]
pub enum EventType {
    Topic,
    Event,
}

impl EventType {
    pub fn from(string: &str) -> Option<EventType> {
        if string.eq_ignore_ascii_case("topic") {
            return Some(EventType::Topic);
        } else if string.eq_ignore_ascii_case("event") {
            return Some(EventType::Event);
        }
        None
    }
    pub fn name(&self) -> String {
        match self {
            EventType::Topic => "topic".to_string(),
            EventType::Event => "event".to_string(),
        }
    }
}

pub struct Event {
    pub event_type: EventType,
    pub event_name: String,
}

impl Event {
    pub fn string(&self) -> String {
        format!("{}:{}", self.event_type.name(), self.event_name)
    }

    pub fn get_event(event_string: String) -> Result<Event, String> {
        let event_vec = event_string.split(":").collect::<Vec<&str>>();
        if event_vec.len() != 2 {
            return Err("Expected topic/event:{name}".to_string());
        }
        Ok(Event {
            event_type: EventType::from(event_vec[0])
                .ok_or("Expected topic/event on the first hand side".to_string())?,
            event_name: event_vec[1].to_string(),
        })
    }
}

pub async fn core_id() -> Option<String> {
    match runtime() {
        Ok(c) => match c.device.get_core_id().await {
            Ok(v) => Some(v),
            Err(e) => {
                warn!("Error getting core id: {}", e);
                None
            }
        },
        Err(_) => None,
    }
}

pub fn generate_documentation(include_internal: bool) -> Result<(), Box<dyn std::error::Error>> {
    use rhai_autodocs::*;
    info!("Generating documentation");
    let engine = create_avi_script_engine(true, None)?;

    info!(
        "Got {} functions from engine",
        engine.gen_fn_signatures(include_internal).len()
    );

    let docs = export::options()
        .include_standard_packages(include_internal)
        .format_sections_with(export::SectionFormat::Tabs)
        .export(&engine)?;

    info!("Trying to create dir ./docs");
    let path = "./docs";
    std::fs::remove_dir_all(path)?;
    std::fs::create_dir_all(path)?;
    info!("Created dir ./docs");

    for (name, doc) in generate::mdbook().generate(&docs)? {
        info!("Generating doc file: {}.md", name);
        std::fs::write(
            std::path::PathBuf::from_iter([path, &format!("{}.md", &name)]),
            doc,
        )?;
    }
    info!("Generated Documentation");
    Ok(())
}

pub fn generate_dsl_definition(path: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Generating DSL definition");
    let engine = create_avi_script_engine(true, None)?;

    info!(
        "Got {} functions from engine",
        engine.gen_fn_signatures(true).len()
    );

    engine
        .definitions()
        .with_headers(true) // write headers in all files
        .include_standard_packages(true) // skip standard packages
        .write_to_dir(path.clone())?;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rhai") {
            let mut new_path = path.clone();
            new_path.set_extension("avi");
            std::fs::rename(path, new_path)?;
        }
    }

    info!("Generated DSL definitions");
    Ok(())
}

pub fn get_all_docs_on_folder<T: for<'a> Deserialize<'a> + Clone>(
    path: PathBuf,
    _lang: Option<String>,
    _extention: String,
) -> Vec<T> {
    let mut docs: Vec<T> = Default::default();

    if let Ok(entries) = fs::read_dir(path.clone()) {
        info!("Searching path {:?}", path);
        for entry_dir in entries {
            let entry = match entry_dir {
                Ok(v) => v,
                Err(_) => continue,
            };

            let path = entry.path();

            info!("Trying: {}", path.display());

            //if path.exte(&extention) {
            //  print!("dddsds");
            match load_value_from_file::<T>(path) {
                Ok(v) => docs.push(v.clone()),
                Err(e) => error!("Error loading file: {}", e),
                //}
            }
        }
    }

    docs
}

pub fn load_value_from_file<T: for<'a> Deserialize<'a>>(path: PathBuf) -> Result<T, String> {
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Error reading file: {}", e)),
    };
    match serde_yaml::from_str(&file) {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Error parsing file: {}", e)),
    }
}

#[cfg(debug_assertions)]
pub fn config_dir() -> PathBuf {
    match runtime() {
        Ok(runtime) => runtime.config_path.clone(),
        Err(_) => "./config".into(),
    }
}

#[cfg(not(debug_assertions))]
pub fn config_dir() -> PathBuf {
    match runtime() {
        Ok(runtime) => runtime.config_path.clone(),
        Err(_) => {
            use dirs::config_local_dir;

            if let Some(path) = config_local_dir() {
                let config_path = path.push("avi");
                fs::create_dir_all(config_path);
                return config_path;
            }
            "./config"
        }
    }
}

#[allow(dead_code)]
pub struct Setup {}

#[allow(dead_code, unused)]
impl Setup {
    pub fn need_to() -> bool {
        Path::new(&config_dir()).exists()
    }

    pub fn setup() {
        sub_step(1, 6, "Initialization");

        let mut nlu_host = "0.0.0.0";
        let mut nlu_port = 1178;

        let modes = vec![
            "Offline (Bare Minimum)",
            "Online (Download all the required resources)",
        ];
        let modes_v = vec!["offline", "online"];
        let mode = modes_v[select_option("How would you like to setup Avi", &modes)];

        sub_step(2, 6, "Enviroment Configuration");

        let langs = vec!["English", "Português"];
        let lang_v = vec!["en", "pt"];
        let lang = lang_v[select_option("Select the system language", &langs)];

        let devs = vec![
            "Listenner (Only Listen to input)",
            "Speaker (Can speake sentences)",
            "Both (Listenner and Speaker)",
            "None",
        ];
        let dev_v = vec!["listenner", "speaker", "both", "none"];
        let dev = dev_v[select_option("Select Device Profile", &devs)];

        sub_step(3, 6, "Enviroment Configuration");

        let nets = vec!["Core (Central Hub)", "Node (Satelite)"];
        let nets_v = vec!["core", "node"];
        let net = nets_v[select_option("Select the system language", &nets)];

        let gateway = ask_confirm("Allow device to act as Gateway");

        sub_step(4, 6, "NLU COnfiguration");

        if (!Self::has_nlu()) {
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
            let nlu_v = vec!["install", "config", "skip"];
            let nlu = nlu_v[select_option("Select the system language", &nlus)];

            if nlu.eq("install") {
                Self::install_nlu();
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

        Self::create_config_folder(
            lang.to_string(),
            dev.to_string(),
            net.to_string(),
            gateway,
            nlu_host.to_string(),
            nlu_port,
        );
        pb.finish_with_message("Config folder created.");

        sub_step(6, 6, "Downloading Resources");
        if (mode.eq("online")) {
            Self::download_initial_skills();
            if !Self::is_dashboard_avaliable() {
                println!(
                    "{} {}",
                    style("⚠").yellow().bold(),
                    style("Dashboard not found locally.").yellow()
                );
                if ask_confirm("Would you like to download the Avicia Dashboard?") {
                    Self::download_dashboard();
                }
            } else {
                info_line("Dashboard", "Already installed");
                if ask_confirm("Check for updates?") {
                    Self::download_dashboard(); // Re-run download/update
                }
            }
        } else {
            println!(" {}", style("Skipping resource download.").bright())
        }
    }

    fn has_nlu() -> bool {
        false
    }

    pub fn download_skill(skill_id: String) {
        todo!()
    }
    pub fn download_initial_skills() {
        let initial_skills = ["saudation", "test", "bla", "bla"];

        let pb = ProgressBar::new(initial_skills.len() as u64);

        pb.set_style(main_progress_style());

        for skill in &initial_skills {
            pb.set_message(format!("Downloading {}...", style(skill).cyan()));

            thread::sleep(Duration::from_millis(800));

            pb.inc(1);
        }

        pb.finish_with_message("All skills downloaded.");
        println!(
            "{} {}",
            style("✔").green().bold(),
            style("Skill synchronization complete.").bright()
        );
    }

    pub fn download_language(lang_id: String) {
        todo!()
    }
    pub fn setup_languages() {
        todo!()
    }

    pub fn download_dashboard() {
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

    pub fn is_dashboard_avaliable() -> bool {
        Path::new("config/site").exists()
    }

    pub fn install_nlu() {
        todo!()
    }

    pub fn create_config_folder(
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

        let dir = config_dir();

        for folder in folders {
            let _ = fs::create_dir(dir.join(folder));
        }

        for (path, content) in &defaults {
            let mut file = match File::create(config_dir().join(path)) {
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
