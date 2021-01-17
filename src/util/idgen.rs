use std::sync::atomic::*;

pub struct IdGen {
    current: u64,
}

impl IdGen {
    pub fn new() -> IdGen {
        IdGen { current: 0 }
    }

    pub fn next(&mut self) -> u64 {
        self.current += 1;

        self.current - 1
    }
}

pub struct IdGenSync {
    current: AtomicU64,
}

impl IdGenSync {
    pub fn new() -> IdGenSync {
        IdGenSync {
            current: AtomicU64::new(0),
        }
    }

    pub fn next(&mut self) -> u64 {
        self.current.fetch_add(1, Ordering::Acquire)
    }
}
