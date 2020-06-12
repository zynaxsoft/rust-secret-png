use rust_secret_png::args::{parse_args, Subcommands};
use rust_secret_png::commands;


fn main() {
    let opt = parse_args();
    match &opt.cmd {
        Subcommands::Encode(subopt) => commands::encode(subopt),
        Subcommands::Decode(subopt) => commands::decode(subopt),
        Subcommands::Remove(subopt) => commands::remove(subopt),
        Subcommands::Print(subopt) => commands::print(subopt),
    };
    println!("Done!");
}
