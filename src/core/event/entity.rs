use crate::core::{
    application::ApplicationId, channel::ChannelId, device::DeviceId, node::NodeId,
    socket::SocketId,
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
