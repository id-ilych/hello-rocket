use super::encode_param;

pub fn construct_twitter_url(options: &str) -> String {
    if options == "" {
        String::from("https://twitter.com")
    } else if &options[..1] == "@" {
        construct_twitter_profile_url(&options[1..])
    } else {
        construct_twitter_search_url(options)
    }
}

pub fn construct_twitter_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

pub fn construct_twitter_search_url(query: &str) -> String {
    let encoded_query = encode_param(query);
    format!("https://twitter.com/search?q={}", encoded_query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        assert_eq!(
            construct_twitter_url(""),
            "https://twitter.com"
        );
    }

    #[test]
    fn test_construct_twitter_url_query() {
        assert_eq!(
            construct_twitter_url("hello world"),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_twitter_url_profile() {
        assert_eq!(
            construct_twitter_url("@fbOpenSource"),
            "https://twitter.com/fbOpenSource"
        );
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        assert_eq!(
            construct_twitter_profile_url("jsjoeio"),
            "https://twitter.com/jsjoeio"
        );
    }

    #[test]
    fn test_construct_twitter_search_url() {
        assert_eq!(
            construct_twitter_search_url("hello world"),
            "https://twitter.com/search?q=hello%20world"
        );
    }
}
