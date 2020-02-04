use std::fmt;
use std::string::{ToString};
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone)]
pub struct ListNode<T>
where T: Copy + Clone + fmt::Display,
{
    pub value: T, 
    next:      Option<Rc<RefCell<ListNode<T>>>>, 
    previous:  Option<Rc<RefCell<ListNode<T>>>>,
}


impl<T> ListNode<T>
where T: Copy + Clone + fmt::Display,
{
    pub fn new(value: T, next: Option<ListNode<T>>, previous: Option<ListNode<T>>) -> ListNode<T>
    {
        let mut this = ListNode {
            value:    value, 
            next:     ListNode::rc_from_node_or_null(next), 
            previous: ListNode::rc_from_node_or_null(previous)
        };
        
        this.update();

        return this;
    }

    pub fn update(&mut self)
    {
        if let Some(rc) = self.next.clone()     
        {
            rc.borrow_mut().set_previous(self); 
        }
        
        if let Some(rc) = self.previous.clone() 
        { 
            rc.borrow_mut().set_next(self); 
        }
    }

    fn  set_next(&mut self, next: &mut Self)
    {
        self.next = Some(Rc::new(RefCell::new(next.clone())));
    }

    fn set_previous(&mut self, previous: &mut Self)
    {
       self.previous = Some(Rc::new(RefCell::new(previous.clone())));
    }

    fn rc_from_node_or_null(item: Option<ListNode<T>>) -> Option<Rc<RefCell<ListNode<T>>>>
    {
        let cur_node: Option<Rc<RefCell<ListNode<T>>>>;

        if let Some(node) = item
        {
            cur_node = Some(Rc::new(RefCell::new(node)));
        } else {
            cur_node = None;
        }

        return cur_node;
    }

    pub fn stringfy(node:  Option<Rc<RefCell<ListNode<T>>>>) -> String
    {
        match node
        {
            Some(rc) => rc.borrow_mut().to_string(),
            None     => String::from("nil"),
        }
    }

    fn _get(&self, next: bool) ->  Option<Rc<RefCell<ListNode<T>>>>
    {
        let child = if next { self.next.clone() } else { self.previous.clone() };
         
        return child;
    }

    pub fn get_next(&self) -> Option<Rc<RefCell<ListNode<T>>>>
    {
        return self._get(true);
    }

    pub fn get_previous(&self) -> Option<Rc<RefCell<ListNode<T>>>>
    {
        return self._get(false);
    }

    pub fn set(&mut self, value: T)
    {
        self.value = value;
    }
}

impl<T> fmt::Display for ListNode<T>
where T: Copy + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(f, "{}", self.value);
    }
}

impl<T> fmt::Debug for ListNode<T>
where T: Copy + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let str_view = format!("{{\n\tListNode: {:?}\n\tNext: {:?}\n\tPrevious: {:?} \n}}", 
            self.to_string(), 
            ListNode::stringfy(self.get_next()), 
            ListNode::stringfy(self.get_previous()), 
        );

        return write!(f, "{}", str_view);
    }
}

impl<T> PartialEq for ListNode<T>
where T: Copy + Clone + fmt::Display + PartialEq,
{
    fn eq(self: &ListNode<T>, other: &ListNode<T>) -> bool
    {
        return self.value == other.value;
    }
}