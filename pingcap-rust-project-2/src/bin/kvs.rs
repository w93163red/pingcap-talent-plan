use anyhow::Result;
use clap::{App, AppSettings, Arg, SubCommand};
use kvs::KvStore;
use std::env;
use std::path::PathBuf;
use std::process::exit;

fn main() -> Result<()> {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("KEY").help("").required(true)))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").help("").required(true))
                .arg(Arg::with_name("VALUE").help("").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm").arg(
                Arg::with_name("KEY")
                    .value_name("rm")
                    .help("")
                    .required(true),
            ),
        )
        .get_matches();
    let work_dir = env::current_dir()?;
    let mut kvs = KvStore::new(work_dir);
    match matches.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("Key is not existed");
            kvs.get(key.into())?;
            exit(1);
        }
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("Key is not existed");
            let value = matches.value_of("VALUE").expect("Value is not existed");
            if kvs.set(key.into(), value.into()).is_ok() {
                exit(0);
            } else {
                exit(1);
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("Key is not existed");
            kvs.remove(key.into())?;
            exit(1);
        }
        _ => unreachable!(),
    }
}
