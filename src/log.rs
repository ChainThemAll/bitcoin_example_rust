use std::env;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn log_init(level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("set_global_default err");
}
