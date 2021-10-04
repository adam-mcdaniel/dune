#[macro_use]
extern crate prettytable;

/*
    Following main function will print :
    +-------------------------+------------------------------+
    | Title 1                 | Title 2                      |
    +-------------------------+------------------------------+
    | This is                 | foo                          |
    | a multiline             |                              |
    | cell                    |                              |
    +-------------------------+------------------------------+
    | Yo dawg ;) You can even | +---------+------+---------+ |
    | print tables            | | ABC     | DEFG | HIJKLMN | |
    | into tables             | +---------+------+---------+ |
    |                         | | foobar  | bar  | foo     | |
    |                         | +---------+------+---------+ |
    |                         | | foobar2 | bar2 | foo2    | |
    |                         | +---------+------+---------+ |
    +-------------------------+------------------------------+
*/
fn main() {
    let table1 = table!(["ABC", "DEFG", "HIJKLMN"],
                        ["foobar", "bar", "foo"],
                        ["foobar2", "bar2", "foo2"]);
    let table2 = table!(["Title 1", "Title 2"],
                        ["This is\na multiline\ncell", "foo"],
                        ["Yo dawg ;) You can even\nprint tables\ninto tables", table1]);
    table2.printstd();
}
