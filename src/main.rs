use cookie_parser::{parse_cookie_string, parse_set_cookie};

fn main() {
    let parsing_result = parse_cookie_string("name1=value1; name2=value2");
    println!("Cookie string result: {:#?}", parsing_result);

    let parsing_result = parse_set_cookie("name1=value1; HttpOnly; Domain=google.com; Path=/");
    println!("Set-Cookie result: {:#?}", parsing_result);
}
