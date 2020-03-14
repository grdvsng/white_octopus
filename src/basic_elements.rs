extern crate regex;
extern crate chrono;

use std::fmt;
use std::string::{ToString};
use chrono::{Utc, DateTime, NaiveDateTime};
use std::cmp::Ordering;
use std::collections::HashMap;
use regex::{Regex, Split};


const COLUMN_WIDTH: usize = 30;


#[derive(Debug, Clone, Copy)]
pub enum DataType
{
    Text,
    Integer,
    Float,
    Boolean,
    Time,
    Table,
    Null,
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
            DataType::Table   => 6,
            _                 => 0,
        }
    }

    fn get_null(&self) -> TData
    {
        match self
        {
            DataType::Text    => TData::Text("0"),
            DataType::Integer => TData::Integer(0), 
            DataType::Float   => TData::Float(0.0),
            DataType::Boolean => TData::Boolean(false),
            DataType::Time    => TData::Time(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)),
            DataType::Table   => TData::Table(Table::new("null".to_string())),
            _                 => TData::Null,
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


#[derive(Clone)]
pub enum TData
{
    Text(&'static str),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Time(DateTime<Utc>),
    Table(Table),
    Null
}

impl TData 
{
    fn get_type(&self) -> DataType
    {
        match *self {
            TData::Text(_)    => DataType::Text,
            TData::Integer(_) => DataType::Integer, 
            TData::Float(_)   => DataType::Float,
            TData::Boolean(_) => DataType::Boolean,
            TData::Time(_)    => DataType::Time,
            _                    => DataType::Null,
        }
    }

    fn stringify(&self) -> String
    {
        match *self {
            TData::Text(x)    => format!("\"{}\"", x),
            TData::Integer(x) => format!("{}", x), 
            TData::Float(x)   => format!("{}", x),
            TData::Boolean(x) => format!("{}", x),
            TData::Time(x)    => format!("\"{}\"", x),
            _                 => format!("{}", "null"),
        }
    }
}

impl PartialOrd for TData 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> 
    {
        Some(self.stringify().cmp(&other.stringify()))
    }
}

impl PartialEq for TData 
{
    fn eq(&self, other: &Self) -> bool 
    {
        (self.get_type() == other.get_type()) && (self.stringify() == other.stringify())
    }
}

impl fmt::Display for TData
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.stringify())
    }
}

impl fmt::Debug for TData
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.stringify())
    }
}


struct StringUtils;
impl   StringUtils 
{
    pub fn part_split(text: String, mut every: usize) -> Vec<String>
    {
        let mut arr                   = Vec::new();
        let mut line                  = "".to_string();
        let (mut step, mut real_step) = (0, 0);
        
        if text.len() < every { return vec![text]; }

        for ch in text.chars()
        {
            real_step += 1;
            line      += &*ch.to_string();
            
            if step == every-1 || real_step == text.len()
            {
                arr.push(line);
                
                step = 0;
                line = "".to_string();

                continue;
            }
            
            step += 1;
        }

        return arr;
    }
}


#[derive(Debug)]
pub enum TableResult 
{
    ColumnExists(String),
    ColumnNotExists(String),
    InvalidTypes(String),
    Ok,
}

impl TableResult
{
    fn get_type(&self) -> usize
    {
        use TableResult::*;

        match *self
        {
            ColumnExists(_)    => 1,
            ColumnNotExists(_) => 2,
            InvalidTypes(_)    => 3,
            _                  => 0,
        }
    }
}

impl PartialEq for TableResult 
{
    fn eq(&self, other: &Self) -> bool 
    {
        return self.get_type() == other.get_type();
    }
}


struct TableUtils;
impl   TableUtils
{
    fn build_row_from_vec_str(vec_str: Vec<Vec<String>>) -> String
    {
        let mut cur_str = "".to_string();
        
        if vec_str.len() > 0
        {    
            let rows_len        = vec_str[0].len();
            let col_lengt       = vec_str.len();
            let mut curent_row  = 0;

            while curent_row < rows_len
            {
                for i in 0..col_lengt
                {
                    cur_str += &*vec_str[i][curent_row];
                }

                curent_row += 1;
                if curent_row < rows_len { cur_str += "\n"; }
            }
        }

        return cur_str;
    }

    fn _build_row<T>(vec_row: &Vec<T>, max_line: usize) -> String
    where T: fmt::Display + Clone
    {
        let mut cur_arr = Vec::new();

        for col in vec_row
        {
            let splited: Vec<String> = StringUtils::part_split(format!("{}", col), COLUMN_WIDTH);
            let mut colm             = Vec::new();
            
            for i in 0..max_line
            {
                if (splited.len()) as isize > i as isize
                {
                    colm.push(format!(" {}{}|", splited[i], " ".repeat((COLUMN_WIDTH - format!("{}", splited[i]).len()) as usize)));
                } else { colm.push(format!(" {}|", " ".repeat(COLUMN_WIDTH))); }
            }

            cur_arr.push(colm);
        }

        return Self::build_row_from_vec_str(cur_arr);
    }

    fn build_row<T>(vec_row: &Vec<T>) -> String
    where T: fmt::Display + Clone
    {
        let mut sorted  = vec_row.clone();
        
        &sorted.sort_by(|a, b| format!("{}", b).len().cmp(&format!("{}", a).len()));

        if sorted.len() > 0
        {
            let line_len = format!("{}", sorted[0]).len();
            let max_line = if line_len > COLUMN_WIDTH { ((line_len as f32 / COLUMN_WIDTH as f32) as f32).ceil() } else { 1.0 };

            return Self::_build_row(vec_row, max_line as usize);
        }

        return "".to_string();
    }

    pub fn stringify(_table: &Table) -> String
    {
        let mut cur_str = Self::build_row(&_table.columns_sorted);
        let line1       = "=".repeat(cur_str.len());
        let line2       = "-".repeat(cur_str.len());
        cur_str         = format!("{0}\n{1}\n{0}", line1, cur_str);
        
        for row in _table.rows.values()
        {
            let mut values: Vec<&TData> = Vec::new();
           
            for val in row.values() { values.push(val); }
            
            cur_str += &*format!("\n{1}\n{0}", line2, Self::build_row(&values));
        }

        return cur_str;
    }

    pub fn json_stringify(_table: &Table) -> String
    {
        let mut cur_str = String::from("[");
        let mut step    = 0;
        let length      = _table.rows.len();

        for row in _table.rows.values()
        {
            cur_str += "{";

            for column_name in &_table.columns_sorted
            {
                cur_str += &*format!("\n\t\"{}\": {},", &column_name, row.get(column_name).unwrap());
            }

            step    += 1;

            cur_str += if step < length {  "\n}, " } else { "\n}" };
        }

        return cur_str + "]"
    }
}

#[derive(Debug, Clone)]
pub struct Table
{
    columns_sorted: Vec<String>,
    columns:      HashMap<String, DataType>,
    rows:         HashMap<usize, HashMap<String, TData>>,
    name:         String,
}


impl Table 
{
    // Basic Table

    pub fn new(name: String) -> Table
    {
        return Table
        { 
            columns_sorted: Vec::new(),
            columns:      HashMap::new(),  
            rows:         HashMap::new(),
            name:         name,
        };
    }

    pub fn json_stringify(&self) -> String
    {
        return TableUtils::json_stringify(self);
    }

    pub fn stringify(&self) -> String
    {
        return TableUtils::stringify(self);
    }

    pub fn add(&mut self, column_name: String,  datatype: DataType ) -> TableResult
    {
        match self.columns.get(&*column_name) 
        {
            Some(_) => return TableResult::ColumnExists(column_name),
            _ => 
            {
                self.columns.insert(column_name.clone(), datatype);
                self.columns_sorted.push(column_name);

                return TableResult::Ok;
            }
        }
    }

    pub fn insert(&mut self, columns_name: Vec<String>, values: Vec<TData>) -> TableResult
    {
        let valid = self.insert_validator(&columns_name);
        
        if TableResult::Ok == valid
        {
            return self._insert(columns_name, values);
        } else {
            return valid;
        }
    }

    fn insert_validator(&self, columns_name: &Vec<String>) -> TableResult
    {
        for column_name in columns_name
        {
            match self.columns.get(&*column_name) 
            {
                Some(_) => (),
                _       => { return TableResult::ColumnNotExists((*column_name.clone()).to_string()); },
            }
        }

        return TableResult::Ok;
    }
    
    fn _insert(&mut self, columns_name: Vec<String>, values: Vec<TData>) -> TableResult
    {
        let mut record: HashMap<String, TData> = HashMap::new();

        for key in self.columns.keys()
        {
            let index = match columns_name.iter().position(|s| &*s == key) { Some(x) => x as isize, _ => -1,};

            if index == -1
            {
                record.insert(String::from(key.clone()), self.columns.get(*&key).unwrap().get_null());
            } else {
                let value = &values[index as usize];

                if &value.get_type() == self.columns.get(*&key).unwrap()
                {
                    record.insert(String::from(key.clone()), value.clone());
                } else { return TableResult::InvalidTypes(format!("Column type: '{:?}', Value type: '{:?}'", self.columns.get(*&key).unwrap(), value.get_type())); }
            }
        }

        self.rows.insert(self.rows.keys().len()+1, record);

        return TableResult::Ok;
    }
}

impl fmt::Display for Table
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", TableUtils::stringify(self))
    }
}