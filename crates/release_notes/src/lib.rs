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

pub fn format_release_note(pr: &PullRequest) -> String {
    format!("- {} (#{})", pr.title, pr.number)
}

pub fn format_release_notes(prs: &[PullRequest]) -> String {
    prs.iter()
        .map(format_release_note)
        .collect::<Vec<_>>()
        .join("\n")
}
