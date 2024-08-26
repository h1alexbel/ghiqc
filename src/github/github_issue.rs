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
use crate::github::issue::Issue;
use octocrab::Octocrab;

/// GitHub issue.
#[derive(Clone)]
pub struct GithubIssue {
    origin: octocrab::models::issues::Issue,
}

impl GithubIssue {
    /// GitHub issue from origin.
    pub async fn new(origin: Issue, github: Octocrab) -> GithubIssue {
        let issue = origin.on_github(github).await;
        GithubIssue { origin: issue }
    }
}

impl GithubIssue {
    /// Issue number.
    pub fn number(self) -> u64 {
        self.origin.number
    }

    /// Issue body.
    pub fn body(self) -> String {
        self.origin
            .body
            .expect("Cannot parse issue body. Probably its NULL.")
    }

    /// GitHub nickname of issue author.
    pub fn author(self) -> String {
        self.origin.user.login
    }

    /// Issue title.
    pub fn title(self) -> String {
        self.origin.title
    }
}
