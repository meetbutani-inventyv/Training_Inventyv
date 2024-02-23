use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

lazy_static! {
    /// This lazy_static is used to store the standard widths occupied by each keyboard character
    #[derive(Debug)]
    pub static ref FONT_DATA: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("0", 0.5);
        m.insert("1", 0.5);
        m.insert("2", 0.5);
        m.insert("3", 0.5);
        m.insert("4", 0.5);
        m.insert("5", 0.5);
        m.insert("6", 0.5);
        m.insert("7", 0.5);
        m.insert("8", 0.5);
        m.insert("9", 0.5);
        m.insert("", 0.0);
        m.insert(" ", 0.25);
        m.insert("!", 0.333);
        m.insert("\"", 0.555);
        m.insert("#", 0.5);
        m.insert("$", 0.5);
        m.insert("%", 1.0);
        m.insert("&", 0.83300006);
        m.insert("'", 0.27800003);
        m.insert("(", 0.333);
        m.insert(")", 0.333);
        m.insert("*", 0.5);
        m.insert("+", 0.57000005);
        m.insert(":", 0.25);
        m.insert("-", 0.333);
        m.insert(".", 0.25);
        m.insert("/", 0.27800003);
        m.insert(",", 0.333);
        m.insert(";", 0.333);
        m.insert("<", 0.57000005);
        m.insert("=", 0.57000005);
        m.insert(">", 0.57000005);
        m.insert("?", 0.5);
        m.insert("@", 0.93000007);
        m.insert("A", 0.72200006);
        m.insert("B", 0.66700006);
        m.insert("C", 0.72200006);
        m.insert("D", 0.72200006);
        m.insert("E", 0.66700006);
        m.insert("F", 0.611);
        m.insert("G", 0.77800006);
        m.insert("H", 0.77800006);
        m.insert("I", 0.38900003);
        m.insert("J", 0.5);
        m.insert("K", 0.77800006);
        m.insert("L", 0.66700006);
        m.insert("M", 0.94400007);
        m.insert("N", 0.72200006);
        m.insert("O", 0.77800006);
        m.insert("P", 0.611);
        m.insert("Q", 0.77800006);
        m.insert("R", 0.72200006);
        m.insert("S", 0.55600005);
        m.insert("T", 0.66700006);
        m.insert("U", 0.72200006);
        m.insert("V", 0.72200006);
        m.insert("W", 1.0);
        m.insert("X", 0.72200006);
        m.insert("Y", 0.72200006);
        m.insert("Z", 0.66700006);
        m.insert("[", 0.333);
        m.insert("\\", 0.27800003);
        m.insert("]", 0.333);
        m.insert("^", 0.58100003);
        m.insert("_", 0.5);
        m.insert("`", 0.333);
        m.insert("a", 0.5);
        m.insert("b", 0.55600005);
        m.insert("c", 0.44400004);
        m.insert("d", 0.55600005);
        m.insert("e", 0.44400004);
        m.insert("f", 0.333);
        m.insert("g", 0.5);
        m.insert("h", 0.55600005);
        m.insert("i", 0.27800003);
        m.insert("j", 0.333);
        m.insert("k", 0.55600005);
        m.insert("l", 0.27800003);
        m.insert("m", 0.83300006);
        m.insert("n", 0.55600005);
        m.insert("o", 0.5);
        m.insert("p", 0.55600005);
        m.insert("q", 0.55600005);
        m.insert("r", 0.44400004);
        m.insert("s", 0.38900003);
        m.insert("t", 0.333);
        m.insert("u", 0.55600005);
        m.insert("v", 0.5);
        m.insert("w", 0.72200006);
        m.insert("x", 0.5);
        m.insert("y", 0.5);
        m.insert("z", 0.44400004);
        m.insert("{", 0.39400002);
        m.insert("|", 0.22000001);
        m.insert("}", 0.39400002);
        m.insert("~", 0.52000004);
        m
    };  
}


/// Cell structure to store the cell detials for making a Row
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cell {
    pub c_height: f64,
    pub c_width: f64,
    pub value: String,
    pub lines: u32
}

/// Row structure to store the row detials for making a Table
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    pub r_height: f64,
    pub r_width: f64,
    pub row_type: RowType,
    pub font_size: f64,
    pub cells: Vec<Cell>
}

/// Table1 structure to store the table details for making a Table in PDF
#[derive(Debug, Serialize, Deserialize)]
pub struct Table1 {
    pub height: f64,
    pub width: f64,
    pub cell_padding: f64,
    pub table_margin: f64,
    pub rows: Vec<Row>
}

/// RowType enum to identify the type of a row (ie. whether it is a HeaderRow or DataRow)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RowType {
    HeaderRow,
    DataRow
}


/// Cell stucture's methods to create a new cell along with calculating it's height
impl Cell {
    pub fn new(val: String, cell_width: f64, cell_padding:f64, font_size:u32) -> Cell {
       let mut cell = Cell {
            c_width: cell_width,
            c_height: 0.0,
            value: val.clone(),
            lines: 0
        };

        cell.split_chars(val, cell_width, cell_padding, font_size);
        cell
    }

    fn split_chars(&mut self, cell_str:String, cell_width:f64, cell_padding:f64, font_size:u32) -> f64 {
        let mut space_occupied = 0.0;
        let mut cell_height = 0.0;
        let mut cell_h =  font_size as f64 + 2.0*cell_padding;
        let cell_space = cell_width - 2.0*cell_padding;
        let mut s = String::new();
        let mut line = 1;

        for j in cell_str.chars() {
            let ch_width = calc_char_width(j, font_size);

            if(space_occupied + ch_width) <= cell_space {
                space_occupied += ch_width;
                s.push(j);
            }
            else {
                space_occupied = ch_width;
                cell_h += font_size as f64 + 2.0*cell_padding;
                s.push('\n');
                s.push(j);
                line += 1;
            }
            
            if cell_h > cell_height {
                cell_height = cell_h;
            }

            self.value = s.clone();
            self.c_height = cell_height;
            self.lines = line;
        }

        cell_height
    } 
}


/// Row stucture's methods to create a new row along with calculating it's height
impl Row {
    pub fn new(cells:Vec<Cell>, cell_width:f64, font_size:f64, row_type:RowType) -> Row {
        let mut row = Row {
            r_width: cell_width*cells.len() as f64,
            r_height: 0.0,
            cells: cells.clone(),
            font_size: font_size,
            row_type: row_type
        };

        row.calc_row_height(cells);
        row
    }

    fn calc_row_height(&mut self, cells:Vec<Cell>) {
        let mut mx_height = 0.0;

        for i in 0..cells.len() {
            if cells[i].c_height > mx_height {
                mx_height = cells[i].c_height;
            }
        }

        self.r_height = mx_height;
    }
}


/// Table stucture's methods to create a new table along with calculating it's height
impl Table1 {
    pub fn new(rows:Vec<Row>, c_padding:f64, t_margin:f64) -> Table1 {
        let mut table = Table1 {
            width: 0.0,
            height: 0.0,
            cell_padding: c_padding,
            table_margin: t_margin,
            rows: rows.clone()
        };

        table.cal_table_height(rows);
        table
    }

    fn cal_table_height(&mut self, rows:Vec<Row>) {
        let mut t_height = 0.0;
        let mut t_width = 0.0;

        for i in 0..rows.len() {
            t_height += rows[i].r_height;
            
            if rows[i].r_width > t_width {
                t_width = rows[i].r_width;
            }
        }

        self.width = t_width;
        self.height = t_height + 2.0*self.table_margin;
    }
}


/// Function to calculate the width occupied by a keyboard character
pub fn calc_char_width(ch:char, font_size:u32) -> f64 {
    if let Some(value) = FONT_DATA.get(ch.to_string().as_str()) {
        return value*font_size as f64;
    }
    else {
        return 0.0;
    }
}


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


#[wasm_bindgen]
pub fn generate_table(content: String) -> String {
    let table_data: Table = match serde_json::from_str(&content) {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return "".to_string();
        }
    };

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


    match serde_json::to_string_pretty(&table_data_content) {
        Ok(output_data) => {
            return output_data;
        }
        Err(err) => {
            println!("Error: {}", err);
            return "Error fetching the data".to_string();
        }
    };
}