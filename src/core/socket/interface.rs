use crate::core::{
    address::Endpoint, event::socket_events::SocketOutput, packet::Packet, socket::ctx::SocketCtx,
    util::duration::Duration,
};

pub trait SocketImpl {
    /// Called when the application wants to initiate a connection
    fn connect(&mut self, ctx: SocketCtx, endpoint: Endpoint) -> Vec<(Duration, SocketOutput)>;

    /// Called when application requests to send data to destination
    fn on_send(&mut self, ctx: SocketCtx, data: Vec<u8>) -> Vec<(Duration, SocketOutput)>;

    /// Called when there is a packet coming from destination
    fn on_receive(&mut self, ctx: SocketCtx, packet: Packet) -> Vec<(Duration, SocketOutput)>;

    /// Close socket
    fn close(&mut self, ctx: SocketCtx) -> Vec<(Duration, SocketOutput)>;
}
