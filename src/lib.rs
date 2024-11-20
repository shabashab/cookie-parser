use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum CookieParseError {
  #[error("Invalid cookie string syntax")]
  ErrorCookieStringSyntax,
  #[error("Cookie string is empty")]
  ErrorCookieStringEmpty
}

#[derive(Debug)]
pub struct CookiePair {
  name: String,
  value: String
}

#[derive(Debug)]
pub struct SetCookie {
  pair: CookiePair,
  secure: bool,
  httpOnly: bool,
  maxAge: Option<String>,
  domain: Option<String>,
  path: Option<String>,
  extensions: Vec<String>,
}

pub fn parse_cookie_string(input: &str) -> Result<Vec<CookiePair>, CookieParseError> {
  let cookie_string = 
    Grammar::parse(Rule::cookie_string, input)
      .map_err(|_| CookieParseError::ErrorCookieStringSyntax)?
      .next()
      .ok_or_else(|| CookieParseError::ErrorCookieStringEmpty)?;

  let cookie_pairs: Result<Vec<CookiePair>, _> = cookie_string
    .into_inner()
    .map(|pair| parse_cookie_pair(&pair))
    .collect();

  return cookie_pairs.map_err(|_| CookieParseError::ErrorCookieStringSyntax)
}

pub fn parse_set_cookie(input: &str) -> Result<SetCookie, CookieParseError> {
  let set_cookie_string =
    Grammar::parse(Rule::set_cookie_string, input)
      .map_err(|_| CookieParseError::ErrorCookieStringSyntax)?
      .next()
      .ok_or_else(|| CookieParseError::ErrorCookieStringEmpty)?;

  let mut set_cookie_iter = set_cookie_string.into_inner();

  let cookie_pair_pair = set_cookie_iter.next().ok_or_else(|| CookieParseError::ErrorCookieStringSyntax)?;
  let cookie_pair = parse_cookie_pair(&cookie_pair_pair)?;

  let mut set_cookie = SetCookie {
    pair: cookie_pair,
    httpOnly: false,
    secure: false,
    domain: None,
    maxAge: None,
    path: None,
    extensions: Vec::new()
  };

  for cookie_attribute in set_cookie_iter {
    if cookie_attribute.as_rule() != Rule::cookie_attribute {
      return Err(CookieParseError::ErrorCookieStringSyntax)
    }

    let inner_attribute = cookie_attribute
      .into_inner()
      .next()
      .ok_or_else(|| CookieParseError::ErrorCookieStringSyntax)?;

    match inner_attribute.as_rule() {
      Rule::cookie_httponly_attribute => set_cookie.httpOnly = true,
      Rule::cookie_secure_attribute => set_cookie.secure = true,
      Rule::cookie_domain_attribute => {
        let value = inner_attribute
          .into_inner()
          .next()
          .ok_or_else(|| CookieParseError::ErrorCookieStringSyntax)?;

        if value.as_rule() != Rule::cookie_domain_attribute_value {
          return Err(CookieParseError::ErrorCookieStringSyntax)
        }

        set_cookie.domain = Some(String::from(value.as_str()));
      },
      Rule::cookie_max_age_attribute => {
        let value = inner_attribute
          .into_inner()
          .next()
          .ok_or_else(|| CookieParseError::ErrorCookieStringSyntax)?;

        if value.as_rule() != Rule::cookie_max_age_attribute_value {
          return Err(CookieParseError::ErrorCookieStringSyntax)
        }

        set_cookie.maxAge = Some(String::from(value.as_str()));
      }
      Rule::cookie_path_attribute => {
        let value = inner_attribute
          .into_inner()
          .next()
          .ok_or_else(|| CookieParseError::ErrorCookieStringSyntax)?;

        if value.as_rule() != Rule::cookie_path_attribute_value {
          return Err(CookieParseError::ErrorCookieStringSyntax)
        }

        set_cookie.path = Some(String::from(value.as_str()));
      },
      Rule::cookie_extension_attribute => {
        set_cookie.extensions.push(String::from(inner_attribute.as_str()));
      }
      _ => return Err(CookieParseError::ErrorCookieStringSyntax)
    }
  }

  return Ok(set_cookie)
}

fn parse_cookie_pair(input: &Pair<Rule>) -> Result<CookiePair, CookieParseError> {
  if input.as_rule() != Rule::cookie_pair {
    return Err(CookieParseError::ErrorCookieStringSyntax)
  }

  let mut iter = input.clone().into_inner();

  let name = iter.next().ok_or_else(|| CookieParseError::ErrorCookieStringSyntax)?;

  if name.as_rule() != Rule::cookie_name {
    return Err(CookieParseError::ErrorCookieStringSyntax)
  }

  let value = iter.next().ok_or_else(|| CookieParseError::ErrorCookieStringSyntax)?;

  if value.as_rule() != Rule::cookie_value {
    return Err(CookieParseError::ErrorCookieStringSyntax)
  }

  return Ok(CookiePair {
    name: String::from(name.as_str()),
    value: String::from(value.as_str())
  })
}