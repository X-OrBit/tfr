use regex::{escape, Captures, Regex};

#[derive(Debug, PartialEq, Eq)]
pub enum TemplateError {
    AsteriskInDirectory,
    DoubleAsterisk,
}

pub struct Template {
    pattern: Regex,
}

impl Template {
    pub fn new(pattern: &str) -> Result<Template, TemplateError> {
        if pattern.rfind('/').unwrap_or(0) > pattern.find('*').unwrap_or(pattern.len()) {
            return Err(TemplateError::AsteriskInDirectory);
        }
        if pattern.contains("**") {
            return Err(TemplateError::DoubleAsterisk);
        }

        let escaped_pattern = escape(&String::from(pattern));
        let pattern = format!(
            "^{}$",
            Regex::new(r#"(^|[^\\])\\\*"#)
                .unwrap()
                .replace_all(&escaped_pattern, |caps: &Captures| format!(
                    "{}([^/]*)",
                    &caps[1]
                ),)
                .replace(r#"\\\*"#, r#"\*"#)
        );

        Ok(Self {
            pattern: Regex::new(&pattern).unwrap(),
        })
    }

    pub fn captures<'a>(&self, string: &'a str) -> Option<Vec<&'a str>> {
        match self.pattern.captures(string) {
            None => None,
            Some(captures) => Some(
                captures
                    .iter()
                    .skip(1)
                    .map(|capture| string.get(capture.unwrap().range()).unwrap())
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let template = Template::new("word").unwrap();
        assert_eq!(template.captures("word"), Some(vec![]));
        assert_eq!(template.captures("not_word"), None);
        assert_eq!(template.captures("word_not"), None);
    }

    #[test]
    fn test_incorrect_template() {
        assert_eq!(
            Template::new("/path/to/*/*.png").err().unwrap(),
            TemplateError::AsteriskInDirectory
        );
        assert_eq!(
            Template::new("/path/to/**.png").err().unwrap(),
            TemplateError::DoubleAsterisk
        );
    }

    #[test]
    fn escaped_asterisk() {
        let template = Template::new(r#"asterisk\*asterisk"#).unwrap();
        assert_eq!(template.captures("asterisk*asterisk"), Some(vec![]));
        assert_eq!(template.captures("asterisk_asterisk"), None);
    }

    #[test]
    fn test_correct_templates() {
        assert_eq!(
            Template::new("/path/to/*.png")
                .unwrap()
                .captures("/path/to/image.png"),
            Some(vec!["image"])
        );

        let template = Template::new("path/to/some_*_filename.*").unwrap();
        assert_eq!(
            template.captures("path/to/some_A_filename.bin"),
            Some(vec!["A", "bin"])
        );
        assert_eq!(
            template.captures("path/to/some_A_filename.jpg"),
            Some(vec!["A", "jpg"])
        );
        assert_eq!(
            template.captures("path/to/some_B_filename.bin"),
            Some(vec!["B", "bin"])
        );
        assert_eq!(
            template.captures("path/to/some_B_filename.jpg"),
            Some(vec!["B", "jpg"])
        );
        assert_eq!(
            template.captures("path/to/some__filename.jpg"),
            Some(vec!["", "jpg"])
        );
        assert_eq!(template.captures("path/to/some_filename.jpg"), None);
    }
}
