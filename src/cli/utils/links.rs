use crate::utils::Link;

pub fn generate_hyperlink(text: &str, url: &str) -> String {
    format!(
        "\u{1b}]8;id={};{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
        "", url, text
    )
}

// Dictionary links
pub fn get_links() -> Vec<Link> {
    todo!()
}
