
// mod problems;
use std::{thread,time,iter::FromIterator};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct QueueNode {
    pub val: i32,
    pub next: Option<Box<QueueNode>>,
}

impl QueueNode {

    #[inline]
    fn new(val:i32) -> QueueNode {
        QueueNode{val:val,next:None}
    }

}

fn test_iter() {
    let mut vec_one:Vec<i32> = vec![1,2,3,];
    let mut vec_two:Vec<i32> = vec![4,5,6,];
    let mut vec_in_vec:Vec<Vec<i32>> = vec![vec_one,vec_two.clone()];
    vec_two.iter_mut().map(|x| {*x+=1;*x}).collect::<Vec<i32>>();
    let mut vec_in_vec2=vec_in_vec.iter_mut().map(|x|x.clone()).collect::<Vec<Vec<i32>>>();
}


#[derive(PartialEq)]
enum PatternItem {
    Begin,
    Any, // * 
    One(usize), // ?
    Str(String), // 
    // End,
}

impl PatternItem {

    fn parse_char(ch:char) -> Self {
        match ch {
            '*' => Self::Any,
            '?' => Self::One(1),
            _ => {
                let mut s = String::new();
                s.push(ch);
                Self::Str(s)
            }
        }
    }

    fn variant_eq(a:&Self,b:&Self ) -> bool {

        match(a,b) {
            (&Self::Begin,&Self::Begin) => true,
            (&Self::Any,&Self::Any) => true,
            (&Self::One(_),&Self::One(_)) => true,
            (&Self::Str(_),&Self::Str(_)) => true,
            _ => false,
        }

    }

}

#[test]
fn test_outside() {

}


#[cfg(test)]
mod test {
    #[test]
    fn testQueueNodeNew() {
        println!("hello test");
    }

}

