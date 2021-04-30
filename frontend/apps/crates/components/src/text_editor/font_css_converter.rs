use super::wysiwyg_types::Font;

/*
    turn:
        "Shesek - Regular, Architects Daughter - Regular"
    into:
        "\"Shesek - Regular\", \"Architects Daughter - Regular\""
*/
pub fn font_to_css(font: &Font) -> String {
    let mut list: Vec<&str> = font.split(", ").collect();

    let mut i = 1;
    for _ in 0..(list.len() - 1) {
        list.insert(i, "\", \"");
        i += 2;
    }
    list.insert(0, "\"");
    list.push("\"");
    list.join("")
}
pub fn font_from_css(font: &String) -> Font {
    // remove quotes from sides
    let font = font.split_at(font.len() - 1);
    let font = font.0.split_at(1).1;

    font.split("\", \"")
        .collect::<Vec<&str>>()
        .join(", ")
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_css_font_single() -> String {
        String::from("\"Shesek - Regular\"")
    }
    fn get_css_font_multiple() -> String {
        String::from("\"Shesek - Regular\", \"Architects Daughter - Regular\"")
    }
    fn get_font_single() -> String {
        String::from("Shesek - Regular")
    }
    fn get_font_multiple() -> String {
        String::from("Shesek - Regular, Architects Daughter - Regular")
    }

    #[test]
    fn test_multiple_font_to_css() {
        let input = get_font_multiple();
        let output = get_css_font_multiple();
        assert_eq!(output, font_to_css(&input));
    }

    #[test]
    fn test_single_font_to_css() {
        let input = get_font_single();
        let output = get_css_font_single();
        assert_eq!(output, font_to_css(&input));
    }

    #[test]
    fn test_multiple_font_from_css() {
        let input = get_css_font_multiple();
        let output = get_font_multiple();
        assert_eq!(output, font_from_css(&input));
    }

    #[test]
    fn test_single_font_from_css() {
        let input = get_css_font_single();
        let output = get_font_single();
        assert_eq!(output, font_from_css(&input));
    }
}
