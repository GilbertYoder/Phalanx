use serde::{Deserialize, Serialize};
use std::cmp::max;

#[derive(Deserialize, Serialize)]
pub struct LamportClock {
    pub time: usize,
}

impl LamportClock {
    pub fn recieve(&mut self, time: usize) {
        self.time = max(self.time, time) + 1;
    }

    pub fn increment(&mut self) {
        self.time += 1;
    }
}

impl LamportClock {
    pub fn new() -> LamportClock {
        LamportClock { time: 0 }
    }
}
