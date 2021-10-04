// #![feature(trace_macros)]
#[macro_use]
extern crate prettytable;
use prettytable::Table;
use prettytable::Row;
use prettytable::Cell;
use prettytable::format::*;
use prettytable::{Attr, color};

// trace_macros!(true);

#[allow(dead_code)]
fn main() {
    let _ = table!();
    let mut table = Table::new();
    table.add_row(row![FrByH2b->"This is a long spanning cell", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    table.add_row(row![]);
    // Add style to a full row
    table.add_row(row![FY => "styled", "bar", "foo"]);
    table.add_row(Row::new(vec![
            Cell::new("foobar2"),
            // Create a cell with a red foreground color
            Cell::new_align("bar2", Alignment::CENTER).with_style(Attr::ForegroundColor(color::RED)).with_hspan(2),
            // Create a cell with red foreground color, yellow background color, with bold characters
            Cell::new("foo2").style_spec("FrByb")])
        );
    for cell in table.column_iter_mut(2) {
        cell.align(Alignment::RIGHT);
    }
    for cell in table.column_iter_mut(1) {
        cell.align(Alignment::CENTER);
    }
    table.printstd();
    println!("Modified : ");
    table.set_element("new_foo", 2, 1).unwrap();
    table.printstd();
    // table.get_format().indent(8);

    // Print a table with some styles on it :
    // FrBybl means : Foregound red, Background yellow, bold, left align
    // d means : Default, do nothing
    ptable!([FrBybl->"A", "B", FrBybr->"C"], [d->123, 234, 345, 456]);

    // You can also apply style to full rows :
    let mut table = table!([Frb => "A", "B", "C"], [1, 2, 3, 4], ["A\nBCCZZZ\nDDD", 2, table]);
    table.set_titles(row!["Title 1", "Title 2"]);
    table.set_format(*consts::FORMAT_DEFAULT);
    table.get_format().indent(8);
    let size = table.printstd();
    println!("Table height = {}", size);
    // println!("{:#?}", table);
}
