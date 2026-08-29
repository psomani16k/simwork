use crate::core::{
    channel::interface::ChannelImpl,
    device::id::DeviceId,
    event::{channel_events::ChannelOutput, device_events::ChannelToDevice},
    sim::ctx::SimCtx,
    util::{
        bandwidth::Bandwidth,
        duration::Duration,
        packet::Packet,
        size::{Size, SizeOf},
        time::SimTime,
    },
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

impl P2PDuplexChannel {
    fn new(
        head: DeviceId,
        tail: DeviceId,
        delay: Duration,
        band_width: Bandwidth,
        inter_frame_data: Size,
    ) -> Self {
        let inter_frame_delay = inter_frame_data / band_width;
        Self {
            head,
            head_busy_till: SimTime::EPOCH,
            tail,
            tail_busy_till: SimTime::EPOCH,
            inter_frame_delay,
            delay,
            band_width,
        }
    }
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

pub struct P2PHalfDuplexChannel {
    head: DeviceId,
    tail: DeviceId,
    channel_busy_till: SimTime,

    inter_frame_delay: Duration,
    delay: Duration,
    band_width: Bandwidth,
}

impl P2PHalfDuplexChannel {
    fn new(
        head: DeviceId,
        tail: DeviceId,
        delay: Duration,
        band_width: Bandwidth,
        inter_frame_data: Size,
    ) -> Self {
        let inter_frame_delay = inter_frame_data / band_width;
        Self {
            head,
            tail,
            channel_busy_till: SimTime::EPOCH,
            inter_frame_delay,
            delay,
            band_width,
        }
    }
}

impl ChannelImpl for P2PHalfDuplexChannel {
    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        source: DeviceId,
        packet: Packet,
    ) -> Vec<(Duration, ChannelOutput)> {
        let mut events = Vec::new();
        if self.channel_busy_till > ctx.now {
            events.push((
                Duration::ZERO,
                ChannelOutput::ToDevice(source, ChannelToDevice::ChannelBusy),
            ));
        }
        let packet_size = packet.size();
        let serialization_delay = packet_size / self.band_width;
        let total_delay = self.delay + serialization_delay;
        let dest;
        if source == self.head {
            dest = self.tail;
        } else if source == self.tail {
            dest = self.head;
        } else {
            return events;
        }
        events.push((
            total_delay,
            ChannelOutput::ToDevice(dest, ChannelToDevice::Data(packet)),
        ));
        events.push((
            total_delay,
            ChannelOutput::ToDevice(source, ChannelToDevice::TransmissionComplete),
        ));
        self.channel_busy_till = ctx.now + total_delay;
        events
    }
}
