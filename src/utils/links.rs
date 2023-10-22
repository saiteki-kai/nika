pub struct Link {
    pub text: String,
    pub base_url: String,
}

pub fn generate_hyperlink(text: &str, url: &str) -> String {
    format!(
        "\u{1b}]8;id={};{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
        "", url, text
    )
}

pub fn get_links() -> Vec<Link> {
    vec![
        Link {
            text: "Jisho.org".into(),
            base_url: "https://jisho.org/search/".into(),
        },
        Link {
            text: "Jpdb.io".into(),
            base_url: "https://jpdb.io/search?q=".into(),
        },
        Link {
            text: "Weblio.jp".into(),
            base_url: "https://www.weblio.jp/content/".into(),
        },
        Link {
            text: "Goo.ne.jp".into(),
            base_url: "https://dictionary.goo.ne.jp/word/".into(),
        },
    ]
}
