mod basic_elements;

use basic_elements::*;


fn main()
{
    let mut t    = Table::new("Users".to_string());

    t.add("Name".to_string(),    DataType::Text);
    t.add("Age".to_string(),     DataType::Integer);
    t.add("Address".to_string(), DataType::Text);

    t.insert(vec!["Name".to_string(), "Age".to_string(), "Address".to_string()], vec![TData::Text("Max"), TData::Integer(35), TData::Text("Oxford 31")]);
    t.insert(vec!["Name".to_string(), "Age".to_string(), "Address".to_string()], vec![TData::Text("Nick"), TData::Integer(15), TData::Text("NY Grow Stree t 12|123 'A' United State of America")]);
    
    println!("{:?}", t);
    println!("\n\n\n{}", t.json_stringify());
    
    println!("\n\n\n{}", t.stringify());
}