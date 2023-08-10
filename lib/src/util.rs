use std::borrow::Cow;

pub fn own_optional_cow(cow: &Option<Cow<'_, str>>) -> Option<Cow<'static, str>> {
    cow.as_ref().map(|cow| Cow::Owned(cow.clone().into_owned()))
}

pub fn trim_to_optcow<'a, T: Into<Cow<'a, str>>>(input: T) -> Option<Cow<'a, str>> {
    return match input.into() {
        Cow::Borrowed(input) => trim_to_optcow_borrow(input),
        Cow::Owned(input) => trim_to_optcow_owned(input),
    };
}

pub fn trim_to_optcow_borrow(input: &str) -> Option<Cow<'_, str>> {
    let output = input.trim();
    match output.len() {
        0 => None,
        _ => Some(Cow::Borrowed(output)),
    }
}

pub fn trim_to_optcow_owned(mut input: String) -> Option<Cow<'static, str>> {
    trim_owned_string(&mut input);
    match input.len() {
        0 => None,
        _ => Some(Cow::Owned(input)),
    }
}

fn trim_owned_string(input: &mut String) {
    // trim start
    let ws_len = input.len() - input.trim_start().len();
    // drop opening whitespace
    let _ = input.drain(0..ws_len);

    // trim end
    input.truncate(input.trim_end().len());
}
