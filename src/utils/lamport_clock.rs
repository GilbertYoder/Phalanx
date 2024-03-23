use std::cmp::max;

pub struct LamportClock {
    pub counter: usize,
}

impl LamportClock {
    pub fn recieve(&mut self, time: usize) {
        self.counter = max(self.counter, time) + 1;
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }
}

impl LamportClock {
    pub fn new() -> LamportClock {
        LamportClock { counter: 0 }
    }
}
