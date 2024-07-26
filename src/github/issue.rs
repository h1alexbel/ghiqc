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
/// GitHub issue.
#[derive(Clone)]
pub struct Issue {
    /// Repository.
    repo: String,
    /// Issue number.
    number: usize,
}

impl Issue {
    /// Issue with number.
    pub fn new(repo: String, number: usize) -> Issue {
        Issue { repo, number }
    }
}

impl Issue {
    /// Fetch issue details.
    pub async fn on_github(self, token: String) -> octocrab::models::issues::Issue {
        let parts: Vec<&str> = self.repo.split('/').collect();
        let github = Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("Cannot create GitHub client");
        let option = github
            .issues(parts[0], parts[1])
            .get(self.number as u64)
            .await;
        match option {
            Ok(response) => response,
            Err(err) => {
                panic!("Cannot fetch {}#{}: {}", self.repo, self.number, err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::github::issue::Issue;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn creates_issue_struct() -> Result<()> {
        let number = 1;
        let repo = "jeff/foo";
        let issue = Issue::new(String::from(repo), number);
        assert_that!(issue.number, is(equal_to(number)));
        assert_that!(issue.repo, is(equal_to(String::from(repo))));
        Ok(())
    }
}
