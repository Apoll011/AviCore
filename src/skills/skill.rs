use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::helpers::fix_module_imports;
use crate::skills::skill_context::SkillContext;
use rhai::{AST, Dynamic, Engine, FuncArgs, ImmutableString, Scope, Variant};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};

/// Represents a standalone skill that can be executed by the Avi system.
///
/// A skill consists of a Dyon module, a runtime environment, and a configuration context.
pub struct Skill {
    /// The filesystem path to the skill.
    #[allow(dead_code)]
    pathname: String,
    /// The name of the skill.
    #[allow(dead_code)]
    name: String,
    engine: Engine,

    ast: Arc<RwLock<AST>>,
    /// The Dyon runtime used to execute the skill.
    scope: Scope<'static>,
    /// The configuration and state of the skill.
    context: SkillContext,
}

impl Skill {
    /// Creates a new `Skill` instance by loading its configuration and logic from the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the skill directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the skill context or module fails to load.
    pub fn new(name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let context = SkillContext::new(&Self::skill_path(&name)?)?;
        let mut engine = create_avi_script_engine(false)?;
        engine.set_default_tag(Dynamic::from(context.clone()));

        Ok(Self {
            pathname: Self::skill_path(&name)?,
            name: name.clone(),
            ast: Arc::new(RwLock::new(Self::get_ast(
                &engine,
                &context.path,
                &context.info.entry,
            )?)),
            scope: Self::create_scope(),
            context,
            engine,
        })
    }

    /// Constructs the path to a skill's directory.
    ///
    fn skill_path(name: &str) -> Result<String, String> {
        match runtime() {
            Ok(c) => Ok(format!("{}/{}", c.skill_path, name)),
            Err(e) => Err(e),
        }
    }

    fn get_ast(
        engine: &Engine,
        path: &str,
        entry: &str,
    ) -> Result<AST, Box<dyn std::error::Error>> {
        let path = &*format!("{}/{}", path, entry);
        match Self::read_file(path) {
            Ok(raw_script) => Ok(engine.compile(fix_module_imports(raw_script)?)?),
            Err(e) => Err(Box::from(e)),
        }
    }

    fn read_file(path: &str) -> Result<String, String> {
        let mut f = File::open(path)
            .map_err(|_err| format!("Cannot open script file '{}'", path.to_string()))?;

        let mut contents = String::new();

        f.read_to_string(&mut contents)
            .map_err(|_err| format!("Cannot read script file '{}'", path.to_string()))?;

        if contents.starts_with("#!") {
            match contents.find('\n') {
                Some(n) => {
                    contents.drain(0..n).count();
                }
                None => contents.clear(),
            }
        };

        Ok(contents)
    }

    /// Initializes a Dyon runtime for the skill.
    fn create_scope() -> Scope<'static> {
        Scope::new()
    }

    /// Starts the skill by running its main module.
    ///
    /// # Errors
    ///
    /// Returns an error if the skill is disabled or if the runtime fails.
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.disabled() {
            return Err("Skill is disabled".into());
        }

        self.run()
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.engine.run_ast_with_scope(
            &mut self.scope,
            &*self.ast.read().map_err(|e| e.to_string())?,
        )?)
    }

    #[allow(dead_code)]
    pub(crate) fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.scope.push_constant("END", true);
        self.run()
    }

    /// Formats an intent name into a Dyon-compatible function name.
    pub fn format_intent_name(name: String) -> String {
        name.split("@").collect::<Vec<&str>>()[1].replace(".", "_")
    }

    /// Executes a specific intent within the skill's Dyon module.
    ///
    /// # Arguments
    ///
    /// * `intent` - The intent to be executed.
    ///
    /// # Errors
    ///
    /// Returns an error if the intent name is missing or if the corresponding Dyon function cannot be found.
    pub fn run_intent(&mut self, intent: Intent) -> Result<bool, Box<dyn std::error::Error>> {
        match intent.intent.clone() {
            Some(v) => {
                if let Some(intent_name) = v.intent_name {
                    println!("{}", intent_name);
                    self.scope.push_constant(
                        "INTENT_NAME",
                        ImmutableString::from(&Self::format_intent_name(intent_name)),
                    );
                    self.scope.push_constant("INTENT", intent.clone());
                } else {
                    return Err("Intent name is not defined".into());
                }
            }
            None => return Err("Intent is not defined".into()),
        }

        self.run()?;
        let _ = self.scope.remove::<ImmutableString>("INTENT");
        let _ = self.scope.remove::<Intent>("INTENT_NAME");
        Ok(true)
    }

    pub fn run_function<T: Variant + Clone>(
        &mut self,
        function_name: &str,
        args: impl FuncArgs,
    ) -> Result<T, Box<dyn std::error::Error>> {
        match self.engine.call_fn::<T>(
            &mut self.scope,
            &*self.ast.read().map_err(|e| e.to_string())?,
            function_name,
            args,
        ) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    /// Checks if the skill is currently disabled.
    fn disabled(&self) -> bool {
        self.context.info.disabled
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.context = SkillContext::new(&self.pathname)?;
        let new_ast = Self::get_ast(&self.engine, &self.context.path, &self.context.info.entry)?;
        *self.ast.write().map_err(|e| e.to_string())? = new_ast;
        Ok(())
    }
}
