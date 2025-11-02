use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
	tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(true)
        .with_ansi(true)
        .compact()
    )
    .init();
}