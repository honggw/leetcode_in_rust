/*
Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal substring consisting of non-space characters only.





 */

use std::cmp::max;

pub fn length_of_last_word(s: String) -> i32 {

    if s.len() == 0 {
        return 0;
    }

    let mut index:usize = s.len() - 1;
    let mut max_len = 0;

    loop {

        if s.as_bytes()[index] == ' ' as u8 {

            if max_len > 0 {
                break;
            }

        } else {
            max_len += 1;
        }

        if index == 0 {
            break;
        } else {
            index -= 1;
        }

    }

    max_len
}

#[test]
fn test() {
    println!("{}",length_of_last_word(String::from(" ")));
    println!("{}",length_of_last_word(String::from("a")));
    println!("{}",length_of_last_word(String::from("")));
    println!("{}",length_of_last_word(String::from("afasfssfa")));
    println!("{}",length_of_last_word(String::from("hallo world")));
    println!("{}",length_of_last_word(String::from("hallo world   ")));

}