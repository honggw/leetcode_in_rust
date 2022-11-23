


// Given the head of a linked list, rotate the list to the right by k places.


// Definition for singly-linked list.

use std::ops::DerefMut;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {

  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val,
    }
  }

  fn new_from_arr(arr:Vec<i32>) -> Option<Box<Self>> {

    if arr.is_empty() {
      return None;
    }

    let mut head : Option<Box<ListNode>> = None;
    let mut node_ptr = &mut head;

    for i in arr {
      *node_ptr = Some(Box::new(Self::new(i)));
      // node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
      node_ptr = &mut node_ptr.as_mut().unwrap().next;
    }

    head

  }

  fn print(&self,no:i32) {

    println!("node no = {},val = {}",no,self.val);

    if self.next.is_some() {
      self.next.as_deref().unwrap().print(no+1);
    }

  }



}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {

  pub struct ListNodeRingIterator<'a> {
    node: Option<&'a ListNode>,
  }

  impl<'a> ListNodeRingIterator<'a> {

    fn new(head:&'a ListNode) -> Self {
      Self {
        node:Some(head),
      }
    }

  }

  impl <'a> Iterator for ListNodeRingIterator<'a> {

    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {

      if self.node.is_none() {
        return None;
      }

      let result =Some(self.node.unwrap().val);
      self.node = if self.node.unwrap().next.is_none() {
        None
      } else {
        Some(self.node.unwrap().next.as_deref().unwrap())
      };

      result
    }

  }

  impl<'a> IntoIterator for &'a ListNode {

    type Item = i32;
    type IntoIter = ListNodeRingIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
      ListNodeRingIterator::new(self)
    }

  }

  fn build_list_from_vec(arr:Vec<i32>) -> Option<Box<ListNode>> {
    if arr.is_empty() {
      return None;
    }

    let mut head: Option<Box<ListNode>> = None;
    let mut node_ptr = &mut head;

    for i in arr {
      *node_ptr = Some(Box::new(ListNode{val:i,next:None}));
      // node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
      node_ptr = &mut node_ptr.as_mut().unwrap().next;
    }

    head

  }

  if head.is_none() {
    return None;
  }

  // let mut cycle = head.as_deref().unwrap().into_iter().cycle();

  let mut arr = head.as_deref().unwrap().into_iter().collect::<Vec<_>>();
  let mut begin = arr.len() - k as usize %arr.len();
  if begin >= arr.len() {
    begin = 0;
  }

  let mut rotate_arr = arr[begin..].to_vec();
  rotate_arr.extend_from_slice(&arr[0..begin]);

  build_list_from_vec(rotate_arr)

}


#[test]
fn test() {
  rotate_right(ListNode::new_from_arr(vec![1,2,3,4,5,]),0).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,3,4,5,]),1).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,3,4,5,]),2).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,3,4,5,]),3).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,3,4,5,]),4).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,3,4,5,]),5).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,]),1).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,]),2).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,]),1).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,]),2).as_deref().unwrap().print(1);
  rotate_right(ListNode::new_from_arr(vec![1,2,]),3).as_deref().unwrap().print(1);
}
