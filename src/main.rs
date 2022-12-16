#![feature(result_option_inspect)]

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "hackit", about = "Hacking toolkit")]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[command(subcommand, about = "Base64 encoding")]
    Base64 (Base64),
    #[command(subcommand, about = "Domain Name System")]
    Dns (Dns),
    #[command(subcommand, about = "Hexadecimal encoding")]
    Hex (Hex),
    #[command(subcommand, about = "JavaScript Object Notation")]
    Json (Json),
    #[command(subcommand, about = "Message Digest 5")]
    Md5 (Md5),
    #[command(subcommand, about = "Network")]
    Network (Network),
    #[command(about = "String length")]
    Strlen { input: String },
    #[command(subcommand, about = "URL (percent) encoding")]
    Url (Url),
    #[command(subcommand, about = "Unix to Unix encoding")]
    Uu (Uu),
}

#[derive(Subcommand)]
enum Base64 {
    #[command(about = "Encode a clear text", arg_required_else_help = true)]
    Encode { input: String },
    #[command(about = "Decode an encoded text", arg_required_else_help = true)]
    Decode { input: String },
}

#[derive(Subcommand)]
enum Dns {
    #[command(about = "Resolve address", arg_required_else_help = true)]
    Resolve { host: String },
}

#[derive(Subcommand)]
enum Hex {
    #[command(about = "Encode a clear text", arg_required_else_help = true)]
    Encode { input: String },
    #[command(about = "Decode an encoded text", arg_required_else_help = true)]
    Decode { input: String },
}

#[derive(Subcommand)]
enum Json {
    #[command(about = "Stringify JSON", arg_required_else_help = true)]
    Compact { input: String },
    #[command(about = "Prettify JSON", arg_required_else_help = true)]
    Pretty { input: String },
    #[command(about = "Test whether JSON is valid", arg_required_else_help = true)]
    Test { input: String },
}

#[derive(Subcommand)]
enum Md5 {
    #[command(about = "Hash message", arg_required_else_help = true)]
    Hash { input: String },
}

#[derive(Subcommand)]
enum Network {
    #[command(about = "List interfaces")]
    Config,
    #[command(about = "Traceroute", arg_required_else_help = true)]
    Traceroute { destination: String },
}

#[derive(Subcommand)]
enum Uu {
    #[command(about = "Encode a clear text", arg_required_else_help = true)]
    Encode { input: String },
    #[command(about = "Decode an encoded text", arg_required_else_help = true)]
    Decode { input: String },
}

#[derive(Subcommand)]
enum Url {
    #[command(about = "Encode", arg_required_else_help = true)]
    Encode { input: String },
    #[command(about = "Decode", arg_required_else_help = true)]
    Decode { input: String },
}

fn main () {
    let args = Cli::parse();
    let err = match args.action {
        Action::Base64 (action) => match action {
            Base64::Encode { input } => hackit::base64::encode(input).inspect(|output| println!("{output}")).err(),
            Base64::Decode { input } => hackit::base64::decode(input).inspect(|output| println!("{output}")).err(),
        },
        Action::Dns (action) => match action {
            Dns::Resolve { host } => {
                let res = hackit::dns::resolve(host);
                
                if let Ok(addresses) = res.clone() {
                    println!("{}", addresses.iter().map(|ip| format!("- {ip}")).collect::<Vec<String>>().join("\n"));
                }

                res.err()
            },
        },
        Action::Hex (action) => match action {
            Hex::Encode { input } => hackit::hex::encode(input).inspect(|output| println!("{output}")).err(),
            Hex::Decode { input } => hackit::hex::decode(input).inspect(|output| println!("{output}")).err(),
        },
        Action::Json (action) => match action {
            Json::Compact { input } => hackit::json::compact(input).inspect(|output| println!("{output}")).err(),
            Json::Pretty { input } => hackit::json::pretty(input).inspect(|output| println!("{output}")).err(),
            Json::Test { input } => {
                let res = hackit::json::test(input);
                let outcome = if res.is_ok() { "valid".green() } else { "invalid".red() };
                println!("JSON is {outcome} !");
                res.err()
            },
        },
        Action::Md5 (action) => match action {
            Md5::Hash { input } => hackit::md5::hash(&input).inspect(|output| println!("{output}")).err(),
        },
        Action::Network (action) => match action {
            Network::Config => {
                let res = hackit::network::config();
                if let Ok(mut interfaces) = res.clone() {
                    interfaces.sort_by(|a, b| a.name.cmp(&b.name));
                    for interface in interfaces {
                        println!("{}: {}",
                            if interface.is_up() { interface.name.green() } else { interface.name.red() },
                            if let Some(mac) = interface.mac { mac.to_string() } else { format!("(no MAC)") },
                        );

                        if interface.ips.len() > 0 {
                            println!("{}", interface.ips.iter().map(|ip| format!("  - {ip}")).collect::<Vec<String>>().join("\n"));
                        }
                    }
                }
                res.err()
            },
            Network::Traceroute { destination: _ } => hackit::network::traceroute().inspect(|output| println!("{output}")).err(),
        },
        Action::Uu (action) => match action {
            Uu::Encode { input } => hackit::uu::encode(input).inspect(|output| println!("{output}")).err(),
            Uu::Decode { input } => hackit::uu::decode(input).inspect(|output| println!("{output}")).err(),
        },
        Action::Url (action) => match action {
            Url::Encode { input } => hackit::url::encode(input).inspect(|output| println!("{output}")).err(),
            Url::Decode { input } => hackit::url::decode(input).inspect(|output| println!("{output}")).err(),
        },
        Action::Strlen { input } => hackit::util::strlen(input).inspect(|output| println!("{output}")).err(),
    };

    if let Some(err) = err {
        eprintln!("❌ {err}");
        std::process::exit(1);
    }
}
