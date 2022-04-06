
/**
 *  Runtime: 1088 ms, faster than 5.66% of Rust online submissions for Group Anagrams.
    Memory Usage: 5.4 MB, less than 39.62% of Rust online submissions for Group Anagrams.
 */

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

    let mut result:Vec<Vec<String>> = Vec::new();

    struct Anagram {
        anagram_token : Vec<u8>,
        words: Vec<String>,
    }

    impl Anagram {

        fn calc_token(word:&String) -> Vec<u8> {

            let mut result:Vec<u8> = vec![0;26];

            word.chars().for_each(|x| {
                let index:usize = x as usize - 'a' as usize;
                result[index] += 1;
            });

            result

        }

        fn token_equal(token1:&Vec<u8>,token2:&Vec<u8>) -> bool  {

            if token1.len() != token2.len() {
                return false;
            }

            token1.iter().zip(token2).filter(|&(a,b)| {a == b}).count() == token1.len() 

        }

        fn new(token:Vec<u8>,word:String) -> Self {

            let mut obj = Self {
                anagram_token:token,
                words:Vec::new(),
            };

            obj.words.push(word);
            obj

        }

        fn belong(&self,token:&Vec<u8>) -> bool {
            Self::token_equal(&self.anagram_token,token) 
        }

        fn join(&mut self,word:String) {
            self.words.push(word);
        }

        fn collect(self,collector:&mut Vec<Vec<String>>) {
            collector.push(self.words);
        }

    }

    let mut anagrams:Vec<Anagram> = Vec::new();

    strs.iter().for_each (|s| {

        let token = Anagram::calc_token(s);

        if let Some(anagram) = anagrams.iter_mut().find( |x| (**x).belong(&token)) {
            anagram.join(s.clone());
        } else {
            anagrams.push(Anagram::new(token, s.clone()));
        }

    }) ;

    while let Some(anagram) = anagrams.pop() {
        anagram.collect(&mut result);
    }

    result

}

#[test]
fn test() {

    let mut input = vec![String::from("eat"),String::from("tea"),String::from("tan"),String::from("ate"),String::from("nat"),String::from("bat")];
    println!("{:?}",group_anagrams(input));

    let mut input = vec![String::from("a")];
    println!("{:?}",group_anagrams(input));

    let mut input = vec![String::from("")];
    println!("{:?}",group_anagrams(input));

}