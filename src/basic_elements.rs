extern crate regex;
extern crate chrono;

#[path="./array_utils.rs"]
mod array_utils;

use regex::Regex;
use std::fmt;
use std::string::{ToString};
use chrono::{Utc, DateTime, NaiveDateTime};
use array_utils::*;
use std::sync::Arc;
use std::cell::RefCell;


#[derive(Debug, Clone, Copy)]
pub enum DataType
{
    Text,
    Integer,
    Float,
    Boolean,
    Time,
    Null
}

impl DataType
{
    fn get_type(&self) -> usize
    {
        match self
        {
            DataType::Text    => 1,
            DataType::Integer => 2, 
            DataType::Float   => 3,
            DataType::Boolean => 4,
            DataType::Time    => 5,
            _                 => 6,
        }
    }

    fn get_null(&self) -> CellData
    {
        match self
        {
            DataType::Text    => CellData::Text("0"),
            DataType::Integer => CellData::Integer(0), 
            DataType::Float   => CellData::Float(0.0),
            DataType::Boolean => CellData::Boolean(false),
            DataType::Time    => CellData::Time(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)),
            _                 => CellData::Null,
        }
    }
}


impl std::cmp::PartialEq for DataType
{
    fn eq(&self, other: &Self) -> bool
    {
        self.get_type() == other.get_type()
    }
}


#[derive(Debug, Clone, Copy)]
pub enum CellData
{
    Text(&'static str),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Time(DateTime<Utc>),
    Null
}


impl CellData 
{
    fn get_type(&self) -> DataType
    {
        match *self {
            CellData::Text(x)    => DataType::Text,
            CellData::Integer(x) => DataType::Integer, 
            CellData::Float(x)   => DataType::Float,
            CellData::Boolean(x) => DataType::Boolean,
            CellData::Time(x)    => DataType::Time,
            _                    => DataType::Null,
        }
    }

    fn stringify(&self) -> String
    {
        match *self {
            CellData::Text(x)    => format!("{}", x),
            CellData::Integer(x) => format!("{}", x), 
            CellData::Float(x)   => format!("{}", x),
            CellData::Boolean(x) => format!("{}", x),
            CellData::Time(x)    => format!("{}", x),
            _                    => format!("{}", "null"),
        }
    }
}

impl fmt::Display for CellData
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.stringify())
    }
}


#[derive(Clone)]
pub struct Cell
{
    row_index: usize,
    col_index: usize,
    value:     CellData,
}

impl Cell 
{
    fn new(row_index: usize, col_index: usize,  value: CellData) -> Cell
    {
        Cell {
            row_index: row_index,
            col_index: col_index,
            value:     value,  
        }
    }
}

impl fmt::Display for Cell
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for Cell
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

pub struct Col
{
    cells:     Vec<Arc<RefCell<Cell>>>,
    name:      &'static str,
    index:     usize,
    data_type: DataType,
}

impl Col {
    pub fn new(name: &'static str, data_type: DataType, index: usize) -> Col
    {
        Col {
            cells:     Vec::new(),
            name:      name,
            index:     index,
            data_type: data_type,
        }
    }

    pub fn add(&mut self, value: CellData) -> Option<Arc<RefCell<Cell>>>
    {
        if (self.data_type == value.get_type()) || (DataType::Null == value.get_type())
        {
            let _cell = Arc::new(RefCell::new(Cell::new(self.cells.len(), self.index, value)));
            
            self.cells.push(_cell.clone());
           
            return Some(_cell);
        }
            
        println!("Current type is '{:?}', but you pushed '{:?}'\n({:?})", &self.data_type, value.get_type(),value );

        return None;
    }
}


pub struct Table
{
    columns:       Vec<Col>,
    columns_names: Vec<&'static str>,
    rows:          Vec<Vec<Arc<RefCell<Cell>>>>,
    pub name:      &'static str,
}


impl Table
{
    pub fn new(columns: Vec<Col>, name: &'static str)-> Table
    {
        let mut t = Table
        {
                columns:       columns,
                columns_names: Vec::new(),
                name:          name,
                rows:          Vec::new(),
            };

        t.update_col();

        return t;
    }

    fn update_col(&mut self)
    {
        let mut step = 0;

        for col in &mut self.columns
        {
            col.index = step;

            self.columns_names.push(col.name.clone());

            step += 1;
        }
    }

    pub fn add(&mut self, column_name: &'static str, data_type: DataType)
    {
        let mut col = Col::new(column_name, data_type, self.columns.len());
        
        for i in 0..self.rows.len()
        {
            self.rows[i].push(col.add(CellData::Null).unwrap());
        }
        
        self.columns.push(col);
        self.columns_names.push(column_name);
    }

    fn _insert(&mut self, column_names: Vec<&'static str>, values: Vec<CellData>) -> bool
    {
        let mut step                          = 0;
        let mut row: Vec<Arc<RefCell<Cell>>>  = Vec::new();

        for name in &self.columns_names
        {
            let index = Array_Utils::index_of(&column_names, name);
            let value = if (index != -1) { values[index as usize] } else { CellData::Null };
            let col   = self.columns[step].add(value);

            if let Some(item) = col
            {
                row.push(item);
                step += 1;
            } else { return false; }
        }

        self.rows.push(row);
        
        return true;
    }

    pub fn insert(&mut self, column_names: Vec<&'static str>, values: Vec<CellData>) -> bool
    {
        let def:  Vec<&'static str> = Array_Utils::filter(&self.columns_names, &|&x| Array_Utils::index_of(&column_names, x) == -1);

        if def.len() > 0 
        { 
            println!("Columns: '{:?}' is not exists!", def); 
        
            return false;
        }  else { 
            return self._insert(column_names, values); 
        }

    }
    
    fn stringify_rows<T>(&self, row: &Vec<T>, max_line_len: usize)-> String
    where T: fmt::Display + fmt::Debug + Clone
    {
        let mut cur_row = String::from("");

        for mut data in row
        {
            let mut value = format!("{}", data);
            let real_val  = 
                if value.len() >= max_line_len
                {
                    format!(r"{}...", (&*value).chars().into_iter().take(max_line_len-5).collect::<String >())
                } else { value };
            
                cur_row += &*format!(" {}{}|", real_val, " ".repeat(max_line_len - real_val.len()) );
        }

        return cur_row;
    }

    fn get_column_header(&self) -> String
    {
        let mut max     = 30;
        let line        = "=".repeat((max * self.columns_names.len()) + (self.columns_names.len()*2));   

        return format!("\n{0}\n{1}\n{0}", line, self.stringify_rows(&self.columns_names, max));
    }

    pub fn stringify(&self) -> String
    {
        let table_params = self.get_column_header();
        let mut cur_str  = table_params;
        let max_line_len = 30;
        let line         = "-".repeat((max_line_len * self.columns_names.len()) + (self.columns_names.len()*2));

        for row in &self.rows
        {
            let mut real_row: Vec<Cell> = Vec::new();

            for cell in row
            {
                real_row.push(cell.borrow().clone());
            }

            cur_str += &*format!("\n{}\n{}", self.stringify_rows(&real_row, max_line_len), line);
        }

        return cur_str;
    }
}

impl fmt::Display for Table
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.stringify())
    }
}