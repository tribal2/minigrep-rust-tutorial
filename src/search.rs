pub fn search_in_string<'a>(query: &'a str, text: &'a String) -> Vec<&'a str> {
    text
        .lines()
        .filter(|line| line.contains(query))
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