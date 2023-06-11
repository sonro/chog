use crate::Changelog;

mod parse;
#[cfg(test)]
mod tests;

impl<'c> Changelog<'c> {
    pub fn new(input: &'c str) -> Self {
        Self {
            unreleased_content: parse::unreleased_content(input),
            last_release: parse::last_release(input),
        }
    }
}

impl<'c> From<&'c str> for Changelog<'c> {
    fn from(input: &'c str) -> Self {
        Changelog::new(input)
    }
}
