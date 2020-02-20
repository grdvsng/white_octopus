mod list_utils;

use list_utils::*;


fn main()
{
    let mut list: List<i32> = List::new();
    
    list.push(1);
    list.push(2);

    println!("{}", list);
}