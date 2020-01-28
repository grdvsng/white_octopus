use std::fmt;
use std::string::{ToString};


// List last and first element
#[derive(Debug, Clone, Copy)]
pub struct Nil;

impl ToString for Nil
{
    fn to_string(&self) -> String
    {
        return String::from("null");
    }
}


enum Item<'a, 'b, 'c, T>
    where
        T: ToString + fmt::Debug + std::cmp::PartialEq,
{
    Cout(&'a T, &'b Item<'a, 'b, 'c, T>, &'c Item<'a, 'b, 'c, T>),
    Null,
}

impl<'a, 'b, 'c, T> Item<'a, 'b, 'c, T>
    where
        T: ToString + fmt::Debug + std::cmp::PartialEq,
{
    fn to_string(&self) -> String
    {
        let value: &T;

        match &self 
        {
            Item::Cout (v, ..) => value = v,
            _                   => return String::from("Nil"),
        }

        return String::from(value.to_string());
    }

    fn get_value(&self) -> Result<&T, String>
    {
        match &self 
        {
            Item::Cout (v, ..) => Ok(v),
            _                  => Err(self.to_string()),
        }
    }
}

impl<'a, 'b, 'c, T> fmt::Display for Item<'a, 'b, 'c, T>
    where
        T: ToString + fmt::Debug + std::cmp::PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{:?}", self.to_string());
    }
}

impl<'a, 'b, 'c, T> fmt::Debug for Item<'a, 'b, 'c, T> 
    where
        T: ToString + fmt::Debug + std::cmp::PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{:?}", self.to_string());
    }
}


struct List<'a, 'b, 'c, T>
    where
        T: ToString + fmt::Debug + std::cmp::PartialEq,
{
    length: isize,
    cursor: Item<'a, 'b, 'c, T>,
}

trait ListMethods<'a, 'b, 'c, T>
    where
        T: ToString + fmt::Debug + std::cmp::PartialEq,
{
    fn new()                                                             -> List<'a, 'b, 'c, T>;
    fn pop(&mut self)                                                    -> Result<&Item<T>, String>;
    fn top(&mut self)                                                    -> Result<&Item<T>, String>;
    fn _is_value_eq(&self, value: Option<&'static T>, item: &Item<T>)    -> bool;
    fn index_of(&self, value: &'static T)                                -> isize;
    fn get(&self, index: isize)                                          -> Result<&Item<T>, String>;
    fn _get(&self, by_val: Option<&'static T>, by_index:  Option<isize>) -> Result<(&Item<T>, isize), String>;
    fn _get_index_by_value(&self, value: &'static T)                     -> Result<(&Item<T>, isize), String>;
    fn _get_by_index(&self, index: isize)                                -> Result<&Item<T>, String>;
    fn get_validator(&self, index: isize)                                -> bool;
    fn append(&mut self, value: &'a T);
    // Clear list
    /*fn clear(&mut self);
    // Set new value on index
    fn set(&mut self, index: isize, value: Item);
    // Iter all value and make with them some action
    fn for_each<F>(&mut self, action: F) 
        where F: Fn(&Item);
    // Sort List by fn
    fn sort<F>(&mut self, sort_function: F)
        where F: Fn(&Item) -> bool; 
*/
}

impl<'a, 'b, 'c, T> ListMethods<'a, 'b, 'c, T> for List<'a, 'b, 'c, T>
    where
        T: ToString + fmt::Debug + std::cmp::PartialEq,
{
    fn new() -> List<'a, 'b, 'c, T>
    {
        let nil: Item<T> = Item::Null;
        
        List {
            length: 0,
            cursor: nil,
        }
    }

    fn pop(&mut self) -> Result<&Item<T>, String>
    {
        if self.length > 1
        {
            return self.get(self.length-1);
        } else {
            return Ok(&self.cursor);
        }
    }

    fn top(&mut self) ->Result<&Item<T>, String>
    {
        if self.length > 1
        {
            return self.get(0);
        } else {
            return Ok(&self.cursor);
        }
    }
    
    fn index_of(&self, value: &'static T) -> isize
    {
        match self._get_index_by_value(&value)
        {
            Ok(result) => return result.1,
            Err(_)     => -1 as isize,
        }
    }

    fn get_validator(&self, index: isize) -> bool
    {
        if index > self.length-1 || self.length == 0
        {
            return false;
        } else { return true; }
    }
    
    fn _is_value_eq(&self, value: Option<&'static T>, item: &Item<T>) -> bool
    {
        match value
        {
            Some(data) =>
                match &item
                {
                    Item::Cout (v, ..) => v == &data,
                    _                  => false,
                },
            _ => false,
        }
    }

    fn _get(&self, by_val: Option<&'static T>, by_index:  Option<isize>) -> Result<(&Item<T>, isize), String>
    {
        match by_val
        {
            Some(v) => 
                match self._get_index_by_value(v)
                {
                    Ok(v)  => Ok(v),
                    Err(e) => Err(e),
                }
            _       =>
                match by_index
                {
                    Some(i) =>   
                        match self.get(i)
                        {
                            Ok(v)  => Ok((v, i)),
                            Err(e) => Err(e),
                        }
                        ,
                    _       => Err(String::from("Need use byVal or byIndex parametr!")),
                }
        }
    }

    fn get(&self, index: isize) -> Result<&Item<T>, String>
    {
        if !self.get_validator(index) 
        { 
            return Result::Err(format!("Index '{}' out of List length('{}').", index, self.length)); 
        } else {
            return self._get_by_index(index);
        }
    }

    fn _get_index_by_value(&self, value: &'static T) -> Result<(&Item<T>, isize), String>
    {
        let mut next = &self.cursor;
        
        for i in 0..self.length
        {
            if self._is_value_eq(Some(&value), &next) { return Ok((&next, i)); }

            match &next
            {
                Item::Cout(_, _, n) => next = n,
                _                   => return Result::Err(String::from("Undefined error!")),
            }
        }

        return Err(String::from(format!("index of vallue: '{:?}', not found", value)));
    }

    fn _get_by_index(&self, index: isize) -> Result<&Item<T>, String>
    {
        let mut next = &self.cursor;
        
        for i in 0..self.length
        {
            if i == index { break; }
            
            match &next
            {
                Item::Cout(_, _, n)   => next = &*n,
                _                    => return Result::Err(String::from("Undefined error!")),
            }
        }

        return Result::Ok(next);
    }

    fn append(&mut self, value: &'a T)
    {
        if self.length == 0
        {
            self.cursor = Item::Cout(value, &Item::Null, &Item::Null);
        }

        self.length += 1;
    }
}
