use std::collections::{BTreeSet, HashMap};

use crate::core::{
    event::{Event, EventId},
    util::time::SimTime,
};

#[derive(Default)]
pub struct EventQueue {
    queue: BTreeSet<EventSortKey>,
    events: HashMap<EventId, Event>,
}

impl EventQueue {
    pub fn push_event(&mut self, event: Event) {
        let sort_key = EventSortKey {
            uid: event.id(),
            timestamp: event.timestamp(),
        };
        self.queue.insert(sort_key);
        self.events.insert(event.id(), event);
    }

    pub fn cancel_event(&mut self, event_id: EventId) {
        if let Some(event) = self.events.get_mut(&event_id) {
            event.cancel();
        };
    }

    pub fn pop_event(&mut self) -> Option<Event> {
        let event_key = self.queue.pop_first();
        match event_key {
            Some(key) => self.events.remove(&key.uid),
            None => None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct EventSortKey {
    pub uid: EventId,
    pub timestamp: SimTime,
}

/// Chronological, and within one instant in the order the events were created.
/// Ids come from a single counter, so that second key is submission order —
/// which is what keeps a layer's output from overtaking itself once handing
/// work across a layer boundary costs no time.
impl Ord for EventSortKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then_with(|| self.uid.cmp(&other.uid))
    }
}

impl PartialOrd for EventSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
