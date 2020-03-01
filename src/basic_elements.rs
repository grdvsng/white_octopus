extern crate regex;
extern crate chrono;

use regex::Regex;
use std::fmt;
use std::string::{ToString};
use chrono::{Utc};


#[derive(Debug, Clone, Copy)]
enum DataType
{
    Text,
    Integer,
    Float,
    Boolean,
    Time,
    Null
}

impl std::cmp::PartialEq for DataType
{
    fn eq(&self, other: &Self) -> bool
    {
        *self == *other
    }
}


#[derive(Debug, Clone, Copy)]
enum CellData
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
        match self
        {
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
    cells:     Vec<Cell>,
    name:      &'static str,
    index:     usize,
    data_type: DataType,
}

impl Col {
    fn new(name: &'static str, index: usize, data_type: DataType) -> Col
    {
        Col {
            cells:     Vec::new(),
            name:      name,
            index:     index,
            data_type: data_type,
        }
    }

    fn add(&mut self, value: CellData)
    {
        if self.data_type == value.get_type()
        {
           self.cells.push(Cell::new(self.cells.len(), self.index, value));
        } else {
            panic!(format!("Current type is '{:?}', but you pushed '{:?}'", &self.data_type, value.get_type()))
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
        let t = Table{
                columns: columns,
                name: name,
            };

        t._column_update();

        return t;
    }

    fn _column_update(&mut self)
    {
        let mut step = 0;

        for col in &self.columns
        {
            col.index = step;

            step += 1;
        }
    }

    pub fn add(&mut self, column_name: &'static str, data_type: DataType)
    {
        let mut col = Col::new(column_name, self.columns.len(), data_type);

        self.columns.push(col);
    }

    pub fn insert(&self, column_names: Vec<&'static str>, values: Vec<CellData>)
    {
        let mut step = 0;

        for column_name in &column_names
        {
            let mut finded = false;

            for col in &self.columns
            {
                if col.name == *column_name
                {
                    col.add(values[step]);
                    finded = true;
                }
            }

            if !finded
            {
                panic!(format!("Column {} not exists!", column_name));
            } else { step += 1; }
        }
    }
}