use tracing::Level;
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::SubscriberBuilder;

/// Initialize the tracing logging
/// # Error
/// Will panic if executed twice
pub(crate) fn init() {
    let subscriber = SubscriberBuilder::default()
        .with_ansi(ansi_term::enable_ansi_support().is_ok())
        .with_max_level(Level::INFO)
        .finish();

    set_global_default(subscriber).expect("log::init executed twice");
}
