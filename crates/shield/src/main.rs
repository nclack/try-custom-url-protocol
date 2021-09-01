use std::{env::current_exe, ffi::OsString};

use anyhow::{anyhow, Error};
use argh::FromArgs;
use tracing::{debug, info, warn};
use tracing_subscriber::{fmt, prelude::*};
use winreg::{enums::HKEY_CLASSES_ROOT, RegKey};

/// (un)Register a custom protocol header for example://
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    #[argh(subcommand)]
    cmd: SubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommand {
    Register(Register),
    Deregister(Deregister),
    Trigger(Trigger),
}

/// Register the URI handler
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "register")]
struct Register {}

/// Remove the URI handler
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "deregister")]
struct Deregister {}

/// Handle the URI
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "trigger")]
struct Trigger {
    #[argh(positional)]
    params: String,
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let log_dir=current_exe()?.ancestors().skip(3).next().unwrap().to_owned();
    println!("HERE {:?}",&log_dir);
    let (file_writer, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        log_dir,
        "shield.log",
    ));

    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish()
            .with(fmt::Layer::default().with_writer(file_writer)),
    )
    .expect("Unable to set global tracing subscriber");

    {
        debug!("{:?}", std::env::args().collect::<Vec<_>>());
    }

    let args: Args = argh::from_env();
    match args.cmd {
        SubCommand::Register(_) => {
            let (key, disposition) =
                RegKey::predef(HKEY_CLASSES_ROOT).create_subkey("example-uri-handler")?;
            debug!("{:?}", disposition);
            key.set_value("", &"URL:Example Launcher")?;
            key.set_value("UseOriginalUrlEncoding", &1u32)?;
            key.set_value("URL Protocol", &OsString::new())?;
            let cmd = format!("\"{}\" \"trigger\" \"%1\"", current_exe()?.to_str().unwrap());
            debug!("{:?}", &cmd);
            let (key, disposition) = key.create_subkey("shell\\open\\command")?;
            debug!("{:?}", disposition);
            key.set_value("", &cmd)?;
        }
        SubCommand::Deregister(_) => {
            RegKey::predef(HKEY_CLASSES_ROOT)
                .delete_subkey_all("example-uri-handler")
                .map_err(|_| anyhow!("Protocol for example:// not found in the registry."))?;
        }
        SubCommand::Trigger(Trigger { params }) => {
            info!("Trigger received: {}", params);
        }
    }

    Ok(())
}
