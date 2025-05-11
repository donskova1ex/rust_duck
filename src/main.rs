use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short='i', global = true, help = "Creating/Initializing migration directory")]
    init_short: bool,
    #[arg(short='c', global = true, value_name = "NAME", help = "Generates a new migrations")]
    create_short: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Creating/Initializing migration directory", alias = "-i")]
    Init,
    #[command(about = "Creating migration file (example: create_users_table)")]
    Create { name: String },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    
    if cli.init_short {
        println!("Initializing migration directory");
        return;
    }
    
    if let Some(name) = cli.create_short {
        println!("Creating migration file {}", name);
    }
    
    match &cli.command { 
        Some(cmd) => match cmd { 
            Commands::Init => println!("Initializing migration directory.. init"),
            Commands::Create { name } => println!("Creating migration file {}", name),
        },
        None => println!("Error: Missing command. Use --help to see available options."),
    }
}
