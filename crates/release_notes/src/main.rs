use clap::Parser;
use release_notes::{PullRequest, format_release_notes};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(
    name = "release-notes",
    about = "Format merged PRs as Markdown release notes; optionally publish a GitHub release."
)]
struct Args {
    /// Publish a GitHub release with this tag. If omitted, notes are printed to stdout.
    #[arg(long, value_name = "TAG")]
    publish: Option<String>,

    /// Repository URL (defaults to `git remote get-url origin`).
    #[arg(long, value_name = "URL")]
    repo: Option<String>,
}

fn resolve_repo(cli: Option<String>) -> anyhow::Result<String> {
    if let Some(url) = cli {
        return Ok(url);
    }
    let out = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()?;
    if !out.status.success() {
        anyhow::bail!(
            "could not determine repository URL via `git remote get-url origin`: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}

fn pipe_to_gh_release(tag: &str, notes: &str) -> anyhow::Result<()> {
    let mut child = Command::new("gh")
        .args(["release", "create", tag, "-t", tag, "-F", "-"])
        .stdin(Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().expect("gh stdin not captured");
        stdin.write_all(notes.as_bytes())?;
    }
    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("`gh release create {tag}` exited with: {status}");
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let repo = resolve_repo(args.repo)?;

    let input = std::io::read_to_string(std::io::stdin())?;
    let prs: Vec<PullRequest> = serde_json::from_str(&input)?;
    let notes = format_release_notes(&prs, &repo);

    if let Some(tag) = args.publish {
        pipe_to_gh_release(&tag, &notes)
    } else {
        println!("{notes}");
        Ok(())
    }
}
