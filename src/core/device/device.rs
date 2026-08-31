use crate::core::{
    channel::id::ChannelId,
    device::{ctx::DeviceCtx, id::DeviceId, interface::DeviceImpl},
    event::{
        EventType,
        channel_events::ChannelEvent,
        device_events::{ChannelToDevice, DeviceEvent, DeviceOutput, DeviceToSelf, NodeToDevice},
        node_events::NodeEvent,
    },
    node::id::NodeId,
    sim::ctx::SimCtx,
    util::{address::MacAddress, duration::Duration, time::SimTime},
};

pub struct Device {
    id: DeviceId,
    node: NodeId,
    channel: ChannelId,
    mac_address: MacAddress,

    device_impl: Box<dyn DeviceImpl>,
}

impl Device {
    pub fn get_id(&self) -> DeviceId {
        self.id
    }

    pub fn handle_event(&mut self, ctx: &SimCtx, event: DeviceEvent) -> Vec<(SimTime, EventType)> {
        let event_data = match event {
            DeviceEvent::FromSelf(from_self) => self.handle_event_from_self(ctx, from_self),
            DeviceEvent::FromNode(from_node) => self.handle_event_from_node(ctx, from_node),
            DeviceEvent::FromChannel(from_channel) => {
                self.handle_event_from_channel(ctx, from_channel)
            }
        };

        event_data
            .into_iter()
            .map(|(delay, data)| {
                let event = match data {
                    DeviceOutput::ToSelf(device_to_self) => {
                        EventType::ToDevice(self.id, DeviceEvent::FromSelf(device_to_self))
                    }
                    DeviceOutput::ToNode(device_to_node) => {
                        EventType::ToNode(self.node, NodeEvent::FromDevice(device_to_node))
                    }
                    DeviceOutput::ToChannel(device_to_channel) => EventType::ToChannel(
                        self.channel,
                        ChannelEvent::FromDevice(self.id, device_to_channel),
                    ),
                };
                (ctx.now + delay, event)
            })
            .collect()
    }

    fn handle_event_from_self(
        &mut self,
        ctx: &SimCtx,
        event: DeviceToSelf,
    ) -> Vec<(Duration, DeviceOutput)> {
        match event {}
    }

    fn handle_event_from_node(
        &mut self,
        ctx: &SimCtx,
        event: NodeToDevice,
    ) -> Vec<(Duration, DeviceOutput)> {
        let ctx = DeviceCtx {
            sim_ctx: ctx,
            mac_addr: self.mac_address,
        };
        match event {
            NodeToDevice::Send(packet, protocol, dest_mac) => self
                .device_impl
                .on_packet_from_node(&ctx, packet, protocol, dest_mac),
        }
    }

    fn handle_event_from_channel(
        &mut self,
        ctx: &SimCtx,
        event: ChannelToDevice,
    ) -> Vec<(Duration, DeviceOutput)> {
        let ctx = DeviceCtx {
            sim_ctx: ctx,
            mac_addr: self.mac_address,
        };
        match event {
            ChannelToDevice::Data(packet) => self.device_impl.on_packet_from_channel(&ctx, packet),
            ChannelToDevice::ReadyToTransmit => self.device_impl.on_channel_free(&ctx),
            ChannelToDevice::ChannelBusy(returned_packet) => {
                self.device_impl.on_channel_busy(&ctx, returned_packet)
            }
        }
    }
}
