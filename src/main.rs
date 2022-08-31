use clap::Parser;

// Read and compile a python file
#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    // the path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    print!("{:?}", args);
}
