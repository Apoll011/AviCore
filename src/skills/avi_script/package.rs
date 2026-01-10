use rhai::def_package;
use rhai::plugin::*;

def_package! {
    /// AviScript package - Custom scripting functionality for AviCore
    pub AviScriptPackage(module) {
        // Combine all your modules into the package
        combine_with_exported_module!(module, "log", crate::skills::avi_script::module::log::log_module);
        combine_with_exported_module!(module, "dialogue", crate::skills::avi_script::module::dialogue::dialogue_module);
        combine_with_exported_module!(module, "skill", crate::skills::avi_script::module::skill::skill_module);
        combine_with_exported_module!(module, "locale", crate::skills::avi_script::module::locale::locale_module);
        combine_with_exported_module!(module, "json", crate::skills::avi_script::module::json::json_module);
        combine_with_exported_module!(module, "constant", crate::skills::avi_script::module::constant::constant_module);
        combine_with_exported_module!(module, "settings", crate::skills::avi_script::module::settings::settings_module);
        combine_with_exported_module!(module, "context", crate::skills::avi_script::module::context::context_module);
        combine_with_exported_module!(module, "fs", crate::skills::avi_script::module::fs::fs_module);
        combine_with_exported_module!(module, "slots", crate::skills::avi_script::module::slots::slots_module);
    }
}
