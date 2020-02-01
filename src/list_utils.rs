use std::fmt;
use std::string::{ToString};


pub struct Item<T>
where T: Copy + Clone + fmt::Display,
{
    pub value: T, 
    next:      Option<*mut Item<T>>, 
    previous:  Option<*mut Item<T>>,
}

impl<T> Copy for Item<T> 
where T: Copy + Clone + fmt::Display,
{}

impl<T> Clone for Item<T> 
where T: Copy + Clone + fmt::Display,
{
    fn clone(&self) -> Item<T>
    {
        return *self;
    } 
}

impl<T> Item<T>
where T: Copy + Clone + fmt::Display,
{
    pub fn new(value: T, next: Option<&mut Item<T>>, previous: Option<&mut Item<T>>) -> Item<T>
    {
        let mut this = Item {
            value: value,
            next:     Item::_get_ptr_if_item_not_null(next),
            previous: Item::_get_ptr_if_item_not_null(previous),
        };

        return this;
    }

    fn _get_ptr_if_item_not_null(item: Option<&mut Item<T>>) -> Option<*mut Item<T>>
    {
        match item 
        {
            Some(i) => Some(i.as_ptr()),
            _       => None,
        }
    }

    fn as_ptr(&mut self) -> *mut Item<T>
    {
        return self as *mut Item<T>;
    }

    pub fn get_next<'a>(self) -> Option<&'a mut Item<T>>
    {
        match self.next 
        {
            Some(next) => Some(unsafe { &mut *next }),
            _          => None,
        }
    }

    pub fn get_previous<'a>(self) -> Option<&'a mut Item<T>>
    {
        match self.previous 
        {
            Some(previous) => Some(unsafe { &mut *previous }),
            _              => None,
        }
    }

    fn update_next(&mut self)
    {
        match self.get_next()
        {
            Some(item) => item.previous = Some(self.as_ptr()),
            _          => self.next     = None,
        }
    }

    fn update_previous(&mut self)
    {
        match self.get_previous()
        {
            Some(item) => item.next     = Some(self.as_ptr()),
            _          => self.previous = None,
        }
    }

    pub fn update(&mut self)
    {
        self.update_next();
        self.update_previous();
    }

    pub fn set(&mut self, value: T)
    {
        self.value = value;
        
        self.update();
    }

    fn _next_redefinition(&mut self, next: &Self)
    {

    }
}

impl<T> fmt::Display for Item<T>
where T: Copy + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{}", self.value);
    }
}

fn get_item_value_like_string<T>(item: Option<&mut Item<T>>) -> String
where T: Copy + Clone + fmt::Display,
{
    match item
    {
        Some(item) => item.to_string(),
        None       => String::from("nil"),
    }
}

impl<T> fmt::Debug for Item<T>
where T: Copy + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let str_view = format!("{{\n\tItem: {:?}\n\tNext: {:?}\n\tPrevious: {:?} \n}}", 
            self.to_string(), 
            get_item_value_like_string(self.get_next()), 
            get_item_value_like_string(self.get_previous()), 
        );

        return write!(f, "{}", str_view);
    }
}

impl<T> PartialEq for Item<T>
where T: Copy + Clone + fmt::Display + PartialEq,
{
    fn eq(self: &Item<T>, other: &Item<T>) -> bool
    {
        return self.value == other.value;
    }
}


pub struct List<T>
where T: Copy + Clone + fmt::Display,
{
    first: Option<*mut Item<T>>,
    last:  Option<*mut Item<T>>,
    length: usize,
}

impl<T> List<T>
where T: Copy + Clone + fmt::Display,
{
    pub fn new() -> List<T>
    {
        List {
            first: None,
            last:  None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T)
    {
        self._append(value, self.length);
    }

    fn _append(&mut self, value: T, index: usize)
    {
        let mut item = Item::new(value, None, None);
        let mut step = 0 as usize; 
        let mut next = self.first;

        for i in 0..self.length
        {        
            if i == index { break; }
            
            next = match next 
            {
                Some(node) => match unsafe { (*node).get_next() } 
                    {
                        Some(node_next) => Some(node_next as *mut Item<T>),
                        _               => None,
                    },
                None => None,   // Only for last Item, never true
            }
        }

        match next {
            Some(node) => (unsafe{ * node })._next_redefinition(&item),
            _          => self.set_last(&mut item),
        }
    }

    fn set_last(&mut self, next: &mut Item<T>)
    {

    }
}


