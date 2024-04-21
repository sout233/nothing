use clap::Parser;

mod core;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from(""),help = "Name to search for")]
    name: String,

    #[arg(short, long, default_value_t = String::from("."),help = "Path to search")]
    path: String,

    #[arg(short, long, default_value_t = 10, help = "Depth of search, set it to 0 to search all")]
    depth: u8,
    
    #[arg(short, long, default_value_t = false, help = "Exact word match")]
    exact_match: bool,

    #[arg(short('D'), long, default_value_t = false, help = "Show debug info")]
    show_debug_info: bool,
}

fn main() {
    let args = Args::parse();

    core::search(&args.name, &args.path, args.depth, args.show_debug_info, args.exact_match);
}
