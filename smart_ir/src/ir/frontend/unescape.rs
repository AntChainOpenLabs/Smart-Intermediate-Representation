// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

//! Utilities for validating string and char literals and turning them into
//! values they represent.
//! From `rust/compiler/rustc_lexer/src/unescape.rs`.

#![allow(dead_code)]

use std::ops::Range;
use std::str::Chars;

/// Errors and warnings that can occur during string unescaping.
#[derive(Debug, PartialEq, Eq)]
pub enum EscapeError {
    /// Expected 1 char, but 0 were found.
    ZeroChars,
    /// Expected 1 char, but more than 1 were found.
    MoreThanOneChar,

    /// Escaped '\' character without continuation.
    LoneSlash,
    /// Invalid escape character (e.g. '\z').
    InvalidEscape,
    /// Raw '\r' encountered.
    BareCarriageReturn,
    /// Raw '\r' encountered in raw string.
    BareCarriageReturnInRawString,
    /// Unescaped character that was expected to be escaped (e.g. raw '\t').
    EscapeOnlyChar,

    /// Numeric character escape is too short (e.g. '\x1').
    TooShortHexEscape,
    /// Invalid character in numeric escape (e.g. '\xz')
    InvalidCharInHexEscape,
    /// Character code in numeric escape is non-ascii (e.g. '\xFF').
    OutOfRangeHexEscape,

    /// '\u' not followed by '{'.
    NoBraceInUnicodeEscape,
    /// Non-hexadecimal value in '\u{..}'.
    InvalidCharInUnicodeEscape,
    /// '\u{}'
    EmptyUnicodeEscape,
    /// No closing brace in '\u{..}', e.g. '\u{12'.
    UnclosedUnicodeEscape,
    /// '\u{_12}'
    LeadingUnderscoreUnicodeEscape,
    /// More than 6 characters in '\u{..}', e.g. '\u{10FFFF_FF}'
    OverlongUnicodeEscape,
    /// Invalid in-bound unicode character code, e.g. '\u{DFFF}'.
    LoneSurrogateUnicodeEscape,
    /// Out of bounds unicode character code, e.g. '\u{FFFFFF}'.
    OutOfRangeUnicodeEscape,

    /// Unicode escape code in byte literal.
    UnicodeEscapeInByte,
    /// Non-ascii character in byte literal.
    NonAsciiCharInByte,
    /// Non-ascii character in byte string literal.
    NonAsciiCharInByteString,

    /// After a line ending with '\', the next line contains whitespace
    /// characters that are not skipped.
    UnskippedWhitespaceWarning,

    /// After a line ending with '\', multiple lines are skipped.
    MultipleSkippedLinesWarning,
}

/// Unescape string wrapping errors
pub fn unescape_str_wrap_error(s: &str) -> String {
    if s.contains(&['\\', '\r'][..]) {
        let mut buf = String::with_capacity(s.len());
        let mut error = false;
        unescape_str_or_byte_str(s, Mode::Str, &mut |_, unescaped_char| {
            match unescaped_char {
                Ok(c) => buf.push(c),
                Err(e) => {
                    error = true;
                    buf = format!("<Lexer error: string: {e:?}>");
                }
            };
        });
        buf
    } else {
        s.to_string()
    }
}

/// What kind of literal do we parse.
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Char,
    Str,
    Byte,
    ByteStr,
    RawStr,
    RawByteStr,
}

impl Mode {
    pub fn in_single_quotes(self) -> bool {
        match self {
            Mode::Char | Mode::Byte => true,
            Mode::Str | Mode::ByteStr | Mode::RawStr | Mode::RawByteStr => false,
        }
    }

    pub fn in_double_quotes(self) -> bool {
        !self.in_single_quotes()
    }

    pub fn is_bytes(self) -> bool {
        match self {
            Mode::Byte | Mode::ByteStr | Mode::RawByteStr => true,
            Mode::Char | Mode::Str | Mode::RawStr => false,
        }
    }
}

fn scan_escape(first_char: char, chars: &mut Chars<'_>, mode: Mode) -> Result<char, EscapeError> {
    if first_char != '\\' {
        // Previous character was not a slash, and we don't expect it to be
        // an escape-only character.
        return match first_char {
            '\t' | '\n' => Err(EscapeError::EscapeOnlyChar),
            '\r' => Err(EscapeError::BareCarriageReturn),
            '\'' if mode.in_single_quotes() => Err(EscapeError::EscapeOnlyChar),
            '"' if mode.in_double_quotes() => Err(EscapeError::EscapeOnlyChar),
            _ => {
                if mode.is_bytes() && !first_char.is_ascii() {
                    // Byte literal can't be a non-ascii character.
                    return Err(EscapeError::NonAsciiCharInByte);
                }
                Ok(first_char)
            }
        };
    }

    // Previous character is '\\', try to unescape it.

    let second_char = chars.next().ok_or(EscapeError::LoneSlash)?;
    let res = match second_char {
        '"' => '"',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        '\'' => '\'',
        '0' => '\0',

        'x' => {
            // Parse hexadecimal character code.

            let hi = chars.next().ok_or(EscapeError::TooShortHexEscape)?;
            let hi = hi.to_digit(16).ok_or(EscapeError::InvalidCharInHexEscape)?;

            let lo = chars.next().ok_or(EscapeError::TooShortHexEscape)?;
            let lo = lo.to_digit(16).ok_or(EscapeError::InvalidCharInHexEscape)?;

            let value = hi * 16 + lo;
            // For a byte literal verify that it is within ASCII range.
            if !mode.is_bytes() && !is_ascii(value) {
                return Err(EscapeError::OutOfRangeHexEscape);
            }
            let value = value as u8;
            value as char
        }

        'u' => {
            // We've parsed '\u', now we have to parse '{..}'.

            if chars.next() != Some('{') {
                return Err(EscapeError::NoBraceInUnicodeEscape);
            }

            // First character must be a hexadecimal digit.
            let mut n_digits = 1;
            let mut value: u32 = match chars.next().ok_or(EscapeError::UnclosedUnicodeEscape)? {
                '_' => return Err(EscapeError::LeadingUnderscoreUnicodeEscape),
                '}' => return Err(EscapeError::EmptyUnicodeEscape),
                c => c
                    .to_digit(16)
                    .ok_or(EscapeError::InvalidCharInUnicodeEscape)?,
            };

            // First character is valid, now parse the rest of the number
            // and closing brace.
            loop {
                match chars.next() {
                    None => return Err(EscapeError::UnclosedUnicodeEscape),
                    Some('_') => continue,
                    Some('}') => {
                        if n_digits > 6 {
                            return Err(EscapeError::OverlongUnicodeEscape);
                        }

                        // Incorrect syntax has higher priority for error reporting
                        // than unallowed value for a literal.
                        if mode.is_bytes() {
                            return Err(EscapeError::UnicodeEscapeInByte);
                        }

                        break std::char::from_u32(value).ok_or({
                            if value > 0x10FFFF {
                                EscapeError::OutOfRangeUnicodeEscape
                            } else {
                                EscapeError::LoneSurrogateUnicodeEscape
                            }
                        })?;
                    }
                    Some(c) => {
                        let digit = c
                            .to_digit(16)
                            .ok_or(EscapeError::InvalidCharInUnicodeEscape)?;
                        n_digits += 1;
                        if n_digits > 6 {
                            // Stop updating value since we're sure that it's is incorrect already.
                            continue;
                        }
                        let digit = digit;
                        value = value * 16 + digit;
                    }
                };
            }
        }
        _ => return Err(EscapeError::InvalidEscape),
    };
    Ok(res)
}

/// Takes a contents of a string literal (without quotes) and produces a
/// sequence of escaped characters or errors.
fn unescape_str_or_byte_str<F>(src: &str, mode: Mode, callback: &mut F)
where
    F: FnMut(Range<usize>, Result<char, EscapeError>),
{
    assert!(mode.in_double_quotes());
    let initial_len = src.len();
    let mut chars = src.chars();
    while let Some(first_char) = chars.next() {
        let start = initial_len - chars.as_str().len() - first_char.len_utf8();
        let unescaped_char = match first_char {
            '\\' => {
                let second_char = chars.clone().next();
                match second_char {
                    Some('\n') => {
                        // Rust language specification requires us to skip whitespaces
                        // if unescaped '\' character is followed by '\n'.
                        // For details see [Rust language reference]
                        // (https://doc.rust-lang.org/reference/tokens.html#string-literals).
                        skip_ascii_whitespace(&mut chars, start, callback);
                        continue;
                    }
                    _ => scan_escape(first_char, &mut chars, mode),
                }
            }
            '\n' => Ok('\n'),
            '\t' => Ok('\t'),
            _ => scan_escape(first_char, &mut chars, mode),
        };
        let end = initial_len - chars.as_str().len();
        callback(start..end, unescaped_char);
    }

    fn skip_ascii_whitespace<F>(chars: &mut Chars<'_>, start: usize, callback: &mut F)
    where
        F: FnMut(Range<usize>, Result<char, EscapeError>),
    {
        let tail = chars.as_str();
        let first_non_space = tail
            .bytes()
            .position(|b| b != b' ' && b != b'\t' && b != b'\n' && b != b'\r')
            .unwrap_or(tail.len());
        if tail[1..first_non_space].contains('\n') {
            // The +1 accounts for the escaping slash.
            let end = start + first_non_space + 1;
            callback(start..end, Err(EscapeError::MultipleSkippedLinesWarning));
        }
        let tail = &tail[first_non_space..];
        if let Some(c) = tail.chars().next() {
            // For error reporting, we would like the span to contain the character that was not
            // skipped.  The +1 is necessary to account for the leading \ that started the escape.
            let end = start + first_non_space + c.len_utf8() + 1;
            if c.is_whitespace() {
                callback(start..end, Err(EscapeError::UnskippedWhitespaceWarning));
            }
        }
        *chars = tail.chars();
    }
}

fn is_ascii(x: u32) -> bool {
    x <= 0x7F
}
