![License](http://img.shields.io/badge/license-BSD-lightgrey.svg)
[![Build Status](https://travis-ci.org/phsym/prettytable-rs.svg?branch=master)](https://travis-ci.org/phsym/prettytable-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/wdh9klb35fed6ik9?svg=true)](https://ci.appveyor.com/project/phsym/tabprint)
[![codecov](https://codecov.io/gh/phsym/prettytable-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/phsym/prettytable-rs)
[![Crates.io](https://img.shields.io/crates/v/prettytable-rs.svg)](https://crates.io/crates/prettytable-rs)
[![Doc.rs](https://docs.rs/prettytable-rs/badge.svg)](https://docs.rs/crate/prettytable-rs/)
[![Doc.rs](https://img.shields.io/badge/docs-master-blue.svg)](http://phsym.github.io/prettytable-rs/master)

# prettytable-rs

A formatted and aligned table printer library for [Rust](https://www.rust-lang.org).

*Copyright &copy; 2018 Pierre-Henri Symoneaux*

> THIS SOFTWARE IS DISTRIBUTED WITHOUT ANY WARRANTY <br>
> Check LICENSE.txt file for more information. <br>

# How to use

  * [Including](#user-content-including)
  * [Basic usage](#user-content-basic-usage)
  * [Using macros](#user-content-using-macros)
  * [Do it with style](#user-content-do-it-with-style)
    * [List of style specifiers](#user-content-list-of-style-specifiers)
    * [List of color specifiers](#user-content-list-of-color-specifiers)
  * [Slicing](#user-content-slicing)
  * [Customize your table look and feel](#user-content-customize-your-table-look-and-feel)
  * [CSV import/export](#user-content-csv-importexport)
    * [Importing](#user-content-importing)
    * [Exporting](#user-content-exporting)
  * [Note on line endings](#user-content-note-on-line-endings)

## Including

Include the library as a dependency to your project by adding the following lines to your **Cargo.toml** file:

```toml
[dependencies]
prettytable-rs = "^0.8"
```

The library requires at least `rust v1.26.0`.

## Basic usage

Start using it like this:

```rust
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

fn main() {
    // Create the table
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    // A more complicated way to add a row:
    table.add_row(Row::new(vec![
        Cell::new("foobar2"),
        Cell::new("bar2"),
        Cell::new("foo2")]));

    // Print the table to stdout
    table.printstd();
}
```

The code above will output

```text
+---------+------+---------+
| ABC     | DEFG | HIJKLMN |
+---------+------+---------+
| foobar  | bar  | foo     |
+---------+------+---------+
| foobar2 | bar2 | foo2    |
+---------+------+---------+
```

## Using macros

For everyday usage consider `table!` macro. This code will produce the same output as above:
```rust
#[macro_use] extern crate prettytable;

fn main() {
    let table = table!(["ABC", "DEFG", "HIJKLMN"],
                       ["foobar", "bar", "foo"],
                       ["foobar2", "bar2", "foo2"]);

    table.printstd();
}
```

The `ptable!` macro combines creating and printing a table:
```rust
#[macro_use] extern crate prettytable;

fn main() {
    let table = ptable!(["ABC", "DEFG", "HIJKLMN"],
                        ["foobar", "bar", "foo"],
                        ["foobar2", "bar2", "foo2"]);
}
```

Tables also support multiline cells content. As a result, you can print a table into another table (yo dawg ;).
For example:
```rust
let table1 = table!(["ABC", "DEFG", "HIJKLMN"],
                    ["foobar", "bar", "foo"],
                    ["foobar2", "bar2", "foo2"]);

let table2 = table!(["Title 1", "Title 2"],
                    ["This is\na multiline\ncell", "foo"],
                    ["Yo dawg ;) You can even\nprint tables\ninto tables", table1]);

table2.printstd();
```
will print
```text
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
```

Rows may have different numbers of cells. The table will automatically adapt to the largest row by printing additional empty cells in smaller rows.

## Do it with style!

Tables can have a styled output with background and foreground colors, bold and italic as configurable settings, thanks to the `term` crate. Alignment in cells can also be set (Left, Right, Center), and a cell can span accross multiple columns.

`term` style attributes are reexported

- directly:
  ```rust
  use prettytable::{Attr, color};

  /* ... */

  table.add_row(Row::new(vec![
      Cell::new("foobar")
          .with_style(Attr::Bold)
          .with_style(Attr::ForegroundColor(color::GREEN)),
      Cell::new("bar")
          .with_style(Attr::BackgroundColor(color::RED))
          .with_style(Attr::Italic(true))
          .with_hspan(2),
      Cell::new("foo")
      ]));
  ```

- through style strings:
  ```rust
  table.add_row(Row::new(vec![
      Cell::new("foobar").style_spec("bFg"),
      Cell::new("bar").style_spec("BriH2"),
      Cell::new("foo")]));
  ```

- using `row!` macro:
  ```rust
  table.add_row(row![bFg->"foobar", BriH2->"bar", "foo"]);
  ```

- using `table!` macro (this one creates a new table, unlike previous examples):
  ```rust
  table!([bFg->"foobar", BriH2->"bar", "foo"]);
  ```

Here
- **bFg** means **bold**, **F**oreground: **g**reen,
- **BriH2** means **B**ackground: **r**ed, **i**talic, **H**orizontal span of **2**.

Another example: **FrBybc** means **F**oreground: **r**ed, **B**ackground: **y**ellow, **b**old, **c**enter.

All cases of styling cells in macros:

- With `row!`, for each cell separately:
  ```rust
  row![FrByb->"ABC", FrByb->"DEFG", "HIJKLMN"];
  ```
- With `row!`, for the whole row:
  ```rust
  row![FY => "styled", "bar", "foo"];
  ```
- With `table!`, for each cell separately:
  ```rust
  table!([FrBybl->"A", FrBybc->"B", FrBybr->"C"], [123, 234, 345, 456]);
  ```
- With `table!`, for whole rows:
  ```rust
  table!([Frb => "A", "B", "C"], [Frb => 1, 2, 3, 4], [1, 2, 3]);
  ```
- With `table!`, mixed styling:
  ```rust
  table!([Frb => "A", "B", "C"], [Frb->1, Fgi->2, 3, 4], [1, 2, 3]);
  ```

### List of style specifiers:

* **F** : **F**oreground (must be followed by a color specifier)
* **B** : **B**ackground (must be followed by a color specifier)
* **H** : **H**orizontal span (must be followed by a number)
* **b** : **b**old
* **i** : **i**talic
* **u** : **u**nderline
* **c** : Align **c**enter
* **l** : Align **l**eft
* **r** : Align **r**ight
* **d** : **d**efault style

### List of color specifiers:

Lowercase letters stand for **usual** colors:
* **r** : Red
* **b** : Blue
* **g** : Green
* **y** : Yellow
* **c** : Cyan
* **m** : Magenta
* **w** : White
* **d** : Black

Uppercase letters stand for **bright** counterparts of the above colors:
* **R** : Bright Red
* **B** : Bright Blue
* ... and so on ...

## Slicing

Tables can be sliced into immutable borrowed subtables.
Slices are of type `prettytable::TableSlice<'a>`.

For example,
```rust
use prettytable::Slice;
/* ... */
let slice = table.slice(2..5);
table.printstd();
```
will print a table with only lines 2, 3 and 4 from `table`.

Other `Range` syntaxes are supported. For example:
```rust
table.slice(..); // Returns a borrowed immutable table with all rows
table.slice(2..); // Returns a table with rows starting at index 2
table.slice(..3); // Returns a table with rows until the one at index 3
```

## Customize look and feel of a table

The look and feel of a table can be customized with `prettytable::format::TableFormat`.

Configurable settings include:
- Borders (left and right)
- Junctions
- Column separators
- Line separators
- Titles (using `table.set_titles()`)

To do this, either:
- create a new `TableFormat` object, then call setters until you get the desired configuration;
- or use the convenient `FormatBuilder` and Builder pattern, shown below

```rust
let mut table = Table::new();
let format = format::FormatBuilder::new()
    .column_separator('|')
    .borders('|')
    .separators(&[format::LinePosition::Top,
                  format::LinePosition::Bottom],
                format::LineSeparator::new('-', '+', '+', '+'))
    .padding(1, 1)
    .build();
table.set_format(format);

table.set_titles(row!["Title 1", "Title 2"]);
table.add_row(row!["Value 1", "Value 2"]);
table.add_row(row!["Value three", "Value four"]);
```

The code above will make the table look like
```
+-------------+------------+
| Title 1     | Title 2    |
| Value 1     | Value 2    |
| Value three | Value four |
+-------------+------------+
```

For convenience, several formats are predefined in `prettytable::format::consts` module.

Some formats and their respective outputs:
- ```rust
  use prettytable::format;

  table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
  ```
  ```
  +-------------+------------+
  | Title 1     | Title 2    |
  +-------------+------------+
  | Value 1     | Value 2    |
  | Value three | Value four |
  +-------------+------------+
  ```
- ```rust
  use prettytable::format;

  table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
  ```
  ```
  Title 1     | Title 2
  ------------+------------
  Value 1     | Value 2
  Value three | Value four
  ```

Check API documentation for the full list of available predefined formats.

## CSV import/export
Tables can be imported from and exported to **CSV**.  This is possible thanks to the default & optional feature `csv`.
> The `csv` feature may become deactivated by default on future major releases.

### Importing
A `Table` can be imported from a string:
```rust
let table = Table::from_csv_string("ABC,DEFG,HIJKLMN\n\
                                    foobar,bar,foo\n\
                                    foobar2,bar2,foo2")?;
```
or from CSV files:
```rust
let table = Table::from_csv_file("input_csv.txt")?;
```
> Those 2 ways of importing CSV assumes a CSV format with `no headers`, and delimited with `commas`

Import can also be done from a CSV reader which allows more customization around the CSV format:
```rust
let reader = /* create a reader */;
/* do something with the reader */
let table = Table::from_csv(reader);
```

### Exporting
Export to a generic `Write`:
```rust
let out = File::create("output_csv.txt")?;
table.to_csv(out)?;
```
or to a `csv::Writer<W: Write>`:
```rust
let writer = /* create a writer */;
/* do something with the writer */
table.to_csv_writer(writer)?;
```

## Note on line endings
By default, the library prints tables with platform specific line ending. This means on Windows,
newlines will be rendered with `\r\n` while on other platforms they will be rendered with `\n`.
Since `v0.6.3`, platform specific line endings are activated though the default feature `win_crlf`, which can be deactivated.
When this feature is deactivated (for instance with the `--no-default-features` flag in cargo), line endings will be rendered with `\n`
on any platform.

This customization capability will probably move to Formatting API in a future release.

Additional examples are provided in the documentation and in [examples](./examples/) directory.
