pub fn next_token(s: &str, start: usize) -> Option<(usize, usize)> {
    let s = &s[start..];
    let mut in_word = false;
    let mut word_start = 0;


    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            if in_word {
                return Some((word_start + start, i - 1 + start));
            }
        } else {
            if !in_word {
                in_word = true;
                word_start = i;
            }
        }
    }


    if in_word {
        Some((word_start + start, s.len() - 1 + start))
    } else {
        None
    }
}
