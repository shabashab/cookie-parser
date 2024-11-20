#[cfg(test)]
mod tests {
    use cookie_parser::{parse_cookie_string, parse_set_cookie, CookiePair, SetCookie};

    #[test]
    fn test_parse_cookie_string_single_cookie() {
        let input = "cookie_name=cookie_value";

        let expected_pair = CookiePair {
            name: String::from("cookie_name"),
            value: String::from("cookie_value"),
        };

        let actual = parse_cookie_string(input);

        match actual {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0], expected_pair)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_cookie_string_multiple_cookies() {
        let input = "cookie_name1=cookie_value1; cookie_name2=cookie_value2";

        let expected_pair_1 = CookiePair {
            name: String::from("cookie_name1"),
            value: String::from("cookie_value1"),
        };
        let expected_pair_2 = CookiePair {
            name: String::from("cookie_name2"),
            value: String::from("cookie_value2"),
        };

        let actual = parse_cookie_string(input);

        match actual {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(result[0], expected_pair_1);
                assert_eq!(result[1], expected_pair_2);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_cookie_empty_cookie_name() {
        let input = "cookie_name1=cookie_value1; =cookie_value2";

        let actual = parse_cookie_string(input);

        match actual {
            Err(cookie_parser::CookieParseError::ErrorCookieStringSyntax) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_cookie_ending_with_delimiter() {
        let input = "cookie_name1=cookie_value1;";

        let actual = parse_cookie_string(input);

        match actual {
            Err(cookie_parser::CookieParseError::ErrorCookieStringSyntax) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_cookie_no_space_between_cookies() {
        let input = "cookie_name1=cookie_value1;cookie_name2=cookie_value_2";

        let actual = parse_cookie_string(input);

        match actual {
            Err(cookie_parser::CookieParseError::ErrorCookieStringSyntax) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_cookie_no_cookie_value() {
        let input = "cookie_name1=; cookie_name2=cookie_value_2";

        let actual = parse_cookie_string(input);

        match actual {
            Err(cookie_parser::CookieParseError::ErrorCookieStringSyntax) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_cookie_delimiter_in_cookie_name() {
        let input = "cookie/name1=cookie_value1; cookie_name2=cookie_value_2";

        let actual = parse_cookie_string(input);

        match actual {
            Err(cookie_parser::CookieParseError::ErrorCookieStringSyntax) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_cookie_delimiter_in_cookie_value() {
        let input = "cookie_name1=cookie/value1; cookie_name2=cookie_value_2";

        let actual = parse_cookie_string(input);

        match actual {
            Err(cookie_parser::CookieParseError::ErrorCookieStringSyntax) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_without_attributes() {
        let input = "cookie_name=cookie_value";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: None,
            max_age: None,
            path: None,
            expires: None,
            http_only: false,
            secure: false,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_http_only() {
        let input = "cookie_name=cookie_value; HttpOnly";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: None,
            max_age: None,
            path: None,
            expires: None,
            http_only: true,
            secure: false,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_secure() {
        let input = "cookie_name=cookie_value; Secure";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: None,
            max_age: None,
            path: None,
            expires: None,
            http_only: false,
            secure: true,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_domain() {
        let input = "cookie_name=cookie_value; Domain=google.com";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: Some(String::from("google.com")),
            max_age: None,
            path: None,
            expires: None,
            http_only: false,
            secure: false,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_path() {
        let input = "cookie_name=cookie_value; Path=/path/to/file";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: None,
            max_age: None,
            path: Some(String::from("/path/to/file")),
            expires: None,
            http_only: false,
            secure: false,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_expires() {
        let input = "cookie_name=cookie_value; Expires=20.09.2044";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: None,
            max_age: None,
            path: None,
            expires: Some(String::from("20.09.2044")),
            http_only: false,
            secure: false,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_max_age() {
        let input = "cookie_name=cookie_value; Max-Age=100000";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: None,
            max_age: Some(String::from("100000")),
            path: None,
            expires: None,
            http_only: false,
            secure: false,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_extensions() {
        let input = "cookie_name=cookie_value; MyKey=MyValue";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: None,
            max_age: None,
            path: None,
            expires: None,
            http_only: false,
            secure: false,
            extensions: vec![String::from("MyKey=MyValue")],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_set_cookie_mixed() {
        let input = "cookie_name=cookie_value; Max-Age=100000; Path=/path/to/file; Domain=google.com; Secure";

        let actual = parse_set_cookie(input);

        let expected = SetCookie {
            pair: CookiePair {
                name: String::from("cookie_name"),
                value: String::from("cookie_value"),
            },
            domain: Some(String::from("google.com")),
            max_age: Some(String::from("100000")),
            path: Some(String::from("/path/to/file")),
            expires: None,
            http_only: false,
            secure: true,
            extensions: vec![],
        };

        match actual {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(_) => assert!(false),
        }
    }
}
