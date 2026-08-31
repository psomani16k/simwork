use crate::core::{sim::ctx::SimCtx, util::address::MacAddress};

pub struct DeviceCtx<'a> {
    pub sim_ctx: &'a SimCtx,
    pub mac_addr: MacAddress,
}
