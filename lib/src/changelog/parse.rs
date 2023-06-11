use crate::Release;

const UNRELEASED_TITLE: &str = "## [Unreleased]";
const RELEASE_PREFIX: &str = "## [";

pub fn unreleased_content(input: &str) -> Option<&str> {
    match input.find(UNRELEASED_TITLE) {
        Some(idx) => {
            let start = match input[idx..].find('\n') {
                Some(start) => start + idx + 1,
                // no content if no newline
                None => return None,
            };
            match input[start..].find(RELEASE_PREFIX) {
                // from after unreleased title until last release title
                Some(end) => Some(&input[start..end + start]),
                // no other releases
                None => match input[start..].find("[Unreleased]:") {
                    // until links at bottom
                    Some(end) => Some(&input[start..end + start]),
                    // until eof
                    None => Some(&input[start..]),
                },
            }
        }
        // no unreleased section
        None => None,
    }
}

pub fn last_release(input: &str) -> Option<Release> {
    let title = match input.find(UNRELEASED_TITLE) {
        Some(idx) => match_release_title(&input[idx + UNRELEASED_TITLE.len()..]),
        None => match_release_title(input),
    }?;

    let clean_title = &title[1..title.len() - 1];

    Some(Release {
        title: clean_title.into(),
        url: match_release_url(input, title),
    })
}

fn match_release_title(input: &str) -> Option<&str> {
    match input.find(RELEASE_PREFIX) {
        Some(idx) => {
            let start = idx + RELEASE_PREFIX.len();
            match input[start..].find(']') {
                Some(end) => Some(&input[(start - 1)..(end + start + 1)]),
                None => None,
            }
        }
        None => None,
    }
}

fn match_release_url<'c>(input: &'c str, title: &'c str) -> Option<&'c str> {
    for (i, _) in input.rmatch_indices(title) {
        let next = i + title.len() + 1;
        if &input[next - 1..next] == ":" {
            // we have a link
            match input[next..].find('\n') {
                Some(end) => return Some(input[next..end + next].trim()),
                // EOF
                None => return Some(input[next..].trim()),
            }
        } else if &input[next - 1..next] == "(" {
            // we have a link
            if let Some(end) = input[next..].find(')') {
                return Some(input[next..end + next].trim());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {}
