mod basic_elements;

use basic_elements::*;


fn main()
{
    let col1 = Col::new("Name", DataType::Text, 0);
    let col2 = Col::new("Age", DataType::Integer, 0);
    let mut t    = Table::new(vec![col1, col2], "Users");
    
    t.insert(vec!["Name", "Age"], vec![CellData::Text("Igor"), CellData::Integer(39)]);
    t.insert(vec!["Name", "Age"], vec![CellData::Text("Vasia"), CellData::Integer(23)]);

    t.add("Address", DataType::Text);
    
    t.insert(vec!["Name", "Age", "Address"], vec![CellData::Text("Max"), CellData::Integer(35), CellData::Text("Oxford 31")]);
    t.insert(vec!["Name", "Age", "Address"], vec![CellData::Text("Nick"), CellData::Integer(15), CellData::Text("NY 12")]);
    
    println!("{}", t);
}