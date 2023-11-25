pub fn format_field_content_lines_owned(lines: &[String]) -> String {
    format_field_content(&lines.join("\n"))
}

pub fn format_field_content_lines(lines: &[&str]) -> String {
    format_field_content(&lines.join("\n"))
}

pub fn format_field_content(content: &str) -> String {
    format!("```diff\n{}\n```", content)
}
