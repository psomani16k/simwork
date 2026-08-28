use crate::core::{
    channel::interface::ChannelImpl,
    device::id::DeviceId,
    event::{channel_events::ChannelOutput, device_events::ChannelToDevice},
    sim::ctx::SimCtx,
    util::{bandwidth::Bandwidth, duration::Duration, packet::Packet, size::SizeOf, time::SimTime},
};

pub struct P2PDuplexChannel {
    head: DeviceId,
    head_busy_till: SimTime,
    tail: DeviceId,
    tail_busy_till: SimTime,

    inter_frame_delay: Duration,
    delay: Duration,
    band_width: Bandwidth,
}

impl ChannelImpl for P2PDuplexChannel {
    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        source: DeviceId,
        packet: Packet,
    ) -> Vec<(Duration, ChannelOutput)> {
        let packet_size = packet.size();
        let serialization_delay = packet_size / self.band_width;
        let mut events = Vec::new();
        let busy_duration = serialization_delay + self.inter_frame_delay;
        let total_delay = self.delay + serialization_delay;

        let dest;
        if source == self.head && self.head_busy_till <= ctx.now {
            dest = self.tail;
            self.head_busy_till = ctx.now + busy_duration;
        } else if source == self.tail && self.tail_busy_till <= ctx.now {
            dest = self.head;
            self.tail_busy_till = ctx.now + busy_duration;
        } else {
            events.push((
                Duration::ZERO,
                ChannelOutput::ToDevice(source, ChannelToDevice::ChannelBusy),
            ));
            return events;
        }

        events.push((
            busy_duration,
            ChannelOutput::ToDevice(source, ChannelToDevice::TransmissionComplete),
        ));
        events.push((
            total_delay,
            ChannelOutput::ToDevice(dest, ChannelToDevice::Data(packet)),
        ));

        events
    }
}
