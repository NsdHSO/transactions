use release_notes::{PullRequest, format_release_note, format_release_notes};

fn pr(number: u64, title: &str) -> PullRequest {
    PullRequest {
        number,
        title: title.to_string(),
        body: None,
        merged_at: None,
        labels: vec![],
    }
}

#[test]
fn test_format_release_note() {
    let p = pr(42, "Fix crash on startup");
    assert_eq!(
        format_release_note(&p, "https://github.com/NsdHSO/transactions"),
        "- Fix crash on startup ([#42](https://github.com/NsdHSO/transactions/pull/42))"
    );
}

#[test]
fn test_format_release_note_trailing_slash() {
    let p = pr(42, "Fix crash on startup");
    assert_eq!(
        format_release_note(&p, "https://github.com/NsdHSO/transactions/"),
        "- Fix crash on startup ([#42](https://github.com/NsdHSO/transactions/pull/42))"
    );
}

#[test]
fn test_format_release_notes() {
    let prs = vec![pr(1, "Add feature A"), pr(2, "Fix bug B")];
    assert_eq!(
        format_release_notes(&prs, "https://github.com/example/repo"),
        "- Add feature A ([#1](https://github.com/example/repo/pull/1))\n\
- Fix bug B ([#2](https://github.com/example/repo/pull/2))"
    );
}

#[test]
fn test_deserialize_pr() {
    let json = r#"{
        "number": 7,
        "title": "Update docs",
        "body": null,
        "merged_at": "2026-06-22T10:00:00Z",
        "labels": [{"name": "documentation"}]
    }"#;
    let pr: PullRequest = serde_json::from_str(json).unwrap();
    assert_eq!(pr.number, 7);
    assert_eq!(pr.title, "Update docs");
    assert_eq!(pr.labels[0].name, "documentation");
}
