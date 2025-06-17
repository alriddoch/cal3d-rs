use safe_arch;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn getTick() -> u128 {
    // TODO: This is currently a number of millis for legacy
    // compatibility. Can it be re-written as native Duration?
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

fn getHz() -> u64 {
    let t1 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros();
    let c1 = safe_arch::read_timestamp_counter();
    for _i in 0..2500000 {}
    let t2 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros();
    // TODO: Look at read_timestamp_counter_p() instead
    let c2 = safe_arch::read_timestamp_counter();
    return (1000000 * (c2 - c1)) / (t2 - t1) as u64;
}

static mut scale_counter: f64 = -1.0;

pub fn getTime() -> f64 {
    unsafe {
        if scale_counter == -1.0 {
            scale_counter = 1.0 / getHz() as f64
        }
        // Go implementation calls BenchStart here, but discards reult
        scale_counter * safe_arch::read_timestamp_counter() as f64
    }
}
