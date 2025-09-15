use clap::{Parser, Subcommand, ValueEnum};
use inquire::{MultiSelect, Select};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    P,
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

    for e in &repos {
        println!("{}", e);
    }

    let cli = Cli::parse();

    let selection = match &cli.command {
        Commands::P => MultiSelect::new("Choose repos:", repos).prompt(),
    }
    .expect("a selction");

    for repo in selection {
        let path = format!("{}/{}", dev_dir, repo);
        let cmd = format!(
            "cd {} && git stash --include-untracked && git checkout main && git pull && yarn upgrade eucalyptusvc/protobufs@latest; exec bash",
            path
        );

        std::process::Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("failed to spawn command");
    }
}
