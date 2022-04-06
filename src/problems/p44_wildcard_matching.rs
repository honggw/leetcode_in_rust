
// https://leetcode.com/problems/wildcard-matching/
// * ? 
/**
 * https://www.geeksforgeeks.org/finite-automata-algorithm-for-pattern-searching/
 * 
 * 
 * 
 */

use std::{thread,time};

pub fn wildcard_matching_0(s:String,p:String) -> bool {

    #[derive(PartialEq)]
    struct PatternNode {
        val:char,
        next:Option<Box<PatternNode>>,
    };

    impl PatternNode {

        fn new(pattern:&str) -> Option<Self>{

            if pattern.len() == 0 {
                return None;
            }

            let mut head = PatternNode {
                val:pattern.chars().nth(0).unwrap(),
                next:None,
            };

            head.new_child(&pattern[1..]);

            Some(head)

        }

        fn new_child(&mut self,pattern:&str) {

            if pattern.len() ==0 {
                return;
            }

            let mut next_char=0;

            if self.val == '*' {
                while next_char <= pattern.len()-1 && pattern.chars().nth(next_char).unwrap() == '*' {
                    next_char += 1;
                }
            }

            if next_char == pattern.len() {
                return;
            }

            self.next = Some(Box::new(PatternNode {
                val:pattern.chars().nth(next_char).unwrap(),
                next:None,
            }));

            if next_char < pattern.len() -1 {
                self.next.as_deref_mut().unwrap().new_child(&pattern[next_char+1 ..]);
            }
        }

        fn match_str(&self,input:&str) -> bool {

            let result = false;

            match self.val {
                '*' => {

                    if self.next == None {
                        return true;
                    } else {
                        for i in 0..=input.len() {
                            if self.next.as_deref().unwrap().match_str(&input[i..]) == true {
                                return true;
                            }
                        }
                    }
                    return false;
                },
                _ => {

                    if input.len() == 0 {
                        return false;
                    }

                    match self.val {
                        '?' => {

                        },
                        _ => {
                            if input.chars().nth(0).unwrap() != self.val {
                                return false;
                            }
                        }
                    }

                    if self.next == None {
                        if input.len() > 1 {
                            return false;
                        } else {
                            return true;
                        }
                    } else {
                        return self.next.as_deref().unwrap().match_str(&input[1..]);
                    }

                }
            }

            result

        }

        fn print(&self) {
            print!("{}",self.val);
            if self.next != None {
                self.next.as_deref().unwrap().print();
            }
        }
    }

    if let Some(head) = PatternNode::new(&p[..]) {
        head.match_str(&s[..])
    } else {
        if s.len() > 0 {
            false
        } else {
            true
        }
    }

}


pub fn wildcard_matching(s:String,p:String) -> bool {

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


        fn print0(&self){

            match self {
                PatternItem::Begin=> {
                    print!("Begin->");
                },
                PatternItem::Any => {
                    print!("Any->");
                },
                PatternItem::One(cnt) => {
                    print!("One({})->",cnt);
                },
                PatternItem::Str(s) => {
                    print!("Str({})->",s);
                }
            }

        }

        fn print(&self) {
            // self.print0();
        }

        fn debug(&self) {
            self.print0();
        }


    }

    #[derive(PartialEq)]
    struct PatternItemNode {
        no: usize,
        item: PatternItem,
        next: Option<Box<PatternItemNode>>,
    }

    impl PatternItemNode {

        fn parse(pattern_str:&str) -> Self {

            let mut head = PatternItemNode  {
                no: 0,
                item: PatternItem::Begin,
                next: None,
            };

            head.new_child(pattern_str);

            head

        }

        fn print(&self) {

            match &self.item {

                PatternItem::Begin => {
                    print!("No.{}_Head,",self.no);
                },
                PatternItem::Any => {
                    print!("No.{}_Any,",self.no);
                },
                PatternItem::One(cnt) => {
                    print!("No.{}_One:{},",self.no,*cnt);
                },
                PatternItem::Str(pattern) => {
                    print!("No.{}_Str:{},",self.no,*pattern);
                }

            }

            if self.next != None {
                print!(" -> ");
                self.next.as_deref().unwrap().print();
            } else {
                println!("");
            }

        }

        fn new_child(&mut self,pattern_str:&str) {
            // println!("new_child:{}",pattern_str);

            if pattern_str.len() == 0 {
                return;
            }

            let mut current_item: PatternItem = PatternItem::parse_char(pattern_str.chars().nth(0).unwrap());
            let mut pos = 1;

            loop {

                if pos >= pattern_str.len() {
                    break;
                }

                let new_item = PatternItem::parse_char(pattern_str.chars().nth(pos).unwrap());
                if ! PatternItem::variant_eq(&new_item,&current_item) {
                    break;
                }

                match new_item {
                    PatternItem::One(u) => {
                        if let PatternItem::One(ref mut current_u) = current_item {
                            *current_u += 1;
                        }
                    },
                    PatternItem::Str(new_str) => {
                        if let PatternItem::Str(ref mut s) = current_item {
                            s.push_str(&new_str[..]);
                        }
                    },
                    _ => {

                    }
                }

                pos += 1;

            }

            self.next = Some(Box::new(Self {
                no: self.no + 1,
                item: current_item,
                next: None,
            }));

            if pos < pattern_str.len() {
                self.next.as_deref_mut().unwrap().new_child(&pattern_str[pos..]);
            } 
        }

        /**
         * find substr from beginning
         * return :
         * 0:  equal
         * 1:  equal and remaining 
         * -1: shoter
         * -2: inequal and remaining
         */
        fn find_sub_str(source:&str,sub:&str) -> i32 {

            if source.len() < sub.len() {
                return -1 ;
            }

            for i in 0..sub.len() {
                if source.chars().nth(i).unwrap() != sub.chars().nth(i).unwrap() {
                    return -2 ;
                }
            }

            if source.len() == sub.len() {
                0
            } else {
                1
            }

        }

        fn find_substr(source:&str,sub:&str) -> i32 {

            let mut result:i32 = -1;

            if source.len() < sub.len()  {
                return result;
            }

            let mut pos:usize = 0;

            loop {

                if pos > source.len() - sub.len() {
                    break;
                }

                let mut found = true;

                for i in 0..sub.len() {
                    if source.chars().nth(i+pos).unwrap() != sub.chars().nth(i).unwrap() {
                        found = false;
                        break;
                    }
                }

                if found {
                    result = pos as i32;
                    break;
                }

                pos += 1;
            }

            result

        }

        /**
         * return : matched, no_more_try
         */
        fn match_str(&self, precede_any:bool,content:&str ) -> (bool,bool) {
            let one_second = time::Duration::from_millis(1000);
            thread::sleep(one_second);
            self.item.debug();
            println!(",no={},content={}",self.no,content);

            let mut result = false;
            match &self.item {

                PatternItem::Begin => {
                    if self.next == None && content.len() == 0 {
                        self.item.print();
                        return (true,false);
                    }

                    if self.next != None {
                        return self.next.as_deref().unwrap().match_str(false,&content[..]);
                    }

                },
                PatternItem::Any => {
                    if self.next == None {
                        self.item.print();
                        return (true,false);
                    } else {
                        return self.next.as_deref().unwrap().match_str(true,content);
                    }

                },

                PatternItem::One(cnt) => loop {

                    if content.len() < *cnt {
                        return (false,true);
                    } 

                    if content.len() == *cnt {
                        if self.next == None {
                            self.item.print();
                            return (true,false);
                        } else {
                            return self.next.as_deref().unwrap().match_str(false,String::from("").as_str()); 
                        }
                    }

                    if precede_any {

                        if self.next == None {
                            return (true,false);
                        }

                        // println!("cnt={},content_len={}",*cnt,content.len());
                        for i in *cnt..content.len() {
                            // println!("i={}",i);
                            let (child_result,_) = self.next.as_deref().unwrap().match_str(false,&content[i..]); 
                            if child_result {
                                self.item.print();
                                return (true,false);
                            } 
                        }
                        return self.next.as_deref().unwrap().match_str(false,String::from("").as_str()); 
                    } else {

                        if self.next == None {
                            return (false,false);
                        }

                        let(child_result,_) = self.next.as_deref().unwrap().match_str(false,&content[*cnt..]) ;
                        
                        if child_result == true {
                            self.item.print();
                            return (true,false);
                        } else {
                            return (false,false);
                        }

                    }

                    break;
                },
                PatternItem::Str(s) => loop {
                    if precede_any {

                        let mut from:usize = 0;

                        loop {

                            let occur = PatternItemNode::find_substr(&content[from..], s.as_str());
                            // println!("find s = {},from = {},occur ={}",s,from,occur);

                            match occur {
                                -1 => {
                                    break;
                                },
                                0.. => {

                                    let next_begin:usize = from + occur as usize +s.len();
                                    // println!("next_begin = {}",next_begin);

                                    if self.next == None {
                                        if next_begin >= content.len() {
                                            return (true,false);
                                        } else {
                                            from = from + occur as usize + 1;
                                        }
                                    } else {
                                        let (child_result,no_more_try) = self.next.as_deref().unwrap().match_str(false,&content[next_begin..]);
                                        if child_result == true {
                                            return (true,false);
                                        } else {
                                            if no_more_try {
                                                return (false,true);
                                            } else {
                                                from = from + occur as usize +1;
                                            }
                                        }
                                    }

                                },
                                _ => {
                                    break;
                                    println!("never happen.");
                                }
                            }

                        }

                        if from > 0 {
                            return (false,false);
                        } else {
                            return (false,true);
                        }

                    } else {

                        match PatternItemNode::find_sub_str(content,s.as_str()) {
                            -1 => {
                                return (false,false);
                            },
                            -2 => {
                                return (false,false);
                            },
                            0 => {
                                if self.next == None {
                                    self.item.print();
                                    return (true,false);
                                } else {
                                    let (child_result,_) = self.next.as_deref().unwrap().match_str(false,&content[s.len()..]) ;
                                    if child_result == true {
                                        self.item.print();
                                        return (true,false);
                                    } else {
                                        return (false,false);
                                    }
                                }
                            },
                            1 => {
                                if self.next == None {
                                    return (false,false);
                                } else {
                                    let (child_result,_) = self.next.as_deref().unwrap().match_str(false,&content[s.len()..]) ;
                                    if child_result == true {
                                        self.item.print();
                                        return (true,false);
                                    } else {
                                        return (false,false);
                                    }
                                }
                            },
                            _ => {
                                println!("never happen.");
                            }
                        }

                    }

                    break;
                },
                _ => {
                    println!("never happen.");
                }

            }

            (result,false)
        }

    }

    let mut result = false;
    // println!("s={},p={}",s,p);

    let head = PatternItemNode::parse(&p[..]);
    head.print();
    let (result,_) = head.match_str(false,&s[..]);
    // println!("result={}",result);
    result

}



pub fn wildcard_matching_1(s:String,p:String) -> bool {

    enum PatternNodeType {
        Head,
        Any,
        AnyOne(usize),
        One(usize),
        FixedStr(String),
        AnyFixedStr(String),
        End,
    }

    impl PatternNodeType {

        fn merge(&mut self,target:&Self) -> bool {

            match(self,target) {
                (Self::Any,Self::Any) => {
                    true
                },
                (Self::One(i),Self::One(_)) => {
                    *i += 1;
                    true
                },
                (Self::FixedStr(s),Self::FixedStr(t)) => {
                    s.push_str(t.as_str());
                    true
                },
                _ => false
            }

        }

        fn parse_node_by_char(c : char) ->Self {
            match c {
                '*' => {
                    Self::Any
                },
                '?' => {
                    Self::One(1)
                },
                _=>{
                    Self::FixedStr(String::from(c))
                }
            }
        }

        fn parse_node(after_any:bool,pattern:&str) -> (Self,&str) {

            if pattern.len() == 0 {
                return (Self::End,pattern);
            }

            let mut pos:usize = 0;
            let mut current_node = Self::parse_node_by_char(pattern.chars().nth(pos).unwrap());

            loop {

                pos += 1;

                if pos >= pattern.len() {
                    break;
                }

                let new_node = Self::parse_node_by_char(pattern.chars().nth(pos).unwrap());
                if ! current_node.merge(&new_node) {
                    break;
                }

            }

            if after_any {
                match &current_node {
                    PatternNodeType::One(i) => {
                        current_node = PatternNodeType::AnyOne(*i);
                    },
                    PatternNodeType::FixedStr(s) => {
                        current_node = PatternNodeType::AnyFixedStr(s.clone());
                    },
                    _ => {}
                }
            }

            if pos>= pattern.len() {
                (current_node,&pattern[0..0])
            } else {
                (current_node,&pattern[pos..])
            }

        }

        fn build_list(pattern:&str) -> HeadNode {

            let mut head = HeadNode {
                next:None
            };

            let mut node_arr:Vec<Option<Box<dyn MatcherNode>>> = Vec::new();
            let mut remaining_pattern = pattern;
            let mut after_any = false;

            loop {

                // println!("remaining pattern = {}",remaining_pattern);

                let (node,remaining_pattern1) = Self::parse_node(after_any, remaining_pattern);
                remaining_pattern = remaining_pattern1;

                match node {
                    Self::Any => {
                        after_any = true;
                        node_arr.push(Some(Box::new(AnyNode{next:None})));
                    },
                    Self::AnyOne(i) => {
                        node_arr.push(Some(Box::new(AnyOneNode{size:i,next:None})));
                        after_any = false;
                    },
                    Self::One(i) => {
                        node_arr.push(Some(Box::new(OneNode{size:i,next:None})));
                        after_any = false;
                    },
                    Self::FixedStr(s) => {
                        node_arr.push(Some(Box::new(FixedStrNode{fixed_str:s,next:None})));
                        after_any = false;
                    },
                    Self::AnyFixedStr(s) => {
                        node_arr.push(Some(Box::new(AnyFixedStrNode{fixed_str:s,next:None})));
                        after_any = false;
                    },
                    _ => {
                        break;
                    }
                }

            }

            let mut current_node:Option<Box<dyn MatcherNode>> = None;

            while let Some(mut x) = node_arr.pop() {
                x.as_deref_mut().unwrap().set_child(current_node);
                current_node = x;
            }

            head.set_child(current_node);
            head

        }

        fn build_list_node_by_node(after_any:bool,pattern: &str) -> Option<Box<dyn MatcherNode>> {

            let ( node_type,remaining_pattern) = Self::parse_node(after_any, pattern);
            let mut current_any = false;
            let mut current_node:Option<Box<dyn MatcherNode>> = None;

            match node_type {
                Self::Any => {
                    current_node = Some(Box::new(AnyNode{next:None}));
                    current_any = true;
                },
                Self::One(size) => {
                    current_node = Some(Box::new(OneNode {size:size,next:None}));
                },
                Self::AnyOne(size) => {
                    current_node = Some(Box::new(AnyOneNode {size:size,next:None}));
                },
                Self::FixedStr(fixed_str) => {
                    current_node = Some(Box::new(FixedStrNode{fixed_str:fixed_str,next:None}));
                },
                Self::AnyFixedStr(fixed_str) => {
                    current_node = Some(Box::new(AnyFixedStrNode{fixed_str:fixed_str,next:None}));
                },
                _ => {}
            }

            if current_node.is_none() {
                None
            } else {
                let next = Self::build_list_node_by_node(current_any, remaining_pattern);
                current_node.as_deref_mut().unwrap().set_child(next);
                current_node
            }

        }

        fn build_list_recursive(pattern:&str) -> HeadNode {
            let mut head = HeadNode {next:None};
            head.set_child(Self::build_list_node_by_node(false,pattern));
            head
        }

    }

    #[derive(Debug)]
    enum SearchingResult {
        Matched,
        NotMatched,
        NoMoreTry,
    }

    trait MatcherNode {

        fn do_search(&self,content:&str) -> SearchingResult; 

        fn search(&self,content:&str)->SearchingResult {
            let result = self.do_search(content);
            /*
            print!("==>");
            self.print_self();
            println!("  content={},result={:?}",content,result);
            */
            result
        }
        
        fn next(&self)->&Option<Box<dyn MatcherNode>>;
        fn set_child(&mut self,next:Option<Box<dyn MatcherNode>>);
        fn print_self(&self);

        fn print_list(&self) {
            self.print_self();
            if ! self.next().is_none() {
                self.next().as_deref().unwrap().print_list();
            }
        }

    }

    struct HeadNode {
        next:Option<Box<dyn MatcherNode>>,
    }

    impl MatcherNode for HeadNode {

        fn do_search(&self,content:&str)-> SearchingResult {
            
            match &self.next {
                None => {
                    if content.len() == 0 {
                        SearchingResult::Matched
                    } else {
                        SearchingResult::NotMatched
                    }

                },
                Some(child) => {
                    child.search(content)
                }
            }

        }

        fn next(&self)->&Option<Box<dyn MatcherNode>> {
            &self.next
        }

        fn set_child(&mut self,next:Option<Box<dyn MatcherNode>>) {
            self.next = next;
        }

        fn print_self(&self) {
            print!("Head->");
        }
 
    }

    struct AnyNode {
        next:Option<Box<dyn MatcherNode>>,
    }

    impl MatcherNode for AnyNode {

        fn do_search(&self,content:&str)-> SearchingResult {

            match &self.next {
                None => {
                    SearchingResult::Matched
                },
                Some(child) => {
                    child.search(content)
                }
            }

        }

        fn next(&self)->&Option<Box<dyn MatcherNode>> {
            &self.next
        }

        fn set_child(&mut self,next:Option<Box<dyn MatcherNode>>) {
            self.next = next;
        }

        fn print_self(&self) {
            print!("AnyNode,");
        }
 
 
    }

    struct OneNode {
        size: usize,
        next:Option<Box<dyn MatcherNode>>,
    }

    impl MatcherNode for OneNode {

        fn do_search(&self,content:&str)-> SearchingResult {

            if content.len() < self.size {
                return SearchingResult::NoMoreTry;
            }

            match &self.next {
                None => {
                    if self.size == content.len() {
                        SearchingResult::Matched
                    } else {
                        SearchingResult::NotMatched
                    }
                },
                Some(child) => {
                    child.search(&content[self.size..])
                }
            }

        }

        fn next(&self)->&Option<Box<dyn MatcherNode>> {
            &self.next
        }

        fn set_child(&mut self,next:Option<Box<dyn MatcherNode>>) {
            self.next = next;
        }
 
        fn print_self(&self) {
            print!("OneNode:{},",self.size);
        }
 
    }


    struct AnyOneNode {
        size: usize,
        next:Option<Box<dyn MatcherNode>>,
    }

    impl MatcherNode for AnyOneNode{

        fn do_search(&self,content:&str)-> SearchingResult {

            if content.len() < self.size {
                return SearchingResult::NoMoreTry;
            }

            match &self.next {
                None => {
                    SearchingResult::Matched
                },
                Some(child) => {

                    for i in self.size .. content.len() {
                        match child.search(&content[i..]) {
                            SearchingResult::Matched => {
                                return SearchingResult::Matched;
                            },
                            SearchingResult::NotMatched => {

                            },
                            SearchingResult::NoMoreTry => {
                                return SearchingResult::NoMoreTry;
                            }

                        }
                    }

                    return  child.search(&content[0..0]);

                }
            }

        }

        fn next(&self)->&Option<Box<dyn MatcherNode>> {
            &self.next
        }

        fn set_child(&mut self,next:Option<Box<dyn MatcherNode>>) {
            self.next = next;
        }

        fn print_self(&self) {
            print!("AnyOneNode:{},",self.size);
        }
 

    }

    struct FixedStrNode {
        fixed_str:String,
        next:Option<Box<dyn MatcherNode>>,
    }

    impl FixedStrNode {

        fn find(&self,content:&str) -> SearchingResult {

            if self.fixed_str.len() > content.len() {
                return SearchingResult::NoMoreTry;
            }

            for i in 0..self.fixed_str.len() {
                if self.fixed_str.chars().nth(i).unwrap() != content.chars().nth(i).unwrap() {
                    return SearchingResult::NotMatched;
                }
            }

            SearchingResult::Matched
        }

    }

    impl MatcherNode for FixedStrNode{

        fn do_search(&self,content:&str)-> SearchingResult {

            match self.find(content) {
                SearchingResult::Matched => {

                },
                SearchingResult::NoMoreTry => {
                    return SearchingResult::NoMoreTry;
                },
                SearchingResult::NotMatched => {
                    return SearchingResult::NotMatched;
                }
            }

            match &self.next {
                None => {
                    return if self.fixed_str.len() == content.len() {
                        SearchingResult::Matched
                    } else {
                        SearchingResult::NotMatched
                    }
                },
                Some(child) => {
                    return if self.fixed_str.len() == content.len() {
                        child.search(&content[0..0])
                    } else {
                        child.search(&content[self.fixed_str.len()..])
                    }
                }
            }
        }

        fn next(&self)->&Option<Box<dyn MatcherNode>> {
            &self.next
        }

        fn set_child(&mut self,next:Option<Box<dyn MatcherNode>>) {
            self.next = next;
        }

        fn print_self(&self) {
            print!("FixedStrNode:{},",self.fixed_str);
        }


    }

    struct AnyFixedStrNode {
        fixed_str:String,
        next:Option<Box<dyn MatcherNode>>,
    }

    #[derive(Debug)]
    enum Occurrence {
        NotFound,
        Found(usize),
    }

    impl AnyFixedStrNode {

        fn find(&self,content:&str) -> Occurrence {

            let mut pos = 0;

            loop {

                if pos + self.fixed_str.len() > content.len() {
                    return Occurrence::NotFound;
                }

                let mut found = true;

                for i in 0..self.fixed_str.len() {
                    if self.fixed_str.chars().nth(i).unwrap() != content.chars().nth(pos+i).unwrap() {
                        found = false;
                        break;
                    }
                }

                if found {
                    return Occurrence::Found(pos);
                }  else {
                    pos += 1;
                }

            }

        }

    }

    impl MatcherNode for AnyFixedStrNode {

        fn do_search(&self,content:&str)-> SearchingResult {

            let mut from  = 0;

            while let Occurrence::Found(pos) = self.find(&content[from..]) {
                // println!("pattern = {},content = {},occurence pos={}",self.fixed_str,content[from..].to_string(),pos);
                match &self.next {
                    None => {
                        if from + pos + self.fixed_str.len() == content.len() {
                            return SearchingResult::Matched;
                        }
                    },
                    Some(child) => {

                        let child_searching_result = if from + pos + self.fixed_str.len() == content.len() {
                            child.search(&content[0..0])
                        } else {
                            child.search(&content[from + pos+self.fixed_str.len() .. ])
                        };

                        // println!("child_searching_result={:?}",child_searching_result);

                        match child_searching_result {
                            SearchingResult::NotMatched => {

                            },
                            _ =>  {
                                return child_searching_result;
                            }
                        }

                    }
                }

                from = from + pos + 1;

            }

            SearchingResult::NoMoreTry
        }

        fn next(&self)->&Option<Box<dyn MatcherNode>> {
            &self.next
        }

        fn set_child(&mut self,next:Option<Box<dyn MatcherNode>>) {
            self.next = next;
        }
 
        fn print_self(&self) {
            print!("AnyFixedStrNode:{},",self.fixed_str);
        }

    }

    // let head = PatternNodeType::build_list(p.as_str());
    // head.print_list();
    let head1 = PatternNodeType::build_list_recursive(p.as_str());
    // head1.print_list();
    // head.find(s.as_str())

    match head1.search(s.as_str()) {
        SearchingResult::Matched => true,
        SearchingResult::NoMoreTry => false,
        SearchingResult::NotMatched => false,
    }

}

pub fn wildcard_matching_proxy(s:String,p:String,expect:bool ) -> bool {
    let result = wildcard_matching_1(s.clone(), p.clone());
    if result == expect {
    } else {
        println!("s={},p={},result={},expect={}",s,p,result,expect);
    }
    result
}

/**
 * abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb
 * **aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb
 * *aa*ba*a*bb*aa*ab*a*aaaaaa*a*aaaa*bbabb*b*b*aaaaaaaaa*a*ba*bbb*a*ba*bb*bb*a*b*bb
 */

/**
  * baaabbabbbaabbbbbbabbbaaabbaabbbbbaaaabbbbbabaaaaabbabbaabaaababaabaaabaaaabbabbbaabbbbbaababbbabaaabaabaaabbbaababaaabaaabaaaabbabaabbbabababbbbabbaaababbabbaabbaabbbbabaaabbababbabababbaabaabbaaabbba
  * *b*ab*bb***abba*a**ab***b*aaa*a*b****a*b*bb**b**ab*ba**bb*bb*baab****bab*bbb**a*a*aab*b****b**ba**abba
  */
pub fn solve_wildcard_matching() {
    println!("hello test");

    // println!("{}",wildcard_matching_proxy(String::from("aaaa"), String::from("***a"),true)); // true

    // wildcard_matching_proxy(String::from("hi"), String::from("*?"),true);
    wildcard_matching_proxy(String::from("abcabczzzde"), String::from("*abc???de*"),true);
    /*
    println!("{}",wildcard_matching_proxy(String::from("hello"),String::from("**ll*"),true)); //true
    println!("{}",wildcard_matching_proxy(String::from("hello"),String::from("**?*"),true)); //true
    println!("{}",wildcard_matching_proxy(String::from("hello"),String::from("**a*"),false)); //false
    println!("{}",wildcard_matching_proxy(String::from("hello"),String::from("*"),true)); // true
    println!("{}",wildcard_matching_proxy(String::from("cb"),String::from("?a"),false)); //false
    println!("{}",wildcard_matching_proxy(String::from("cb"),String::from("****"),true)); //true
    println!("{}",wildcard_matching_proxy(String::from(""),String::from(""),true)); //true
    println!("{}",wildcard_matching_proxy(String::from("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb"), String::from("**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb"),false));
    println!("{}",wildcard_matching_proxy(String::from("mississippi"), String::from("m??*ss*?i*pi"),false));//false
    println!("{}",wildcard_matching_proxy(String::from("abcabczzzde"), String::from("*abc???de*"),false)); // true
    println!("{}",wildcard_matching_proxy(String::from("aaaa"), String::from("***a"),true)); // true
    println!("{}",wildcard_matching_proxy(String::from("c"), String::from("*?*"),true)); // true
    println!("{}",wildcard_matching_proxy(String::from("hi"), String::from("*?"),true)); // true
    println!("{}",wildcard_matching_proxy(String::from("ab"), String::from("*?*?*"),true)); // true
    println!("{}",wildcard_matching_proxy(String::from("baaabbabbbaabbbbbbabbbaaabbaabbbbbaaaabbbbbabaaaaabbabbaabaaababaabaaabaaaabbabbbaabbbbbaababbbabaaabaabaaabbbaababaaabaaabaaaabbabaabbbabababbbbabbaaababbabbaabbaabbbbabaaabbababbabababbaabaabbaaabbba"), String::from("*b*ab*bb***abba*a**ab***b*aaa*a*b****a*b*bb**b**ab*ba**bb*bb*baab****bab*bbb**a*a*aab*b****b**ba**abba"),true)); // true
    */
    println!("end");
}



#[test]
fn test_wildcard_matching() {
    println!("hello test");
    assert_eq!(wildcard_matching_proxy(String::from("hello"),String::from("**ll*"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("hello"),String::from("**?*"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("hello"),String::from("**a*"),false),false);
    assert_eq!(wildcard_matching_proxy(String::from("hello"),String::from("*"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("cb"),String::from("?a"),false),false);
    assert_eq!(wildcard_matching_proxy(String::from("cb"),String::from("****"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from(""),String::from(""),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb"), String::from("**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb"),false),false);
    assert_eq!(wildcard_matching_proxy(String::from("mississippi"), String::from("m??*ss*?i*pi"),false),false);
    assert_eq!(wildcard_matching_proxy(String::from("abcabczzzde"), String::from("*abc???de*"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("aaaa"), String::from("***a"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("c"), String::from("*?*"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("hi"), String::from("*?"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("ab"), String::from("*?*?*"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("aaaaba"), String::from("*aa??"),true),true);
    assert_eq!(wildcard_matching_proxy(String::from("baaabbabbbaabbbbbbabbbaaabbaabbbbbaaaabbbbbabaaaaabbabbaabaaababaabaaabaaaabbabbbaabbbbbaababbbabaaabaabaaabbbaababaaabaaabaaaabbabaabbbabababbbbabbaaababbabbaabbaabbbbabaaabbababbabababbaabaabbaaabbba"), String::from("*b*ab*bb***abba*a**ab***b*aaa*a*b****a*b*bb**b**ab*ba**bb*bb*baab****bab*bbb**a*a*aab*b****b**ba**abba"),false),false);

    /*
    println!("{}",wildcard_matching(String::from("hello"),String::from("**ll*"))); //true
    println!("{}",wildcard_matching(String::from("hello"),String::from("**?*"))); //true
    println!("{}",wildcard_matching(String::from("hello"),String::from("**a*"))); //false
    println!("{}",wildcard_matching(String::from("hello"),String::from("*"))); // true
    println!("{}",wildcard_matching(String::from("cb"),String::from("?a"))); //false
    println!("{}",wildcard_matching(String::from("cb"),String::from("****"))); //true
    println!("{}",wildcard_matching(String::from(""),String::from(""))); //true
    println!("{}",wildcard_matching(String::from("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb"), String::from("**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb"))); //false
    println!("{}",wildcard_matching(String::from("mississippi"), String::from("m??*ss*?i*pi")));//false
    println!("{}",wildcard_matching(String::from("abcabczzzde"), String::from("*abc???de*"))); // true
    println!("{}",wildcard_matching(String::from("aaaa"), String::from("***a"))); // true
    println!("{}",wildcard_matching(String::from("c"), String::from("*?*"))); // true
    println!("{}",wildcard_matching(String::from("hi"), String::from("*?"))); // true
    println!("{}",wildcard_matching(String::from("ab"), String::from("*?*?*"))); // true
    println!("{}",wildcard_matching(String::from("aaaaba"), String::from("*aa??"))); // true
    */

    // println!("{}",wildcard_matching_1(String::from("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb"), String::from("**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb"))); //false
 
    println!("{}",wildcard_matching_1(String::from("baaabbabbbaabbbbbbabbbaaabbaabbbbbaaaabbbbbabaaaaabbabbaabaaababaabaaabaaaabbabbbaabbbbbaababbbabaaabaabaaabbbaababaaabaaabaaaabbabaabbbabababbbbabbaaababbabbaabbaabbbbabaaabbababbabababbaabaabbaaabbba"), String::from("*b*ab*bb***abba*a**ab***b*aaa*a*b****a*b*bb**b**ab*ba**bb*bb*baab****bab*bbb**a*a*aab*b****b**ba**abba"))); // false
    println!("end");
}

