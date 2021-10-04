#[macro_use]
extern crate prettytable;
use prettytable::{Row, Cell, format::Alignment};


fn main() {

    /*
        The following code will output

        +---------------+---------------+--------------+
        |         A table with horizontal span         |
        +===============+===============+==============+
        | This is a cell with span of 2 | span of 1    |
        +---------------+---------------+--------------+
        | span of 1     | span of 1     | span of 1    |
        +---------------+---------------+--------------+
        |    This cell with a span of 3 is centered    |
        +---------------+---------------+--------------+
    */

    let mut table: prettytable::Table = table![
        [H2 -> "This is a cell with span of 2", "span of 1"],
        ["span of 1", "span of 1", "span of 1"],
        [H03c -> "This cell with a span of 3 is centered"]
        ];
    table.set_titles(Row::new(vec![
        Cell::new_align("A table with horizontal span", Alignment::CENTER).with_hspan(3)
    ]));
    table.printstd();
}