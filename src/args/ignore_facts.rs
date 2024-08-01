use std::collections::HashMap;

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

pub fn ignores(issue: String, facts: HashMap<String, Vec<String>>) -> bool {
    let title = facts.get("title").expect("Failed to obtain title facts");
    // let collected = title.iter()
    //     .filter(
    //         |f|
    //         if f.starts_with("!") {
    //             return if f[1..].starts_with("*") {
    //                 !issue.title.starts_with(f.clone())
    //             } else {
    //                 !issue.title.eq(f.clone())
    //             };
    //         } else {
    //             if f.starts_with("*") {
    //                 issue.title.starts_with(f.clone())
    //             } else {
    //                 issue.title.eq(f.clone())
    //             }
    //         }
    //     )
    //     .collect();
    let matches_title = title.iter().any(|f| {
        if f.starts_with("!") {
            if f[1..].starts_with("*") {
                !issue.starts_with(&f[1..])
            } else {
                !issue.eq(f)
            }
        } else {
            if f.starts_with("*") {
                issue.starts_with(&f[1..])
            } else {
                issue.eq(f)
            }
        }
    });
    matches_title
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use hamcrest::{equal_to, HamcrestMatcher, is};

    use crate::args::ignore_facts::{ignores, parse_facts};

    #[test]
    fn parses_facts_in_map() -> Result<()> {
        let facts = parse_facts(
            vec![
                String::from("author:jeff"),
                String::from("label:enhancement"),
                String::from("title:new feature request"),
                String::from("title:!*something"),
            ]
        );
        let author = vec![
            String::from("jeff")
        ];
        assert_that!(facts.get("author").unwrap().to_vec(), is(equal_to(author)));
        let label = vec![
            String::from("enhancement")
        ];
        assert_that!(facts.get("label").unwrap().to_vec(), is(equal_to(label)));
        let title = vec![
            String::from("new feature request"),
            String::from("!*something"),
        ];
        assert_that!(facts.get("title").unwrap().to_vec(), is(equal_to(title)));
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Parsing error in line invalid:something: unsupported syntax")]
    fn panics_on_supported_syntax() {
        parse_facts(
            vec![
                String::from("author:jeff"),
                String::from("invalid:something"),
            ]
        );
    }

    #[test]
    fn ignores_on_title_exact_match() -> Result<()> {
        let ignore = ignores(
            String::from("new feature request"),
            parse_facts(
                vec![
                    String::from("title:new feature request"),
                    String::from("title:!*something"),
                ]
            ),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }

    #[test]
    pub fn ignores_on_title_star_match() -> Result<()> {
        let ignore = ignores(
            String::from("new feature request: ABC"),
            parse_facts(
                vec![
                    String::from("title:*new feature request")
                ]
            ),
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }
    
    #[test]
    pub fn ignores_on_partial_facts_match() -> Result<()> {
        let ignore = ignores(
            String::from("something!"),
            parse_facts(
                vec![
                    String::from("title:*something"),
                    String::from("title:different")
                ]
            )
        );
        assert_that!(ignore, is(equal_to(true)));
        Ok(())
    }
}
