use super::encode_param;

pub fn construct_google_search_url(query: &str) -> String {
    let encoded_query = encode_param(query);
    format!("https://google.com/search?q={}", encoded_query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        assert_eq!(
            construct_google_search_url("hello"),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        assert_eq!(
            construct_google_search_url("hello world"),
            "https://google.com/search?q=hello%20world"
        );
    }
}
