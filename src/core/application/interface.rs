use crate::core::{
    application::ctx::ApplicationCtx,
    event::application_events::{ApplicationOutput, ConnectionStatus, SocketError},
    util::{duration::Duration, size::Size},
};

pub trait ApplicationImpl {
    /// called when the application is started
    fn on_start(&mut self, ctx: ApplicationCtx) -> Vec<(Duration, ApplicationOutput)>;

    /// Socket layer requesting for more data if available
    fn on_sendable(&mut self, ctx: ApplicationCtx, buf: &mut [u8]) -> (Duration, Size);

    /// Called to inform application of how many bytes were sent in the last send request
    fn send_callback(
        &mut self,
        ctx: ApplicationCtx,
        sent: Size,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// Erros thrown by socket arrive here
    fn on_socket_error(
        &mut self,
        ctx: ApplicationCtx,
        err: SocketError,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// Called when there is an update from the socket regarding the connection status
    fn on_connection_status_update(
        &mut self,
        ctx: ApplicationCtx,
        status: ConnectionStatus,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// Called when socket has data to give to the application
    fn on_receive(
        &mut self,
        ctx: ApplicationCtx,
        data: Vec<u8>,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// called when the application is to be stopped
    fn on_stop(&mut self, ctx: ApplicationCtx) -> Vec<(Duration, ApplicationOutput)>;
}
