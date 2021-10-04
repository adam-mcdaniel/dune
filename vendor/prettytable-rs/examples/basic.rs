#[macro_use]
extern crate prettytable;
use prettytable::{Table, Row, Cell};

/*
    Following main function will print :
    +---------+------+---------+
    | ABC     | DEFG | HIJKLMN |
    +---------+------+---------+
    | foobar  | bar  | foo     |
    +---------+------+---------+
    | foobar2 | bar2 | foo2    |
    +---------+------+---------+
    Modified :
    +---------+------+---------+
    | ABC     | DEFG | HIJKLMN |
    +---------+------+---------+
    | foobar  | bar  | foo     |
    +---------+------+---------+
    | foobar2 | bar2 | new_foo |
    +---------+------+---------+
*/
fn main() {
    let mut table = Table::new();
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    table.add_row(Row::new(vec![Cell::new("foobar2"), Cell::new("bar2"), Cell::new("foo2")]));
    table.printstd();
    println!("Modified : ");
    table.set_element("new_foo", 2, 1).unwrap();
    table.printstd();

    // The same table can be built the following way :
    let _table = table!(["ABC", "DEFG", "HIJKLMN"],
                        ["foobar", "bar", "foo"],
                        ["foobar2", "bar2", "foo2"]);

    // Or directly print it like this
    let _table = ptable!(["ABC", "DEFG", "HIJKLMN"],
                         ["foobar", "bar", "foo"],
                         ["foobar2", "bar2", "foo2"]);
}
