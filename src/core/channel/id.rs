use crate::core::util::id::IdGenerator;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ChannelId(u64);

impl IdGenerator {
    pub fn new_channel_id(&mut self) -> ChannelId {
        let id = self.get_id();
        ChannelId(id)
    }
}
