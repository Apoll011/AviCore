use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum FallbackType {
    NotUnderstood,
    NotInstalled,
    NotLoaded,
    NotEnabled,
    BadSkill,
    ErrorOnSkill,
    ErrorOnCore,
    ErrorOnNetwork,
    ErrorOnOther,
    ErrorOnUnknown,
    ErrorOnTimeout,
}

pub trait FallbackHandler: Send + Sync {
    fn fallback_type(&self) -> FallbackType;
    fn run(&self, message: &str, args: &[String]);
}

pub static FALLBACK_MANAGER: Lazy<Mutex<FallbackManager>> = Lazy::new(|| {
    Mutex::new(FallbackManager::new())
});

pub struct FallbackManager {
    handlers: HashMap<FallbackType, Arc<dyn FallbackHandler>>,
}

impl FallbackManager {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<H: FallbackHandler + 'static>(&mut self, handler: H) {
        self.handlers.insert(handler.fallback_type(), Arc::new(handler));
    }

    pub fn handle(&self, fallback_type: &FallbackType, message: &str, args: &[String]) {
        if let Some(handler) = self.handlers.get(fallback_type) {
            handler.run(message, args);
        } else {
            eprintln!("⚠️ No fallback registered for {:?}", fallback_type);
        }
    }
}

pub fn handle(fallback_type: &FallbackType, message: &str, args: &[String]) {
    FALLBACK_MANAGER.lock().unwrap().handle(fallback_type, message, args);
}


#[macro_export]
macro_rules! create_fallback {
    ($name:ident, $ty:expr, $body:block) => {
        pub struct $name;

        impl $crate::skills::fallback::FallbackHandler for $name {
            fn fallback_type(&self) -> $crate::skills::fallback::FallbackType {
                $ty
            }

            fn run(&self, _message: &str, _args: &[String]) {
                $body
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn register() {
                $crate::skills::fallback::FALLBACK_MANAGER
                    .lock()
                    .unwrap()
                    .register($name);
            }
        }
    };

    ($name:ident, $ty:expr, message = $msg:expr) => {
        pub struct $name;

        impl $crate::skills::fallback::FallbackHandler for $name {
            fn fallback_type(&self) -> $crate::skills::fallback::FallbackType {
                $ty
            }

            fn run(&self, message: &str, _args: &[String]) {
                println!("{}", $msg.replace("{}", message)); //TODO: Change to Say method
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn register() {
                $crate::skills::fallback::FALLBACK_MANAGER
                    .lock()
                    .unwrap()
                    .register($name);
            }
        }
    };
}

create_fallback!(NotUnderstood, FallbackType::NotUnderstood, message = "Sorry, I didn't understand '{}'");
create_fallback!(NetworkError, FallbackType::ErrorOnNetwork, {
    println!("NetworkError fallback triggered!");
});