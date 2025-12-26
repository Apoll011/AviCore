use std::sync::Arc;
use avi_device::device::AviDevice;
use tokio::sync::Mutex;
use crate::api::api::Api;
use crate::skills::manager::SkillManager;

pub trait Action {
    fn new(device: &Arc<AviDevice>, api: &Arc<Mutex<Api>>, skill_manager: &Arc<Mutex<SkillManager>>) -> Self;
    async fn register(&mut self);
}