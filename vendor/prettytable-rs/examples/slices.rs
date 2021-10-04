#[macro_use]
extern crate prettytable;

use prettytable::Slice;

fn main() {
    let mut table = table![[0, 0, 0], [1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4], [5, 5, 5]];
    table.set_titles(row!["t1", "t2", "t3"]);

    let slice = table.slice(..);
    let slice = slice.slice(2..);
    let slice = slice.slice(..3);

    /*
        Will print
        +----+----+----+
        | t1 | t2 | t3 |
        +====+====+====+
        | 2  | 2  | 2  |
        +----+----+----+
        | 3  | 3  | 3  |
        +----+----+----+
        | 4  | 4  | 4  |
        +----+----+----+
    */
    slice.printstd();

    // This is equivalent to
    let slice = table.slice(2..5);
    slice.printstd();
}
