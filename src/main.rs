mod uu;

use clap::{App, SubCommand, Arg};

fn main() {
    let matches = App::new(clap::crate_name!())
        .subcommand(SubCommand::with_name("uu")
            .subcommand(SubCommand::with_name("decode")
                .arg(Arg::with_name("string")
                    .index(1)
                )
            )
        )
        .get_matches();

    match matches.subcommand() {
        ("uu", Some(matches)) => {
            match matches.subcommand() {
                ("decode", Some(matches)) => {
                    let encoded = matches.value_of("string").unwrap();
                    println!("{}", uu::decode(encoded));
                },
                _ => {},
            };
        },
        _ => {},
    };
}
