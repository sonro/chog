use chog::Unreleased;

#[test]
fn prop_access() {
    let url = "testurl";
    let content = "testcontent";
    let rel = Unreleased::with("testurl", "testcontent");
    assert_eq!("Unreleased", rel.title_string());
    assert_eq!(Some(content), rel.content());
    assert_eq!(Some(url), rel.url());
    assert_eq!(None, rel.date());
}

#[test]
fn edit_empty_content() {
    let mut rel = Unreleased::with_url("testurl");
    let content = rel.mut_content();
    content.push_str("testcontent");
    assert_eq!(Some("testcontent"), rel.content());
}

#[test]
fn edit_some_content() {
    let mut rel = Unreleased::with("testurl", "test");
    let content = rel.mut_content();
    content.push_str("content");
    assert_eq!(Some("testcontent"), rel.content());
}

#[test]
fn set_url() {
    let mut rel = Unreleased::with_content("testcontent");
    assert_eq!(None, rel.url());

    {
        let testurl = String::from("testurl");
        rel.set_url(testurl);
    }
    assert_eq!(Some("testurl"), rel.url());

    rel.set_url("url2");
    assert_eq!(Some("url2"), rel.url());
}

#[test]
fn set_content() {
    let mut rel = Unreleased::with_url("testurl");
    assert_eq!(None, rel.content());

    {
        let testcontent = String::from("testcontent");
        rel.set_content(testcontent);
    }
    assert_eq!(Some("testcontent"), rel.content());

    rel.set_content("testcontent 2");
    assert_eq!(Some("testcontent 2"), rel.content());
}
