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
use octocrab::Octocrab;

use crate::report::github_report::GithubReport;
use crate::report::report::Report;
use crate::report::stdout_report::StdoutReport;

/// Report fork.
pub struct ReportFork {
    fork: bool,
    github: Octocrab,
    repo: String,
    issue: usize,
}

impl ReportFork {
    /// New fork.
    // @todo #9:35min Ctor has too many arguments.
    //  We have to pass too many arguments into ReportFork. Some arguments are
    //  even redundant in some cases: `github`, `repo`, and `issue`. Let's find
    //  out how to refactor this code.
    pub fn new(
        fork: bool,
        github: Octocrab,
        repo: String,
        issue: usize,
    ) -> ReportFork {
        ReportFork {
            fork,
            github,
            repo,
            issue,
        }
    }
}

impl Report for ReportFork {
    async fn publish(self, response: String) {
        if self.fork {
            StdoutReport::new().publish(response).await;
        } else {
            GithubReport::new(self.github, self.repo, self.issue)
                .publish(response)
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::github::github::github;
    use crate::report::report::Report;
    use crate::report::report_fork::ReportFork;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[tokio::test]
    async fn forks_stdout_on_true() -> Result<()> {
        let dummy = String::from("this is dummy text");
        testing_logger::setup();
        ReportFork::new(
            true,
            github(String::from("dummy")),
            String::from("jeff/foo"),
            1,
        )
        .publish(dummy.clone())
        .await;
        testing_logger::validate(|logs| {
            assert_that!(
                logs[2].body.clone(),
                is(equal_to(format!("Answer: {}", dummy)))
            );
        });
        Ok(())
    }
}
