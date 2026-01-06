#[macro_export]
macro_rules! locale {
    ($key:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c.language_system.get_translation($key),
            Err(_) => None,
        }
    };
}

#[macro_export]
macro_rules! get_translation_list {
    ($key:expr) => {
        match crate::ctx::runtime() {
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
        crate::dialogue::utils::speak($a, false)
    };
}

#[macro_export]
macro_rules! publish {
    ($topic: expr) => {
        publish!($topic, Vec::new())
    };
    ($topic: expr, $data: expr) => {
        match crate::ctx::runtime() {
            Ok(c) => match c.device.publish($topic, $data).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error publishing: {}", e.to_string())),
            },
            Err(e) => Err(e),
        }
    };
}

#[macro_export]
macro_rules! set_ctx {
    ($key:expr, $value:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c.context.set(
                crate::context::ContextScope::Global,
                $key.to_string(),
                serde_json::json!($value),
                None,
                false,
            ),
            Err(_) => (),
        }
    };
    ($key:expr, $value:expr, $ttl:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c.context.set(
                crate::context::ContextScope::Global,
                $key.to_string(),
                serde_json::json!($value),
                Some(Duration::from_secs_f64(ttl)),
                false,
            ),
            Err(_) => (),
        }
    };
    ($key:expr, $value:expr, $ttl:expr, persistent: $persistent:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c.context.set(
                crate::context::ContextScope::Global,
                $key.to_string(),
                serde_json::json!($value),
                Some(Duration::from_secs_f64(ttl)),
                $persistent,
            ),
            Err(_) => (),
        }
    };
    ($key:expr, $value:expr, $ttl:expr, $persistent:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c.context.set(
                crate::context::ContextScope::Global,
                $key.to_string(),
                serde_json::json!($value),
                Some(Duration::from_secs_f64(ttl)),
                $persistent,
            ),
            Err(_) => (),
        }
    };
    ($key:expr, $value:expr, persistent: $persistent:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c.context.set(
                crate::context::ContextScope::Global,
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
        match crate::ctx::runtime() {
            Ok(c) => c.context.get(&crate::context::ContextScope::Global, $key),
            Err(_) => None,
        }
    };
    (device, $key:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => match c.device.get_ctx($key).await {
                Ok(v) => Ok(v),
                Err(e) => Err(format!("Error getting context: {}", e.to_string())),
            },
            Err(e) => Err(e),
        }
    };
    (skill: $name:expr, $key:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c
                .context
                .get(&crate::context::ContextScope::Skill($name), $key),
            Err(_) => None,
        }
    };
}

#[macro_export]
macro_rules! has_ctx {
    ($key:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c.context.has(&crate::context::ContextScope::Global, $key),
            Err(_) => false,
        }
    };
    (skill: $name:expr, $key:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c
                .context
                .has(&crate::context::ContextScope::Skill($name), $key),
            Err(_) => false,
        }
    };
}

#[macro_export]
macro_rules! remove_ctx {
    ($key:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c
                .context
                .remove(&crate::context::ContextScope::Global, $key),
            Err(_) => (),
        }
    };
    (skill: $name:expr, $key:expr) => {
        match crate::ctx::runtime() {
            Ok(c) => c
                .context
                .remove(&crate::context::ContextScope::Skill($name), $key),
            Err(_) => (),
        }
    };
}

#[macro_export]
macro_rules! lang {
    () => {
        match crate::ctx::runtime() {
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
            c.rt.spawn(async move {
                $($b)*
            });
        }
    };
}