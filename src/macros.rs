#[macro_export]
macro_rules! locale {
    ($key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.language_system.get_translation($key),
            Err(_) => None,
        }
    };
}

#[macro_export]
macro_rules! get_translation_list {
    ($key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.language_system.get_translation_list($key),
            Err(_) => Vec::new(),
        }
    };
}

#[macro_export]
macro_rules! speak {
    (locale: $a: expr) => {
        match locale!($a) {
            Some(v) => speak!(&v),
            None => (),
        }
    };
    ($a: expr) => {
        $crate::dialogue::utils::speak($a, false)
    };
}

#[macro_export]
macro_rules! publish {
    ($topic: expr) => {
        $crate::publish!($topic, Vec::new())
    };
    ($topic: expr, $data: expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => match c.device.publish($topic, $data).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error publishing: {}", e.to_string())),
            },
            Err(e) => Err(e),
        }
    };
}

#[macro_export]
macro_rules! subscribe {
    ($topic: expr, async: $body:expr) => {
        match $crate::ctx::runtime() {
            Ok(runtime) => match runtime.device.subscribe_async($topic, $body).await {
                Ok(_) => {
                    ::log::info!("Subscribed to {}", $topic);
                    Ok(())
                },
                Err(e) => Err(format!("Error subscribing: {}", e.to_string())),
            },
            Err(e) => Err(format!("Error subscribing: {}", e.to_string())),
        }
    };
    ($topic:expr, $body:expr) => {
        match $crate::ctx::runtime() {
            Ok(runtime) => match runtime.device.subscribe($topic, $body).await {
                Ok(_) => {
                    ::log::info!("Subscribed to {}", $topic);
                    Ok(())
                },
                Err(e) => Err(format!("Error subscribing: {}", e.to_string())),
            },
            Err(e) => Err(format!("Error subscribing: {}", e.to_string())),
        }
    };
    ($topic:expr, captures: [$($cap:ident),*], async: |$from:ident, $top:ident, $data:ident| $body:block) => {{
        $(let $cap = $cap.clone();)*

        match $crate::ctx::runtime() {
            Ok(runtime) => {
                let result = runtime.device.subscribe_async($topic, move |$from, $top, $data| {
                    $(let $cap = $cap.clone();)*

                    async move {
                        $body
                    }
                }).await;

                match result {
                    Ok(_) => {
                        ::log::info!("Subscribed to {}", $topic);
                        Ok(())
                },
                    Err(e) => Err(format!("Error subscribing: {}", e.to_string())),
                }
            },
            Err(e) => Err(format!("Error subscribing: {}", e.to_string())),
        }
    }};
}

#[macro_export]
macro_rules! set_ctx {
    ($key:expr, $value:expr) => {
        $crate::set_ctx!($key, $value, persistent: false);
    };
    (device, $key:expr, $value:expr) => {
        match runtime() {
            Ok(c) => match c.device.update_ctx($key, json!($value)).await {
                Ok(v) => Ok(v),
                Err(e) => Err(format!("Error setting device ctx: {}", e.to_string())),
            },
            Err(e) => Err(e),
        }
    };
    ($key:expr, $value:expr, $ttl:expr) => {
        $crate::set_ctx!($key, $value, $ttl, false);
    };
    ($key:expr, $value:expr, $ttl:expr, persistent: $persistent:expr) => {
        $crate::set_ctx!($key, $value, $ttl, $persistent);
    };
    ($key:expr, $value:expr, $ttl:expr, $persistent:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.context.set(
                $crate::context::ContextScope::Global,
                $key.to_string(),
                serde_json::json!($value),
                Some(Duration::from_secs_f64(ttl)),
                $persistent,
            ),
            Err(_) => (),
        }
    };
    ($key:expr, $value:expr, persistent: $persistent:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.context.set(
                $crate::context::ContextScope::Global,
                $key.to_string(),
                serde_json::json!($value),
                None,
                $persistent,
            ),
            Err(_) => (),
        }
    };
}

#[macro_export]
macro_rules! get_ctx {
    ($key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.context.get(&$crate::context::ContextScope::Global, $key),
            Err(_) => None,
        }
    };
    (device, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => match c.device.get_ctx($key).await {
                Ok(v) => Some(v),
                Err(e) => {
                    ::log::warn!("Error getting context: {}", e.to_string());
                    None
                }
            },
            Err(_) => None,
        }
    };
    (skill: $name:expr, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c
                .context
                .get(&$crate::context::ContextScope::Skill($name), $key),
            Err(_) => None,
        }
    };
}

#[macro_export]
macro_rules! has_ctx {
    ($key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.context.has(&$crate::context::ContextScope::Global, $key),
            Err(_) => false,
        }
    };
    (device, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.devce.has_ctx($key),
            Err(_) => false,
        }
    };
    (skill: $name:expr, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c
                .context
                .has(&$crate::context::ContextScope::Skill($name), $key),
            Err(_) => false,
        }
    };
}

#[macro_export]
macro_rules! remove_ctx {
    ($key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => {
                c.context
                    .remove(&$crate::context::ContextScope::Global, $key);
                Ok(())
            }
            Err(e) => Err(e),
        }
    };
    (device, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => match c.device.delete_ctx($key).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error removing device ctx: {}", e.to_string())),
            },
            Err(e) => Err(e),
        }
    };
    (skill: $name:expr, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => {
                c.context
                    .remove(&$crate::context::ContextScope::Skill($name), $key);
                Ok(())
            }
            Err(e) => Err(e),
        }
    };
}

#[macro_export]
macro_rules! lang {
    () => {
        match $crate::ctx::runtime() {
            Ok(c) => c.lang.to_string(),
            Err(_) => "en".to_string(),
        }
    };
}

#[macro_export]
macro_rules! user_name {
    () => {
        match runtime() {
            Ok(c) => c.user.get_name().to_string(),
            Err(_) => "User".to_string(),
        }
    };
}

#[macro_export]
macro_rules! process_reply_text {
    ($text:expr) => {
        match runtime() {
            Ok(c) => c.reply_manager.process_text($text).await,
            Err(e) => Err(e),
        }
    };
}

#[macro_export]
macro_rules! rt_spawn {
    {$($b:tt)*} => {
        if let Ok(c) = runtime() {
            ::log::info!("Spawning Thread in current handler.");
            c.rt.spawn(async move {
                $($b)*
            });
        }
    };
}

#[macro_export]
macro_rules! core_id {
    () => {
        match $crate::ctx::runtime() {
            Ok(c) => match c.device.get_core_id().await {
                Ok(v) => Ok(v),
                Err(e) => Err(format!("Error getting core id: {}", e.to_string())),
            },
            Err(e) => Err(e),
        }
    };
}

#[macro_export]
macro_rules! get_user {
    () => {
        match get_ctx!("user") {
            Some(user) => match serde_json::from_value::<$crate::user::User>(user) {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            None => None,
        }
    };
    (device) => {
        match get_ctx!(device, "avi.user") {
            Some(user) => match serde_json::from_value::<$crate::user::User>(user) {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            None => None,
        }
    };
}

#[macro_export]
macro_rules! register_action {
    ($action_type:ty, { $($field:ident: $value:expr),* $(,)? }) => {{
        ::log::info!("Registering action '{:?}'", stringify!($action_type));
        type Config = <$action_type as $crate::actions::action:: Action>::Config;
        if let Ok(mut action) = <$action_type>::new(Config {
            $($field: $value),*
        }) {
            ::log::info!("Action {:?} registered!", stringify!($action_type));
            action.register().await;
        }
    }};

    ($action_type:ty) => {{
        register_action!($action_type, {})
    }};
}

#[macro_export]
macro_rules! watch_dir {
    ($path:expr, $duration:expr, async: |$event:ident| $action:block) => {{
        use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
        use std::sync::mpsc::channel;
        use tokio::runtime::Handle;

        let path = $path.to_string();
        let handle = Handle::current();

        ::log::info!("Watching directory: {}", path);
        std::thread::spawn(move || {
            let (tx, rx) = channel();
            let mut debouncer = new_debouncer($duration, tx).expect("Watcher fail");

            debouncer.watcher()
                .watch(std::path::Path::new(&path), RecursiveMode::Recursive)
                .expect("Path fail");

            for result in rx {
                if let Ok(events) = result {
                    for $event in events {
                        handle.spawn(async move {
                            $action
                        });
                    }
                }
            }
        });
    }};
    ($path:expr, $duration:expr, captures: [$($cap:ident),*], async: |$event:ident| $action:block) => {
        use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
        use std::sync::mpsc::channel;
        use tokio::runtime::Handle;

        let path = $path.to_string();
        let handle = Handle::current();

        $(let $cap = $cap.clone();)*

        ::log::info!("Watching directory: {}", path);
        std::thread::spawn(move || {
            let (tx, rx) = channel();
            let mut debouncer = new_debouncer($duration, tx).expect("Watcher fail");

            debouncer.watcher()
                .watch(std::path::Path::new(&path), RecursiveMode::Recursive)
                .expect("Path fail");

            for result in rx {
                if let Ok(events) = result {
                    for $event in events {
                        $(let $cap = $cap.clone();)*

                        handle.spawn(async move {
                            $action
                        });
                    }
                }
            }
        });
    };
    ($path:expr, $duration:expr, $callback:expr) => {
        use notify_debouncer_mini::{new_debouncer, notify::{RecursiveMode}};
        use std::sync::mpsc::channel;
        use std::path::Path;

        let (tx, rx) = channel();

        let mut debouncer = new_debouncer($duration, tx)
            .expect("Failed to create debouncer");

        debouncer.watcher()
            .watch(Path::new($path), RecursiveMode::Recursive)
            .expect("Failed to watch path");

        ::log::info!("Watching directory: {}", path);
        for result in rx {
            match result {
                Ok(events) => {
                    for event in events {
                        $callback(event);
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }
    };
}
