use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::helpers::fix_module_imports;
use crate::skills::skill_context::SkillContext;
use crate::utils::{Event, EventType};
use crate::{rt_spawn, subscribe};
use log::error;
use memory_size_derive::{DeepSize, DeepSizeTree};
use rhai::{AST, Dynamic, Engine, FnPtr, FuncArgs, ImmutableString, Scope, Variant};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Represents a standalone skill that can be executed by the Avi system.
///
/// A skill consists of a Rhai module, a runtime environment, and a configuration context.
#[derive(Clone, DeepSize, DeepSizeTree)]
#[allow(dead_code)]
pub struct Skill {
    /// The filesystem path to the skill.
    pathname: Arc<str>,
    /// The name of the skill.
    name: Arc<str>,
    #[deep_size(opaque)]
    /// The Rhai engine instance
    engine: Arc<Engine>,
    #[deep_size(opaque)]
    /// Compiled AST (shared for thread safety)
    ast: Arc<RwLock<AST>>,
    #[deep_size(opaque)]
    /// The Rhai scope used to execute the skill.
    scope: Arc<RwLock<Scope<'static>>>,
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
        Arc::<Engine>::get_mut(&mut engine)
            .ok_or("Failed to get mutable engine")?
            .set_default_tag(Dynamic::from(context.clone()));

        let ast = Arc::new(RwLock::new(Self::compile_ast(
            &engine,
            &context.path,
            &context.info.entry,
        )?));

        Ok(Self {
            pathname: Arc::from(pathname),
            name: Arc::from(name),
            engine,
            ast,
            scope: Arc::new(RwLock::new(Self::create_scope())),
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
        let skill_path = Arc::clone(&self.pathname);
        let ast = Arc::clone(&self.ast);
        let scope = Arc::clone(&self.scope);
        let engine = Arc::clone(&self.engine);

        rt_spawn! {
            let _ = Self::subscribe_internal(subscriptions, skill_path, ast, scope, engine).await;
        }

        self.run()
    }

    #[allow(dead_code)]
    /// Subscribes to all events defined in the skill's configuration
    pub async fn subscribe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Self::subscribe_internal(
            self.context.info.subscription.clone(),
            Arc::clone(&self.pathname),
            Arc::clone(&self.ast),
            Arc::clone(&self.scope),
            Arc::clone(&self.engine),
        )
        .await
    }

    async fn subscribe_internal(
        subscriptions: Vec<String>,
        _skill_path: Arc<str>,
        ast: Arc<RwLock<AST>>,
        scope: Arc<RwLock<Scope<'static>>>,
        engine: Arc<Engine>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for subscription in subscriptions {
            let event = Event::get_event(subscription.to_string())?;

            if matches!(event.event_type, EventType::TOPIC) {
                // Clone Arc and create new engine instance for closure
                let ast_clone = Arc::clone(&ast);
                let scope_clone = Arc::clone(&scope);
                let engine_clone = Arc::clone(&engine);

                subscribe!(event.event_name.clone(), move |from, _topic, data| {
                    let mut scope_guard = match scope_clone.write() {
                        Ok(v) => v,
                        Err(_) => return,
                    };

                    // Prepare scope with event data
                    scope_guard.push_constant("EVENT_NAME", event.string());
                    scope_guard.push_constant("EVENT_DATA", data);
                    scope_guard.push_constant("EVENT_FROM", from.to_string());

                    // Execute the skill
                    if let Ok(ast_guard) = ast_clone.read() {
                        let _ = engine_clone.run_ast_with_scope(&mut scope_guard, &ast_guard);
                    }

                    // Clean up scope
                    let _ = scope_guard.remove::<ImmutableString>("EVENT_NAME");
                    let _ = scope_guard.remove::<Vec<u8>>("EVENT_DATA");
                    let _ = scope_guard.remove::<String>("EVENT_FROM");
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

        self.engine.run_ast_with_scope(
            &mut *self.scope.write().map_err(|e| e.to_string())?,
            &ast_guard,
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    /// Stops the skill execution
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.scope
            .write()
            .map_err(|e| e.to_string())?
            .push_constant("END", true);
        self.run()?;
        let _ = self
            .scope
            .write()
            .map_err(|e| e.to_string())?
            .remove::<bool>("END");
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
        let intent_name = intent
            .clone()
            .intent
            .ok_or("Intent is not defined")?
            .0
            .intent_name;

        let formatted_name = Self::format_intent_name(&intent_name);

        {
            let mut scope = self.scope.write().map_err(|e| e.to_string())?;
            scope.push_constant("INTENT_NAME", ImmutableString::from(formatted_name));
            scope.push_constant("INTENT", intent);
        }

        let result = self.run();

        // Clean up scope
        {
            let mut scope = self.scope.write().map_err(|e| e.to_string())?;
            let _ = scope.remove::<ImmutableString>("INTENT_NAME");
            let _ = scope.remove::<Intent>("INTENT");
        }
        match result {
            Ok(_) => {}
            Err(e) => error!("Error running the intent: {}", e),
        };
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

        Ok(self.engine.call_fn::<T>(
            &mut *self.scope.write().map_err(|e| e.to_string())?,
            &ast_guard,
            function_name,
            args,
        )?)
    }

    /// Calls a specific function within the skill
    pub fn run_function_ptr<T: Variant + Clone>(
        &mut self,
        function: FnPtr,
        args: impl FuncArgs,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let ast_guard = self
            .ast
            .read()
            .map_err(|e| format!("Failed to acquire AST lock: {}", e))?;

        Ok(function.call(&self.engine, &ast_guard, args)?)
    }

    /// Checks if the skill is currently disabled.
    pub fn is_disabled(&self) -> bool {
        self.context.info.disabled
    }

    /// Returns the skill name
    pub fn name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    /// Returns the skill pathname
    pub fn pathname(&self) -> PathBuf {
        (self.pathname.to_string()).into()
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
