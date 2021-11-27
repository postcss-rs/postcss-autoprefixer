use std::borrow::Cow;

use crate::regex;

pub fn get_prefix<'a>(prop: &'a str) -> Option<regex::Match> {
    let regex = regex!(r"^(-\w+-)");
    regex.captures(prop).and_then(|cap| cap.get(0))
}

pub fn remove_prefix<'a>(prop: &'a str) -> Cow<'a, str> {
    let regex = regex!(r"^-\w+-");
    regex.replace(prop, "")
}
