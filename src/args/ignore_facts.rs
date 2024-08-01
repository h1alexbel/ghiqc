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
use std::collections::HashMap;

/// Parse facts from list of strings (lines).
// @todo #27:30min Read ignore.ghiqc file as list of strings.
//  We should read file first in the main.rs, and then pass list of strings in
//  parse_facts function.
pub fn parse_facts(content: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut facts: HashMap<String, Vec<String>> = HashMap::new();
    let mut author = vec![];
    let mut label = vec![];
    let mut title = vec![];
    for line in content {
        if line.starts_with("author:") {
            author.push(String::from(&line[7..]));
        } else if line.starts_with("label:") {
            label.push(String::from(&line[6..]));
        } else if line.starts_with("title:") {
            title.push(String::from(&line[6..]));
        } else {
            panic!("Parsing error in line {}: unsupported syntax", line);
        }
    }
    facts.insert(String::from("author"), author);
    facts.insert(String::from("label"), label);
    facts.insert(String::from("title"), title);
    facts
}

/// Ignore title?
// @todo #27:35min Implement ignores_author.
//  Let's do this similarly to ignores_title function. Check
//  README.md#ignore-issues-from-checking for more information about the rules.
//  Don't forget to remove this puzzle.
// @todo #27:35min Implement ignores_label.
//  Let's do this similarly to ignores_title function. Check
//  README.md#ignore-issues-from-checking for more information about the rules.
//  Don't forget to remove this puzzle.
pub fn ignores_title(
    issue: String,
    facts: HashMap<String, Vec<String>>,
) -> bool {
    facts
        .get("title")
        .expect("Failed to obtain title facts")
        .iter()
        .any(|f| {
            if f.starts_with('!') {
                if f[1..].starts_with('*') {
                    !issue.starts_with(&f[2..])
                } else {
                    !issue.eq(&f[1..])
                }
            } else {
                if f.starts_with('*') {
                    issue.starts_with(&f[1..])
                } else {
                    issue.eq(f)
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    use crate::args::ignore_facts::{ignores_title, parse_facts};

    #[allow(clippy::unwrap_used)]
    #[test]
    fn parses_facts_in_map() -> Result<()> {
        let facts = parse_facts(vec![
            String::from("author:jeff"),
            String::from("label:enhancement"),
            String::from("title:new feature request"),
            String::from("title:!*something"),
        ]);
        let author = vec![String::from("jeff")];
        assert_that!(
            facts.get("author").unwrap().to_vec(),
            is(equal_to(author))
        );
        let label = vec![String::from("enhancement")];
        assert_that!(facts.get("label").unwrap().to_vec(), is(equal_to(label)));
        let title = vec![
            String::from("new feature request"),
            String::from("!*something"),
        ];
        assert_that!(facts.get("title").unwrap().to_vec(), is(equal_to(title)));
        Ok(())
    }

    #[test]
    #[should_panic(
        expected = "Parsing error in line invalid:something: unsupported syntax"
    )]
    fn panics_on_supported_syntax() {
        parse_facts(vec![
            String::from("author:jeff"),
            String::from("invalid:something"),
        ]);
    }

    #[test]
    fn ignores_on_title_exact_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("new feature request"),
            parse_facts(vec![String::from("title:new feature request")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    pub fn ignores_on_title_star_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("new feature request: ABC"),
            parse_facts(vec![String::from("title:*new feature request")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    pub fn ignores_on_partial_facts_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("something!"),
            parse_facts(vec![
                String::from("title:*something"),
                String::from("title:different"),
            ]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    pub fn does_not_ignore_when_unmatch() -> Result<()> {
        let ignore = ignores_title(
            String::from("different"),
            parse_facts(vec![String::from("title:exact")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    pub fn does_not_ignore_on_star_unmatch() -> Result<()> {
        let ignore = ignores_title(
            String::from("different"),
            parse_facts(vec![String::from("title:*something")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    pub fn ignores_on_inverse_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("i'm good one!"),
            parse_facts(vec![String::from("title:!bad")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    pub fn does_not_ignore_on_inverse_unmatch() -> Result<()> {
        let ignore = ignores_title(
            String::from("bad"),
            parse_facts(vec![String::from("title:!bad")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    pub fn ignores_on_inverse_star_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("good one"),
            parse_facts(vec![String::from("title:!*good")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }
}
