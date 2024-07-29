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
use crate::github::github_issue::GithubIssue;
use crate::probe::probe_deep_infra::ProbeMessage;

/// Assistant.
pub fn assistant(issue: GithubIssue) -> Vec<ProbeMessage> {
    vec![
        ProbeMessage::new(
            String::from("system"),
            String::from(
                "You are developer tasked to review incoming bug reports that developers are submit on GitHub Issue platform"
            ),
        ), ProbeMessage::new(
            String::from("user"),
            format!(
                "Please review the following bug report and generate a simple summary.\
                 Summary should contain 1 main quality problem related to this bug report formulation, and 1 specific suggestion on how to improve it.\
                 Summary should be short, less than 30 words.\
                 Don't generate any other info.\
                 Bug report: {}",
                issue.body()
            ),
        ),
    ]
}
