use convert_case::{ Case, Casing };
use colored::*;
use crate::constants::COLORS;

pub fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        let mut truncated = s[..max.saturating_sub(3)].to_string();
        truncated.push_str("...");
        truncated
    } else {
        s.to_string()
    }
}

pub fn format_line(columns: &[&str], cols: &[usize], widths: &[usize]) -> String {
    cols.iter()
        .zip(widths)
        .filter_map(|(&i, &width)| {
            columns.get(i).map(|col| {
                let truncated = truncate(col, width);
                format!("{:<width$}", truncated, width = width)
                    .color(COLORS[i % COLORS.len()])
                    .to_string()
            })
        })
        .collect::<Vec<_>>()
        .join("   ")
}

pub fn format_header(columns: &[&str], cols: &[usize], widths: &[usize]) -> String {
    cols.iter()
        .zip(widths)
        .filter_map(|(&i, &width)| {
            columns.get(i).map(|col| {
                let title_case = col.to_case(Case::Title);
                let truncated = truncate(&title_case, width);
                let styled = truncated
                    .color(COLORS[i % COLORS.len()])
                    .underline()
                    .bold()
                    .to_string();
                let padding_len = width.saturating_sub(truncated.chars().count());
                format!("{}{}", styled, " ".repeat(padding_len))
            })
        })
        .collect::<Vec<_>>()
        .join("   ")
}

pub fn compute_column_widths(
    rows: &[Vec<String>],
    cols: &[usize],
    max_width: usize,
) -> Vec<usize> {
    cols.iter()
        .map(|&col_index| {
            rows.iter()
                .filter_map(|row| row.get(col_index))
                .map(|cell| cell.chars().count())
                .max()
                .map(|len| len.min(max_width))
                .unwrap_or(0)
        })
        .collect()
}