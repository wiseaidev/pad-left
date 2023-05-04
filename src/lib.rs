use std::fmt::Write;

/// Pads a string on the left with a specified character to reach a given length.
///
/// # Arguments
/// * `str` - The string to be padded.
/// * `len` - The total length of the resulting string.
/// * `chr` - The character used for padding.
///
/// # Returns
/// A new `String` that is the padded string.
///
/// # Examples
///
/// ```
/// use pad_left::left_pad;
///
/// assert_eq!(left_pad("".to_string(), 0, ' '), "");
/// assert_eq!(left_pad("".to_string(), 10, ' '), "          ");
/// assert_eq!(left_pad("hello".to_string(), 5, ' '), "hello");
/// assert_eq!(left_pad("hello".to_string(), 10, ' '), "     hello");
/// assert_eq!(left_pad("hello".to_string(), 10, '*'), "*****hello");
/// assert_eq!(left_pad("".to_string(), 10, ' '), "          ".to_string());
/// assert_eq!(left_pad("hello".to_string(), 0, ' '), "hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 5, ' '), "hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 10, ' '), "     hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 15, ' '), "          hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 10, '\0'), "     hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 15, '\0'), "          hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 10, '-'), "-----hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 15, '-'), "----------hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 10, '🚀'), "🚀🚀🚀🚀🚀hello".to_string());
/// assert_eq!(left_pad("hello".to_string(), 15, '🚀'), "🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀hello".to_string());
/// ```
///
/// # Complexity
///
/// # Time complexity
/// O(log(n)), where `n` is the length difference between the input string and the desired length.
///
/// # Space complexity
/// O(log(n)), where `n` is the length difference between the input string and the desired length,
/// since we are constructing a new string with the padded characters. However, the actual space
/// used may be less than this if the `ch` parameter is a space character and the length difference
/// is less than 20, in which case we construct a pre-allocated string instead of dynamically creating one.
pub fn left_pad(str: String, len: usize, ch: char) -> String {
    // get the difference in length between the target length and the length of the string
    let len_diff = len.saturating_sub(str.len());
    // if the difference is zero, return the string
    if len_diff == 0 {
        return str;
    }
    // if the character is null, default to space
    let ch = if ch == '\0' { ' ' } else { ch };
    // if the character is space and the difference in length is less than 20, use a preallocated string for padding
    if ch == ' ' && len_diff < 20 {
        let mut pad = String::with_capacity(len_diff);
        pad.push_str(&"                    "[..len_diff]);
        pad.push_str(&str);
        return pad;
    }
    // initialize an empty string for padding
    let mut pad = String::with_capacity(len_diff);
    // initialize an empty string for the repeated character
    let mut ch_str = String::new();
    // initialize the length of the padding to be the length difference
    let mut len = len_diff;
    // while the length of the padding is greater than zero
    while len > 0 {
        // if the length of the padding is odd, add the character to the padding string
        if len & 1 == 1 {
            pad.push(ch);
        }
        // divide the length of the padding by 2
        len >>= 1;
        // if the length of the padding is still greater than zero
        if len > 0 {
            // clear the repeated character string and push the character onto it
            ch_str.clear();
            ch_str.push(ch);
            // append the repeated character to the padding string
            pad.write_str(&ch_str.repeat(len)).unwrap();
        }
    }
    // append the original string to the padding string
    pad.push_str(&str);
    // return the padded string
    pad
}
