#[macro_export]
macro_rules! speak {
    (locale: $a: expr) => {
        match $crate::dialogue::languages::locale($a) {
            Some(v) => speak!(&v),
            None => {
                ::log::warn!("Attempted to speak missing locale key: {}", $a);
            }
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
            Ok(c) => {
                ::log::trace!("Publishing to topic: {}", $topic);
                match c.device.publish($topic, $data).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let err_msg = format!("Error publishing to {}: {}", $topic, e.to_string());
                        ::log::error!("{}", err_msg);
                        Err(err_msg)
                    }
                }
            }
            Err(e) => {
                ::log::error!("Failed to publish: runtime not available: {}", e);
                Err(e)
            }
        }
    };
}

#[macro_export]
macro_rules! subscribe {
    ($topic: expr, async: $body:expr) => {
        match $crate::ctx::runtime() {
            Ok(runtime) => {
                let topic = $topic.to_string();
                ::log::trace!("Subscribing async to topic: {}", topic);
                let _ = runtime.device.subscribe_async(&topic, $body).await.map_err(|e| {
                    let err_msg = format!("Error subscribing async to {}: {}", topic, e.to_string());
                    ::log::error!("{}", err_msg);
                    err_msg
                }).map(|_| {
                    ::log::info!("Subscribed async to {}", topic);
                });
            },
            Err(e) => {
                let err_msg = format!("Error subscribing async (runtime error): {}", e.to_string());
                ::log::error!("{}", err_msg);
            },
        }
    };
    ($topic:expr, $body:expr) => {
        match $crate::ctx::runtime() {
            Ok(runtime) => {
                let topic = $topic.to_string();
                ::log::trace!("Subscribing to topic: {}", topic);
                let _ = runtime.device.subscribe(&topic, $body).await.map_err(|e| {
                    let err_msg = format!("Error subscribing to {}: {}", topic, e.to_string());
                    ::log::error!("{}", err_msg);
                    err_msg
                }).map(|_| {
                    ::log::info!("Subscribed to {}", topic);
                });
            },
            Err(e) => {
                let err_msg = format!("Error subscribing (runtime error): {}", e.to_string());
                ::log::error!("{}", err_msg);
            },
        }
    };
    ($topic:expr, captures: [$($cap:ident),*], async: |$from:ident, $top:ident, $data:ident| $body:block) => {{
        $(let $cap = $cap.clone();)*

        match $crate::ctx::runtime() {
            Ok(runtime) => {
                ::log::trace!("Subscribing async (with captures) to topic: {}", $topic);
                let result = runtime.device.subscribe_async($topic, move |$from, $top, $data| {
                    $(let $cap = $cap.clone();)*

                    async move {
                        $body
                    }
                }).await;

                match result {
                    Ok(_) => {
                        ::log::info!("Subscribed async (with captures) to {}", $topic);
                },
                    Err(e) => {
                        let err_msg = format!("Error subscribing async (with captures) to {}: {}", $topic, e.to_string());
                        ::log::error!("{}", err_msg);
                    }
                }
            },
            Err(e) => {
                let err_msg = format!("Error subscribing (runtime error): {}", e.to_string());
                ::log::error!("{}", err_msg);
            },
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
            Ok(c) => {
                ::log::trace!("Setting device context: {}={:?}", $key, $value);
                match c.device.update_ctx($key, json!($value)).await {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        let err_msg = format!("Error setting device ctx {}: {}", $key, e.to_string());
                        ::log::error!("{}", err_msg);
                        Err(err_msg)
                    }
                }
            },
            Err(e) => {
                ::log::error!("Failed to set device context: runtime not available: {}", e);
                Err(e)
            },
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
                Some(::std::time::Duration::from_secs_f64(ttl)),
                $persistent,
            ),
            Err(e) => ::log::error!("Failed to set context: runtime not available: {}", e),
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
            Err(e) => ::log::error!("Failed to set context: runtime not available: {}", e),
        }
    };
    (skill: $skill:expr, $key:expr, $value:expr, $ttl:expr, $persistent:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.context.set(
                $crate::context::ContextScope::Skill($skill),
                $key.to_string(),
                serde_json::json!($value),
                Some(::std::time::Duration::from_secs($ttl)),
                $persistent,
            ),
            Err(e) => ::log::error!("Failed to set context: runtime not available: {}", e),
        }
    };
}

#[macro_export]
macro_rules! get_ctx {
    ($key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c.context.get(&$crate::context::ContextScope::Global, $key),
            Err(e) => {
                ::log::error!("Failed to get context: runtime not available: {}", e);
                None
            }
        }
    };
    (device, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => {
                ::log::trace!("Getting device context: {}", $key);
                match c.device.get_ctx($key).await {
                    Ok(v) => Some(v),
                    Err(e) => {
                        ::log::warn!("Error getting device context {}: {}", $key, e.to_string());
                        None
                    }
                }
            }
            Err(e) => {
                ::log::error!("Failed to get device context: runtime not available: {}", e);
                None
            }
        }
    };
    (skill: $name:expr, $key:expr) => {
        match $crate::ctx::runtime() {
            Ok(c) => c
                .context
                .get(&$crate::context::ContextScope::Skill($name), $key),
            Err(e) => {
                ::log::error!("Failed to get skill context: runtime not available: {}", e);
                None
            }
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
macro_rules! register_action {
    ($action_type:ty, { $($field:ident: $value:expr),* $(,)? }) => {{
        ::log::info!("Registering action: {}", stringify!($action_type));
        type Config = <$action_type as $crate::actions::action::Action>::Config;
        match <$action_type>::new(Config {
            $($field: $value),*
        }) {
            Ok(mut action) => {
                ::log::debug!("Action {} initialized, registering...", stringify!($action_type));
                action.register().await;
                ::log::info!("Action {} registered successfully", stringify!($action_type));
            },
            Err(e) => {
                ::log::error!("Failed to initialize action {}: {}", stringify!($action_type), e);
            }
        }
    }};

    ($action_type:ty, if: $condition:expr, { $($field:ident: $value:expr),* $(,)? }) => {{
        if $condition {
            register_action!($action_type, { $($field: $value),* });
        } else {
            ::log::info!("Ignoring action: {}", stringify!($action_type));
        }
    }};

    ($action_type:ty) => {{
        register_action!($action_type, {})
    }};

    ($action_type:ty, if: $condition:expr) => {{
        if $condition {
            register_action!($action_type, {});
        } else {
            ::log::info!("Ignoring action: {}", stringify!($action_type));
        }
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

        ::log::info!("Starting directory watcher for: {}", path);
        std::thread::spawn(move || {
            let (tx, rx) = channel();
            let mut debouncer = match new_debouncer($duration, tx) {
                Ok(d) => d,
                Err(e) => {
                    ::log::error!("Failed to create debouncer for {}: {}", path, e);
                    return;
                }
            };

            if let Err(e) = debouncer.watcher().watch(std::path::Path::new(&path), RecursiveMode::Recursive) {
                ::log::error!("Failed to watch path {}: {}", path, e);
                return;
            }

            ::log::debug!("Watcher active for {}", path);

            for result in rx {
                match result {
                    Ok(events) => {
                        for $event in events {
                            ::log::trace!("File event in {}: {:?}", path, $event);
                            handle.spawn(async move {
                                $action
                            });
                        }
                    },
                    Err(e) => ::log::warn!("Watcher error in {}: {:?}", path, e),
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
                Err(e) => ::log::warn!("Watch error: {:?}", e),
            }
        }
    };
}
