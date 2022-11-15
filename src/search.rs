/// Search for a query in a string
///
/// # Examples
///
/// ```
/// let text = "\
/// Several lines
/// of text
/// to search for".to_string();
///
/// // Case sensitive search
/// let results = minigrep::search::search_in_string("se", &text, true);
/// assert_eq!(1, results.len());
///
/// // Case insensitive search
/// let results = minigrep::search::search_in_string("se", &text, false);
/// assert_eq!(2, results.len());
/// ```
pub fn search_in_string<'a>(
    query: &'a str,
    text: &'a String,
    case_sensitive: bool,
) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in text.lines() {
    //     if case_sensitive {
    //         if line.contains(query) {
    //             results.push(line);
    //         }
    //     } else {
    //         if line.to_lowercase().contains(&query.to_lowercase()) {
    //             results.push(line);
    //         }
    //     }
    // }

    // results

    text
        .lines()
        .filter(|line| {
            if case_sensitive {
                line.contains(query)
            } else {
                line
                    .to_lowercase()
                    .contains(&query.to_lowercase())
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_2_results() {
        let text = "\
Hola mundo
Xopa mundo
Hello world".to_string();

        let results = search_in_string("mundo", &text);
        assert_eq!(2, results.len());
    }
}