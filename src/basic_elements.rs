extern crate regex;
extern crate chrono;

use regex::Regex;
use std::fmt;
use std::string::{ToString};
use chrono::{Utc};


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
            DataType::Text => 1,
            DataType::Text => 2, 
            DataType::Text => 3,
            DataType::Text => 4,
            DataType::Text => 5,
            _              => 6,
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
    Time(Utc),
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
}


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


pub struct Col
{
    rows:      Vec<Cell>,
    name:      &'static str,
    index:     usize,
    data_type: DataType,
}

impl Col {
    pub fn new(name: &'static str, data_type: DataType, index: usize) -> Col
    {
        Col {
            rows:      Vec::new(),
            name:      name,
            index:     index,
            data_type: data_type,
        }
    }

    pub fn add(&mut self, value: CellData) -> bool
    {
        if self.data_type == value.get_type()
        {
           self.rows.push(Cell::new(self.rows.len(), self.index, value));
           
           return true;
        } else {
            println!("Current type is '{:?}', but you pushed '{:?}'", &self.data_type, value.get_type());

            return false;
        }
    }
}


pub struct Table
{
    columns: Vec<Col>,
    name: &'static str,
}


impl Table
{
    pub fn new(columns: Vec<Col>, name: &'static str)-> Table
    {
        let mut t = Table{
                columns: columns,
                name: name,
            };

        t._column_update();

        return t;
    }

    fn _column_update(&mut self)
    {
        let mut step = 0;

        for col in &mut self.columns
        {
            col.index = step;

            step += 1;
        }
    }

    pub fn add(&mut self, column_name: &'static str, data_type: DataType)
    {
        let col = Col::new(column_name, data_type, self.columns.len());

        self.columns.push(col);
    }

    pub fn insert(&mut self, column_names: Vec<&'static str>, values: Vec<CellData>) -> bool
    {
        let mut step = 0;

        for column_name in column_names
        {
            let mut finded = false;

            for col in &mut self.columns
            {
                if col.name == column_name
                {
                    finded = col.add(values[step]);
                }
            }

            if !finded
            {
                println!("Column {} not exists!", column_name);

                return false;
            } else { step += 1; }
        }

        return true;
    }
}