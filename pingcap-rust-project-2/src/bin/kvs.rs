use clap::{App, Arg, SubCommand, AppSettings};
use kvs::KvStore;
use std::process;
use std::process::exit;
use anyhow::Result;

fn main() -> Result<()> {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("get").
                arg(
                    Arg::with_name("KEY").help("").required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("set").
                arg(
                Arg::with_name("KEY").help("").required(true)
                )
                .arg(
                    Arg::with_name("VALUE").help("").required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(
                Arg::with_name("KEY").value_name("rm").help("").required(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        },
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        },
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        },
        _ => {
            unreachable!()
        }
    }

}

