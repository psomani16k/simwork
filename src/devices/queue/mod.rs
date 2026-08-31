pub mod fifo_drop_tail_queue;

use crate::core::util::packet::{Packet, id::PacketId};

/// Proof of an uncommitted `pop`. Not `Clone`/`Copy`: the only source is
/// `PacketQueue::pop` and the only sinks are `commit` and `requeue`, so a
/// pop cannot be settled twice.
#[must_use = "an unsettled receipt leaks the popped packet's reserved slot"]
#[derive(Debug, PartialEq, Eq)]
pub struct PopReceipt(PacketId);

impl PopReceipt {
    /// Only queue implementations should create receipts, one per `pop`.
    fn new(packet: &Packet) -> Self {
        Self(packet.id())
    }

    fn id(&self) -> PacketId {
        self.0
    }
}

pub trait PacketQueue {
    /// Attempts to enqueue a packet, returning `false` if the queue is full.
    fn push(&mut self, packet: Packet) -> bool;

    /// Dequeues the next packet according to the queue's scheduling discipline.
    /// Its slot stays reserved until the receipt is settled via `commit` or `requeue`.
    fn pop(&mut self) -> Result<(Packet, PopReceipt), PacketQueuePopError>;

    /// Settles a `pop`: the popped packet's reserved slot is freed for reuse.
    fn commit(&mut self, receipt: PopReceipt);

    /// Reverses a `pop`: takes the packet back so it is the next one popped. Cannot fail.
    fn requeue(&mut self, receipt: PopReceipt, packet: Packet);

    /// Returns the number of packets currently in the queue, counting a
    /// popped-but-unsettled packet's reserved slot.
    fn len(&self) -> usize;

    /// Returns the maximum number of packets this queue can hold.
    fn capacity(&self) -> usize;

    /// Returns `true` if no more packets can be enqueued.
    fn is_full(&self) -> bool;

    /// Returns `true` if the queue holds no packets.
    fn is_empty(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
pub enum PacketQueuePopError {
    QueueEmpty,
    UncommittedPacket,
}
