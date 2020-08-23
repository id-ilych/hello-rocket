use super::encode_param;

pub fn construct_github_url(options: &str) -> String {
    if options == "" {
        String::from("https://github.com")
    } else {
        let encoded_query = encode_param(options);
        format!("https://github.com/{}", encoded_query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_url() {
        assert_eq!(
            construct_github_url(""),
            "https://github.com"
        );
    }

    #[test]
    fn test_construct_github_url_query() {
        assert_eq!(
            construct_github_url("hello world"),
            "https://github.com/hello%20world"
        );
    }
}
