extern crate regex;
extern crate chrono;
extern crate json;


use std::fmt;
use std::string::{ToString};
use chrono::{Utc, DateTime, NaiveDateTime};
use std::cmp::Ordering;
use std::collections::HashMap;
use regex::{Regex, Split};
use json::*;


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
            TData::Table(_)   => DataType::Table,
            _                 => DataType::Null,
        }
    }

    fn stringify(&self) -> String
    {
        match self {
            TData::Text(x)    => format!("\"{}\"", x),
            TData::Integer(x) => format!("{}", x), 
            TData::Float(x)   => format!("{}", x),
            TData::Boolean(x) => format!("{}", x),
            TData::Time(x)    => format!("\"{}\"", x),
            TData::Table(x)   => format!("\"{}\"", x),
            _                 => format!("{}", "null"),
        }
    }

    fn to_json(&self) -> JsonValue
    {
        match self 
        {
            TData::Text(x)    => (*x).into(),
            TData::Integer(x) => (*x as usize).into(),
            TData::Float(x)   => (*x).into(),
            TData::Boolean(x) => (*x).into(),
            TData::Time(x)    => format!("\"{}\"", x).into(),
            TData::Table(x)   => x.to_json(),
            _                 => JsonValue::Null,
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


#[derive(Debug)]
pub enum TableResult 
{
    ColumnExists(String),
    ColumnNotExists(String),
    InvalidTypes(String),
    RequiredFieldNotExist(String),
    ValuesLenNeqPropertiesName(String),
    Ok,
}

impl TableResult
{
    fn get_type(&self) -> usize
    {
        use TableResult::*;

        match *self
        {
            ColumnExists(_)               => 1,
            ColumnNotExists(_)            => 2,
            InvalidTypes(_)               => 3,
            RequiredFieldNotExist(_)      => 4,
            ValuesLenNeqPropertiesName(_) => 5,
            _                             => 0,
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


struct TableForameter;
impl   TableForameter
{
    fn build_row<T>(rows: &Vec<T>) -> String
    where T: fmt::Display + Clone
    {
        let mut cur_str    = "".to_string();
        let max_line       = Self::get_max_column_length(rows);
        let mut second_row = Vec::new();

        for col in rows
        {
            let formated = format!("{}", col);
            let splited;

            if formated.len() <= COLUMN_WIDTH
            {
                splited = formated;

                if max_line > 1 { second_row.push("".to_string()); }
            } else {
                splited = formated[0..COLUMN_WIDTH].to_string();
                second_row.push(formated[COLUMN_WIDTH..].to_string());
            }

            cur_str += &*format!(" {}{}|", splited, " ".repeat((COLUMN_WIDTH - splited.len()) as usize));
        }

        if second_row.len() > 0 { cur_str += &*format!("\n{}", Self::build_row(&second_row)); } 
        
        return cur_str;
    }

    fn get_max_column_length<T>(rows: &Vec<T>) -> usize
    where T: fmt::Display + Clone
    {
        let mut sorted  = rows.clone();
        
        &sorted.sort_by(|a, b| format!("{}", b).len().cmp(&format!("{}", a).len()));

        if sorted.len() > 0
        {
            let line_len = format!("{}", sorted[0]).len();
            let max_line = (if line_len > COLUMN_WIDTH { ((line_len as f32 / COLUMN_WIDTH as f32) as f32).ceil() } else { 1.0 }) as usize;

            return max_line;
        }

        return 1;
    }

    pub fn stringify(_table: &Table) -> String
    {
        let mut cur_str = Self::build_row(&_table.columns_names);
        let line1       = "=".repeat(cur_str.len());
        let line2       = "-".repeat(cur_str.len());
        cur_str         = format!("{0}\n{1}\n{0}", line1, cur_str);
        
        for row in &_table.get_like_rows()
        {
            cur_str += &*format!("\n{1}\n{0}", line2, Self::build_row(row));
        }

        return cur_str;
    }
}

#[derive(Debug, Clone)]
pub struct TColumn
{
    name:     String,
    class:    DataType,
    index:    usize,
    required: bool
}

impl TColumn
{
    pub fn new(name: String, class: DataType, index: usize, required: bool) -> TColumn
    {
        return TColumn
        {
            name:     name,
            class:    class,
            index:    index,
            required: required,
        };
    }
}


#[derive(Debug, Clone)]
pub struct Record
{
    map:   HashMap<String, TData>,
    index: usize,
}

impl Record 
{
    
    pub fn new(index: usize) -> Record
    {
        return Record{index: index, map: HashMap::new(), };
    }

    fn to_json(&self) -> JsonValue
    {
        let mut json_object = json::JsonValue::new_object();

        for key in self.map.keys()
        {
            json_object[key] = self.map.get(key).unwrap().to_json();
        }

        return json_object;
    }
}


impl std::ops::IndexMut<String> for Record
{
    fn index_mut(&mut self, index: String) -> &mut Self::Output 
    {
        if let Some(x) = self.map.get_mut(&*index)
        {
            return self.map.get_mut(&*index).unwrap();
        } else {
            self.map.insert(index.clone(), TData::Null);
            
            return self.map.get_mut(&*index).unwrap();
        }
    }
}


impl std::ops::Index<String> for Record 
{
    type Output = TData;

    fn index(&self, index: String) -> &Self::Output 
    {
        return self.map.get(&*index).unwrap();
    }
}

impl fmt::Display for Record
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:?}", self.map)
    }
}


#[derive(Debug, Clone)]
pub struct Table
{
    columns_names:  Vec<String>,
    columns:        HashMap<String, TColumn>,
    name:           String,
    cout:           usize,
    records:        Vec<Record>
}


impl Table 
{
    pub fn new(name: String) -> Table
    {
        return Table
        { 
            columns_names: Vec::new(),
            columns:       HashMap::new(),
            name:          name,
            cout:          0,
            records:       Vec::new(),
        };
    }

    pub fn add(&mut self, column_name: String,  datatype: DataType, required: bool) -> TableResult
    {
        let col = TColumn::new(column_name.clone(), datatype, self.columns.keys().len(), required);
        
        match self.columns.get(&*column_name) 
        {
            Some(_) => return TableResult::ColumnExists(column_name),
            _ => 
            {
                self.columns.insert(column_name.clone(), col);
                self.columns_names.push(column_name.clone());

                return TableResult::Ok;
            }
        }
    }

    pub fn insert(&mut self, columns_name: Vec<String>, values: Vec<TData>) -> TableResult
    {
        let valid = self.insert_validator(&columns_name, &values);
        
        if TableResult::Ok == valid
        {
            return self._insert(columns_name, values);
        } else {
            return valid;
        }
    }

    fn insert_validator(&self, columns_name: &Vec<String>, values: &Vec<TData>) -> TableResult
    {
        for column_name in columns_name
        {
            if columns_name.len() != values.len()
            {
                return TableResult::ValuesLenNeqPropertiesName(format!("{} != {}", columns_name.len(), values.len()));
            }

            match self.columns.get(&*column_name) 
            {
                Some(_) => (),
                _       => { return TableResult::ColumnNotExists((*column_name.clone()).to_string()); },
            }
        }

        return self.insert_validator_check_required(columns_name);
    }
    
    fn insert_validator_check_required(&self, columns_name: &Vec<String>) -> TableResult
    {
        for col in self.columns.values()
        {
            if  col.required && None == columns_name.iter().position(|r| *r == col.name)
            {
                return TableResult::RequiredFieldNotExist(format!("{}", col.name));
            }
        }

        return TableResult::Ok;
    }

    fn _insert(&mut self, columns_name: Vec<String>, values: Vec<TData>) -> TableResult
    {
        let mut record = Record::new(self.records.len() + 1);

        for col in self.columns.values_mut()
        {
            let index = match columns_name.iter().position(|s| *s == col.name) { Some(x) => x as isize, _ => -1,};

            if index == -1
            {
                record[col.name.clone()] = TData::Null;
            } else {
                let value = &values[index as usize];

                if &value.get_type().get_type() == &col.class.get_type()
                {
                     record[col.name.clone()] = value.clone();
                } else { return TableResult::InvalidTypes(format!("Column type: '{:?}', Value type: '{:?}'", col.class, value.get_type())); }
            }
        }

        self.records.push(record);
        self.cout = self.records.len();
        
        return TableResult::Ok;
    }

    fn get_like_rows(&self) -> Vec<Vec<TData>>
    {
        let mut table = Vec::new();

        for record in self.records.clone()
        {
            let mut row = Vec::new();

            for name in &self.columns_names
            {
                row.push(record[name.clone()].clone());
            }

            table.push(row);
        }

        return table;
    }
    
    pub fn select(&self, query: &dyn Fn(&Record) -> bool) -> Vec<Record>
    {
        let mut results = Vec::new();

        for rec in self.records.clone()
        {
            if query(&rec)
            {
                results.push(rec);
            }
        }

        return results;
    } 
    
    #[warn(unused_must_use)]
    pub fn to_json(&self) -> JsonValue
    {
        let mut _records = JsonValue::new_array();

        for record in &self.records
        {
            _records.push(record.to_json());
        }

        return object!
        {
            "name"    => &*self.name,
            "cout"    => self.cout,
            "records" => _records,
        };
    }
}

impl fmt::Display for Table
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", TableForameter::stringify(self))
    }
}