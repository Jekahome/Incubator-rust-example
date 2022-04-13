use std::borrow::Cow;
use std::env;
//use std::io::{Error, ErrorKind};

const CONFIG_PATH_DEFAULT: &str = "/etc/app/app.conf";
const APP_CONF: &str = "APP_CONF";

fn get_env() -> Option<String> {
    env::var(APP_CONF).ok()
}

fn get_value_args(var_arg: &str) -> Option<String> {
    if env::args_os().len() < 2 {
        return None;
    }
    let mut result: Option<String> = None;
    for argument in env::args_os() {
        if argument
            .to_str()
            .and_then(|v| v.find(var_arg).map_or(None, |_byte| Some(v)))
            .and_then(|v| {
                v.find("=")
                    .map_or(Some(("", "")), |byte| Some(v.split_at(byte + 1)))
            })
            .and_then(|(_, last)| {
                result = Some(last.to_string());
                Some(())
            })
            .is_some()
        {
            break;
        }
    }

    return result;
}

fn path<'a>() -> Cow<'a, str> {
    let mut path = Cow::Borrowed(CONFIG_PATH_DEFAULT);

    if let Some(_path) = get_env() {
        path = Cow::Owned(_path);
    }

    if let Some(_path) = get_value_args("--conf") {
        if _path.is_empty() {
            eprintln!("Error: arguments --conf can not be empty !");
        } else {
            path = Cow::Owned(_path);
        }
    }

    path
}

fn main() {
    println!("path:{}", path());
}

#[test]
fn path_test() {
    let _path = path();

    get_value_args("--conf")
        .and_then(|args_path| {
            if args_path.is_empty() {
                eprintln!("Error: arguments --conf can not be empty !");
            } else {
                assert_eq!(_path.trim(), args_path.trim());
            }
            Some(())
        })
        .or_else(|| {
            get_env().and_then(|env_path| {
                assert_eq!(_path.trim(), env_path.trim());
                Some(())
            });
            Some(())
        });
}
