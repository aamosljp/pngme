use clap::Parser;

#[derive(Parser)]
#[command(name = "pngme")]
pub struct Args {
    #[arg(long, value_name = "Command", default_value = "encode")]
    pub cmd: String,
    #[arg(short, long, value_name = "File")]
    pub file: String,
    #[arg(long, value_name = "Chunk Type", default_value = "ruSt")]
    pub chunk_type: String,
    #[arg(short, long, value_name = "Message", default_value = "Hello, world!")]
    pub message: String,
    #[arg(short, long, value_name = "Output File", default_value = "output.png")]
    pub output: String,
}

