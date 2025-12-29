use std::sync::Arc;
use avi_device::device::AviDevice;
use tokio::sync::Mutex;
use crate::api::api::Api;
use crate::skills::manager::SkillManager;

pub trait Action {
    type Config;

    fn new(device: &Arc<AviDevice>, config: Self::Config) -> Self;
    async fn register(&mut self);
}