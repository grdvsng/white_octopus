mod basic_elements;

use basic_elements::*;


fn main()
{
    let mut t = Table::new("Users".to_string());

    t.add("Name".to_string(),    DataType::Text,    true);
    t.add("Age".to_string(),     DataType::Integer, false);
    t.add("Address".to_string(), DataType::Text,    false);

    t.insert(vec!["Name".to_string(), "Age".to_string(), "Address".to_string()], vec![TData::Text("Max"), TData::Integer(35), TData::Text("Oxford 31")]);
    t.insert(vec!["Name".to_string(), "Age".to_string(), "Address".to_string()], vec![TData::Text("Nick"), TData::Integer(15), TData::Text("NY Grow Stree t 12|123 'A' United State of America")]);
    t.insert(vec!["Name".to_string(), "Age".to_string()], vec![TData::Text("Alice"), TData::Integer(19)]);
    t.insert(vec!["Name".to_string(), "Age".to_string()], vec![TData::Text("Angel"), TData::Integer(29)]);

    println!("{}", t);

    println!("{:?}", t.select(&|rec| { rec["Age".to_string()] > TData::Integer(25) }));
    println!("{}", t.to_json());
}