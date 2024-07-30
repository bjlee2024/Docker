use std::process::{Command, exit};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run { command: Vec<String> },
    Child { command: Vec<String> },
}

fn main() {
    println!(
        "This process is running on {}/{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { command }) => run(command),
        Some(Commands::Child { command }) => child(command),
        None => {
            println!("Usage: cargo run -- <run|child> <command>");
            exit(1);
        }
    }
}

fn run(args: Vec<String>) {
    println!("Running {:?} as PID {}", args, std::process::id());

    let mut command = Command::new(std::env::current_exe().unwrap());
    command.arg("child").args(&args);

    let mut child = command.spawn().expect("Failed to spawn child process");
    let status = child.wait().expect("Failed to wait for child process");

    exit(status.code().unwrap_or(1));
}

fn child(args: Vec<String>) {
    println!("Running {:?} as PID {}", args, std::process::id());

    let mut command = Command::new(&args[0]);
    command.args(&args[1..]);

    let status = command.status().expect("Failed to execute command");

    exit(status.code().unwrap_or(1));
}
