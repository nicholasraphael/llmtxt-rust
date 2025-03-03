mod errors;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMSTxt {
    pub title: String,
    pub summary: String,
    pub info: String,
    pub sections: Vec<Section>,
}

impl LLMSTxt {
    pub fn new() -> LLMSTxt {
        LLMSTxt {
            title: String::new(),
            summary: String::new(),
            info: String::new(),
            sections: vec![],
        }
    }

    pub fn to_json(self: &Self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // TODO: improve error handling
    pub fn parse_meta(self: &mut Self, txt: &str) {
        let meta_re = Regex::new(
            r"(?m)^\s*#\s*(?P<title>.+?)\s*\n+\s*(?:^>\s*(?P<summary>.+?)\s*\n+)?(?P<info>[\s\S]*)",
        )
        .unwrap();
        if let Some(caps) = meta_re.captures(txt) {
            let title = caps.name("title").map_or("", |m| m.as_str());
            let summary = caps.name("summary").map_or("", |m| m.as_str());
            let info = caps.name("info").map_or("", |m| m.as_str());

            self.title = title.to_string();
            self.summary = summary.to_string();
            self.info = info.to_string();
        } else {
            return;
        }
    }

    // TODO: improve error handling, handle mutiple # headings
    pub fn parse(self: &mut Self, sections: &str) -> Result<(), errors::ParseLLMSError> {
        // TODO: refactor get_sections
        let sections_re = Regex::new(r"(?m)^[ \t]*##[ \t]*(.*?)$").unwrap();

        let parts: Vec<&str> = sections_re.split(sections).collect();

        let captures: Vec<String> = sections_re
            .captures_iter(sections)
            .map(|cap| cap[1].to_string())
            .collect();

        let mut result = Vec::new();
        result.push(parts[0]);

        for i in 0..captures.len() {
            result.push(captures[i].as_str());
            if i + 1 < parts.len() {
                result.push(parts[i + 1]);
            }
        }

        let start = &result[0];
        self.parse_meta(start);

        let rest = &result[1..];

        // TODO: refactor collect_sections

        let mut section_map = HashMap::new();
        for chunk in rest.chunks(2) {
            if chunk.len() == 2 {
                section_map.insert(chunk[0], chunk[1]);
            }
        }

        for (_, value) in section_map.into_iter() {
            let mut section = Section { links: Vec::new() };
            match self.parse_links(value) {
                Ok(links) => {
                    section.links = links;
                }
                Err(error) => return Err(error),
            }
            self.sections.push(section);
        }

        Ok(())
    }

    // TODO: improve error handling
    pub fn parse_links(self: &Self, txt: &str) -> Result<Vec<Link>, errors::ParseLLMSError> {
        let link_re =
            Regex::new(r"-\s*\[(?P<title>[^\]]+)\]\((?P<url>[^\)]+)\)(?::\s*(?P<desc>.*?))?")
                .unwrap();

        let mut links = Vec::new();
        for cap in link_re.captures_iter(txt) {
            let title = &cap["title"];
            let url = &cap["url"];
            let desc = cap.name("desc").map(|m| m.as_str()).unwrap_or("");

            let link = Link {
                title: title.to_string(),
                url: url.to_string(),
                description: desc.to_string(),
            };
            links.push(link);
        }

        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_links() {
        let link_str = r#"
            - [Link title](https://link_url): Optional link details
            - [foo2](http://foo2): stuff"
        "#;
        let llmp = LLMSTxt::new();
        let links = match llmp.parse_links(&link_str) {
            Ok(links) => links,
            Err(error) => panic!("{:?}", error),
        };
        assert_eq!(links[0].title, "Link title");
        assert_eq!(links[0].url, "https://link_url");
    }

    #[test]
    fn parse_sections() {
        let sections = r#"
            # First bit.

            Some other

            ## S1

            - [foo](http://foo)
            - [foo2](http://foo2): stuff

            ## S2

            - [foo3](http://foo3)
        "#;
        let mut llmp = LLMSTxt::new();
        match llmp.parse(&sections) {
            Ok(()) => (),
            Err(error) => panic!("{:?}", error),
        };
        assert!(llmp.sections.len() > 0);
        assert!(llmp.sections[0].links[0].title.contains("foo"));
        assert_eq!(llmp.title, "First bit.".to_string());
    }
}
