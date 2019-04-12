use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, fmt, net::SocketAddr};

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum LogLevel {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "error")]
    Error,

    #[serde(rename = "warn")]
    Warn,

    #[serde(rename = "info")]
    Info,

    #[serde(rename = "debug")]
    Debug,

    #[serde(rename = "trace")]
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogLevel::None => "none",
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        };

        write!(f, "{}", s)
    }
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub host: SocketAddr,
    pub log_level: LogLevel,
}

impl Configuration {
    pub fn try_new() -> Result<Self, ConfigError> {
        let mut c = Config::new();

        // Start off by merging in the "default" configuration file
        c.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("BUSY_ENV").unwrap_or_else(|_| "development".into());
        c.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        c.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of BUSY)
        // Eg.. `BUSY_DEBUG=1 ./target/busy` would set the `debug` key
        c.merge(Environment::with_prefix("busy"))?;

        // You may also programmatically change settings
        // c.set("host.name", "127.0.0.1")?;

        // Now that we're done, let's access our configuration
        let host_ip = c.get_str("host.ip")?;
        let host_port = c.get_int("host.port")?;
        let log_level = c.get("log.level")?;

        // You can deserialize (and thus freeze) the entire configuration as
        // c.try_into()

        let config = Self {
            host: build_host_addr(&host_ip, host_port),
            log_level,
        };

        Ok(config)
    }
}

fn build_host_addr(host_ip: &str, host_port: i64) -> SocketAddr {
    let addr_string = &format!("{}:{}", host_ip, host_port);

    addr_string.parse().unwrap()
}
