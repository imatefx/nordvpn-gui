pub fn clean_multiline_string(input: String) -> String {
    return input
        .replace("\r", "")
        .replace("\\", "")
        .replace("-", "")
        .replace("/", "")
        .replace("|", "")
        .replace("\"", "")
        .trim()
        .replace("\n", "<br>");
}

pub fn clean_multiline_string_url(input: String) -> String {
    return input.replace("\r", "").trim().replace("\n", "<br>");
}
