use std::borrow::Cow;

pub fn own_optional_cow(cow: &Option<Cow<'_, str>>) -> Option<Cow<'static, str>> {
    cow.as_ref().map(|cow| Cow::Owned(cow.clone().into_owned()))
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
    let len = input.len();
    let ws_len = len - input.trim_start().len();
    for i in 0..ws_len {
        let _ = input.remove(i);
    }

    // trim end
    let ws_len = len - input.trim_end().len();
    input.truncate(ws_len);
}
