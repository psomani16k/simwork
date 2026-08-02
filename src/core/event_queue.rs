use std::{collections::LinkedList, rc::Rc};

use crate::core::event::Event;

pub struct EventQueue {
    queue: LinkedList<Rc<dyn Event>>,
}

impl EventQueue {}
