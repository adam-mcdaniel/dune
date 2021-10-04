#[macro_use]
extern crate prettytable;
use prettytable::format;

fn main() {
    let mut table = table!(["Value 1", "Value 2"], ["Value three", "Value four"]);
    table.set_titles(row!["Title 1", "Title 2"]);

    // Print
    // +-------------+------------+
    // | Title 1     | Title 2    |
    // +-------------+------------+
    // | Value 1     | Value 2    |
    // | Value three | Value four |
    // +-------------+------------+
    println!("FORMAT_NO_LINESEP_WITH_TITLE :");
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
    println!("");

    // Print
    // -------------------------
    //  Title 1      Title 2
    // =========================
    //  Value 1      Value 2
    // -------------------------
    //  Value three  Value four
    // -------------------------
    println!("FORMAT_NO_COLSEP :");
    table.set_format(*format::consts::FORMAT_NO_COLSEP);
    table.printstd();
    println!("");

    // Print
    // +-------------------------+
    // | Title 1      Title 2    |
    // +=========================+
    // | Value 1      Value 2    |
    // | Value three  Value four |
    // +-------------------------+
    println!("FORMAT_BORDERS_ONLY :");
    table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    table.printstd();
    println!("");

    // Custom format can be implemented using `prettytable::format::FormatBuilder`
    // Example to print
    // +-------------+------------+
    // | Title 1     | Title 2    |
    // | Value 1     | Value 2    |
    // | Value three | Value four |
    // +-------------+------------+
    println!("Custom :");
    table.set_format(format::FormatBuilder::new()
                         .column_separator('|')
                         .borders('|')
                         .separators(&[format::LinePosition::Top,
                                       format::LinePosition::Bottom],
                                     format::LineSeparator::new('-', '+', '+', '+'))
                         .padding(1, 1)
                         .build());
    table.printstd();

    // Customized format with unicode
    // Example to print
    // ┌─────────────┬────────────┐
    // │ Title 1     │ Title 2    │
    // ├─────────────┼────────────┤
    // │ Value 1     │ Value 2    │
    // ├─────────────┼────────────┤
    // │ Value three │ Value four │
    // └─────────────┴────────────┘
    println!("With unicode:");
    table.set_format(format::FormatBuilder::new()
                         .column_separator('│')
                         .borders('│')
                         .separators(&[format::LinePosition::Top],
                                     format::LineSeparator::new('─', '┬', '┌', '┐'))
                         .separators(&[format::LinePosition::Intern],
                                     format::LineSeparator::new('─', '┼', '├', '┤'))
                         .separators(&[format::LinePosition::Bottom],
                                     format::LineSeparator::new('─', '┴', '└', '┘'))
                         .padding(1, 1)
                         .build());
    table.printstd();

    // Customized format with unicode and different padding
    // Example to print
    // ┌───────────────┬──────────────┐
    // │  Title 1      │  Title 2     │
    // ├───────────────┼──────────────┤
    // │  Value 1      │  Value 2     │
    // ├───────────────┼──────────────┤
    // │  Value three  │  Value four  │
    // └───────────────┴──────────────┘
    // Change individual format settings
    println!("With unicode and padding:");
    table.get_format().padding(2, 2);
    table.printstd();
}
