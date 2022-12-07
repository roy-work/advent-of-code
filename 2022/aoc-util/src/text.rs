/// Computes the relative offset from one character of another character.
///
/// ```rust
/// let char_in_question = 'c';
/// let offset = aoc_util::char_to_relative_ord(char_in_question, 'a');
/// assert!(offset == 2);
/// ```
pub fn char_to_relative_ord(ch: char, base_char: char) -> i64 {
    let ch = i64::from(u32::from(ch));
    let base = i64::from(u32::from(base_char));
    ch.checked_sub(base).unwrap()
}

/// Unicode codepoint of a `char`, as an `i64`.
///
/// ```rust
/// let cp = aoc_util::ord('a');
/// assert!(cp == 0x61);
/// ```
pub fn ord(ch: char) -> i64 {
    i64::from(u32::from(ch))
}
