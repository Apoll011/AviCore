use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::helpers::fix_module_imports;
use crate::skills::skill_context::SkillContext;
use crate::utils::{Event, EventType};
use crate::{rt_spawn, subscribe};
use rhai::{Dynamic, Engine, FuncArgs, ImmutableString, Scope, Variant, AST};
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Represents a standalone skill that can be executed by the Avi system.
///
/// A skill consists of a Rhai module, a runtime environment, and a configuration context.
pub struct Skill {
    /// The filesystem path to the skill.
    pathname: String,
    /// The name of the skill.
    name: String,
    /// The Rhai engine instance
    engine: Engine,
    /// Compiled AST (shared for thread safety)
    ast: Arc<RwLock<AST>>,
    /// The Rhai scope used to execute the skill.
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
        let pathname = Self::skill_path(&name)?;
        let context = SkillContext::new(&pathname)?;

        let mut engine = create_avi_script_engine(false, Some(pathname.clone()))?;
        engine.set_default_tag(Dynamic::from(context.clone()));

        let ast = Arc::new(RwLock::new(Self::compile_ast(
            &engine,
            &context.path,
            &context.info.entry,
        )?));

        Ok(Self {
            pathname,
            name,
            engine,
            ast,
            scope: Self::create_scope(),
            context,
        })
    }

    /// Constructs the path to a skill's directory.
    fn skill_path(name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let config = runtime()?;
        Ok(format!("{}/{}", config.skill_path, name))
    }

    /// Compiles the AST from the skill's entry file
    fn compile_ast(
        engine: &Engine,
        path: &str,
        entry: &str,
    ) -> Result<AST, Box<dyn std::error::Error>> {
        let file_path = Path::new(path).join(entry);
        let raw_script = Self::read_file(&file_path)?;
        let processed_script = fix_module_imports(raw_script)?;
        Ok(engine.compile(processed_script)?)
    }

    /// Reads a script file from disk
    fn read_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut contents = fs::read_to_string(path)
            .map_err(|e| format!("Cannot read script file '{}': {}", path.display(), e))?;

        // Remove shebang if present
        if contents.starts_with("#!") {
            if let Some(newline_pos) = contents.find('\n') {
                contents.drain(0..=newline_pos);
            } else {
                contents.clear();
            }
        }

        Ok(contents)
    }

    /// Initializes a new Rhai scope for the skill.
    fn create_scope() -> Scope<'static> {
        Scope::new()
    }

    /// Starts the skill by running its main module and subscribing to events.
    ///
    /// # Errors
    ///
    /// Returns an error if the skill is disabled or if the runtime fails.
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_disabled() {
            return Err("Skill is disabled".into());
        }

        let subscriptions = self.context.info.subscription.clone();
        let skill_path = self.pathname.clone();
        let ast = Arc::clone(&self.ast);
        let scope = self.scope.clone();

        rt_spawn! {
            let _ = Self::subscribe_internal(subscriptions, skill_path, ast, scope).await;
        }

        self.run()
    }

    async fn subscribe_internal(
        subscriptions: Vec<String>,
        skill_path: String,
        ast: Arc<RwLock<AST>>,
        scope: Scope<'static>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for subscription in subscriptions {
            let event = Event::get_event(subscription.to_string())?;

            if matches!(event.event_type, EventType::TOPIC) {
                // Clone Arc and create new engine instance for closure
                let ast_clone = Arc::clone(&ast);
                let scope_clone = scope.clone();
                let path_clone = skill_path.clone();

                subscribe!(event.event_name.clone(), move |from, _topic, data| {
                    let mut scope_clone = scope_clone.clone();
                    let engine = match create_avi_script_engine(false, Some(path_clone.clone())) {
                        Ok(eng) => eng,
                        Err(_) => return, // Can't handle event without engine
                    };

                    // Prepare scope with event data
                    scope_clone.push_constant("EVENT_NAME", event.string());
                    scope_clone.push_constant("EVENT_DATA", data);
                    scope_clone.push_constant("EVENT_FROM", from.to_string());

                    // Execute the skill
                    if let Ok(ast_guard) = ast_clone.read() {
                        let _ = engine.run_ast_with_scope(&mut scope_clone, &*ast_guard);
                    }

                    // Clean up scope
                    let _ = scope_clone.remove::<ImmutableString>("EVENT_NAME");
                    let _ = scope_clone.remove::<Vec<u8>>("EVENT_DATA");
                    let _ = scope_clone.remove::<String>("EVENT_FROM");
                });
            }
        }

        Ok(())
    }

    /// Runs the skill's main execution
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let ast_guard = self
            .ast
            .read()
            .map_err(|e| format!("Failed to acquire AST lock: {}", e))?;

        self.engine
            .run_ast_with_scope(&mut self.scope, &*ast_guard)?;
        Ok(())
    }

    /// Stops the skill execution
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.scope.push_constant("END", true);
        self.run()?;
        let _ = self.scope.remove::<bool>("END");
        Ok(())
    }

    /// Formats an intent name into a Rhai-compatible function name.
    pub fn format_intent_name(name: &str) -> String {
        name.split('@').nth(1).unwrap_or(name).replace('.', "_")
    }

    /// Executes a specific intent within the skill's Rhai module.
    ///
    /// # Arguments
    ///
    /// * `intent` - The intent to be executed.
    ///
    /// # Errors
    ///
    /// Returns an error if the intent name is missing or if execution fails.
    pub fn run_intent(&mut self, intent: Intent) -> Result<bool, Box<dyn std::error::Error>> {
        let intent_data = intent.intent.as_ref().ok_or("Intent is not defined")?;

        let intent_name = intent_data
            .intent_name
            .as_ref()
            .ok_or("Intent name is not defined")?;

        let formatted_name = Self::format_intent_name(intent_name);

        self.scope
            .push_constant("INTENT_NAME", ImmutableString::from(formatted_name));
        self.scope.push_constant("INTENT", intent);

        let result = self.run();

        // Clean up scope
        let _ = self.scope.remove::<ImmutableString>("INTENT_NAME");
        let _ = self.scope.remove::<Intent>("INTENT");

        result?;
        Ok(true)
    }

    /// Calls a specific function within the skill
    pub fn run_function<T: Variant + Clone>(
        &mut self,
        function_name: &str,
        args: impl FuncArgs,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let ast_guard = self
            .ast
            .read()
            .map_err(|e| format!("Failed to acquire AST lock: {}", e))?;

        Ok(self
            .engine
            .call_fn::<T>(&mut self.scope, &*ast_guard, function_name, args)?)
    }

    /// Checks if the skill is currently disabled.
    pub fn is_disabled(&self) -> bool {
        self.context.info.disabled
    }

    /// Returns the skill name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the skill pathname
    pub fn pathname(&self) -> &str {
        &self.pathname
    }

    /// Reloads the skill from disk
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.context = SkillContext::new(&self.pathname)?;

        let new_ast =
            Self::compile_ast(&self.engine, &self.context.path, &self.context.info.entry)?;

        *self
            .ast
            .write()
            .map_err(|e| format!("Failed to acquire AST write lock: {}", e))? = new_ast;

        Ok(())
    }

    /// Returns the size of the Skill struct in bytes
    pub fn size_in_bytes(&self) -> usize {
        use std::mem::size_of_val;

        size_of_val(self)
    }
}

impl std::fmt::Debug for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Skill")
            .field("name", &self.name)
            .field("pathname", &self.pathname)
            .field("disabled", &self.is_disabled())
            .field("size_bytes", &self.size_in_bytes())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_intent_name() {
        assert_eq!(
            Skill::format_intent_name("skill@light.turn_on"),
            "light_turn_on"
        );
        assert_eq!(Skill::format_intent_name("skill@ask.time"), "ask_time");
    }
}
