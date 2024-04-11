use clap_verbosity_flag::{LogLevel, Verbosity};

#[derive(Copy, Clone, Debug, Default)]
pub struct DefaultLevel;

impl LogLevel for DefaultLevel {
    fn default() -> Option<clap_verbosity_flag::Level> {
        if shadow_rs::is_debug() {
            Some(clap_verbosity_flag::Level::Trace)
        } else {
            Some(clap_verbosity_flag::Level::Info)
        }
    }
}

pub trait LogInit {
    fn init(&self);
}

impl<L> LogInit for Verbosity<L>
where
    L: clap_verbosity_flag::LogLevel,
{
    fn init(&self) {
        if let Some(level) = self.log_level() {
            std::env::set_var("LOG_LEVEL", level.to_string());
            let builder = tracing_subscriber::fmt().with_writer(std::io::stderr);
            let builder = match level {
                clap_verbosity_flag::Level::Error => builder.with_max_level(tracing::Level::ERROR),
                clap_verbosity_flag::Level::Warn => builder.with_max_level(tracing::Level::WARN),
                clap_verbosity_flag::Level::Info => builder.with_max_level(tracing::Level::INFO),
                clap_verbosity_flag::Level::Debug => builder.with_max_level(tracing::Level::DEBUG),
                clap_verbosity_flag::Level::Trace => builder.with_max_level(tracing::Level::TRACE),
            };
            if level < clap_verbosity_flag::Level::Debug {
                builder.with_target(false).without_time().init();
            } else {
                builder.init();
            }
        }
    }
}
