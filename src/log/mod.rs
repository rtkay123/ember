use tracing::{event, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn start() {
    let crate_name = env!("CARGO_CRATE_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // comma separate these
                format!("{crate_name}=debug").into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    event!(Level::INFO, "{} v{} has started", crate_name, pkg_version);
}
