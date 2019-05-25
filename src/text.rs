//! Utils for diff text
use crate::basic;
use crate::format_table;
use ansi_term::{Colour, Style};
use prettytable::{Cell, Row};
use std::fmt;

/// Container for inline text diff result. Can be pretty-printed by Display trait.
#[derive(Debug, PartialEq)]
pub struct InlineChangeset<'a> {
    old: Vec<&'a str>,
    new: Vec<&'a str>,
    separator: &'a str,
    highlight_whitespace: bool,
    insert_style: Style,
    insert_whitespace_style: Style,
    remove_style: Style,
    remove_whitespace_style: Style,
}

impl<'a> InlineChangeset<'a> {
    pub fn new(old: Vec<&'a str>, new: Vec<&'a str>) -> InlineChangeset<'a> {
        InlineChangeset {
            old,
            new,
            separator: "",
            highlight_whitespace: true,
            insert_style: Colour::Green.normal(),
            insert_whitespace_style: Colour::White.on(Colour::Green),
            remove_style: Colour::Red.strikethrough(),
            remove_whitespace_style: Colour::White.on(Colour::Red),
        }
    }
    /// Highlight whitespaces in case of insert/remove?
    pub fn set_highlight_whitespace(mut self, val: bool) -> Self {
        self.highlight_whitespace = val;
        self
    }

    /// Style of inserted text
    pub fn set_insert_style(mut self, val: Style) -> Self {
        self.insert_style = val;
        self
    }

    /// Style of inserted whitespace
    pub fn set_insert_whitespace_style(mut self, val: Style) -> Self {
        self.insert_whitespace_style = val;
        self
    }

    /// Style of removed text
    pub fn set_remove_style(mut self, val: Style) -> Self {
        self.remove_style = val;
        self
    }

    /// Style of removed whitespace
    pub fn set_remove_whitespace_style(mut self, val: Style) -> Self {
        self.remove_whitespace_style = val;
        self
    }

    /// Set output separator
    pub fn set_separator(mut self, val: &'a str) -> Self {
        self.separator = val;
        self
    }

    /// Returns Vec of changes
    pub fn diff(&self) -> Vec<basic::DiffOp<'a, &str>> {
        basic::diff(&self.old, &self.new)
    }

    fn apply_style(&self, style: Style, whitespace_style: Style, a: &[&str]) -> String {
        a.join(self.separator)
            .chars()
            .map(|i| {
                let style = if i.is_whitespace() && self.highlight_whitespace {
                    whitespace_style
                } else {
                    style
                };
                style.paint(i.to_string()).to_string()
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn remove_color(&self, a: &[&str]) -> String {
        self.apply_style(self.remove_style, self.remove_whitespace_style, a)
    }

    fn insert_color(&self, a: &[&str]) -> String {
        self.apply_style(self.insert_style, self.insert_whitespace_style, a)
    }
    /// Returns formatted string with colors
    pub fn format(&self) -> String {
        let diff = self.diff();
        let mut out: Vec<String> = Vec::with_capacity(diff.len());
        for op in diff {
            match op {
                basic::DiffOp::Equal(a) => out.push(a.join(self.separator)),
                basic::DiffOp::Insert(a) => out.push(self.insert_color(a)),
                basic::DiffOp::Remove(a) => out.push(self.remove_color(a)),
                basic::DiffOp::Replace(a, b) => {
                    out.push(self.remove_color(a));
                    out.push(self.insert_color(b));
                }
            }
        }
        out.join(self.separator)
    }
}

impl<'a> fmt::Display for InlineChangeset<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.format())
    }
}

pub fn diff_chars<'a>(old: &'a str, new: &'a str) -> InlineChangeset<'a> {
    let old: Vec<&str> = old.split("").filter(|&i| i != "").collect();
    let new: Vec<&str> = new.split("").filter(|&i| i != "").collect();

    InlineChangeset::new(old, new)
}
/// Split string by non-alphanumeric characters keeping delemiters
pub fn split_words(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (idx, matched) in text.match_indices(|c: char| !c.is_alphanumeric()) {
        if last != idx {
            result.push(&text[last..idx]);
        }
        result.push(matched);
        last = idx + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

/// Diff two strings by words (contiguous)
pub fn diff_words<'a>(old: &'a str, new: &'a str) -> InlineChangeset<'a> {
    InlineChangeset::new(split_words(old), split_words(new))
}

fn color_multilines(color: Colour, s: &str) -> String {
    s.split('\n')
        .map(|i| color.paint(i).to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Container for line-by-line text diff result. Can be pretty-printed by Display trait.
#[derive(Debug, PartialEq, Eq)]
pub struct LineChangeset<'a> {
    old: Vec<&'a str>,
    new: Vec<&'a str>,

    names: Option<(&'a str, &'a str)>,
    diff_only: bool,
    show_lines: bool,
    trim_new_lines: bool,
    aling_new_lines: bool,
}

impl<'a> LineChangeset<'a> {
    pub fn new(old: Vec<&'a str>, new: Vec<&'a str>) -> LineChangeset<'a> {
        LineChangeset {
            old,
            new,
            names: None,
            diff_only: false,
            show_lines: true,
            trim_new_lines: true,
            aling_new_lines: false,
        }
    }

    /// Sets names for side-by-side diff
    pub fn names(mut self, old: &'a str, new: &'a str) -> Self {
        self.names = Some((old, new));
        self
    }
    /// Show only differences for side-by-side diff
    pub fn set_diff_only(mut self, val: bool) -> Self {
        self.diff_only = val;
        self
    }
    /// Show lines in side-by-side diff
    pub fn set_show_lines(mut self, val: bool) -> Self {
        self.show_lines = val;
        self
    }
    /// Trim new lines in side-by-side diff
    pub fn set_trim_new_lines(mut self, val: bool) -> Self {
        self.trim_new_lines = val;
        self
    }
    /// Align new lines inside diff
    pub fn set_aling_new_lines(mut self, val: bool) -> Self {
        self.aling_new_lines = val;
        self
    }
    /// Returns Vec of changes
    pub fn diff(&self) -> Vec<basic::DiffOp<'a, &str>> {
        basic::diff(&self.old, &self.new)
    }

    fn prettytable_process(&self, a: &[&str], color: Option<Colour>) -> (String, usize) {
        let mut start = 0;
        let mut stop = a.len();
        if self.trim_new_lines {
            for (index, element) in a.iter().enumerate() {
                if *element != "" {
                    break;
                }
                start = index + 1;
            }
            for (index, element) in a[start..stop].iter().rev().enumerate() {
                if *element != "" {
                    break;
                }
                stop = index;
            }
        }
        let out = &a[start..stop];
        if let Some(color) = color {
            (
                out.iter()
                    .map(|i| color.paint(*i).to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .replace("\t", "    "),
                start,
            )
        } else {
            (out.join("\n").replace("\t", "    "), start)
        }
    }
    fn prettytable_process_replace(
        &self,
        old: &[&str],
        new: &[&str],
    ) -> ((String, String), (usize, usize)) {
        let (old, old_offset) = self.prettytable_process(old, None);
        let (new, new_offset) = self.prettytable_process(new, None);

        let mut old_out = String::new();
        let mut new_out = String::new();

        for op in diff_words(&old, &new).diff() {
            match op {
                basic::DiffOp::Equal(a) => {
                    old_out.push_str(&a.join(""));
                    new_out.push_str(&a.join(""));
                }
                basic::DiffOp::Insert(a) => {
                    new_out.push_str(&color_multilines(Colour::Green, &a.join("")));
                }
                basic::DiffOp::Remove(a) => {
                    old_out.push_str(&color_multilines(Colour::Red, &a.join("")));
                }
                basic::DiffOp::Replace(a, b) => {
                    old_out.push_str(&color_multilines(Colour::Red, &a.join("")));
                    new_out.push_str(&color_multilines(Colour::Green, &b.join("")));
                }
            }
        }

        ((old_out, new_out), (old_offset, new_offset))
    }
    /// Prints side-by-side diff in table
    pub fn prettytable(&self) {
        let mut table = format_table::new();
        if let Some((old, new)) = &self.names {
            let mut header = vec![];
            if self.show_lines {
                header.push(Cell::new(""));
            }
            header.push(Cell::new(&Colour::Cyan.paint(old.to_string()).to_string()));
            if self.show_lines {
                header.push(Cell::new(""));
            }
            header.push(Cell::new(&Colour::Cyan.paint(new.to_string()).to_string()));
            table.set_titles(Row::new(header));
        }
        let mut old_lines = 1;
        let mut new_lines = 1;
        let mut out: Vec<(usize, String, usize, String)> = Vec::new();
        for op in &self.diff() {
            match op {
                basic::DiffOp::Equal(a) => {
                    let (old, offset) = self.prettytable_process(a, None);
                    if !self.diff_only {
                        out.push((old_lines + offset, old.clone(), new_lines + offset, old));
                    }
                    old_lines += a.len();
                    new_lines += a.len();
                }
                basic::DiffOp::Insert(a) => {
                    let (new, offset) = self.prettytable_process(a, Some(Colour::Green));
                    out.push((old_lines, "".to_string(), new_lines + offset, new));
                    new_lines += a.len();
                }
                basic::DiffOp::Remove(a) => {
                    let (old, offset) = self.prettytable_process(a, Some(Colour::Red));
                    out.push((old_lines + offset, old, new_lines, "".to_string()));
                    old_lines += a.len();
                }
                basic::DiffOp::Replace(a, b) => {
                    let ((old, new), (old_offset, new_offset)) =
                        self.prettytable_process_replace(a, b);
                    out.push((old_lines + old_offset, old, new_lines + new_offset, new));
                    old_lines += a.len();
                    new_lines += b.len();
                }
            };
        }
        for (old_lines, old, new_lines, new) in out {
            if self.trim_new_lines && old.trim() == "" && new.trim() == "" {
                continue;
            }
            if self.show_lines {
                table.add_row(row![old_lines, old, new_lines, new]);
            } else {
                table.add_row(row![old, new]);
            }
        }
        table.printstd();
    }

    fn remove_color(&self, a: &[&str]) -> String {
        Colour::Red.strikethrough().paint(a.join("\n")).to_string()
    }

    fn insert_color(&self, a: &[&str]) -> String {
        Colour::Green.paint(a.join("\n")).to_string()
    }

    pub fn format(&self) -> String {
        let diff = self.diff();
        let mut out: Vec<String> = Vec::with_capacity(diff.len());
        for op in diff {
            match op {
                basic::DiffOp::Equal(a) => out.push(a.join("\n")),
                basic::DiffOp::Insert(a) => out.push(self.insert_color(a)),
                basic::DiffOp::Remove(a) => out.push(self.remove_color(a)),
                basic::DiffOp::Replace(a, b) => {
                    out.push(self.remove_color(a));
                    out.push(self.insert_color(b));
                }
            }
        }
        out.join("\n")
    }
}

impl<'a> fmt::Display for LineChangeset<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.format())
    }
}

pub fn diff_lines<'a>(old: &'a str, new: &'a str) -> LineChangeset<'a> {
    let old: Vec<&str> = old.lines().collect();
    let new: Vec<&str> = new.lines().collect();

    LineChangeset::new(old, new)
}

#[test]
fn test_basic() {
    println!("diff_chars: {}", diff_chars("abefcd", "zadqwc"));
    println!(
        "diff_chars: {}",
        diff_chars(
            "The quick brown fox jumps over the lazy dog",
            "The quick brown dog leaps over the lazy cat"
        )
    );
    println!(
        "diff_chars: {}",
        diff_chars(
            "The red brown fox jumped over the rolling log",
            "The brown spotted fox leaped over the rolling log"
        )
    );
    println!(
        "diff_chars: {}",
        diff_chars(
            "The red    brown fox jumped over the rolling log",
            "The brown spotted fox leaped over the rolling log"
        )
        .set_highlight_whitespace(true)
    );
    println!(
        "diff_words: {}",
        diff_words(
            "The red brown fox jumped over the rolling log",
            "The brown spotted fox leaped over the rolling log"
        )
    );
    println!(
        "diff_words: {}",
        diff_words(
            "The quick brown fox jumps over the lazy dog",
            "The quick, brown dog leaps over the lazy cat"
        )
    );
}

#[test]
fn test_split_words() {
    assert_eq!(split_words("Hello World"), ["Hello", " ", "World"]);
    assert_eq!(split_words("Hello😋World"), ["Hello", "😋", "World"]);
    assert_eq!(
        split_words("The red brown fox\tjumped, over the rolling log"),
        [
            "The", " ", "red", " ", "brown", " ", "fox", "\t", "jumped", ",", " ", "over", " ",
            "the", " ", "rolling", " ", "log"
        ]
    );
}

#[test]
fn test_diff_lines() {
    let code1_a = r#"
void func1() {
    x += 1
}

void func2() {
    x += 2
}
    "#;
    let code1_b = r#"
void func1(a: u32) {
    x += 1
}

void functhreehalves() {
    x += 1.5
}

void func2() {
    x += 2
}

void func3(){}
"#;
    println!("diff_lines:");
    println!("{}", diff_lines(code1_a, code1_b));
    println!("====");
    diff_lines(code1_a, code1_b)
        .names("left", "right")
        .set_aling_new_lines(true)
        .prettytable();
}

#[test]
fn test_diff_words_issue_1() {
    let d1 = diff_words(
        "und meine Unschuld beweisen!",
        "und ich werde meine Unschuld beweisen!",
    );
    println!("diff_words: {} {:?}", d1, d1.diff());
    let d2 = diff_words(
        "Campaignings aus dem Ausland gegen meine Person ausfindig",
        "Campaignings ausfindig",
    );
    println!("diff_words: {} {:?}", d2, d2.diff());
    let d3 = diff_words("des kriminellen Videos", "des kriminell erstellten Videos");
    println!("diff_words: {} {:?}", d3, d3.diff());
}
