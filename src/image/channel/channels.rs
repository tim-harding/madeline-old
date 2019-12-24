use super::Channel;
use std::collections::HashMap;

use crate::utils::{Id, Vector2Int};

pub struct Channels {
    next_id: Id,
    channels: HashMap<Id, Channel>,
}

impl Channels {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            channels: HashMap::new(),
        }
    }

    pub fn add(&mut self, size: Vector2Int) -> Id {
        self.next_id += 1;
        self.channels.insert(self.next_id, Channel::new(size));
        self.next_id
    }

    pub fn get(&mut self, id: Id) -> &mut Channel {
        let channel = self.channels.get_mut(&id);
        match channel {
            Some(channel) => channel,
            None => unreachable!(),
        }
    }
}
