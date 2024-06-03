use std::fmt::Display;

use itertools::Itertools as _;

pub trait TableDisplay {
    fn table_headings() -> Box<[String]>;
    fn table_row(&self) -> Box<[String]>;
}

pub fn display_as_list<T: Display>(items: &[T], heading: &str) -> String {
    let items = items.iter().map(|item| format!("* {item}")).join("\n");
    format!("### {heading}\n\n{items}")
}

pub fn display_as_table<T: TableDisplay>(items: &[T], heading: &str) -> String {
    let mut result = String::new();
    let rows = items.iter().map(T::table_row).collect::<Vec<_>>();

    // calculate a max width for each heading based on the longest row in the column
    let headings = T::table_headings();
    let headings = headings
        .iter()
        .enumerate()
        .map(|(i, heading)| {
            let max_width = rows
                .iter()
                .map(|row| row[i].len())
                .chain(std::iter::once(heading.len()))
                .max()
                .unwrap_or(heading.len());
            (heading, max_width)
        })
        .collect::<Vec<_>>();

    // add the section heading
    result.push_str(&format!("### {heading}\n\n"));

    // add the table headings
    result.push_str(&format!(
        "| {} |\n",
        headings
            .iter()
            .map(|(heading, max_width)| format!("{heading:<max_width$}"))
            .join(" | ")
    ));

    // add the separator
    result.push_str(&format!(
        "| {} |\n",
        headings
            .iter()
            .map(|(_, max_width)| "-".repeat(*max_width))
            .join(" | ")
    ));

    // add the rows
    for row in rows {
        result.push_str(&format!(
            "| {} |\n",
            row.iter()
                .enumerate()
                .map(|(i, cell)| {
                    let max_width = headings[i].1;
                    format!("{cell:<max_width$}")
                })
                .join(" | ")
        ));
    }

    result.trim().to_string()
}
