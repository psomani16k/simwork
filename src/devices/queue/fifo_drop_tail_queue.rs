use std::collections::VecDeque;

use crate::{
    core::util::packet::{Packet, id::PacketId},
    devices::queue::PacketQueue,
};

use super::{PacketQueuePopError, PopReceipt};

pub struct FifoDropTailQueue {
    // id of the popped-but-unsettled packet; its slot stays reserved
    receipt: Option<PacketId>,
    queue: VecDeque<Packet>,
    capacity: usize,
}

impl FifoDropTailQueue {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity >= 1, "queue capacity must be at least 1");
        Self {
            receipt: None,
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
}

impl PacketQueue for FifoDropTailQueue {
    fn push(&mut self, packet: Packet) -> bool {
        if self.is_full() {
            return false;
        }
        self.queue.push_back(packet);
        true
    }

    fn pop(&mut self) -> Result<(Packet, PopReceipt), PacketQueuePopError> {
        if self.receipt.is_some() {
            return Err(PacketQueuePopError::UncommittedPacket);
        }
        let Some(packet) = self.queue.pop_front() else {
            return Err(PacketQueuePopError::QueueEmpty);
        };
        let receipt = PopReceipt::new(&packet);
        self.receipt = Some(receipt.id());
        Ok((packet, receipt))
    }

    fn commit(&mut self, receipt: PopReceipt) {
        assert_eq!(
            self.receipt,
            Some(receipt.id()),
            "receipt does not match this queue's outstanding pop"
        );
        self.receipt = None;
    }

    fn requeue(&mut self, receipt: PopReceipt, packet: Packet) {
        assert_eq!(
            self.receipt,
            Some(receipt.id()),
            "receipt does not match this queue's outstanding pop"
        );
        self.receipt = None;
        self.queue.push_front(packet);
    }

    fn len(&self) -> usize {
        self.queue.len() + self.receipt.is_some() as usize
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn is_full(&self) -> bool {
        self.len() >= self.capacity
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
