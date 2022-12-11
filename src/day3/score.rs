
fn lowercase_score(c: char) -> u32 {
    return c as u32 - 'a' as u32 + 1;
}

fn uppercase_score(c: char) -> u32 {
    return c as u32 - 'A' as u32 + 27;
}

pub fn char_score(c: char) -> u32 {
    if 'a' <= c && c <= 'z' {
        return lowercase_score(c);
    }
    else {
        return uppercase_score(c);
    }
}

