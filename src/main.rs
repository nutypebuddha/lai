use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "laverna",
    about = "Vedic reasoning engine — 9-graha wheel, NAND-to-bankai compute",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ping,
    Info,
    Solve {
        #[arg(short, long)]
        query: String,
    },
    Mcp,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ping => println!("pong"),
        Commands::Info => {
            println!("laverna {}", env!("CARGO_PKG_VERSION"));
            println!("edition 2021");
        }
        Commands::Solve { query } => {
            let intent = laverna::shikai::parse_query_intent(&query);
            let keywords = laverna::zanpakuto::extract_keywords(
                &laverna::zanpakuto::normalize_query_text(&query),
            );
            let domain = laverna::shikai::determine_query_domain(&query);
            println!("intent: {intent}");
            println!("domain: {domain}");
            println!("keywords: {keywords:?}");
        }
        Commands::Mcp => {
            #[cfg(feature = "mcp")]
            {
                eprintln!("mcp server not yet implemented");
                std::process::exit(1);
            }
            #[cfg(not(feature = "mcp"))]
            {
                eprintln!("mcp feature not enabled; rebuild with --features mcp");
                std::process::exit(1);
            }
        }
    }
}
