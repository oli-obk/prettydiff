#[macro_use]
extern crate prettytable;
use ansi_term::Colour;
use difference::{Changeset, Difference};
use prettytable::format;
use prettytable::Table;

fn format_table(table: &mut Table) {
    table.set_format(
        format::FormatBuilder::new()
            .column_separator('│')
            .borders('│')
            .separators(
                &[format::LinePosition::Top],
                format::LineSeparator::new('─', '┬', '┌', '┐'),
            )
            .separators(
                &[format::LinePosition::Title],
                format::LineSeparator::new('─', '┼', '├', '┤'),
            )
            .separators(
                &[format::LinePosition::Intern],
                format::LineSeparator::new('─', '┼', '├', '┤'),
            )
            .separators(
                &[format::LinePosition::Bottom],
                format::LineSeparator::new('─', '┴', '└', '┘'),
            )
            .padding(1, 1)
            .build(),
    );
}

fn color2str(color: ansi_term::Style, text: &str) -> String {
    format!("{}", color.paint(text))
}

/// Returns two strings with highlighted changes
pub fn diff_chars(left: &str, right: &str) -> (String, String) {
    let Changeset { diffs, .. } = Changeset::new(left, right, "");
    let mut left = String::new();
    let mut right = String::new();

    for diff in &diffs {
        match diff {
            Difference::Same(text) => {
                left.push_str(text);
                right.push_str(text);
            }
            Difference::Rem(text) => left.push_str(&color2str(Colour::Red.bold(), text)),
            Difference::Add(text) => right.push_str(&color2str(Colour::Green.bold(), text)),
        }
    }
    (left, right)
}

pub struct DiffOpt {
    pub left: String,
    pub right: String,

    pub left_name: Option<String>,
    pub right_name: Option<String>,
    pub diff_only: bool,
}

/// Prints side-by-side diff table
pub fn diff_text(opt: DiffOpt) {
    let mut table = Table::new();
    format_table(&mut table);

    let left_name = opt.left_name.unwrap_or("".to_string());
    let right_name = opt.right_name.unwrap_or("".to_string());

    table.set_titles(row![
        format!("Name: {}", Colour::Cyan.paint(left_name)),
        format!("Name: {}", Colour::Cyan.paint(right_name))
    ]);

    // Tabs breaks prettytable
    let left = opt.left.replace("\t", "    ");
    let right = opt.right.replace("\t", "    ");

    let Changeset { diffs, .. } = Changeset::new(&right, &left, "\n");

    for (i, diff) in diffs.iter().enumerate() {
        match diff {
            Difference::Same(text) => {
                if !opt.diff_only {
                    table.add_row(row![text, text]);
                }
            }
            Difference::Add(added) => match diffs.get(i - 1) {
                Some(Difference::Rem(removed)) => {
                    let (l, r) = diff_chars(added, removed);
                    table.add_row(row![l, r]);
                }
                _ => {
                    table.add_row(row![Br->added, ""]);
                }
            },
            Difference::Rem(removed) => match diffs.get(i + 1) {
                Some(Difference::Rem(_)) | Some(Difference::Same(_)) => {
                    table.add_row(row!["", Bg->removed]);
                }
                _ => {}
            },
        }
    }

    table.printstd();
}
