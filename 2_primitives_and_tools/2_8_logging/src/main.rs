#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;
#[macro_use]
extern crate slog_scope;
extern crate chrono;

use slog::{Drain, Duplicate, FnValue, Level, Logger, Never, OwnedKVList, PushFnValue, Record};
use slog_async::Async;
use std::fs::OpenOptions;
use std::io;

/// # Simple custom structured logging.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///
/// use Structured_logging::*;
///
/// let drain = slog_json::Json::new(std::io::stdout())
///      .set_pretty(false)
///        .build()
///        .fuse();
///    let d_stdout = slog_async::Async::new(drain).build().fuse();
///
///    let drain = slog_json::Json::new(std::io::stderr())
///        .set_pretty(false)
///        .build()
///        .fuse();
///    let d_stderr = slog_async::Async::new(drain).build().fuse();
///
///    let drain_base = Duplicate::new(
///        CustomLevelFilter::new(d_stderr, Level::Warning, CmpLevel::Less),
///        CustomLevelFilter::new(d_stdout, Level::Info, CmpLevel::Greater),
///    ).fuse();
///
///
///    //  Global logging
///    let _guard = slog_scope::set_global_logger(Logger::root(
///        drain_base,
///        o!(
///                "msg" => PushFnValue(move |record : &Record, ser| {
///                    ser.emit(record.msg())
///                }),
///                "time" => PushFnValue(move |_ : &Record, ser| {
///                    ser.emit(chrono::Utc::now().to_rfc3339())
///                }),
///               "file"=>"app.log",
///                "lvl" => FnValue(move |rinfo : &Record| {
///                    rinfo.level().as_str()
///                }),
///         ),
///    ));
///
///    info!("Info message using the global logger");
///    debug!("debug");
///    error!("Error occurred");
///
///    slog_debug!(slog_scope::logger(), "slog_debug");
///    slog_error!(slog_scope::logger(), "Error occurred");
/// ```
mod Structured_logging {
    use super::*;

    /// Type of error level comparison.
    pub enum CmpLevel {
        Less = 0,
        Greater,
    }

    /// The implementation of struct slog :: LevelFilter.
    /// The logic of filtering the error levels in a partition priorities.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate slog;
    /// extern crate slog_async;
    /// extern crate slog_term;
    /// use slog::*;
    ///
    /// fn main(){
    ///   use Structured_logging::*;
    ///
    ///   let decorator = slog_term::TermDecorator::new().stdout().build();
    ///   let d_stdout = slog_term::CompactFormat::new(decorator).build().fuse();
    ///   let d_stdout = slog_async::Async::new(d_stdout).build().fuse();
    ///
    ///   let decorator = slog_term::PlainDecorator::new(std::io::stderr());
    ///   let d_stderr = slog_term::FullFormat::new(decorator).build().fuse();
    ///   let d_stderr = slog_async::Async::new(d_stderr).build().fuse();
    ///
    ///   let drain_base = Duplicate::new(
    ///     CustomLevelFilter::new(d_stderr, Level::Warning, CmpLevel::Less),
    ///     CustomLevelFilter::new(d_stdout, Level::Info, CmpLevel::Greater),
    ///   ).fuse();
    ///
    ///   let root = Logger::root(drain_base,o!());
    ///
    ///   info!(root, "{method} {path}", method = "POST", path = "/some"; );
    /// }
    /// ```
    pub struct CustomLevelFilter<D: Drain>(pub D, pub Level, pub CmpLevel);

    /// Implement struct CustomLevelFilter.
    impl<D: Drain> CustomLevelFilter<D> {
        /// Create CustomLevelFilter.
        pub fn new(drain: D, level: Level, cmp: CmpLevel) -> Self {
            CustomLevelFilter(drain, level, cmp)
        }
    }
    /// Implement Drain trait for struct CustomLevelFilter.
    /// Custom logic compare error slog::Level.
    impl<D: Drain> Drain for CustomLevelFilter<D> {
        type Ok = ();
        type Err = Never;
        fn log(
            &self,
            record: &Record,
            logger_values: &OwnedKVList,
        ) -> std::result::Result<Self::Ok, Self::Err> {
            match self.2 {
                CmpLevel::Less => {
                    if record.level() <= self.1 {
                        self.0.log(record, logger_values);
                    }
                }
                CmpLevel::Greater => {
                    if record.level() >= self.1 {
                        self.0.log(record, logger_values);
                    }
                }
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test() {

            let drain_base = Duplicate::new(
                CustomLevelFilter::new(slog::Discard, Level::Warning, CmpLevel::Less),
                CustomLevelFilter::new(slog::Discard, Level::Info, CmpLevel::Greater),
            ).fuse();

            let root: slog::Logger = Logger::root(drain_base, o!("key" => "value"));
            let root_new: slog::Logger = root.new(o!("key_new" => "value_new"));
            assert!(true);
        }
    }

}

fn main() {
    use Structured_logging::*;

    // Global logging

    let drain = slog_json::Json::new(std::io::stdout())
        .set_pretty(false)
        .build()
        .fuse();
    let d_stdout = slog_async::Async::new(drain).build().fuse();

    let drain = slog_json::Json::new(std::io::stderr())
        .set_pretty(false)
        .build()
        .fuse();
    let d_stderr = slog_async::Async::new(drain).build().fuse();

    let drain_base = Duplicate::new(
        CustomLevelFilter::new(d_stderr, Level::Warning, CmpLevel::Less),
        CustomLevelFilter::new(d_stdout, Level::Info, CmpLevel::Greater),
    ).fuse();


    let _guard = slog_scope::set_global_logger(Logger::root(
        drain_base,
        o!(
                "msg" => PushFnValue(move |record : &Record, ser| {
                    ser.emit(record.msg())
                }),
                "time" => PushFnValue(move |_ : &Record, ser| {
                    ser.emit(chrono::Utc::now().to_rfc3339())
                }),
                "file"=>"app.log",
                "lvl" => FnValue(move |rinfo : &Record| {
                    rinfo.level().as_str()
                }),
         ),
    ));

    info!("Info message using the global logger");
    debug!("debug");
    error!("Error occurred");

    slog_debug!(slog_scope::logger(), "slog_debug");
    slog_error!(slog_scope::logger(), "Error occurred");


    // Local logging

    let log_path = "access.log";
    let file: std::fs::File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    let drain = slog_json::Json::new(file)
        .set_pretty(false)
        .set_newlines(true)
        .build()
        .fuse();

    let drain_file = slog_async::Async::new(drain).build().fuse();

    let root = Logger::root(
        drain_file,
        o!(
                "msg" => PushFnValue(move |record : &Record, ser| {
                    ser.emit(record.msg())
                }),
                "time" => PushFnValue(move |_ : &Record, ser| {
                    ser.emit(chrono::Utc::now().to_rfc3339())
                }),
                "file"=>"app.log",
                "lvl" => FnValue(move |rinfo : &Record| {
                    rinfo.level().as_str()
                }),
                ),
    );

    slog_scope::scope(&root, || {
        info!( "http"; "method" => "POST", "path" => "/some");
    });


}
