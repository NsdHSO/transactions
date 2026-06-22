use clap::Parser;
use release_notes::{format_release_notes, PullRequest};

#[derive(Parser)]
#[command(name = "release-notes")]
struct Args {
    /// Tag or version to collect notes since.
    #[arg(long)]
    since: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();

    let input = std::io::read_to_string(std::io::stdin())?;
    let prs: Vec<PullRequest> = serde_json::from_str(&input)?;
    println!("{}", format_release_notes(&prs));
    Ok(())
}
