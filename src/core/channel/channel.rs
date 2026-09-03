use std::collections::HashMap;

use crate::core::{
    channel::{id::ChannelId, interface::ChannelImpl},
    device::id::DeviceId,
    event::{
        EventType,
        channel_events::{ChannelEvent, ChannelOutput, ChannelToSelf, DeviceToChannel},
        device_events::DeviceEvent,
    },
    sim::ctx::SimCtx,
    util::{address::MacAddress, duration::Duration, time::SimTime},
};

pub struct Channel {
    id: ChannelId,
    devices: HashMap<MacAddress, DeviceId>,

    channel_impl: Box<dyn ChannelImpl>,
}

impl Channel {
    pub fn get_id(&self) -> ChannelId {
        self.id
    }

    pub fn handle_event(&mut self, ctx: &SimCtx, event: ChannelEvent) -> Vec<(SimTime, EventType)> {
        let event_data = match event {
            ChannelEvent::FromSelf(from_self) => self.handle_event_from_self(ctx, from_self),
            ChannelEvent::FromDevice(source, from_device) => {
                self.handle_event_from_device(ctx, source, from_device)
            }
        };

        event_data
            .into_iter()
            .filter_map(|(delay, data)| {
                let event = match data {
                    ChannelOutput::ToSelf(channel_to_self) => {
                        EventType::ToChannel(self.id, ChannelEvent::FromSelf(channel_to_self))
                    }
                    ChannelOutput::ToDevice(mac, channel_to_device) => {
                        let device_id = *self.devices.get(&mac)?;
                        EventType::ToDevice(device_id, DeviceEvent::FromChannel(channel_to_device))
                    }
                };
                Some((ctx.now + delay, event))
            })
            .collect()
    }

    fn handle_event_from_self(
        &mut self,
        ctx: &SimCtx,
        event: ChannelToSelf,
    ) -> Vec<(Duration, ChannelOutput)> {
        match event {}
    }

    fn handle_event_from_device(
        &mut self,
        ctx: &SimCtx,
        source: MacAddress,
        event: DeviceToChannel,
    ) -> Vec<(Duration, ChannelOutput)> {
        match event {
            DeviceToChannel::Send(packet) => {
                self.channel_impl.on_packet_from_device(ctx, source, packet)
            }
        }
    }
}
