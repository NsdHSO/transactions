use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub merged_at: Option<String>,
    pub labels: Vec<Label>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub name: String,
}

/// Format one PR line as Markdown: `- <title> ([#<n>](<repo>/pull/<n>))`.
/// `repo_url` is normalized by stripping a trailing `/` so callers may pass
/// either `https://github.com/O/R` or `https://github.com/O/R/`.
pub fn format_release_note(pr: &PullRequest, repo_url: &str) -> String {
    let base = repo_url.trim_end_matches('/');
    format!(
        "- {} ([#{}]({}/pull/{}))",
        pr.title, pr.number, base, pr.number
    )
}

/// Format many PRs joined by newlines.
pub fn format_release_notes(prs: &[PullRequest], repo_url: &str) -> String {
    prs.iter()
        .map(|p| format_release_note(p, repo_url))
        .collect::<Vec<_>>()
        .join("\n")
}
