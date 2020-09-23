use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::process;
use std::intrinsics::unreachable;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("get").
                arg(
                    Arg::with_name("KEY").required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("set").
                arg(
                Arg::with_name("KEY").required(true)
                )
                .arg(
                    Arg::with_name("VALUE").required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(
                Arg::with_name("KEY").value_name("rm").required(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(_matches)) => {
            // unimplemented!()
            eprintln!("unimlpemented");
            process::exit(1);
        },
        ("set", Some(_matches)) => {
            eprintln!("unimlpemented");
            process::exit(1);
        },
        ("rm", Some(_matches)) => {
            eprintln!("unimlpemented");
            process::exit(1);
        },
        _ => {
            unreachable!()
        }
    }

}
