use clap::{ Parser, Subcommand };

// Command enum
#[derive( Subcommand ) ]
enum Commands {
    Install,
    Uninstall,
}

// Create derive-based argument parser with subcommands
#[ derive( Parser ) ]
#[ clap( author = "Joseph Meadows", version = "0.0.1", about = "An application for managing the Parrhesia Workspace", long_about = "This is a longer about" ) ]
struct Cli {
    #[ command( subcommand ) ]
    command: Commands,
}

fn main() {
    // Parse arguments
    let args = Cli::parse();

    // Execute command
    match args.command {
        Commands::Install => {
            println!("Installing...");
        },
        Commands::Uninstall => {
            println!("Uninstalling...");
        },
    }
}