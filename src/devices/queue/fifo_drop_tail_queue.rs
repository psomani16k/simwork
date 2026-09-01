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

#[cfg(test)]
mod tests {
    use super::FifoDropTailQueue;
    use crate::{
        core::util::{id::IdGenerator, packet::Packet},
        devices::queue::{PacketQueue, PacketQueuePopError},
    };

    /// A packet whose only distinguishing feature is its id.
    fn packet(ids: &IdGenerator) -> Packet {
        Packet::from_raw_data(b"payload".to_vec(), ids.new_packet_id())
    }

    #[test]
    fn new_queue_is_empty() {
        let queue = FifoDropTailQueue::new(4);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), 4);
        assert!(queue.is_empty());
        assert!(!queue.is_full());
    }

    #[test]
    #[should_panic(expected = "queue capacity must be at least 1")]
    fn zero_capacity_panics() {
        FifoDropTailQueue::new(0);
    }

    #[test]
    fn pops_in_push_order() {
        let ids = IdGenerator::new();
        let mut queue = FifoDropTailQueue::new(3);
        let pushed: Vec<_> = (0..3)
            .map(|_| {
                let packet = packet(&ids);
                let id = packet.id();
                assert!(queue.push(packet));
                id
            })
            .collect();

        for expected in pushed {
            let (popped, receipt) = queue.pop().expect("queue holds a packet");
            assert_eq!(popped.id(), expected);
            queue.commit(receipt);
        }
        assert!(queue.is_empty());
    }

    #[test]
    fn push_drops_the_tail_when_full() {
        let ids = IdGenerator::new();
        let mut queue = FifoDropTailQueue::new(2);
        let first = packet(&ids);
        let first_id = first.id();
        assert!(queue.push(first));
        assert!(queue.push(packet(&ids)));
        assert!(queue.is_full());

        // the arriving packet is dropped, not the head
        assert!(!queue.push(packet(&ids)));
        assert_eq!(queue.len(), 2);
        let (popped, _receipt) = queue.pop().expect("queue holds a packet");
        assert_eq!(popped.id(), first_id);
    }

    #[test]
    fn pop_on_empty_queue_errors() {
        let mut queue = FifoDropTailQueue::new(2);
        assert_eq!(queue.pop().unwrap_err(), PacketQueuePopError::QueueEmpty);
    }

    #[test]
    fn pop_before_settling_the_previous_one_errors() {
        let ids = IdGenerator::new();
        let mut queue = FifoDropTailQueue::new(2);
        queue.push(packet(&ids));
        queue.push(packet(&ids));

        let (_packet, receipt) = queue.pop().expect("queue holds a packet");
        assert_eq!(
            queue.pop().unwrap_err(),
            PacketQueuePopError::UncommittedPacket
        );

        // settling the outstanding pop unblocks the next one
        queue.commit(receipt);
        assert!(queue.pop().is_ok());
    }

    #[test]
    fn unsettled_pop_keeps_its_slot_reserved() {
        let ids = IdGenerator::new();
        let mut queue = FifoDropTailQueue::new(1);
        assert!(queue.push(packet(&ids)));

        let (_packet, receipt) = queue.pop().expect("queue holds a packet");
        assert_eq!(queue.len(), 1);
        assert!(queue.is_full());
        assert!(!queue.push(packet(&ids)));

        queue.commit(receipt);
        assert!(queue.is_empty());
        assert!(queue.push(packet(&ids)));
    }

    #[test]
    fn requeued_packet_is_popped_next() {
        let ids = IdGenerator::new();
        let mut queue = FifoDropTailQueue::new(2);
        let first = packet(&ids);
        let first_id = first.id();
        let second = packet(&ids);
        let second_id = second.id();
        queue.push(first);
        queue.push(second);

        let (popped, receipt) = queue.pop().expect("queue holds a packet");
        assert_eq!(popped.id(), first_id);
        queue.requeue(receipt, popped);

        // the reserved slot is handed back to the packet, so nothing is lost
        assert_eq!(queue.len(), 2);
        assert!(queue.is_full());

        let (popped, receipt) = queue.pop().expect("queue holds a packet");
        assert_eq!(popped.id(), first_id);
        queue.commit(receipt);
        let (popped, receipt) = queue.pop().expect("queue holds a packet");
        assert_eq!(popped.id(), second_id);
        queue.commit(receipt);
    }

    #[test]
    fn requeue_of_a_full_queue_does_not_overflow_capacity() {
        let ids = IdGenerator::new();
        let mut queue = FifoDropTailQueue::new(1);
        queue.push(packet(&ids));

        let (popped, receipt) = queue.pop().expect("queue holds a packet");
        queue.requeue(receipt, popped);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.len(), queue.capacity());
        assert!(!queue.push(packet(&ids)));
    }

    #[test]
    #[should_panic(expected = "receipt does not match")]
    fn commit_with_a_foreign_receipt_panics() {
        let ids = IdGenerator::new();
        let mut mine = FifoDropTailQueue::new(1);
        let mut theirs = FifoDropTailQueue::new(1);
        mine.push(packet(&ids));
        theirs.push(packet(&ids));

        let (_packet, foreign_receipt) = theirs.pop().expect("queue holds a packet");
        let (_packet, _receipt) = mine.pop().expect("queue holds a packet");
        mine.commit(foreign_receipt);
    }

    #[test]
    #[should_panic(expected = "receipt does not match")]
    fn requeue_with_a_foreign_receipt_panics() {
        let ids = IdGenerator::new();
        let mut mine = FifoDropTailQueue::new(1);
        let mut theirs = FifoDropTailQueue::new(1);
        mine.push(packet(&ids));
        theirs.push(packet(&ids));

        let (foreign_packet, foreign_receipt) = theirs.pop().expect("queue holds a packet");
        let (_packet, _receipt) = mine.pop().expect("queue holds a packet");
        mine.requeue(foreign_receipt, foreign_packet);
    }

    #[test]
    #[should_panic(expected = "receipt does not match")]
    fn commit_without_an_outstanding_pop_panics() {
        let ids = IdGenerator::new();
        let mut queue = FifoDropTailQueue::new(1);
        let mut other = FifoDropTailQueue::new(1);
        other.push(packet(&ids));

        let (_packet, foreign_receipt) = other.pop().expect("queue holds a packet");
        queue.commit(foreign_receipt);
    }
}
