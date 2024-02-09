use std::fs;
use serde::{Deserialize, Serialize};
use crate::common_modules::{Cell, Row, Table as Table1, RowType};

/// HeaderRow structure to store the header row's information of a table
#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderRow {
    #[serde(rename="fontSize")]
    font_size: u32,
    title: Vec<String>
}

/// DataRow structure to store the data row's information of a table
#[derive(Debug, Serialize, Deserialize)]
pub struct DataRow {
    #[serde(rename="fontSize")]
    font_size: u32,
    rows: Vec<Vec<String>>
}

/// Table structure to store the overall information of a table
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    headerRow: HeaderRow,
    dataRows: DataRow,
    pageWidth: u32
}


pub fn main() {
    // Reading the table data from the json file
    let file_content = match fs::read_to_string("src/json_data/TableData.json") {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    // Deserializing the table data
    let table_data: Table = match serde_json::from_str(&file_content) {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    // println!("{:#?}", table_data);


    let cell_padding = 5.0;
    let table_margin = 5.0;
    let page_width = table_data.pageWidth as f64;
    let no_of_cells = table_data.headerRow.title.len();
    let cell_width = (page_width-(2.0*table_margin))/no_of_cells as f64;
    println!("Total cells: {}", no_of_cells);

    let mut rows: Vec<Row> = Vec::new();


    // ====================================== Header row ===================================== //

    let mut cells: Vec<Cell> = Vec::new();

    for i in 0..no_of_cells {
        let c = Cell::new(table_data.headerRow.title[i].clone(), cell_width as f64, cell_padding, table_data.headerRow.font_size);
        cells.push(c);

    }
    
    let r = Row::new(cells, cell_width, table_data.headerRow.font_size as f64, RowType::HeaderRow);
    rows.push(r);



    // ====================================== Data rows ===================================== //

    for r in 0..table_data.dataRows.rows.len() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..table_data.dataRows.rows[r].len() {
            let c = Cell::new(table_data.dataRows.rows[r][i].clone(), cell_width as f64, cell_padding, table_data.dataRows.font_size);
            cells.push(c);
        }

        let r = Row::new(cells, cell_width as f64, table_data.dataRows.font_size as f64, RowType::DataRow);
        rows.push(r);
    }


    // Creating a table instance from the vector of rows data
    let table_data_content = Table1::new(rows.clone(), cell_padding, table_margin);


    // Serializind the table data and storing it in a json file
    match serde_json::to_string_pretty(&table_data_content) {
        Ok(output_data) => {
            let _ = fs::write("src/json_data/tableOutput.json", output_data);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
}