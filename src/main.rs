use std::fmt;
use std::string::{ToString};


#[derive(Debug, Clone, Copy)]
struct Null;

impl ToString for Null
{
    fn to_string(&self) -> String
    {
        String::from("null")
    }
}


struct Item<V, N>
{
    value: V,
    next:  N,
}

impl<V, N> Item<V, N> 
    where 
        V: ToString + fmt::Debug,
        N: ToString + fmt::Debug,
{
    fn new(val: V, next: N) -> Item<V, N>
    {
        return Item {
            value: val,
            next: next,
        }
    }
}

impl<V, N> fmt::Display for Item<V, N> 
    where 
        V: ToString + fmt::Debug,
        N: ToString + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{}", &self.value.to_string());
    }
}

impl<V, N> fmt::Debug for Item<V, N> 
    where 
        V: ToString + fmt::Debug,
        N: ToString + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{:?}", &self.value.to_string());
    }
}


struct List();

#[derive(Clone)]
struct _List<Item>
{
    last:  Item,
    length: isize,
}

impl List
{
    fn new() -> _List<Item<Null, Null>>
    {
        return _List {
            last:   Item::new(Null, Null),
            length: 0,
        }
    }
}

impl<V, N> fmt::Display for _List<Item<V, N>>
    where 
        V: ToString + fmt::Debug,
        N: ToString + fmt::Debug,
{
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut stringfy = String::from("[");
        let first        =  &self.last;

        for i in 0..self.length
        {
            stringfy.push_str(&format!("\n\t{}: {:?}", i, first));
        }
        
        stringfy.push_str(&String::from("\n}"));

        return write!(f, "{}", stringfy);
    }
}

impl<T> fmt::Debug for _List<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{:?}", "1");
    }
}

impl<T> _List<T> 
    where 
        T: ToString + fmt::Debug + std::fmt::Display,
{
    fn push<V>(&mut self, elem: V) -> _List<Item<V, &T>>
        where V: ToString + fmt::Debug,
    {
        return _List {
            last:   Item::new(elem, &self.last),
            length: self.length + 1,  
        }
    }
}


fn main() {
    let item1 = Item::new("123", Null);
    let item2 = Item::new(456, &item1);
    let item3 = Item::new(true, &item2);
    let mut _list = List::new();
    let mut new_list1 = _list.push("123");
    let new_list = new_list1.push(item3);

    println!("{}", new_list);
}