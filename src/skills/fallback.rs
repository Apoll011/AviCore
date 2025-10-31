
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

pub struct FallbackEntry {
    pub ty: FallbackType,
    pub run: fn(&str, &[String]),
}

inventory::collect!(FallbackEntry);

pub fn handle(fallback_type: &FallbackType, message: Option<&str>, args: Option<&[String]>) {
    let msg = message.unwrap_or("");
    let args_ref = args.unwrap_or(&[]);

    // Find a registered fallback whose type matches and run it
    if let Some(entry) = inventory::iter::<FallbackEntry>
        .into_iter()
        .find(|e| &e.ty == fallback_type)
    {
        (entry.run)(msg, args_ref);
    } else {
        eprintln!("⚠️ No fallback implemented for {:?}", fallback_type);
    }
}

#[macro_export]
macro_rules! handle {
    ($ft:expr, $msg:expr, $args:expr) => {
        $crate::skills::fallback::handle($ft, Some($msg), Some($args))
    };

    ($ft:expr, $msg:expr) => {
        $crate::skills::fallback::handle($ft, Some($msg), None)
    };

    ($ft:expr) => {
        $crate::skills::fallback::handle($ft, None, None)
    };
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
            #[allow(non_snake_case)]
            pub fn __run(message: &str, args: &[String]) {
                // Create a temporary instance since the handler is ZST
                let tmp = $name;
                tmp.run(message, args);
            }
        }

        ::inventory::submit! {
            $crate::skills::fallback::FallbackEntry {
                ty: $ty,
                run: $name::__run,
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
            #[allow(non_snake_case)]
            pub fn __run(message: &str, args: &[String]) {
                let tmp = $name;
                tmp.run(message, args);
            }
        }

        ::inventory::submit! {
            $crate::skills::fallback::FallbackEntry {
                ty: $ty,
                run: $name::__run,
            }
        }
    };
}

create_fallback!(NotUnderstood, FallbackType::NotUnderstood, message = "Sorry, I didn't understand '{}'");
create_fallback!(NetworkError, FallbackType::ErrorOnNetwork, {
    println!("NetworkError fallback triggered!");
});
create_fallback!(NotInstalled, FallbackType::NotInstalled, message = "Sorry, But I don't know how to answer that yet.");