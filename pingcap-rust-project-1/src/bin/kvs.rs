use clap::{App, Arg, SubCommand};
use kvs::KvStore;

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
        ("get", Some())
    }

}
