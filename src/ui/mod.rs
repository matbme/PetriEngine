use comfy_table::*;
use comfy_table::presets::UTF8_FULL;

pub trait UITable {
    /// Table header
    fn header(&self) -> Vec<&str>;

    /// Table rows
    fn rows(&self) -> Vec<Vec<String>>;

    fn print_table(&self) {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .set_header(self.header());

        for row in self.rows() {
            table.add_row(row);
        }

        println!("{table}");
    }
}
