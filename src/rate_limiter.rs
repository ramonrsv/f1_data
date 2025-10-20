use governor::DefaultDirectRateLimiter;
pub use governor::Quota;
pub use nonzero_ext::nonzero;

/// A simple rate limiter providing a minimal interface required by this crate.
///
/// This is a thin wrapper around [`governor`](https://crates.io/crates/governor)'s rate limiter,
/// although the underlying implementation is not part of the public API and is subject to change.
#[derive(Debug)]
pub struct RateLimiter {
    quota: Quota,
    rate_limiter: DefaultDirectRateLimiter,
}

impl RateLimiter {
    /// Create a new rate limiter with the given [`Quota`].
    pub fn new(quota: Quota) -> Self {
        let rate_limiter = DefaultDirectRateLimiter::direct(quota);
        Self { quota, rate_limiter }
    }

    /// Synchronously wait until the rate limiter allows another request.
    pub fn wait_until_ready(&self) {
        while self.rate_limiter.check().is_err() {
            std::thread::sleep(self.quota.replenish_interval() / 100);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    use nonzero_ext::nonzero;

    use crate::tests::asserts::*;

    use super::*;

    fn wait_until_n_ready(limiter: &RateLimiter, n: usize) {
        for _ in 0..n {
            limiter.wait_until_ready();
        }
    }

    fn spawn_and_join_threads(limiter: &Arc<RateLimiter>, thread_count: usize, requests_per_thread: usize) {
        (0..thread_count)
            .map(|_| {
                let limiter = Arc::clone(&limiter);
                thread::spawn(move || {
                    wait_until_n_ready(&limiter, requests_per_thread);
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|handle| {
                handle.join().unwrap();
            });
    }

    #[test]
    fn basic_rate_limiting_and_burst() {
        let quota = Quota::per_second(nonzero!(10u32)).allow_burst(nonzero!(5u32));
        let limiter = RateLimiter::new(quota);

        let start = Instant::now();
        wait_until_n_ready(&limiter, 5);
        let elapsed = start.elapsed();

        // First 5 requests should complete immediately
        assert_lt!(elapsed, Duration::from_millis(5));

        for _ in 0..10 {
            let start = Instant::now();
            limiter.wait_until_ready();
            let elapsed = start.elapsed();

            // Subsequent requests should wait, ~100ms each
            assert_ge!(elapsed, Duration::from_millis(90));
            assert_lt!(elapsed, Duration::from_millis(110));
        }

        // Wait one second (10 tokens @10/s) to let the limiter replenish
        // It should only accumulate 5 burst tokens
        thread::sleep(Duration::from_millis(1000));

        let start = Instant::now();
        wait_until_n_ready(&limiter, 5);
        let elapsed = start.elapsed();

        // First subsequent 5 requests should complete immediately
        assert_lt!(elapsed, Duration::from_millis(5));

        for _ in 0..10 {
            let start = Instant::now();
            limiter.wait_until_ready();
            let elapsed = start.elapsed();

            // Subsequent requests should wait, ~100ms each
            assert_ge!(elapsed, Duration::from_millis(90));
            assert_lt!(elapsed, Duration::from_millis(110));
        }
    }

    #[test]
    fn multi_threaded_rate_limiting_and_burst() {
        let quota = Quota::per_second(nonzero!(10u32)).allow_burst(nonzero!(10u32));
        let limiter = Arc::new(RateLimiter::new(quota));

        let start = Instant::now();
        spawn_and_join_threads(&limiter, 5, 2);
        let elapsed = start.elapsed();

        // First 10 requests should complete immediately
        assert_lt!(elapsed, Duration::from_millis(5));

        let start = Instant::now();
        spawn_and_join_threads(&limiter, 5, 2);
        let elapsed = start.elapsed();

        // Subsequent requests should wait, ~100ms each, ~1000ms total
        assert_ge!(elapsed, Duration::from_millis(990));
        assert_lt!(elapsed, Duration::from_millis(1010));

        // Wait two seconds (20 tokens @10/s) to let the limiter replenish
        // It should only accumulate 10 burst tokens
        thread::sleep(Duration::from_millis(2000));

        let start = Instant::now();
        spawn_and_join_threads(&limiter, 5, 2);
        let elapsed = start.elapsed();

        // First subsequent 10 requests should complete immediately
        assert_lt!(elapsed, Duration::from_millis(5));

        let start = Instant::now();
        spawn_and_join_threads(&limiter, 5, 2);
        let elapsed = start.elapsed();

        // Subsequent requests should wait, ~100ms each, ~1000ms total
        assert_ge!(elapsed, Duration::from_millis(990));
        assert_lt!(elapsed, Duration::from_millis(1010));
    }
}
