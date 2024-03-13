use clap::{Parser, Subcommand};
use highway_core::types::BridgeConfig;
use std::path::PathBuf;

mod commands;
mod reactor;

#[derive(Parser)]
#[command(name = "Highway Bridge")]
#[command(author = "Rakan Al-Hneiti <rakan.alhneiti@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Cross chain bridge", long_about = None)]
struct Cli {
	/// Sets a custom config file
	#[arg(short, long, value_name = "FILE")]
	config: Option<PathBuf>,
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	// Process all events at a specific block
	ReplayBlock {
		/// lists test values
		#[arg(short, long)]
		list: bool,
	},
	Run {},
}

#[tokio::main]
async fn main() {
	let cli = Cli::parse();

	tracing_subscriber::fmt::init();

	let config_path = match cli.config {
		Some(config_path) => config_path,
		None => {
			println!("No config specified");
			return;
		},
	};

	let data = std::fs::read_to_string(config_path).expect("Unable to read file");

	let config: BridgeConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");

	match &cli.command {
		Some(Commands::ReplayBlock { list }) =>
			if *list {
				println!("Printing testing lists...");
			} else {
				println!("Not printing testing lists...");
			},
		Some(Commands::Run {}) | None => {
			commands::run(config).await;
		},
	}
}
