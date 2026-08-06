use std::{cmp::Reverse, collections::BTreeSet};

use crate::core::event::Event;

#[derive(Default)]
pub struct EventQueue {
    queue: BTreeSet<Reverse<Event>>,
}

impl EventQueue {}
