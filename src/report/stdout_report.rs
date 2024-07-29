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
use log::{debug, info};

/// Report to stdout.
pub struct StdoutReport {}

impl StdoutReport {
    /// New stdout report.
    pub fn new() -> StdoutReport {
        StdoutReport {}
    }
}

impl Default for StdoutReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for StdoutReport {
    async fn publish(self, text: String) {
        debug!("Printing response to the stdout...");
        info!("Answer: {}", text);
    }
}

#[cfg(test)]
mod tests {
    use crate::report::report::Report;
    use crate::report::stdout_report::StdoutReport;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[tokio::test]
    async fn reports_to_std() -> Result<()> {
        testing_logger::setup();
        let report = StdoutReport::new();
        let dummy = "dummy text";
        report.publish(String::from(dummy)).await;
        testing_logger::validate(|logs| {
            assert_that!(logs.len(), is(equal_to(2)));
            assert_that!(
                &logs[0].body,
                is(equal_to("Printing response to the stdout..."))
            );
            assert_that!(
                logs[1].body.clone(),
                is(equal_to(format!("Answer: {}", dummy)))
            );
        });
        Ok(())
    }
}
