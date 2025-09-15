use clap::{Parser, ValueEnum};
use inquire::Select;

#[derive(Parser)]
struct Cli {
    /// Command to run (optional, will prompt if missing)
    #[arg(value_enum)]
    command: Option<Command>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Command {
    Foo,
    Bar,
    Baz,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Foo => f.write_str("Foo"),
            Command::Bar => f.write_str("Bar"),
            Command::Baz => f.write_str("Baz"),
        }
    }
}

fn main() {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    let dev_dir = format!("{}/dev/euc", home);

    let repos = std::fs::read_dir(&dev_dir)
        .expect("reading directory")
        .map(|e| {
            e.expect("entry to be valid")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();

    for e in repos {
        println!("{}", e);
    }

    let args = Cli::parse();

    let command = match args.command {
        Some(cmd) => cmd,
        None => Select::new(
            "Choose a command:",
            vec![Command::Foo, Command::Bar, Command::Baz],
        )
        .prompt()
        .expect("Selection failed"),
    };

    println!("Selected: {:?}", command);
}
