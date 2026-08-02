pub trait Event {
    fn cancel(&mut self);
    fn is_cancelled(&self) -> bool;
    fn notify(&self);
}
