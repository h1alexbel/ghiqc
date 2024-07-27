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
/*!
Application entry point.
 */
#[allow(unused_imports)]
#[macro_use]
extern crate hamcrest;
use crate::args::cli::Cli;
use crate::args::env::env;
use crate::github::github_issue::GithubIssue;
use crate::github::issue::Issue;
use crate::probe::deep_infra_request::{ProbeDeepInfra, ProbeMessage};
use crate::probe::probe_request::ProbeRequest;
use clap::Parser;
use log::{debug, info};

/// Arguments.
pub mod args;
/// GitHub.
pub mod github;
/// Probes.
pub mod probe;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if args.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }
    debug!("Issue to check: {} ({} repo)", args.issue, args.repo);
    debug!("STDOUT: {}", args.stdout);
    debug!("Reading GITHUB_TOKEN from environment...");
    let ghtoken = env(String::from("GITHUB_TOKEN"));
    debug!("Reading DEEPINFRA_TOKEN from environment...");
    let deeptoken = env(String::from("DEEPINFRA_TOKEN"));
    let issue =
        GithubIssue::new(Issue::new(args.repo, args.issue), ghtoken).await;
    info!(
        "Issue says (created by @{}): {}",
        issue.clone().author(),
        issue.clone().body()
    );
    let response = ProbeDeepInfra::new(
        String::from("https://api.deepinfra.com/v1/openai/chat/completions"),
        deeptoken,
    )
    .complete(vec![ProbeMessage::new(
        String::from("user"),
        // @todo #7:30min Develop a prompt as probe message.
        //  Let's create a prompt for a probe message that would
        //  analyze quality of given bug report.
        String::from("Hello!"),
    )])
    .await;
    info!("{}", response);
}
