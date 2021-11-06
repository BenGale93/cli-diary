pub fn date_superscript(day: u32) -> &'static str {
    let unit = day % 10;

    match unit {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

#[cfg(test)]
mod tests {
    use super::date_superscript;
    #[test]
    fn date_superscript_st() {
        assert_eq!("st", date_superscript(21));
    }
    #[test]
    fn date_superscript_nd() {
        assert_eq!("nd", date_superscript(12));
    }
    #[test]
    fn date_superscript_rd() {
        assert_eq!("rd", date_superscript(23));
    }
    #[test]
    fn date_superscript_th() {
        assert_eq!("th", date_superscript(17));
    }
}
