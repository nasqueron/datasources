use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

/*  -------------------------------------------------------------
    Regexp definitions, used in builder
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

lazy_static! {
    static ref RE_KEY: Regex = Regex::new(
        // %%key%%
        r"%%(.*?)%%"
    ).unwrap();
}

/*  -------------------------------------------------------------
    Language

    Each language entry from the registry is a key/value map.
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Debug)]
pub struct Language {
    pub fields: HashMap<String, Vec<String>>,
}

impl Language {

    ///
    /// Parser
    ///

    pub fn parse_document(document: &str, restrict_to_language: bool) -> Vec<Self> {
        document
            .split("\n%%\n")
            .skip(1) // Metadata File-Date: <date>
            .filter(|&entry| !restrict_to_language || entry.contains("Type: language"))
            .map(|entry| Self::parse_entry(entry))
            .collect()
    }

    pub fn parse_entry(entry: &str) -> Self {
        let mut fields = HashMap::new();

        let mut key = String::new();
        let mut value= String::new();
        let mut has_value = false;

        // Pitfall: some values can extend to several lines
        for line in entry.split("\n") {
            if line.contains(": ") {
                // Save previous value
                if has_value {
                    fields
                        .entry(key)
                        .or_insert(Vec::new())
                        .push(value);
                }

                // <key>: <value> line
                let mut tokens = line.splitn(2, ": ");
                key = String::from(tokens.next().unwrap());
                value =  String::from(tokens.next().unwrap());
                has_value = true;
            } else {
                // Multiline value. Append the line to previous value.
                value = format!("{} {}", &value.trim(), line.trim())
            }
        }
        if has_value {
            fields
                .entry(key)
                .or_insert(Vec::new())
                .push(value);
        }

        Self {
            fields,
        }
    }

    ///
    /// Builder
    ///

    pub fn get_field(&self, tag: &str, separator: &str) -> Option<String> {
        self.fields
            .get(tag)
            .map(|values| values.join(separator))
    }

    pub fn get_id(&self) -> Option<String> {
        self.get_field("Subtag", "-")
            .or_else(|| self.get_field("Tag", "-"))
    }

    pub fn build_full_description(&self, separator: &str) -> String {
        let mut full_description = self.get_field("Description", separator)
            .unwrap_or("<no description in IANA registry>".to_string());

        if self.fields.contains_key("Deprecated") {
            full_description.push_str(" [deprecated]");
        }

        if let Some(should_use) = self.get_field("Preferred-Value", separator) {
            full_description.push_str("; preferred value: ");
            full_description.push_str(&should_use);

        }

        if let Some(comments) = self.get_field("Comments", separator) {
            full_description.push_str("; ");
            full_description.push_str(&comments);
        }



        full_description
    }

    pub fn format(&self, format: &str, separator: &str) -> String {
        let mut formatted = String::from(format);

        if formatted.contains("%%id%%") {
            let id = self.get_id().unwrap_or(String::new());
            formatted = formatted.replace("%%id%%", &id);
        }

        if formatted.contains("%%fulldescription%%") {
            let description = self.build_full_description(separator);
            formatted = formatted.replace("%%fulldescription%%", &description);
        }

        for (key , values) in &self.fields {
            let value = values.join(separator);

            formatted = formatted.replace(
                &format!("%%{}%%", &key),
                &value
            );
        }

        RE_KEY
            .replace_all(&formatted, "")
            .to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_format() {
        let liquids = vec!["Water".to_string(), "Air".to_string()];

        let mut fields = HashMap::new();
        fields.insert("Liquid".to_string(), liquids);
        fields.insert("Model".to_string(), vec!["Newtonian".to_string()]);

        let language = Language { fields };

        assert_eq!(
            "Water or Air use Newtonian physic.",
            &language.format("%%Liquid%% use %%Model%% physic.", " or ")
        );

        assert_eq!(
            "Water or Air use Newtonian physic.",
            &language.format("%%Liquid%% use %%Prefix%%%%Model%% physic.", " or ")
        );

        assert_eq!(
            "", &language.format("", "")
        );
    }
}
