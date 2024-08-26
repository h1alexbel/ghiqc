// The MIT License (MIT)
//
// Copyright (c) 2024 Aliaksei Bialiauski
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use crate::args::ignore_facts::{ignores_dim, ignores_title};
use crate::args::ignore_file::IgnoreFile;
use crate::github::github_issue::GithubIssue;

/// Ignore issue?
/// * `issue`: Issue
/// * `file`: File with facts
#[allow(clippy::if_same_then_else)]
pub fn ignore_issue(issue: GithubIssue, file: IgnoreFile) -> bool {
    let facts = file.facts();
    let mut result = false;
    let labels = issue.clone().labels();
    for label in labels {
        if ignores_dim(label, String::from("label"), facts.clone()) {
            result = true
        }
    }
    if ignores_title(issue.clone().title(), facts.clone()) {
        result = true
    } else if ignores_dim(issue.author(), String::from("author"), facts.clone())
    {
        result = true
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::args::ignore_file::IgnoreFile;
    use crate::args::ignore_issue::ignore_issue;
    use crate::github::github::github;
    use crate::github::github_issue::GithubIssue;
    use crate::github::issue::Issue;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    // @todo #36:30min Enable returns_true_on_ignore test when fake GitHub
    //  instance will be ready. After fake GitHub instance will be connected,
    //  we should pass fake credentials and repo information in order to test
    //  #ignore_issue.
    #[ignore]
    #[tokio::test]
    async fn returns_true_on_ignore() -> Result<()> {
        let issue = GithubIssue::new(
            Issue::new(String::from("jeff/foo"), 1),
            github(String::from("<gh token here>")),
        )
        .await;
        let file = IgnoreFile::new(String::from("ignore.ghiqc"));
        let ignore = ignore_issue(issue, file);
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }
}
