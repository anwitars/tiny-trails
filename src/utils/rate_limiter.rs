use std::sync::Arc;

use governor::{clock::QuantaInstant, middleware::NoOpMiddleware};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor,
};

pub fn start_rate_limiter() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    let config = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(1)
        .finish()
        .unwrap();

    let limiter = config.limiter().clone();

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
