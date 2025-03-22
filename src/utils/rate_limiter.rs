use std::sync::Arc;

use governor::{clock::QuantaInstant, middleware::NoOpMiddleware};
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor, GovernorLayer,
};

/// Starts the rate limiter thread and returns the tower layer.
pub fn start_rate_limiter() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    // create a rate limiter with 1 request per second and a burst size of 1
    // TODO: later there should be a way to configure this
    // with either a yaml app config file, or environment variables (or both)
    let config = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(1)
        .finish()
        .unwrap();

    let limiter = config.limiter().clone();

    // every minute, clean up the limiter
    std::thread::spawn(move || {
        let interval = std::time::Duration::from_secs(60);
        loop {
            std::thread::sleep(interval);
            limiter.retain_recent();
        }
    });

    GovernorLayer {
        config: Arc::new(config),
    }
}
