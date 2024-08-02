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
        if let Some(value) = line.strip_prefix("author:") {
            author.push(String::from(value));
        } else if let Some(value) = line.strip_prefix("label:") {
            label.push(String::from(value));
        } else if let Some(value) = line.strip_prefix("title:") {
            title.push(String::from(value));
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
pub fn ignores_title(
    issue: String,
    facts: HashMap<String, Vec<String>>,
) -> bool {
    facts
        .get("title")
        .expect("Failed to obtain title facts")
        .iter()
        .any(|f| {
            if let Some(value) = f.strip_prefix('!') {
                if let Some(start) = value.strip_prefix('*') {
                    !issue.starts_with(start)
                } else {
                    !issue.eq(value)
                }
            } else if let Some(star) = f.strip_prefix('*') {
                issue.starts_with(star)
            } else {
                issue.eq(f)
            }
        })
}

fn contains_in_array(array: &str, value: String) -> bool {
    let inside = &array.replace(']', "");
    let mut values = inside.split(',');
    values.any(|f| f.eq(&value))
}

/// Ignore issue in dimension `dim`?
pub fn ignores_dim(
    issue: String,
    dim: String,
    facts: HashMap<String, Vec<String>>,
) -> bool {
    facts
        .get(&dim)
        .unwrap_or_else(|| panic!("Failed to obtain {} facts", dim))
        .iter()
        .any(|f| {
            if let Some(value) = f.strip_prefix('!') {
                if let Some(array) = value.strip_prefix('[') {
                    !contains_in_array(array, issue.clone())
                } else {
                    !issue.eq(value)
                }
            } else if let Some(value) = f.strip_prefix('[') {
                contains_in_array(value, issue.clone())
            } else {
                issue.eq(f)
            }
        })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    use crate::args::ignore_facts::{ignores_dim, ignores_title, parse_facts};

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
    fn ignores_on_title_star_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("new feature request: ABC"),
            parse_facts(vec![String::from("title:*new feature request")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    fn ignores_on_partial_facts_match() -> Result<()> {
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
    fn does_not_ignore_when_unmatch() -> Result<()> {
        let ignore = ignores_title(
            String::from("different"),
            parse_facts(vec![String::from("title:exact")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn does_not_ignore_on_star_unmatch() -> Result<()> {
        let ignore = ignores_title(
            String::from("different"),
            parse_facts(vec![String::from("title:*something")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn ignores_on_inverse_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("i'm good one!"),
            parse_facts(vec![String::from("title:!bad")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    fn does_not_ignore_on_inverse_unmatch() -> Result<()> {
        let ignore = ignores_title(
            String::from("bad"),
            parse_facts(vec![String::from("title:!bad")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn ignores_on_inverse_star_match() -> Result<()> {
        let ignore = ignores_title(
            String::from("good one"),
            parse_facts(vec![String::from("title:!*good")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn ignores_author_on_exact_match() -> Result<()> {
        let ignore = ignores_dim(
            String::from("jeff"),
            String::from("author"),
            parse_facts(vec![String::from("author:jeff")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    fn skips_author_on_inverse_match() -> Result<()> {
        let ignore = ignores_dim(
            String::from("jeff"),
            String::from("author"),
            parse_facts(vec![String::from("author:!jeff")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn skips_on_inverse_array_match() -> Result<()> {
        let ignore = ignores_dim(
            String::from("jeff"),
            String::from("author"),
            parse_facts(vec![String::from("author:![jeff,foo,bar]")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn ignores_on_array_match() -> Result<()> {
        let ignore = ignores_dim(
            String::from("jeff"),
            String::from("author"),
            parse_facts(vec![String::from("author:[foo,bar,jeff]")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    fn ignores_label_too() -> Result<()> {
        let ignore = ignores_dim(
            String::from("enhancement"),
            String::from("label"),
            parse_facts(vec![String::from("label:enhancement")]),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    fn skips_label() -> Result<()> {
        let ignore = ignores_dim(
            String::from("bug"),
            String::from("label"),
            parse_facts(vec![String::from("label:!bug")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn skips_on_empty_dim_facts() -> Result<()> {
        let ignore = ignores_dim(
            String::from("foo"),
            String::from("author"),
            parse_facts(vec![String::from("label:none")]),
        );
        assert_that!(ignore, is(equal_to(false)));
        Ok(())
    }
}
