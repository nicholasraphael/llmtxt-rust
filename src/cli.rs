use clap::Parser;
use llmtxt::LLMSTxt;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    json: bool,

    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(args.path.as_str());
    let contents = fs::read_to_string(path).unwrap();

    let mut llms_txt = LLMSTxt::new();
    llms_txt.parse(contents.as_str()).unwrap();

    if args.json {
        let s = llms_txt.to_json();
        println!("{:?}", s);
    }
}
