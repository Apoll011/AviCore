use std::sync::Arc;
use avi_device::device::AviDevice;

pub trait Action {
    type Config;

    fn new(device: &Arc<AviDevice>, config: Self::Config) -> Self;
    async fn register(&mut self);
}