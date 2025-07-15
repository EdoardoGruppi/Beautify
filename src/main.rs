use clap::Parser;
use clap::{ ArgAction };
use regex::Regex;
use std::io::{ self, BufRead };

use crate::helpers::transpose;
mod helpers;
mod constants;

#[derive(Parser, Debug)]
#[command(name = "beautify")]
#[command(about = "Prettify tabular outputs.")]
struct Args {
    /// Maximum width of each column
    #[arg(long, default_value_t = 50)]
    max_width: usize,

    /// Transpose the table (flip rows and columns)
    #[arg(long, action = ArgAction::SetTrue)]
    transpose: bool,

    /// Regex separator between columns
    #[arg(long, default_value = r"\s{2,}")]
    separator: String,

    /// Whether the first line is a header
    #[arg(long = "header", action = ArgAction::SetTrue, default_value_t = true)]
    #[arg(long = "no-header", action = ArgAction::SetFalse)]
    header: bool,

    /// Which columns to print (0-indexed) comma-separeted
    #[arg(long, value_delimiter = ',', value_parser = clap::value_parser!(usize))]
    cols: Option<Vec<usize>>,
}

fn main() {
    let args = Args::parse();
    let separator = Regex::new(&args.separator).expect("Invalid separator regex");
    let stdin = io::stdin();
    let lines = stdin.lock().lines().flatten();
    let rows: Vec<Vec<String>> = lines
        .map(|line|
            separator
                .split(&line)
                .map(|s| s.to_string())
                .collect()
        )
        .collect();
    if rows.is_empty() {
        return;
    }

    let mut rows = rows; // make it mutable
    if args.transpose {
        rows = transpose(&rows);
    }

    let selected_cols: Vec<usize> = args.cols
        .clone()
        .unwrap_or_else(|| (0..rows[0].len()).collect());
    let col_widths = helpers::compute_column_widths(&rows, &selected_cols, args.max_width);
    if args.header {
        println!(
            "{}",
            helpers::format_header(
                &rows[0].iter().map(String::as_str).collect::<Vec<_>>(),
                &selected_cols,
                &col_widths
            )
        );
        for row in &rows[1..] {
            println!(
                "{}",
                helpers::format_line(
                    &row.iter().map(String::as_str).collect::<Vec<_>>(),
                    &selected_cols,
                    &col_widths
                )
            );
        }
    } else {
        for row in &rows {
            println!(
                "{}",
                helpers::format_line(
                    &row.iter().map(String::as_str).collect::<Vec<_>>(),
                    &selected_cols,
                    &col_widths
                )
            );
        }
    }
}
