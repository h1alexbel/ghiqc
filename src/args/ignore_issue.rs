use crate::args::ignore_facts::{ignores_dim, ignores_title};
use crate::args::ignore_file::IgnoreFile;
use crate::github::github_issue::GithubIssue;

/// Ignore issue?
/// * `issue`: Issue
/// * `file`: File with facts
pub fn ignore_issue(issue: GithubIssue, file: IgnoreFile) -> bool {
    let facts = file.facts();
    let mut result = false;
    if ignores_title(issue.clone().title(), facts.clone()) {
        result = true
    } else if ignores_dim(
        issue.author(),
        String::from("author"),
        facts.clone(),
    ) {
        result = true
    } else if ignores_dim(
        issue.labels(),
        String::from("label"),
        facts,
    ) {
        result = true
    }
    result
}
