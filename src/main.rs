use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
struct Options {
    #[clap(short)]
    file: Option<PathBuf>,
    #[clap(short, default_value = "1", env = "A")]
    a: usize,
    #[clap(short, default_value = "2", env = "B")]
    b: usize,
    #[clap(short, default_value = "3", env = "C")]
    c: usize,
}

fn main() {
    let args = Options::parse();
    if let Some(conf_file) = args.file {
        kankyo::load_from_path(conf_file, false).unwrap();
    }
    let args = Options::parse();
    println!("{:?}", args);
}
