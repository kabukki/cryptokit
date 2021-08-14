mod uu;

use clap::{App, SubCommand, Arg};

fn main() {
    let matches = App::new(clap::crate_name!())
        .subcommand(SubCommand::with_name("uu")
            .subcommand(SubCommand::with_name("encode")
                .arg(Arg::with_name("string")
                    .index(1)
                )
            )
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
                ("encode", Some(matches)) => {
                    let decoded = matches.value_of("string").unwrap();
                    println!("{}", uu::encode(decoded));
                },
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
