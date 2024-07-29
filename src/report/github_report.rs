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
use crate::report::report::Report;
use log::debug;
use octocrab::Octocrab;
use tracing::info;

/// GitHub report in issue comments.
#[derive(Clone)]
pub struct GithubReport {
    /// GitHub.
    github: Octocrab,
    /// Repo.
    repo: String,
    /// Issue number.
    issue: usize,
}

impl GithubReport {
    /// New GitHub report.
    pub fn new(github: Octocrab, repo: String, issue: usize) -> GithubReport {
        GithubReport {
            github,
            repo,
            issue,
        }
    }
}

impl Report for GithubReport {
    async fn publish(self, text: String) {
        debug!("Publishing to {}#{}...", self.repo, self.issue);
        let parts: Vec<&str> = self.repo.split('/').collect();
        self.github
            .issues(parts[0], parts[1])
            .create_comment(self.issue as u64, text)
            .await
            .expect("Cannot post report to the GitHub issue comments");
        info!(
            "Report should be delivered to {}#{}!",
            self.repo, self.issue
        );
    }
}
