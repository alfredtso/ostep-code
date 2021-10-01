use std::thread::sleep;
use std::time::Duration;

/// Use sleep
pub fn spin(t: u64) {
    sleep(Duration::new(t, 0));
}

