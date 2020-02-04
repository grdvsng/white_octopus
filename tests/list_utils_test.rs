#[path = "../src/list_utils.rs"] mod list_utils;
use list_utils::{ListNode};


#[test]
fn test_ListNode()
{
    let mut next     = ListNode::new("next",     None, None);
    let mut previous = ListNode::new("previous", None, None);
    let mut tail     = ListNode::new("tail",     Some(next), Some(previous));
    
    assert_eq!(tail.value, "tail");
    assert_eq!(tail.get_next().unwrap().borrow_mut().value, "next");
    assert_eq!(tail.get_previous().unwrap().borrow_mut().value, "previous");
    
    tail.get_next().unwrap().borrow_mut().set("next_updated");
    
    assert_eq!(tail.get_next().unwrap().borrow_mut().value, "next_updated");
}

#[test]
fn test_list()
{
    //let mut list: List<i32> = List::new();
    
    //list.append(1);
    //list.append(2);
}