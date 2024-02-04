use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Cell {
    c_height: u16,
    c_width: u16,
    value: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Row {
    r_height: u16,
    r_width: u16,
    cells: Vec<Cell>
}

impl Row {
    fn calc_row_dimensions(&mut self) {

        let mut final_r_height = 0;
        for i in 0..self.cells.len() {
            if self.cells[i].c_height > final_r_height {
                final_r_height = self.cells[i].c_height;
            }
        }
        self.r_height = final_r_height;


        for i in 0..self.cells.len() {
            self.cells[i].c_height = self.r_height;
        }

        
        let mut total_r_width = 0;
        for i in 0..self.cells.len() {
            total_r_width += self.cells[i].c_width;
        }
        self.r_width = total_r_width;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    height: u16,
    width: u16,
    rows: Vec<Row>
}

impl Table {
    fn calc_table_dimensions(&mut self) {

        let mut total_table_height = 0;
        for i in 0..self.rows.len() {
            total_table_height += self.rows[i].r_height;
        }
        self.height = total_table_height;



        let mut final_table_width = 0;
        for i in 0..self.rows.len() {
            if self.rows[i].r_width > final_table_width {
                final_table_width = self.rows[i].r_width;
            }
        }
        self.width = final_table_width;
    }


    fn update_row_height(&mut self) {
        let row_no = 3;
        let cell_of_row = 1;
        let new_height = 77;

        for i in 0..self.rows.len() {
            if i == row_no-1 {
                for j in 0..self.rows[i].cells.len() {
                    if j == cell_of_row-1 {
                        self.rows[i].cells[j].c_height = new_height;
                    }
                }
            }
        }

        for i in 0..self.rows.len() {
            self.rows[i].calc_row_dimensions();
        }
    
        self.calc_table_dimensions();


        match serde_json::to_string(&self){
            Ok(output_data) => {
                let _ = fs::write("output.json", output_data);
            }
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };
    }
}


fn main() {
    let file_content = match fs::read_to_string("input_data.json") {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    let mut table: Table = match serde_json::from_str(&file_content) {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    for i in 0..table.rows.len() {
        table.rows[i].calc_row_dimensions();
    }


    table.calc_table_dimensions();


    match serde_json::to_string(&table) {
        Ok(output_data) => {
            let _ = fs::write("output.json", output_data);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    // table.update_row_height();

}