//! CSV impl and reexported types

extern crate csv;

pub use self::csv::{Reader, Writer, Result, ReaderBuilder};
use std::path::Path;
use std::io::{Read, Write};

impl<'a> super::TableSlice<'a> {
    /// Write the table to the specified writer.
    pub fn to_csv<W: Write>(&self, w: W) -> Result<Writer<W>> {
        self.to_csv_writer(Writer::from_writer(w))
    }

    /// Write the table to the specified writer.
    ///
    /// This allows for format customisation.
    pub fn to_csv_writer<W: Write>(&self,
                                mut writer: Writer<W>)
                                -> Result<Writer<W>> {
        for title in self.titles {
            writer.write_record(title.iter().map(|c| c.get_content()))?;
        }
        for row in self.rows {
            writer.write_record(row.iter().map(|c| c.get_content()))?;
        }

        writer.flush()?;
        Ok(writer)
    }
}

impl super::Table {
    /// Create a table from a CSV string
    ///
    /// For more customisability use `from_csv()`
    pub fn from_csv_string(csv_s: &str) -> Result<Self> {
        Ok(Self::from_csv(
            &mut ReaderBuilder::new()
                .has_headers(false)
                .from_reader(csv_s.as_bytes())))
    }

    /// Create a table from a CSV file
    ///
    /// For more customisability use `from_csv()`
    pub fn from_csv_file<P: AsRef<Path>>(csv_p: P) -> Result<Self> {
        Ok(Self::from_csv(
            &mut ReaderBuilder::new()
                .has_headers(false)
                .from_path(csv_p)?))
    }

    /// Create a table from a CSV reader
    pub fn from_csv<R: Read>(reader: &mut Reader<R>) -> Self {
        Self::init(reader
                        .records()
                        .map(|row| {
                                super::Row::new(row.unwrap()
                                            .into_iter()
                                            .map(|cell| super::Cell::new(&cell))
                                            .collect())
                            })
                        .collect())
    }

    
    /// Write the table to the specified writer.
    pub fn to_csv<W: Write>(&self, w: W) -> Result<Writer<W>> {
        self.as_ref().to_csv(w)
    }

    /// Write the table to the specified writer.
    ///
    /// This allows for format customisation.
    pub fn to_csv_writer<W: Write>(&self, writer: Writer<W>) -> Result<Writer<W>> {
        self.as_ref().to_csv_writer(writer)
    }
}


#[cfg(test)]
mod tests {
    use {Table, Row, Cell};

    static CSV_S: &'static str = "ABC,DEFG,HIJKLMN\n\
                                foobar,bar,foo\n\
                                foobar2,bar2,foo2\n";

    fn test_table() -> Table {
        let mut table = Table::new();
        table
            .add_row(Row::new(vec![Cell::new("ABC"), Cell::new("DEFG"), Cell::new("HIJKLMN")]));
        table.add_row(Row::new(vec![Cell::new("foobar"), Cell::new("bar"), Cell::new("foo")]));
        table.add_row(Row::new(vec![Cell::new("foobar2"),
                                    Cell::new("bar2"),
                                    Cell::new("foo2")]));
        table
    }

    #[test]
    fn from() {
        assert_eq!(test_table().to_string().replace("\r\n", "\n"),
                    Table::from_csv_string(CSV_S)
                        .unwrap()
                        .to_string()
                        .replace("\r\n", "\n"));
    }

    #[test]
    fn to() {
        assert_eq!(
            String::from_utf8(
                test_table()
                    .to_csv(Vec::new())
                    .unwrap()
                    .into_inner()
                    .unwrap()
                ).unwrap(),
                CSV_S);
    }

    #[test]
    fn trans() {
        assert_eq!(
            Table::from_csv_string(
                &String::from_utf8(
                    test_table()
                        .to_csv(Vec::new())
                        .unwrap()
                        .into_inner()
                        .unwrap()
                ).unwrap()
            ).unwrap()
            .to_string()
            .replace("\r\n", "\n"),
            test_table().to_string().replace("\r\n", "\n"));
    }

    #[test]
    fn extend_table() {
        let mut table = Table::new();
        table.add_row(Row::new(vec![Cell::new("ABC"), Cell::new("DEFG"), Cell::new("HIJKLMN")]));
        table.extend(vec![vec!["A", "B", "C"]]);
        let t2 = table.clone();
        table.extend(t2.rows);
        assert_eq!(table.get_row(1).unwrap().get_cell(2).unwrap().get_content(), "C");
        assert_eq!(table.get_row(2).unwrap().get_cell(1).unwrap().get_content(), "DEFG");
    }
}