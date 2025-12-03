use std::sync::atomic::{AtomicU32, Ordering};
static COUNTER: AtomicU32 = AtomicU32::new(0);



struct ID{
    value: i32
}

impl ID{
    fn new() -> ID{
        let value = COUNTER.fetch_add(1, Ordering::Relaxed);
        ID{
            value: value as i32
        }
    }
    pub fn value(&self) -> i32 {  // Add this getter
        self.value
    }
}