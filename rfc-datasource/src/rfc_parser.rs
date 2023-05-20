use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

/*  -------------------------------------------------------------
    Regexp definitions, used in parser and builder
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

lazy_static!{
    static ref RE_RFC: Regex = Regex::new(
        // <id> <description> <metadata...>
        r"(\d+) (.*?) (\(.*\))"
    ).unwrap();

    static ref RE_RFC_METADATA: Regex = Regex::new(
        // (...) (...) (...)
        r"\((.*?)\)"
    ).unwrap();

    static ref RE_ID: Regex = Regex::new(
        // %%9id%%
        r"\%(\d+)id\%"
    ).unwrap();
}

/*  -------------------------------------------------------------
    RFC
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Debug)]
pub struct Rfc {
    pub id: i32,
    pub description: String,

    pub metadata: HashMap<String, String>,
    pub untagged_metadata: Vec<String>,
}

impl Rfc {

    ///
    /// Parser
    ///

    pub fn parse_document(document: &str) -> Vec<Self> {
        let lines: Vec<_> = document.lines().collect();

        let start_index = lines
            .iter()
            .position(|&line| line.starts_with("0001"))
            .unwrap_or(0);

        let document = lines[start_index..].join("\n");

        Self::parse_blocks(&document)
    }

    fn parse_blocks(document: &str) -> Vec<Self> {
        document
            .split("\n\n")
            .map(|block| Self::parse_block(block))
            .filter(|rfc| rfc.is_some())
            .map(|rfc| rfc.unwrap())
            .collect()
    }

    pub fn parse_block(block: &str) -> Option<Self> {
        let rfc_expression: Vec<&str> = block
            .split("\n")
            .map(|line| line.trim_start())
            .collect();

        Self::parse_line(&rfc_expression.join(" "))
    }

    fn parse_line(line: &str) -> Option<Self> {
        match RE_RFC.captures(line) {
            None => None,

            Some(caps) => {
                match caps.len() {
                    4 => {
                        let (metadata, untagged_metadata) = Self::parse_metadata_line(
                            caps.get(3)?.as_str()
                        );

                        Some(Rfc {
                            id: caps.get(1)?.as_str().parse::<i32>().ok()?,
                            description: caps.get(2)?.as_str().to_string(),
                            metadata,
                            untagged_metadata,
                        })
                    },
                    _ => None,
                }
            }
        }
    }

    fn parse_metadata_line(expression: &str) -> (HashMap<String, String>, Vec<String>) {
        let mut metadata = HashMap::new();
        let mut untagged_metadata = Vec::new();

        RE_RFC_METADATA
            .captures_iter(expression)
            .map(|cap| cap.get(1).unwrap().as_str())
            .for_each(|value| {
                if value.contains(":") {
                    let parts: Vec<_> = value.splitn(2, ": ").collect(); // K: V
                    metadata.insert(parts[0].to_owned(), parts[1].to_owned());
                } else {
                    untagged_metadata.push(String::from(value));
                }
            });

        (metadata, untagged_metadata)
    }

    ///
    /// Builder
    ///

    pub fn get_status (&self) -> Option<String> {
        self.metadata
            .get("Status")
            .map(|value| String::from(value))
    }

    pub fn get_full_status_metadata (&self) -> Vec<String> {
        let mut all_metadata: Vec<String> = self.untagged_metadata
            .iter()
            .map(|value| format!("{}.", value))
            .collect();

        all_metadata.extend(
            self.metadata
                .iter()
                .filter(|&(key, _value)| key != "DOI" && key != "Format")
                .map(|(key, value)| format!("{}: {}.", key, value))
        );

        all_metadata
    }

    pub fn get_full_status (&self) -> String {
        self.get_full_status_metadata()
            .join(" ")
    }

    ///
    /// Format
    ///

    pub fn format(&self, format: &str) -> String {
        // Replace expressions like %%4id%% %%5id%%
        let matches = RE_ID
            .captures_iter(&format)
            .map(|caps| caps.get(1).unwrap()
                .as_str()
                .parse::<usize>().unwrap());

        let mut formatted_rfc = String::from(format);
        for len in matches {
            formatted_rfc = formatted_rfc.replace(
                &format!("%%{}id%%", len.clone()),
                &zerofill(self.id,  len.clone()),
            );
        }

        // Replace straightforward variables
        formatted_rfc
            .replace("%%id%%", &self.id.to_string())
            .replace("%%description%%", &self.description)
            .replace("%%status%%", &self.get_status().unwrap_or(String::new()))
            .replace("%%fullstatus%%", &self.get_full_status())
    }
}

/*  -------------------------------------------------------------
    Helper methods
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

fn zerofill(number: i32, width: usize) -> String {
    format!("{:0>width$}", number, width = width)
}

/*  -------------------------------------------------------------
    Unit tests
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_zerofill () {
        // Test case 1: number is smaller than width (usual case)
        assert_eq!(zerofill(42, 5), "00042");

        // Test case 2: number is equal to width
        assert_eq!(zerofill(12345, 5), "12345");

        // Test case 3: number is larger than width
        assert_eq!(zerofill(987654, 4), "987654");

        // Test case 4: number is zero
        assert_eq!(zerofill(0, 3), "000");

        // Test case 5: width is zero
        assert_eq!(zerofill(987, 0), "987");
    }

}
