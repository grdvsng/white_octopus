#[path="../src/basic_elements.rs"]
mod basic_elements;

use basic_elements::*;

#[test]
fn test_table()
{
    let col1 = Col::new("Name", DataType::Text, 0);
    let col2 = Col::new("Age", DataType::Integer, 0);
    let mut t    = Table::new(vec![col1, col2], "Users");
    
    t.insert(vec!["Name", "Age"], vec![CellData::Text("Igor"), CellData::Integer(39)]);
}