use crate::Message;

pub trait MessageAdder<Level> {
    fn notify(&mut self, message: Message<Level>);
    fn with_notify(&mut self, message: Message<Level>) -> &mut Self {
        self.notify(message);
        self
    }
}
