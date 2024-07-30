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

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::args::ignore_facts::parse_facts;
    use hamcrest::{equal_to, is, HamcrestMatcher};

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
            String::from("!*something")
        ];
        assert_that!(facts.get("title").unwrap().to_vec(), is(equal_to(title)));
        Ok(())
    }

    #[test]
    fn panics_on_supported_syntax() -> Result<()> {
        let facts = parse_facts(
            vec![
                String::from("author:jeff"),
                String::from("invalid:something")
            ]
        );
        Ok(())
    }
}
