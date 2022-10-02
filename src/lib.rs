#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Version<'a> {
    Major,
    Minor,
    Patch,
    Custom(&'a str),
}
