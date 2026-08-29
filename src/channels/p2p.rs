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
        if source != self.head && source != self.tail {
            return vec![];
        }

        let packet_size = packet.size();
        let serialization_delay = packet_size / self.band_width;
        let busy_duration = serialization_delay + self.inter_frame_delay;
        let total_delay = self.delay + serialization_delay;
        let mut events = Vec::new();

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

pub struct P2PHalfDuplexChannel {
    head: DeviceId,
    head_busy_till: SimTime,
    tail: DeviceId,
    tail_busy_till: SimTime,

    inter_frame_delay: Duration,
    delay: Duration,
    band_width: Bandwidth,
}

impl ChannelImpl for P2PHalfDuplexChannel {
    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        source: DeviceId,
        packet: Packet,
    ) -> Vec<(Duration, ChannelOutput)> {
        if source != self.head && source != self.tail {
            return vec![];
        }

        let packet_size = packet.size();
        let serialization_delay = packet_size / self.band_width;
        let busy_duration = serialization_delay + self.inter_frame_delay;
        let total_delay = self.delay + serialization_delay;
        let mut events = Vec::new();

        let dest;
        if source == self.head && self.head_busy_till <= ctx.now {
            dest = self.tail;
            self.head_busy_till = ctx.now + busy_duration;
            self.tail_busy_till = ctx.now + total_delay;
        } else if source == self.tail && self.tail_busy_till <= ctx.now {
            dest = self.head;
            self.tail_busy_till = ctx.now + busy_duration;
            self.head_busy_till = ctx.now + total_delay;
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

pub struct P2PSimplexChannel {
    head: DeviceId,
    tail: DeviceId,
    channel_busy_till: SimTime,

    inter_frame_delay: Duration,
    delay: Duration,
    band_width: Bandwidth,
}

impl ChannelImpl for P2PSimplexChannel {
    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        source: DeviceId,
        packet: Packet,
    ) -> Vec<(Duration, ChannelOutput)> {
        if source != self.head {
            return vec![];
        }
        let mut events = Vec::new();
        if self.channel_busy_till > ctx.now {
            events.push((
                Duration::ZERO,
                ChannelOutput::ToDevice(source, ChannelToDevice::ChannelBusy),
            ));
            return events;
        }
        let packet_size = packet.size();
        let serialization_delay = packet_size / self.band_width;
        let busy_duration = serialization_delay + self.inter_frame_delay;
        let total_delay = self.delay + serialization_delay;
        self.channel_busy_till = ctx.now + busy_duration;
        events.push((
            total_delay,
            ChannelOutput::ToDevice(self.tail, ChannelToDevice::Data(packet)),
        ));
        events.push((
            busy_duration,
            ChannelOutput::ToDevice(self.head, ChannelToDevice::TransmissionComplete),
        ));
        events
    }
}
