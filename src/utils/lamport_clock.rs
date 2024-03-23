
pub struct LamportClock {
    pub counter: usize,
}

impl LamportClock {
    pub fn increment(&mut self) {
        self.counter += 1;
    }
}

impl LamportClock {
    pub fn new() -> LamportClock {
        LamportClock {
            counter: 0
        }
    }
}