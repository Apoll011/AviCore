/// A trait defining the behavior of an action in the Avi system.
/// 
/// Actions are responsible for registering themselves and handling specific tasks
/// such as intent execution or dialogue management.
pub trait Action {
    /// The configuration type required to initialize the action.
    type Config;

    /// Creates a new instance of the action with the given configuration.
    fn new(config: Self::Config) -> Self;

    /// Registers the action, typically by subscribing to relevant device topics.
    async fn register(&mut self);
}