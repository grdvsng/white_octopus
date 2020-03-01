mod basic_elements;

use basic_elements::*;


fn main()
{
    let mut my_col = Col::new(0);
    let mut my_row: Row<String> = Row::new("Name");
    let mut my_row_next: Row<usize> = Row::new("Years");
    my_row.set(String::from("Vasia"));
    my_row_next.set(32);
    my_col.insert(&mut my_row);
    my_col.insert(&mut my_row_next);

    println!("{}", my_col.stringify());
}