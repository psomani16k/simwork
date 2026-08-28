use crate::core::util::{id::IdGenerator, time::SimTime};

#[derive(Debug, Default)]
pub struct SimCtx {
    pub now: SimTime,
    pub id_generator: IdGenerator,
}
