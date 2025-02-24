use url::Url;

pub fn get_base_url(url: &str) -> String {
    let parsed_url = Url::parse(url).expect("Couldn't parse URL");
    format!(
        "{}://{}",
        parsed_url.scheme(),
        parsed_url.host_str().unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_url() {
        assert_eq!(get_base_url("https://example.com/"), "https://example.com");
    }

    #[test]
    fn test_get_base_url_with_path() {
        assert_eq!(
            get_base_url("https://example.com/path/to/page"),
            "https://example.com"
        );
    }

    #[test]
    fn test_get_base_url_with_query_string() {
        assert_eq!(
            get_base_url("https://example.com/path/to/page?query=1"),
            "https://example.com"
        );
    }

    #[test]
    #[should_panic]
    fn test_with_incorrect_url() {
        get_base_url("incorrect-url");
    }
}
