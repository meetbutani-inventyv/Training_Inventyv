use serde;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::collections::HashMap;


// ============================= Student task ============================= 

/// Student structure to store student data
#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
    pub percentage: Option<f64>,
    pub grade: Option<String>
}



// ============================= Employee task ============================= 

/// Employee structure to store employee data
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub skills: Vec<Skills>,
    pub position: Option<Position>,
    #[serde(rename(serialize = "experiance(y)", deserialize = "experiance(y)"))]
    pub experiance: Option<u8>
}

/// Position enum to store a employee's position
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Position {
    #[serde(rename="Software Developer")]
    SoftwareDeveloper,
    #[serde(rename="Jr. Software Developer")]
    JrSoftwareDeveloper,
    #[serde(rename="Sr. Software Developer")]
    SrSoftwareDeveloper,
    #[serde(rename="Team Lead")]
    TeamLead,
    #[serde(rename="Project Manager")]
    ProjectManager
}

/// Skills enum to store skills of a person
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Skills {
    C,
    #[serde(rename="C#")]
    CS,
    Rust,
    Java,
    Python
}



// ============================== Table task ============================== 

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

/// Table structure to store the table details for making a Table in PDF
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
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
impl Table {
    pub fn new(rows:Vec<Row>, c_padding:f64, t_margin:f64) -> Table {
        let mut table = Table {
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



// ============================== API task ============================== 

/// This structure is used for returning a custom API response
#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    pub status: u32,
    pub message_key: String,
    pub data: T,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Students {
    pub id: u32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
    pub percentage: Option<f64>,
    pub grade: Option<String>
}
