use std::borrow::Cow;

pub fn own_optional_cow(cow: &Option<Cow<'_, str>>) -> Option<Cow<'static, str>> {
    cow.as_ref().map(|cow| Cow::Owned(cow.clone().into_owned()))
}
