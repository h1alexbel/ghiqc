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
use anyhow::Result;
use assert_cmd::Command;
use std::str;

#[test]
#[allow(clippy::question_mark_used)]
fn outputs_help() -> Result<()> {
    let assertion = Command::cargo_bin("ghiqc")?.arg("--help").assert();
    let bytes = assertion.get_output().stdout.as_slice();
    let output = str::from_utf8(bytes)?;
    assert!(output.contains("--help"));
    assert!(output.contains("Print help"));
    assert!(output.contains("--repo"));
    assert!(output.contains("-r"));
    assert!(output.contains("--issue"));
    assert!(output.contains("-i"));
    assert!(output.contains("--stdout"));
    assert!(output.contains("--verbose"));
    assert!(output.contains("-v"));
    Ok(())
}

#[test]
#[allow(clippy::question_mark_used)]
#[cfg_attr(target_os = "windows", ignore)]
fn outputs_usage() -> Result<()> {
    let assertion = Command::cargo_bin("ghiqc")?.arg("--help").assert();
    let bytes = assertion.get_output().stdout.as_slice();
    let output = str::from_utf8(bytes)?;
    assert!(
        output.contains("Usage: ghiqc [OPTIONS] --repo <REPO> --issue <ISSUE>")
    );
    Ok(())
}
