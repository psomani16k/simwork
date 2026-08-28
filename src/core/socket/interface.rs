use crate::core::{
    event::socket_events::SocketOutput,
    sim::ctx::SimCtx,
    util::{address::Endpoint, duration::Duration, packet::Packet},
};

pub trait SocketImpl {
    /// Called when the application wants to initiate a connection
    fn connect(&mut self, ctx: &SimCtx, endpoint: Endpoint) -> Vec<(Duration, SocketOutput)>;

    /// Called when application requests to send data to destination
    fn on_send(&mut self, ctx: &SimCtx, data: Vec<u8>) -> Vec<(Duration, SocketOutput)>;

    /// Called when there is a packet coming from destination
    fn on_receive(&mut self, ctx: &SimCtx, packet: Packet) -> Vec<(Duration, SocketOutput)>;

    /// Close socket
    fn close(&mut self, ctx: &SimCtx) -> Vec<(Duration, SocketOutput)>;
}
