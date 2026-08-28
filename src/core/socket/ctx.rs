use crate::core::{
    application::id::ApplicationId,
    node::id::NodeId,
    socket::id::SocketId,
    util::{address::Port, time::SimTime},
};

pub struct SocketCtx {
    pub node: NodeId,
    pub id: SocketId,
    pub application: ApplicationId,
    pub now: SimTime,
    pub port: Port,
}
