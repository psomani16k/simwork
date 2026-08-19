use crate::core::{
    application::id::ApplicationId, channel::id::ChannelId, device::id::DeviceId, node::id::NodeId,
    socket::id::SocketId,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Entity {
    Application(ApplicationId),
    Socket(SocketId),
    Node(NodeId),
    Device(DeviceId),
    Channel(ChannelId),
    Sim,
}
