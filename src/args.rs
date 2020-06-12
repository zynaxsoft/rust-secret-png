use std::path::PathBuf;

use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "Secret PNG", about = "Hide a message in a PNG file!")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, StructOpt)]
pub enum Subcommands {
    Encode(EncodeStruct),
    Decode(DecodeStruct),
    Remove(RemoveStruct),
    Print(PrintStruct),
}

#[derive(Debug, StructOpt)]
pub struct EncodeStruct {
    pub path: PathBuf,

    #[structopt(short = "t", long)]
    pub chunk_type: String,

    #[structopt(short, long)]
    pub message: String,

    #[structopt(short, long)]
    pub dest: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct DecodeStruct {
    pub path: PathBuf,

    #[structopt(short = "t", long)]
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveStruct {
    pub path: PathBuf,

    #[structopt(short = "t", long)]
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct PrintStruct {
    pub path: PathBuf,
}

pub fn parse_args() -> Opt {
    Opt::from_args()
}
