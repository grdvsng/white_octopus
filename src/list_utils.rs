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


enum Item<T>
    where
        T: ToString + fmt::Debug,
{
    First (T, Nil, Nil),
    Top   (T, Nil, Box<Item<T>>),
    Midle (T, Box<Item<T>>, Box<Item<T>>),
    Last  (T, Box<Item<T>>, Nil),
    Null,
}

impl<T> Item<T>
    where
        T: ToString + fmt::Debug,
{
    fn to_string(&self) -> String
    {
        let value: &T;

        match &self 
        {
            Item::First (v, ..) => value = v,
            Item::Top   (v, ..) => value = v,
            Item::Midle (v, ..) => value = v,
            Item::Last  (v, ..) => value = v,
            _                   => return String::from("Nil"),
        }

        return String::from(value.to_string());
    }
}

impl<T> fmt::Display for Item<T> 
    where
        T: ToString + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{:?}", self.to_string());
    }
}

impl<T> fmt::Debug for Item<T> 
    where
        T: ToString + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{:?}", self.to_string());
    }
}


struct List<T>
    where
        T: ToString + fmt::Debug,
{
    length: isize,
    cursor:  Item<T>,
}

trait ListMethods<T>
    where
        T: ToString + fmt::Debug,
{
    // Constructor
    fn new()                     -> List<T>;
    // rem Last element from List and return his ref
    fn pop(&mut self)            -> Result<&Item<T>, String>;
    // rem First element from List and return his ref
    fn top(&mut self)            -> Result<&Item<T>, String>;
    // return Index of value in List or -1
    //fn index_of<T>(&self, value: T) -> isize;
    // Return value bye index or null if index bigger then length
    fn get(&self, index: isize)  -> Result<&Item<T>, String>;
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

impl<T> ListMethods<T> for List<T>
    where
        T: ToString + fmt::Debug,
{
    fn new() -> List<T>
    {
        let nil: Item<T> = Item::Null;
        
        List {
            length: 0,
            cursor: nil,
        }
    }

    fn pop(&mut self) -> Result<&Item<T>, String>
    {
        if (self.length > 1) 
        {
            return self.get(self.length-1);
        } else {
            return Ok(&self.cursor);
        }
    }

    fn top(&mut self) ->Result<&Item<T>, String>
    {
        if (self.length > 1) 
        {
            return self.get(0);
        } else {
            return Ok(&self.cursor);
        }
    }

    fn get(&self, index: isize) -> Result<&Item<T>, String>
    {
        let nil: Item<T> = Item::Null;
        let mut next     = &self.cursor;

        if (index > self.length-1 || self.length == 0)
        {
            return Result::Err(format!("Index '{}' out of List length('{}').", index, self.length));
        } 

        for i in 0..self.length
        {
            if i == index { break; }
            
            match &next
            {
                Item::Top(s, p, n)   => next = &*n,
                Item::Midle(s, p, n) => next = &*n,
                _                    => return Result::Err(String::from("Undefined error!")),
            }
        }

        return Result::Ok(next);
    }
}
