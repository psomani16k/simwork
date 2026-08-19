use crate::core::util::id::IdGenerator;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeId(u64);

impl IdGenerator {
    pub fn new_node_id(&mut self) -> NodeId {
        let id = self.get_id();
        NodeId(id)
    }
}
