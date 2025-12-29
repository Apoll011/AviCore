pub trait Action {
    type Config;

    fn new(config: Self::Config) -> Self;
    async fn register(&mut self);
}