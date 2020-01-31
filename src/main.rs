mod list_utils;

use list_utils::*;

fn main() 
{
    let mut item1 = Item::new("a", None, None);
    let mut item2 = Item::new("b", Some(&mut item1), None);
    let item3 = Item::new("c", Some(&mut item2.get_next().unwrap()), Some(&mut item2));
    
    println!("{}, {}, {}", item1, item2, item3);

    item2.get_next().unwrap().set("what?");
    println!("[{:?}, {:?}, {:?}]", item1, item2, item3);
}