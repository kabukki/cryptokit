use clap::{App, SubCommand, Arg};
use hackit::uu;
use hackit::base64;

fn main() {
    let matches = App::new(clap::crate_name!())
        .subcommand(SubCommand::with_name("uu")
            .about("Unix to Unix encoding")
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
        .subcommand(SubCommand::with_name("base64")
            .about("Base64 encoding")
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
                    let clear = matches.value_of("string").unwrap();
                    println!("{}", uu::encode(clear));
                },
                ("decode", Some(matches)) => {
                    let encoded = matches.value_of("string").unwrap();
                    println!("{}", uu::decode(encoded));
                },
                _ => {},
            };
        },
        ("base64", Some(matches)) => {
            match matches.subcommand() {
                ("encode", Some(matches)) => {
                    let clear = matches.value_of("string").unwrap().as_bytes();
                    println!("{}", base64::encode(clear));
                },
                ("decode", Some(matches)) => {
                    let encoded = matches.value_of("string").unwrap();
                    println!("{}", base64::decode(encoded));
                },
                _ => {},
            };
        },
        _ => {},
    };
}
