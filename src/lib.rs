use core::cmp;
use std::io::{self, BufRead};
use std::ops;
use colored::Colorize;

pub fn column_diff(source_col: usize, target_col: usize) {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line_parts = get_line_parts(&line, source_col, target_col);
        if let Some(line_parts) = line_parts {
            let target_col = line_parts.target_col;
            let mut line = line.clone();
            line.replace_range(
                target_col.clone(),
                &diff_color(line_parts.source_str,
                            &line[target_col]));
            println!("{}", line);
        } else {
            println!("{}", line);
        }
    }
}

struct LineParts<'a> {
    source_str: &'a str,
    target_col: ops::Range<usize>
}

fn get_line_parts(line: &String, source_col: usize,
                  target_col: usize) -> Option<LineParts> {
    let mut cols = get_line_cols(line);
    if source_col > cols.len() || target_col > cols.len() {
        return None;
    }

    Some(LineParts {
        source_str: &line[cols[source_col - 1].clone()],
        target_col: cols.swap_remove(target_col - 1)
    })
}

fn get_line_cols(line: &String) -> Vec<ops::Range<usize>> {
    let mut col_vec: Vec<ops::Range<usize>> = Vec::new();
    let mut pre_is_whitespace = true;
    let mut col_begin: Option<usize> = None;

    for (i, ch) in line.chars().enumerate() {
        if ch.is_ascii_whitespace() {
            if !pre_is_whitespace {
                col_vec.push(col_begin.unwrap()..i);
                col_begin = None;
            }
            pre_is_whitespace = true;
        } else {  // ch is not whitespace
            if pre_is_whitespace {
                col_begin = Some(i);
            }
            pre_is_whitespace = false;
        }
    }

    if let Some(begin) = col_begin {
        col_vec.push(begin..line.len());
    }

    col_vec
}

fn diff_color(source: &str, target: &str) -> String {
    let prefix = common_prefix(source, target);
    format!("{}{}", prefix, target.strip_prefix(prefix).unwrap().green())
}

fn common_prefix<'a>(s1: &'a str, s2: &str) -> &'a str {
    let len = cmp::min(s1.len(), s2.len());

    for i in 0..len {
        if s1[i..=i] != s2[i..=i] {
            return &s1[0..i];
        }
    }
    &s1[0..len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_cols() {
        let line = String::from("abc sd  k");
        let col_vec = get_line_cols(&line);
        assert_eq!(col_vec.len(), 3);
        assert_eq!(col_vec[0], 0..3);
        assert_eq!(col_vec[1], 4..6);
        assert_eq!(col_vec[2], 8..9);

        let line = String::from("  abc sd  ");
        let col_vec = get_line_cols(&line);
        assert_eq!(col_vec.len(), 2);
        assert_eq!(col_vec[0], 2..5);
        assert_eq!(col_vec[1], 6..8);
    }

    #[test]
    fn test_get_line_parts() {
        let line = String::from("abc de");
        let line_part = get_line_parts(&line, 2, 1).unwrap();
        assert_eq!(line_part.source_str, "de");
        assert_eq!(line_part.target_col, 0..3);

        let line = String::from("abc");
        let line_part = get_line_parts(&line, 2, 1);
        assert!(line_part.is_none());
    }

    #[test]
    fn test_common_prefix() {
        assert_eq!(common_prefix("abc", "abcd"), "abc");
        assert_eq!(common_prefix("abce", "abcd"), "abc");
        assert_eq!(common_prefix("abcd", "abc"), "abc");
        assert_eq!(common_prefix("eabcd", "abc"), "");
    }
}
