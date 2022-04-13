extern crate config;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate serde;

use config::*;
use dotenv::dotenv;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::fmt;

/// # Hierarchical typed configuration structure for configuration.
///
/// Module based on the structure of the configuration file.
///
/// A priority:
/// 1. Default value in `Rust` sources;
/// 2. Value read from `TOML` file;
/// 3. Value set by environment variable.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///
///    use configuration::*;
///
///    fn main() -> Result<(), Box<Error>> {
///        let config: Config = AppConfig::priority_config("config.toml")?;
///
///        assert_eq!("127.0.0.1", config.get_str("db.mysql.host")?);
///
///        Ok(())
///    }
/// ```
mod configuration {

    use super::*;

    const REDIS_PORT: u16 = 6379;
    const REDIS_HOST: &'static str = "127.0.0.1";

    /// Configuration parameter `mode`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Mode {
        pub debug: bool,
    }
    /// Default Value for `Mode`.
    impl Default for Mode {
        fn default() -> Self {
            Mode { debug: false }
        }
    }
    /// Configuration parameter `server`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Server {
        pub shard_url: Cow<'static, str>,
        pub http_port: u16,
        pub grpc_port: u16,
        pub healthz_port: u16,
        pub metrics_port: u16,
    }
    /// Default Value for `Server`.
    impl Default for Server {
        fn default() -> Self {
            Server {
                shard_url: "http://127.0.0.1".into(),
                http_port: 8081,
                grpc_port: 8082,
                healthz_port: 10025,
                metrics_port: 9199,
            }
        }
    }
    /// Configuration parameter `db`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Db {
        pub mysql: MySQL,
        pub redis: Redis,
    }
    /// Default Value for `Db`.
    impl Default for Db {
        fn default() -> Self {
            Db {
                mysql: Default::default(),
                redis: Default::default(),
            }
        }
    }

    /// Configuration parameter `redis`.
    /// Setting for the `db` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Redis {
        pub addrs: Vec<Addr>,
    }
    /// Default Value for `Redis`.
    impl Default for Redis {
        fn default() -> Self {
            Redis {
                addrs: vec![Default::default()],
            }
        }
    }
    /// Configuration parameter `addr`.
    /// Setting for the `redis` parameter.
    #[derive(Debug, Serialize, PartialEq)]
    pub struct Addr {
        pub host: Cow<'static, str>,
        pub port: u16,
    }
    /// Default Value for `Addr`.
    impl Default for Addr {
        fn default() -> Self {
            Addr {
                host: "127.0.0.1".into(),
                port: 6379,
            }
        }
    }

    /// Implemented Deserialize for coexistence of error types in the `Addr` enumeration.
    /// If there is no data, the field is populated with the default value.
    impl<'de> Deserialize<'de> for Addr {
        fn deserialize<D>(deserializer: D) -> Result<Addr, D::Error>
        where
            D: Deserializer<'de>,
        {
            enum Field {
                HOST,
                PORT,
            };

            impl<'de> Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct FieldVisitor;

                    impl<'de> Visitor<'de> for FieldVisitor {
                        type Value = Field;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("`host` or `port`")
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                        {
                            match value {
                                "host" => Ok(Field::HOST),
                                "port" => Ok(Field::PORT),
                                _ => Err(de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }

                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }

            struct AddrVisitor;

            impl<'de> Visitor<'de> for AddrVisitor {
                type Value = Addr;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct Addr")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Addr, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let host = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                    let port = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                    Ok(Addr {
                        host: host,
                        port: port,
                    })
                }

                fn visit_map<V>(self, mut map: V) -> Result<Addr, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut host = None;
                    let mut port = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::HOST => {
                                if host.is_some() {
                                    return Err(de::Error::duplicate_field("host"));
                                }
                                host = Some(map.next_value()?);
                            }
                            Field::PORT => {
                                if port.is_some() {
                                    return Err(de::Error::duplicate_field("port"));
                                }
                                port = Some(map.next_value()?);
                            }
                        }
                    }

                    let host = host.unwrap_or(REDIS_HOST.into());
                    let port = port.unwrap_or(REDIS_PORT);

                    Ok(Addr {
                        host: host,
                        port: port,
                    })
                }
            }

            const FIELDS: &'static [&'static str] = &["host", "port"];
            deserializer.deserialize_struct("Addr", FIELDS, AddrVisitor)
        }
    }

    /// Configuration parameter `mysql`.
    /// Setting for the `db` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct MySQL {
        pub host: Cow<'static, str>,
        pub port: u16,
        pub user: Cow<'static, str>,
        pub pass: Cow<'static, str>,
        pub databases: Databases,
        pub connections: Connections,
    }
    /// Default Value for `MySQL`.
    impl Default for MySQL {
        fn default() -> Self {
            MySQL {
                host: "127.0.0.1".into(),
                port: 3306,
                user: "root".into(),
                pass: "".into(),
                databases: Default::default(),
                connections: Default::default(),
            }
        }
    }

    /// Configuration parameter `databases`.
    /// Setting for the `mysql` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Databases {
        pub dating: Cow<'static, str>,
        pub social: Cow<'static, str>,
    }
    /// Default Value for `Databases`.
    impl Default for Databases {
        fn default() -> Self {
            Databases {
                dating: "dating".into(),
                social: "social".into(),
            }
        }
    }

    /// Configuration parameter `connections`.
    /// Setting for the `mysql` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Connections {
        pub max_idle: u16,
        pub max_open: u16,
    }
    /// Default Value for `Connections`.
    impl Default for Connections {
        fn default() -> Self {
            Connections {
                max_idle: 30,
                max_open: 30,
            }
        }
    }

    /// Configuration parameter `ms`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Ms {
        pub openvidu: Openvidu,
    }
    /// Default Value for `Ms`.
    impl Default for Ms {
        fn default() -> Self {
            Ms {
                openvidu: Default::default(),
            }
        }
    }

    /// Configuration parameter `openvidu`.
    /// Setting for the `ms` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Openvidu {
        pub host: Cow<'static, str>,
        pub grpc_port: u16,
        pub metrics_port: u16,
    }
    /// Default Value for `Openvidu`.
    impl Default for Openvidu {
        fn default() -> Self {
            Openvidu {
                host: "127.0.0.1".into(),
                grpc_port: 8080,
                metrics_port: 9321,
            }
        }
    }

    /// Configuration parameter `log`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Log {
        pub app: LogLevel,
        pub access: LogLevel,
        pub user: LogLevel,
    }
    /// Default Value for `Log`.
    impl Default for Log {
        fn default() -> Self {
            Log {
                app: LogLevel {
                    level: ErrorLevel::INFO,
                },
                access: LogLevel {
                    level: ErrorLevel::INFO,
                },
                user: LogLevel {
                    level: ErrorLevel::INFO,
                },
            }
        }
    }

    /// Enumeration contains types of possible errors.
    #[derive(Debug, Serialize, PartialEq)]
    pub enum ErrorLevel {
        DEBUG,
        INFO,
        WARN,
        ERROR,
        FATAL,
        PANIC,
        EMPTY,
    }
    /// Implemented Deserialize for coexistence of error types in the `ErrorLevel` enumeration.
    impl<'de> Deserialize<'de> for ErrorLevel {
        fn deserialize<D>(deserializer: D) -> Result<ErrorLevel, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct FieldVisitor;

            impl<'de> Visitor<'de> for FieldVisitor {
                type Value = ErrorLevel;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str(
                        "Possible values (in ascending order):\n\
                         \"debug\", \"info\", \"warn\", \"error\", \"fatal\", \"panic\", \"\"",
                    )
                }

                fn visit_str<E>(self, value: &str) -> Result<ErrorLevel, E>
                where
                    E: de::Error,
                {
                    match value {
                        "debug" => Ok(ErrorLevel::DEBUG),
                        "info" => Ok(ErrorLevel::INFO),
                        "warn" => Ok(ErrorLevel::WARN),
                        "error" => Ok(ErrorLevel::ERROR),
                        "fatal" => Ok(ErrorLevel::FATAL),
                        "panic" => Ok(ErrorLevel::PANIC),
                        "" => Ok(ErrorLevel::EMPTY),
                        _ => Err(de::Error::unknown_field(
                            value,
                            &["debug", "info", "warn", "error", "fatal", "panic", ""],
                        )),
                    }
                }
            }

            deserializer.deserialize_identifier(FieldVisitor)
        }
    }
    /// Default Value for `ErrorLevel`.
    impl Default for ErrorLevel {
        fn default() -> Self {
            ErrorLevel::EMPTY
        }
    }

    /// Implement convert ErrorLevel in String.
    impl From<ErrorLevel> for String {
        fn from(e: ErrorLevel) -> Self {
            match e {
                ErrorLevel::DEBUG => "debug".to_string(),
                ErrorLevel::INFO => "info".to_string(),
                ErrorLevel::WARN => "warn".to_string(),
                ErrorLevel::ERROR => "error".to_string(),
                ErrorLevel::FATAL => "fatal".to_string(),
                ErrorLevel::PANIC => "panic".to_string(),
                ErrorLevel::EMPTY => "".to_string(),
            }
        }
    }

    /// Configuration parameter `level`.
    /// Setting for the `log` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct LogLevel {
        pub level: ErrorLevel,
    }
    /// Default Value for `LogLevel`.
    impl Default for LogLevel {
        fn default() -> Self {
            LogLevel {
                level: ErrorLevel::EMPTY,
            }
        }
    }

    /// Configuration parameter `auth`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Auth {
        pub user_password_salt: Cow<'static, str>,
        pub renewal_duration: Cow<'static, str>,
    }
    /// Default Value for `Auth`.
    impl Default for Auth {
        fn default() -> Self {
            Auth {
                user_password_salt: "".into(),
                renewal_duration: "5m".into(),
            }
        }
    }

    /// Configuration parameter `app`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct App {
        pub shutdown_timeout: Cow<'static, str>,
        pub live_stream: LiveStream,
        pub setup_stream: SetupStream,
    }
    /// Default Value for `App`.
    impl Default for App {
        fn default() -> Self {
            App {
                shutdown_timeout: "30s".into(),
                live_stream: Default::default(),
                setup_stream: Default::default(),
            }
        }
    }
    /// Configuration parameter `setup_stream`.
    /// Setting for the `app` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SetupStream {
        pub idle_timeout: Cow<'static, str>,
        pub starting_timeout: Cow<'static, str>,
    }
    /// Default Value for `SetupStream`.
    impl Default for SetupStream {
        fn default() -> Self {
            SetupStream {
                idle_timeout: "5s".into(),
                starting_timeout: "20s".into(),
            }
        }
    }

    /// Configuration parameter `live_stream`.
    /// Setting for the `app` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct LiveStream {
        pub max_message_length: u16,
        pub idle_timeout: Cow<'static, str>,
        pub starting_timeout: Cow<'static, str>,
        pub visit: Visit,
        pub preview: Preview,
    }
    /// Default Value for `LiveStream`.
    impl Default for LiveStream {
        fn default() -> Self {
            LiveStream {
                max_message_length: 1000,
                idle_timeout: "5s".into(),
                starting_timeout: "20s".into(),
                visit: Default::default(),
                preview: Default::default(),
            }
        }
    }

    /// Configuration parameter `visit`.
    /// Setting for the `live_stream` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Visit {
        pub idle_timeout: Cow<'static, str>,
        pub starting_timeout: Cow<'static, str>,
    }
    /// Default Value for `Visit`.
    impl Default for Visit {
        fn default() -> Self {
            Visit {
                idle_timeout: "5s".into(),
                starting_timeout: "20s".into(),
            }
        }
    }

    /// Configuration parameter `preview`.
    /// Setting for the `live_stream` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Preview {
        pub idle_timeout: Cow<'static, str>,
        pub starting_timeout: Cow<'static, str>,
    }
    /// Default Value for `Preview`.
    impl Default for Preview {
        fn default() -> Self {
            Preview {
                idle_timeout: "5s".into(),
                starting_timeout: "20s".into(),
            }
        }
    }

    /// Configuration parameter `background`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Background {
        pub finalizer: Finalizer,
        pub recounter: Recounter,
        pub watchdog: Watchdog,
    }
    /// Default Value for `Background`.
    impl Default for Background {
        fn default() -> Self {
            Background {
                finalizer: Default::default(),
                recounter: Default::default(),
                watchdog: Default::default(),
            }
        }
    }

    /// Configuration parameter `finalizer`.
    /// Setting for the `background` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Finalizer {
        pub period: Cow<'static, str>,
        pub limit: Cow<'static, str>,
    }
    /// Default Value for `Finalizer`.
    impl Default for Finalizer {
        fn default() -> Self {
            Finalizer {
                period: "10s".into(),
                limit: "50".into(),
            }
        }
    }

    /// Configuration parameter `recounter`.
    /// Setting for the `background` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Recounter {
        pub period: Cow<'static, str>,
        pub limit: Cow<'static, str>,
        pub lock_timeout: Cow<'static, str>,
    }
    /// Default Value for `Recounter`.
    impl Default for Recounter {
        fn default() -> Self {
            Recounter {
                period: "5s".into(),
                limit: "50".into(),
                lock_timeout: "4s".into(),
            }
        }
    }

    /// Configuration parameter `watchdog`.
    /// Setting for the `background` parameter.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Watchdog {
        pub period: Cow<'static, str>,
        pub limit: Cow<'static, str>,
        pub lock_timeout: Cow<'static, str>,
    }
    /// Default Value for `Watchdog`.
    impl Default for Watchdog {
        fn default() -> Self {
            Watchdog {
                period: "5s".into(),
                limit: "10".into(),
                lock_timeout: "4s".into(),
            }
        }
    }

    /// Configuration parameter `ice`.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Ice {
        pub servers: Vec<Cow<'static, str>>,
    }
    /// Default Value for `Ice`.
    impl Default for Ice {
        fn default() -> Self {
            Ice {
                servers: vec!["turn:access_token:qwerty@127.0.0.1:3478".into()],
            }
        }
    }

    /// The main structure contains all the configuration settings.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct AppConfig {
        pub mode: Mode,
        pub server: Server,
        pub db: Db,
        pub ms: Ms,
        pub log: Log,
        pub auth: Auth,
        pub app: App,
        pub background: Background,
        pub ice: Ice,
    }

    /// Create a config with priority.
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///    use configuration::*;
    ///
    ///    fn main() -> Result<(), Box<Error>> {
    ///        let config: Config = AppConfig::priority_config("config.toml")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    impl AppConfig {
        pub fn priority_config(name: &str) -> Result<Config, Box<Error>> {
            let my_conf: AppConfig = Default::default();
            let temp_config: config::Config = Config::try_from(&my_conf).unwrap();

            let mut config = Config::new();
            config.merge(temp_config).unwrap();

            config.merge(config::File::with_name(name))?;

            config.merge(config::Environment::new().separator("_"))?;

            Ok(config)
        }
    }

    /// Default Value for `AppConfig`.
    impl Default for AppConfig {
        fn default() -> Self {
            AppConfig {
                mode: Default::default(),
                server: Default::default(),
                db: Default::default(),
                ms: Default::default(),
                log: Default::default(),
                auth: Default::default(),
                app: Default::default(),
                background: Default::default(),
                ice: Default::default(),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_mode_debug() {
            let mut config: Config = AppConfig::priority_config("config.toml").unwrap();

            let debug: bool = config.get_bool("mode.debug").unwrap();

            config.set("mode.debug", !debug as bool);

            assert_ne!(debug, config.get_bool("mode.debug").unwrap());
        }

        #[test]
        fn test_db_mysql_host() {
            let mut config: Config = AppConfig::priority_config("config.toml").unwrap();

            config.set("db.mysql.host", "127.0.0.2");

            assert_eq!("127.0.0.2", config.get_str("db.mysql.host").unwrap());
        }

        #[test]
        fn test_environment() {
            // $ MODE_DEBUG=true app
            let mut config: Config = AppConfig::priority_config("config.toml").unwrap();

            if let Ok(path) = env::current_dir().and_then(|a| Ok(a.join(".env"))) {
                dotenv::from_path(path);

                config
                    .merge(config::Environment::new().separator("_"))
                    .unwrap();

                assert_ne!(false, config.get_bool("mode.debug").unwrap());
            } else {
                assert!(false);
            }
        }

        #[test]
        fn test_db_redis_addrs() {
            let mut config: Config = AppConfig::priority_config("config.toml").unwrap();

            config
                .merge(File::from_str("[[db.redis.addrs]]", FileFormat::Toml))
                .unwrap();

            config.set("db.redis.addrs[0].host", "1.2.3.4");
            config.set("db.redis.addrs[1].port", 535);

            let addrs: Vec<Addr> = config.get::<Vec<Addr>>("db.redis.addrs").unwrap();

            assert_eq!(addrs[0].port, 6379);
            assert_eq!(addrs[1].host, "127.0.0.1");
        }

    }
}

use configuration::*;

fn main() -> Result<(), Box<Error>> {
    let config: Config = AppConfig::priority_config("config.toml").unwrap();

    assert_eq!("127.0.0.1", config.get_str("db.mysql.host").unwrap());

    Ok(())
}
