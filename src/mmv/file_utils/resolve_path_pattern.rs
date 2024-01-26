use regex::{Captures, Regex};

pub fn resolve_path_pattern(path_pattern: &str, captures: Vec<&str>) -> String {
    let placement_regex = Regex::new(r#"#(\d+)"#).unwrap();

    placement_regex
        .replace_all(path_pattern, |capture: &Captures| {
            let index = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            if 1 <= index && index <= captures.len() {
                captures[index - 1]
            } else {
                path_pattern.get(capture.get(0).unwrap().range()).unwrap()
            }
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(resolve_path_pattern("", vec![]), "");
        assert_eq!(resolve_path_pattern("", vec!["capture"]), "");
        assert_eq!(resolve_path_pattern("pattern", vec![]), "pattern");
        assert_eq!(resolve_path_pattern("pattern", vec!["capture"]), "pattern");
        assert_eq!(resolve_path_pattern("#1", vec!["capture"]), "capture");
        assert_eq!(resolve_path_pattern("#1", vec![""]), "");
    }

    #[test]
    fn multiple_usage_test() {
        assert_eq!(
            resolve_path_pattern("double #1 #1", vec!["capture"]),
            "double capture capture"
        );
        assert_eq!(
            resolve_path_pattern("double #1 #2 #1", vec!["capture", "double"]),
            "double capture double capture"
        );
        assert_eq!(resolve_path_pattern("#1#1", vec!["test"]), "testtest");
        assert_eq!(resolve_path_pattern("#1#1", vec![""]), "");
    }

    #[test]
    fn wrong_patterns_test() {
        assert_eq!(resolve_path_pattern("#0, #1, #2", vec!["ok"]), "#0, ok, #2");
        assert_eq!(resolve_path_pattern("#0, #1", vec![]), "#0, #1");
        assert_eq!(resolve_path_pattern("#0", vec![]), "#0");
    }
}
