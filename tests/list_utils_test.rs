#[path = "../src/list_utils.rs"] mod list_utils;
use list_utils::{Item};


#[test]
fn test_item()
{
    let mut next       = Item::new("next",     None, None);
    let mut previous   = Item::new("previous", None, None);
    let mut item       = Item::new("self",     Some(&mut next), Some(&mut previous));
    
    assert_eq!(item.value, "self");
    assert_eq!(item.get_next().unwrap().value, "next");
    assert_eq!(item.get_previous().unwrap().value, "previous");
    
    item.update();
    next.set("next_updated");
    previous.set("previous_updated");

    
    assert_eq!(item.get_next().unwrap().value, "next_updated");
    assert_eq!(item.get_previous().unwrap().value, "previous_updated");
}