use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::skill_context::SkillContext;
use rhai::{AST, Dynamic, Engine, FuncArgs, ImmutableString, Scope, Variant};
use crate::skills::avi_script::package::AviScriptPackage;

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

    ast: AST,
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
    pub fn new(name: String, avi_script_package: &AviScriptPackage) -> Result<Self, Box<dyn std::error::Error>> {
        let context = SkillContext::new(&Self::skill_path(&name)?)?;
        let mut engine = create_avi_script_engine(avi_script_package)?;
        engine.set_default_tag(Dynamic::from(context.clone()));

        Ok(Self {
            pathname: Self::skill_path(&name)?,
            name: name.clone(),
            ast: Self::get_ast(&engine, &context.path, &context.info.entry)?,
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
        Ok(engine.compile_file(format!("{}/{}", path, entry).into())?)
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
        Ok(self.engine.run_ast_with_scope(&mut self.scope, &self.ast)?)
    }

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
        match self
            .engine
            .call_fn::<T>(&mut self.scope, &self.ast, function_name, args)
        {
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
}
