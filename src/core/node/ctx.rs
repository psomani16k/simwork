use crate::core::util::time::SimTime;

pub struct NodeCtx {
    pub now: SimTime,
}

impl NodeCtx {
    pub fn new(now: SimTime) -> Self {
        NodeCtx { now }
    }
}
