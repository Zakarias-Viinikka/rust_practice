use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

#[derive(Clone)]
pub struct Settings {
    delay_ms: Arc<AtomicU64>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            delay_ms: Arc::new(AtomicU64::new(500)),
        }
    }

    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.delay_ms.load(Ordering::Relaxed))
    }

    pub fn faster(&self) {
        let cur = self.delay_ms.load(Ordering::Relaxed);
        self.delay_ms
            .store(cur.saturating_sub(100).max(100), Ordering::Relaxed);
    }

    pub fn slower(&self) {
        self.delay_ms.fetch_add(100, Ordering::Relaxed);
    }
}
