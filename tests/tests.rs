extern crate rand;
extern crate ping;

use std::time::Duration;

use rand::random;

#[test]
fn basic() {
    let addr = "127.0.0.1".parse().unwrap();
    let timeout = Duration::from_secs(1);
    ping::ping(addr, Some(timeout), Some(166), Some(3), Some(5), Some(&random())).unwrap();
}
