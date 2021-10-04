#[macro_use]
extern crate prettytable;
use prettytable::{Table, Row, Cell};

use prettytable::{Attr, color};

#[allow(dead_code)]
fn main() {
    let _ = table!();
    let mut table = Table::new();
    // Add style to a cell
    table.add_row(row![FrByb->"ABC", "DEFG", "HIJKLMN"]);
    // Add style to a full row
    table.add_row(row![FY => "styled", "bar", "foo"]);
    table.add_row(Row::new(vec![
            Cell::new("foobar2"),
            // Create a cell with a red foreground color
            Cell::new("bar2").with_style(Attr::ForegroundColor(color::RED)),
            // Create a cell with red foreground color, yellow background color, with bold characters
            Cell::new("foo2").style_spec("FrByb"),
            // Using the cell! macro
            cell!(Fr->"red")])
        );

    table.printstd();

    // Print a table with some styles on it :
    // FrBybl means : Foregound red, Background yellow, bold, left align
    ptable!([FrBybl->"A", "B", FrBybr->"C"], [123, 234, 345, 456], [Fg => 1, 2, 3]);

    // You can also apply style to full rows :
    let mut table = table!([Frb => "A", "B", "C"], [1, 2, 3, 4], ["A\nBCCZZZ\nDDD", 2, table]);
    // Set a title line, with all text centered in the cell
    table.set_titles(row![c => "Title 1", "Title 2"]);
    table.printstd();
}
