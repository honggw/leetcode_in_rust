use std::{thread,time,iter::FromIterator};

pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {

    #[inline]
    fn new(val: i32) -> ListNode {
        ListNode { next: None, val }
    }

    fn build_from(arr:Vec<i32>) -> Option<Box<ListNode>> {

        let mut result:Option<Box<ListNode>> = None;
        let mut node_ptr = &mut result;

        for val in arr.iter() {
            *node_ptr = Some(Box::new(ListNode::new(*val)));
            node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
        }

        /*
        arr.iter().map( |&val| {
            *node_ptr = Some(Box::new(ListNode::new(1)));
            node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
        });
        */

        result
    }

    fn print(&self) {
        print!("{},",self.val);
        if self.next != None {
            self.next.as_deref().unwrap().print();
        } else {
            println!("");
        }
    } 

    fn insert(&mut self,val:i32) {
        let mut new_node = Some(Box::new(ListNode::new(val)));
        new_node.as_deref_mut().unwrap().next = self.next.clone();
        self.next = new_node;
    }

    fn push(&mut self ,val:i32) {

        if self.next == None {
            self.next =  Some(Box::new(ListNode::new(val)));
        } else {
            self.next.as_deref_mut().unwrap().push(val);
        }

    }

}

impl Solution {
    //0. two sum
    pub fn solve_two_sum_problem() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        println!("two sum result = {:?}", Solution::two_sum(nums, target));
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for left in 0..nums.len() as i32 {
            for right in left..nums.len() as i32 {
                if left == right {
                    continue;
                }

                println!(
                    "left = {},right = {}",
                    nums[left as usize], nums[right as usize]
                );

                if nums[left as usize] + nums[right as usize] == target {
                    return vec![left, right];
                }
            }
        }

        vec![]
    }

    //1. add two number
    pub fn solve_add_two_number_problem() {
        let mut l1_header = ListNode::new(-1);
        let mut l2_header = ListNode::new(-1);

        //set l1
        let mut point: &mut ListNode = &mut l1_header;
        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();

        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();

        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();

        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();

        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();


        //set l2
        point = &mut l2_header;
        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();

        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();

        point.next = Some(Box::new(ListNode::new(9)));
        point = point.next.as_deref_mut().unwrap();

        let mut result = Solution::add_two_number(l1_header.next, l2_header.next);
        point = result.as_deref_mut().unwrap();

        loop {
            println!("result:{}",point.val);
            if point.next.is_none() {
                break;
            }
            point = point.next.as_deref_mut().unwrap();
        }
    }

    pub fn add_two_number(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        let mut header = Some(Box::new(ListNode::new(0)));
        let mut currentNode = header.as_deref_mut().unwrap();
        let mut l1Node:&ListNode = l1.as_deref().unwrap();
        let mut l2Node:&ListNode = l2.as_deref().unwrap();
        let mut end1 = ListNode::new(-1);
        let mut end2 = ListNode::new(-1);

        loop {
            let mut the_end = true;

            if l1Node.val >=0 {
                currentNode.val += l1Node.val ;
            }
            if l1Node.next.is_some() {
                l1Node = l1Node.next.as_deref().unwrap();
                the_end = false;
            } else {
                l1Node = &mut end1;
            }

            if l2Node.val >= 0 {
                currentNode.val += l2Node.val;
            }
            if l2Node.next.is_some() {
                l2Node = l2Node.next.as_deref().unwrap();
                the_end = false;
            } else {
                l2Node = &mut end2;
            }

            let mut acc = 0;

            if currentNode.val > 9 {
                acc = currentNode.val /10;
                currentNode.val = currentNode.val %10;
            }

            if the_end {
                if acc >0 {
                    currentNode.next = Some(Box::new(ListNode::new(acc)));
                }
                break;
            } else {
                currentNode.next = Some(Box::new(ListNode::new(acc)));
                currentNode = currentNode.next.as_deref_mut().unwrap();
            }

        }

        header
    }

    //2 length of longest substring
    pub fn length_of_longest_substring(s: String ) -> i32 {

        let mut longest:i32 = 0;
        let mut sub_start:i32= 0;
        let mut current:i32= 0;

        let mut char_occurence:[i32;128] = [-1;128];

        loop {

            if current >= s.len() as i32 {
                let new_longest = current - sub_start;
                if new_longest > longest {
                    longest = new_longest;
                }
                break;
            }

            let current_char:u8 = s.as_bytes()[current as usize];
            let last_occur = char_occurence[current_char as usize];

            if sub_start <= last_occur {
                let new_longest = current-sub_start;
                if new_longest > longest {
                    longest = new_longest;
                } 
                sub_start = last_occur + 1;
            }
            char_occurence[current_char as usize ] = current;
            current +=1;
        
        }

        longest
    }

    pub fn solve_length_of_longest_substring() {
        let result = Solution::length_of_longest_substring(String::from("bbbbb"));
        println!("longest substring = {}",result);
    }

    pub fn find_median_sorted_arrays(nums1:Vec<i32>,nums2:Vec<i32>) -> f64 {

        0.0 
    }

    pub fn longest_palindrome(s:String)->String {

        if s.len() == 0 {
            return String::from("");
        }

        #[derive(Debug)]
        struct Slice {
            pub start:i32,
            pub end: i32 ,
        }

        let mut slices: Vec<Slice> = Vec::new();
        let mut result:String = s.chars().take(1).collect();

        for pos in 0..s.len() as i32 {

            //new slice
            let pre = pos -1;
            if pre >=0 && s.as_bytes()[pre as usize] == s.as_bytes()[pos as usize]{
                slices.push(Slice {start:pos,end: -1 ,});
            }

            let prepre = pos - 2;
            if prepre >=0 && s.as_bytes()[prepre as usize] == s.as_bytes()[pos as usize]{
                slices.push(Slice {start:pre,end: -1 ,});
            }

            // evalue existing slices
            for slice in slices.iter_mut() {

                let new_start = slice.start - 1;

                if new_start >= 0 && s.as_bytes()[new_start as usize] == s.as_bytes()[pos as usize] {
                    slice.start = new_start;

                    // still ongoing
                    if pos < s.len() as i32 -1 && new_start > 0 {
                        continue;
                    }

                    // end of slice due to reaching end of string
                    slice.end = pos;
                } else {
                    // end of the slice due to mismatch
                    slice.end = pos - 1;
                }

            }

            slices.retain( |slice| {
                if slice.end == -1 {
                    true
                } else {
                    // update the result
                    if slice.end - slice.start + 1 > result.len() as i32 {
                        result = s.chars().skip(slice.start as usize).take((slice.end - slice.start + 1) as usize).collect();
                    }
                    false
                }
            });

        }

        result
    }

    pub fn solve_longest_palindrome() {

        let problem = String::from("a");
        let result = Solution::longest_palindrome(problem.clone());
        println!("problem={},result={}",problem ,result);

        let problem = String::from("");
        let result = Solution::longest_palindrome(problem.clone());
        println!("problem={},result={}",problem ,result);

        let problem = String::from("hello");
        let result = Solution::longest_palindrome(problem.clone());
        println!("problem={},result={}",problem ,result);

        let problem = String::from("bb");
        let result = Solution::longest_palindrome(problem.clone());
        println!("problem={},result={}",problem ,result);

        let problem = String::from("abb");
        let result = Solution::longest_palindrome(problem.clone());
        println!("problem={},result={}",problem ,result);


        let problem = String::from("akaakaaka");
        let result = Solution::longest_palindrome(problem.clone());
        println!("problem={},result={}",problem ,result);

    }

    pub fn zigzag_conversion(s:String,num_rows:i32) -> String {

        let mut result = String::from("");

        if num_rows < 1 {
            return result;
        }

        if num_rows == 1 {
            return s;
        }

        let col_len : usize = 2 * num_rows as usize  -2;
        let cols_per_line = s.len()/col_len;
        let col_width : usize = num_rows as usize - 1;
        let col_tail = s.len() % col_len;
        // println!("\n\ns={},s.len()={},num_rows={}",s.clone(),s.len(),num_rows.clone());
        // println!("col_len={},cols_per_line={},col_width={},col_tail={}\n\n",col_len.clone(),cols_per_line.clone(),col_width.clone(),col_tail.clone());

        for row in 0..num_rows as usize {

            let second = num_rows as usize -1 - row;
            let mut col_offset = 0;

            for col in 0..cols_per_line {
                let mut column:Vec<char> = vec![' ';col_width];

                // let col_offset = col*col_len;
                column[0] = s.chars().nth(col_offset+row).unwrap();
                if second > 0 && second < col_width {
                    column[second] = s.chars().nth(num_rows as usize + col_offset + second-1).unwrap();
                }

                result.push_str(column.into_iter().collect::<String>().as_str());
                col_offset += col_len;
            }

            //tail col
            
            // println!("col_offset = {},row={},second={}",col_offset.clone(),row.clone(),second.clone());
            if row < col_tail  {
                let mut column:Vec<char> = vec![' ';col_width];
                column[0] = s.chars().nth(col_offset+row).unwrap();
                if second > 0 && col_tail >= num_rows as usize + second  {
                    column[second] = s.chars().nth(num_rows as usize + col_offset + second -1 ).unwrap();
                } 
                result.push_str(column.into_iter().collect::<String>().as_str());
            }

            //end of row
            result.push('\n');

        }

        result
    }

    pub fn zigzag_conversion_0(s:String,num_rows:i32) -> String {
        let mut result = String::from("");

        if num_rows < 1 {
            return result;
        }

        if num_rows == 1 {
            return s;
        }

        let col_len : usize = 2 * num_rows as usize  -2;
        let cols_per_line = s.len()/col_len;
        let col_width : usize = num_rows as usize - 1;
        let col_tail = s.len() % col_len;
        // println!("\n\ns={},s.len()={},num_rows={}",s.clone(),s.len(),num_rows.clone());
        // println!("col_len={},cols_per_line={},col_width={},col_tail={}\n\n",col_len.clone(),cols_per_line.clone(),col_width.clone(),col_tail.clone());

        for row in 0..num_rows as usize {

            let second = num_rows as usize -1 - row;
            let mut col_offset = 0;

            for col in 0..cols_per_line {
                result.push(s.chars().nth(col_offset+row).unwrap());
                if second > 0 && second < col_width {
                    result.push(s.chars().nth(num_rows as usize + col_offset + second-1).unwrap());
                }

                col_offset += col_len;
            }

            //tail col
            
            // println!("col_offset = {},row={},second={}",col_offset.clone(),row.clone(),second.clone());
            if row < col_tail  {
                result.push(s.chars().nth(col_offset+row).unwrap());
                if second > 0 && col_tail >= num_rows as usize + second  {
                    result.push(s.chars().nth(num_rows as usize + col_offset + second -1 ).unwrap());
                } 
            }

            //end of row

        }

        result
    }


    pub fn zigzag_conversion_1(s:String,num_rows:i32) -> String {

        let mut result:String = String::from("");

        if num_rows == 1 {
            return s;
        }

        let last_row:usize = num_rows as usize - 1;

        let col_len : usize = 2 * num_rows as usize  -2;

        let mut row:usize = 0;

        while row <= last_row {

            let second:usize = 2*last_row -  row ;
            let if_second = second > last_row && second < col_len;
            let mut col_offset = 0;

            loop {
                let first_char = col_offset+row;
                if first_char >= s.len() {
                    break;
                }

                result.push(s.chars().nth(first_char).unwrap());

                if if_second {
                    let second_char = col_offset + second;
                    if second_char >= s.len() {
                        break;
                    } else {
                        result.push(s.chars().nth(second_char).unwrap());
                    }
                } 

                col_offset += col_len;
                if col_offset >= s.len() {
                    break;
                }

            }

            row += 1;
        }

        result
    }

    pub fn solve_zigzag_conversion() {
        let s = String::from("0123456789ABCD");
        let sleep_time = time::Duration::from_secs(1);
        let start:usize = 1;
        let end:usize = 1;
        // let end:usize = s.len() + 1;

        // for num_row in start..end {
        //     let mut result = Solution::zigzag_conversion(s.clone(),num_row as i32); 
        //     println!("result{} = \n{}",num_row,result);
        //     println!("");
        //     thread::sleep(sleep_time);
        // }

        println!("result = {}",Solution::zigzag_conversion_0(String::from("PAYPALISHIRING"), 3));
        println!("result = {}",Solution::zigzag_conversion_1(String::from("PAYPALISHIRING"), 3));

    }

    pub fn reverse_integer(x: i32) -> i32 {
        let mut result:i64 = 0 ; 
        let mut problem:i64 = x.clone() as i64;

        loop {
            let current_digit = problem%10;
            result += current_digit;

            problem = problem/10;
            if problem != 0 {
                result = result*10;
            } else {
                break;
            }

        }

        if result > i32::MAX as i64|| result < i32::MIN as i64{
            result = 0
        }

        result as i32
    }


    pub fn solve_reverse_integer() {
        println!("reverse 123 = {}",Solution::reverse_integer(123));
        println!("reverse -123 = {}",Solution::reverse_integer(-123));
        println!("reverse -120 = {}",Solution::reverse_integer(-120));
        println!("reverse 0 = {}",Solution::reverse_integer(0));
        println!("reverse {} = {}",i32::MAX,Solution::reverse_integer(i32::MAX));
        println!("max = {}",i32::MAX);
        println!("min = {}",i32::MIN);

    }


    pub fn string_to_integer(s:String ) -> i32 {

        let mut minus = false;
        let mut begin = false;
        let mut result:i64 = 0;

        for iter in s.chars().into_iter() {

            if !begin {
                match iter {
                    ' ' | '0' => {
                        continue;
                    },
                    '-' => {
                        minus = true;
                        begin = true;
                        continue;
                    },
                    '+' => {
                        begin = true;
                        continue;
                    },
                    _ => {

                    }
                }
            }

            match iter.to_digit(10) {
                Some(i) => {
                    if i == 0 && result == 0 {

                    } else {
                        result = result * 10 + i as i64;
                        begin = true;
                    }

                },
                None => {
                    break;
                }
            }
        }

        if minus {
            result = -result;

            if result < i32::MIN as i64{
                result = i32::MIN as i64;
            }
        } else {
            if result > i32::MAX as i64 {
                result = i32::MAX as i64;
            }
        }

        result as i32
    }

    pub fn solve_string_to_integer() {
        // println!("{}",Solution::string_to_integer(String::from("-123432")));
        // println!("{}",Solution::string_to_integer(String::from("-12343223424223")));
        // println!("{}",Solution::string_to_integer(String::from("   123432")));
        // println!("{}",Solution::string_to_integer(String::from("00000-42a1234")));
        println!("{}",Solution::string_to_integer(String::from("21474836460")));
    }

    pub fn is_palindrome (x: i32) -> bool {

        if x < 0 {
            return false;
        }

        let mut origin:i64 = x.clone() as i64;
        let mut reverse:i64 = 0;

        loop {

            if origin == 0 {
                break;
            }

            let digit:i64 = origin % 10;
            if reverse >= 0 {
                reverse = reverse*10;
            }
            reverse = reverse + digit;

            origin = origin/10;
            println!("reverse = {}",reverse.clone());
        }

        reverse == x as i64
    }

    pub fn solve_is_palindrome() {
        Solution::is_palindrome(121);
    }

    pub fn regular_expression_matching(s:String,p:String) -> bool {
        // . a* .* 
        false
    }

    pub fn solve_regular_expression_matching() {
        assert!(! Solution::regular_expression_matching(String::from("pp"),String::from("p")));
    }

    pub fn container_with_most_water(height:Vec<i32>) -> i32 {

        let mut max = 0;
        let height_len = height.len();
        let mut start_max_height:i32 = 0;

        for (start , iter_start) in height.iter().enumerate() {

            if *iter_start <= start_max_height {
                continue;
            }

            if start == height_len - 1 {
                break;
            }

            let mut max_height_end = 0;

            for (end , iter_end) in height.iter().skip(start+1).enumerate().rev() {

                if max_height_end > *iter_end {
                    continue;
                }

                let h = if iter_start > iter_end { iter_end} else { iter_start };
                let area = h * (end as i32+1);
                if area > max {
                    max = area;
                }
                max_height_end = *iter_end;

            }

            start_max_height = *iter_start;

        }

        max
    }

    pub fn container_with_most_water1(height:Vec<i32>) -> i32 {

        let mut result:i32 = 0 ;
        let mut max_height = 0;

        for i in 0..height.len()-1 {

           if height[i] <= max_height {
                continue;
            }

            let mut max_height1= 0;

            for j in (i..(height.len() )).rev() {
                if height[j] <= max_height1 {
                    continue;
                }
                let new_area = (j-i) as i32 * if height[i] > height[j] { height[j]} else {height[i]};
                if new_area > result {
                    result = new_area;
                }
                max_height1 = height[j];
                // println!("i={},j={},new_area = {}",i,j,new_area);
            }

            max_height = height[i];
        }

        println!("result = {}",result);
        result
    }

    pub fn container_with_most_water2(height:Vec<i32>) -> i32 {

        let mut result:i32 = 0;
        let mut begin = 0;
        let mut end = height.len() -1;

        loop {

            let width = end - begin ;

            if width <= 0 {
                break;
            }

            let mut area = 0;

            if height[begin] > height[end] {
                area = height[end] as usize * width;
                end = end - 1;
            } else {
                area = height[begin] as usize * width;
                begin = begin + 1;
            }

            if area as i32> result {
                result = area as i32;
            }

        }
        println!("result = {}",result);

        result

    }


    pub fn solve_container_with_most_water() {

        Solution::container_with_most_water1(vec!(1,1));
        Solution::container_with_most_water1(vec!(4,3,2,1,4));
        Solution::container_with_most_water1(vec!(1,8,6,2,5,4,8,3,7));
        Solution::container_with_most_water1(vec!(1,2,1));
 
        Solution::container_with_most_water2(vec!(1,1));
        Solution::container_with_most_water2(vec!(4,3,2,1,4));
        Solution::container_with_most_water2(vec!(1,8,6,2,5,4,8,3,7));
        Solution::container_with_most_water2(vec!(1,2,1));
        // let height = vec!(1,2,3,4,5,6);

        // for (i,iter) in height.iter().enumerate() {
        //     for (j,iter1) in height.iter().skip(i+1).enumerate().rev() {
        //         println!("i={},{}; j={},{}",i,iter,j,iter1);
        //     }
        // }

    }

    pub fn longest_common_prefix(strs:Vec<String>) -> String {

        let mut result = String::from("");
        let mut pos = 0;

        loop {

            let mut ch:u8= 0 ;

            for i in 0..strs.len() {

                let  item = &strs[i];

                if item.len() <= pos {
                    ch = 0 ;
                    break;
                }

                if ch == 0 {
                    ch = item.bytes().nth(pos).unwrap() ;
                } else if ch != item.bytes().nth(pos).unwrap() {
                    ch = 0 ;
                    break;
                }

            }

            if ch == 0  {
                break;
            } else {
                result.push(ch as char);
            }

            pos += 1;

        }

        result

    }

    pub fn solve_longest_common_prefix() {

        let mut strs = vec![String::from("fly"),String::from("flow"),String::from("flee"),];
        println!("{}",Solution::longest_common_prefix(strs.clone()));
        strs.push(String::from("free"));
        println!("{}",Solution::longest_common_prefix(strs.clone()));

    }

    pub fn three_sum(nums: Vec<i32>) ->Vec<Vec<i32>> {

        let mut result:Vec<Vec<i32>> =Vec::new();

        for i in 0 .. nums.len() {
            for j in i+1 .. nums.len() {
                for k in j+1 .. nums.len() {

                    let n1 = nums[i];
                    let n2 = nums[j];
                    let n3 = nums[k];

                    if n1 + n2 + n3 == 0 {

                        let mut one_result = vec!(n1,n2,n3);

                        for i in 0..one_result.len() {
                            for j in i..one_result.len() {
                                if one_result[i] >= one_result[j] {
                                    let old_i = one_result[i];
                                    one_result[i] = one_result[j];
                                    one_result[j] = old_i;
                                }
                            }
                        }

                        let mut exists = false;
                        for i in 0..result.len() {
                            let another_result = &result[i];
                            if another_result[0] == one_result[0] && another_result[1] == one_result[1] && another_result[2] == one_result[2] {
                                exists = true;
                                break;
                            }
                        }

                        if !exists {
                            result.push(one_result);
                        }

                    }

                }
            }
        }

        result
    }

    pub fn three_sum_2(nums: Vec<i32>) ->Vec<Vec<i32>> {

        let mut result:Vec<Vec<i32>> =Vec::new();
        let mut input = nums.clone();

        for i in 0..input.len() {
            for j in i+1 .. input.len() {
                if input[i] > input[j] {
                    let old_i = input[i];
                    input[i] = input[j];
                    input[j] = old_i;
                }
            }
        }

        // println!("orderd input = {:?}",input);

        for i in 0..input.len() {
            if input[i] > 0 {
                break;
            }
            for j in i+1..input.len() {

                if input[i] + input[j] > 0 {
                    break;
                }

                for k in j+1..input.len() {

                    let sum = input[i] + input[j] + input[k];

                    if sum > 0 {
                        break;
                    }

                    if sum ==0 {

                        let mut exists = false;

                        for z in 0..result.len() {
                            let one = &result[z];
                            if input[i] == one[0] && input[j] == one[1] && input[k] == one[2] {
                                exists = true;
                                break;
                            }
                        }

                        if !exists {
                            result.push(vec!(input[i],input[j],input[k]));
                        }

                    }

                }
            }
        }

        result

    }

    pub fn three_sum_3(nums: Vec<i32>) ->Vec<Vec<i32>> {

        let mut result:Vec<Vec<i32>> =Vec::new();

        if nums.len() < 3 {
            return result;
        }

        let mut input = nums.clone();

        for i in 0..input.len() {
            for j in i+1..input.len() {
                if input[i] > input[j] {
                    let old_i = input[i];
                    input[i] = input[j];
                    input[j] = old_i;
                }
            }
        }

        // println!("input = {:?}",input);

        fn next_low(nums0:&[i32],old:usize) -> usize {

            if old == 0 {
                return nums0.len();
            }
            let mut new_low = old -1 ;

            loop {
                if nums0[old] == nums0[new_low] {
                    if new_low == 0 {
                        return nums0.len();
                    } else {
                        new_low = new_low - 1;
                    }
                } else {
                    return new_low;
                }
            }
        }

        fn next_high(nums0:&[i32],old:usize) -> usize {

            let mut new_high = old + 1;

            loop {
                if new_high >= nums0.len() {
                    return new_high;
                }
                if nums0[old] == nums0[new_high] {
                    new_high = new_high+1;
                } else {
                    break;
                }
            }

            new_high

        }



        fn my_two_sum(nums:&[i32], target:i32) -> Vec<Vec<i32>> {

            let mut result:Vec<Vec<i32>> =Vec::new();
            if nums.len() < 2 {
                return result;
            }

            let medium = target/2;
            let mut low:usize ;
            let mut start:usize = 0;
            let mut end = nums.len() -1;

            loop {

                low = (start+end)/2;
                if low == start || nums[low] == medium {
                    break;
                }

                if nums[low] < medium {
                    start = low;
                } else {
                    end = low;
                }

            }

            let new_low = next_low(nums, low);
            if new_low == nums.len() {
                low = 0;
            } else {
                low = new_low+1;
            }

            // println!("low = {}, target = {},nums.len={},nums={:?}",low,target,nums.len(),nums);

            let mut high = low+1;

            loop {

                let two_sum = nums[low] + nums[high] - target;

                if two_sum == 0 {
                    result.push(vec![nums[low],nums[high]]);
                }

                if two_sum > 0 {
                    low = next_low(nums, low);
                } else if two_sum < 0 {
                    high = next_high(nums,high);
                } else {
                    low = next_low(nums, low);
                    high = next_high(nums,high);
                }

                if low == nums.len() || high == nums.len() {
                    break;
                }

            }

            result

        }

        let mut i:usize = 0;

        loop {

            if i >= input.len() {
                break;
            }

            if input[i] > 0 {
                break;
            }

            let two_sum_result = my_two_sum(&input[i+1..], -input[i]);
            for mut one_result in two_sum_result {
                one_result.insert(0,input[i]);
                result.push(one_result);
            }

            i = next_high(&input[0..],i);
            // println!("next i = {}",i);

        }

        result

    }



    pub fn solve_three_sum() {

        let mut input = vec!(-1,0,1,2,-1,-4);

        println!("result={:?}",Solution::three_sum_3(input));

        input = vec!(
            34,55,79,28,46,33,2,48,31,-3,84,71,52,-3,93,15,21,-43,57,-6,86,56,94,74,83,-14,28,-66,46,-49,62,-11,43,65,77,12,47,61,26,1,13,29,55,-82,76,26,15,-29,36,-29,10,-70,69,17,49
        );

        // println!("result={:?}",Solution::three_sum_3(input));

        input = vec!(
            82597,-9243,62390,83030,-97960,-26521,-61011,83390,-38677,12333,75987,46091,83794,19355,-71037,-6242,-28801,324,1202,-90885,-2989,-95597,-34333,35528,5680,89093,-90606,50360,-29393,-27012,53313,65213,99818,-82405,-41661,-3333,-51952,72135,-1523,26377,74685,96992,92263,15929,5467,-99555,-43348,-41689,-60383,-3990,32165,65265,-72973,-58372,12741,-48568,-46596,72419,-1859,34153,62937,81310,-61823,-96770,-54944,8845,-91184,24208,-29078,31495,65258,14198,85395,70506,-40908,56740,-12228,-40072,32429,93001,68445,-73927,25731,-91859,-24150,10093,-60271,-81683,-18126,51055,48189,-6468,25057,81194,-58628,74042,66158,-14452,-49851,-43667,11092,39189,-17025,-79173,13606,83172,92647,-59741,19343,-26644,-57607,82908,-20655,1637,80060,98994,39331,-31274,-61523,91225,-72953,13211,-75116,-98421,-41571,-69074,99587,39345,42151,-2460,98236,15690,-52507,-95803,-48935,-46492,-45606,-79254,-99851,52533,73486,39948,-7240,71815,-585,-96252,90990,-93815,93340,-71848,58733,-14859,-83082,-75794,-82082,-24871,-15206,91207,-56469,-93618,67131,-8682,75719,87429,-98757,-7535,-24890,-94160,85003,33928,75538,97456,-66424,-60074,-8527,-28697,-22308,2246,-70134,-82319,-10184,87081,-34949,-28645,-47352,-83966,-60418,-15293,-53067,-25921,55172,75064,95859,48049,34311,-86931,-38586,33686,-36714,96922,76713,-22165,-80585,-34503,-44516,39217,-28457,47227,-94036,43457,24626,-87359,26898,-70819,30528,-32397,-69486,84912,-1187,-98986,-32958,4280,-79129,-65604,9344,58964,50584,71128,-55480,24986,15086,-62360,-42977,-49482,-77256,-36895,-74818,20,3063,-49426,28152,-97329,6086,86035,-88743,35241,44249,19927,-10660,89404,24179,-26621,-6511,57745,-28750,96340,-97160,-97822,-49979,52307,79462,94273,-24808,77104,9255,-83057,77655,21361,55956,-9096,48599,-40490,-55107,2689,29608,20497,66834,-34678,23553,-81400,-66630,-96321,-34499,-12957,-20564,25610,-4322,-58462,20801,53700,71527,24669,-54534,57879,-3221,33636,3900,97832,-27688,-98715,5992,24520,-55401,-57613,-69926,57377,-77610,20123,52174,860,60429,-91994,-62403,-6218,-90610,-37263,-15052,62069,-96465,44254,89892,-3406,19121,-41842,-87783,-64125,-56120,73904,-22797,-58118,-4866,5356,75318,46119,21276,-19246,-9241,-97425,57333,-15802,93149,25689,-5532,95716,39209,-87672,-29470,-16324,-15331,27632,-39454,56530,-16000,29853,46475,78242,-46602,83192,-73440,-15816,50964,-36601,89758,38375,-40007,-36675,-94030,67576,46811,-64919,45595,76530,40398,35845,41791,67697,-30439,-82944,63115,33447,-36046,-50122,-34789,43003,-78947,-38763,-89210,32756,-20389,-31358,-90526,-81607,88741,86643,98422,47389,-75189,13091,95993,-15501,94260,-25584,-1483,-67261,-70753,25160,89614,-90620,-48542,83889,-12388,-9642,-37043,-67663,28794,-8801,13621,12241,55379,84290,21692,-95906,-85617,-17341,-63767,80183,-4942,-51478,30997,-13658,8838,17452,-82869,-39897,68449,31964,98158,-49489,62283,-62209,-92792,-59342,55146,-38533,20496,62667,62593,36095,-12470,5453,-50451,74716,-17902,3302,-16760,-71642,-34819,96459,-72860,21638,47342,-69897,-40180,44466,76496,84659,13848,-91600,-90887,-63742,-2156,-84981,-99280,94326,-33854,92029,-50811,98711,-36459,-75555,79110,-88164,-97397,-84217,97457,64387,30513,-53190,-83215,252,2344,-27177,-92945,-89010,82662,-11670,86069,53417,42702,97082,3695,-14530,-46334,17910,77999,28009,-12374,15498,-46941,97088,-35030,95040,92095,-59469,-24761,46491,67357,-66658,37446,-65130,-50416,99197,30925,27308,54122,-44719,12582,-99525,-38446,-69050,-22352,94757,-56062,33684,-40199,-46399,96842,-50881,-22380,-65021,40582,53623,-76034,77018,-97074,-84838,-22953,-74205,79715,-33920,-35794,-91369,73421,-82492,63680,-14915,-33295,37145,76852,-69442,60125,-74166,74308,-1900,-30195,-16267,-60781,-27760,5852,38917,25742,-3765,49097,-63541,98612,-92865,-30248,9612,-8798,53262,95781,-42278,-36529,7252,-27394,-5021,59178,80934,-48480,-75131,-54439,-19145,-48140,98457,-6601,-51616,-89730,78028,32083,-48904,16822,-81153,-8832,48720,-80728,-45133,-86647,-4259,-40453,2590,28613,50523,-4105,-27790,-74579,-17223,63721,33489,-47921,97628,-97691,-14782,-65644,18008,-93651,-71266,80990,-76732,-47104,35368,28632,59818,-86269,-89753,34557,-92230,-5933,-3487,-73557,-13174,-43981,-43630,-55171,30254,-83710,-99583,-13500,71787,5017,-25117,-78586,86941,-3251,-23867,-36315,75973,86272,-45575,77462,-98836,-10859,70168,-32971,-38739,-12761,93410,14014,-30706,-77356,-85965,-62316,63918,-59914,-64088,1591,-10957,38004,15129,-83602,-51791,34381,-89382,-26056,8942,5465,71458,-73805,-87445,-19921,-80784,69150,-34168,28301,-68955,18041,6059,82342,9947,39795,44047,-57313,48569,81936,-2863,-80932,32976,-86454,-84207,33033,32867,9104,-16580,-25727,80157,-70169,53741,86522,84651,68480,84018,61932,7332,-61322,-69663,76370,41206,12326,-34689,17016,82975,-23386,39417,72793,44774,-96259,3213,79952,29265,-61492,-49337,14162,65886,3342,-41622,-62659,-90402,-24751,88511,54739,-21383,-40161,-96610,-24944,-602,-76842,-21856,69964,43994,-15121,-85530,12718,13170,-13547,69222,62417,-75305,-81446,-38786,-52075,-23110,97681,-82800,-53178,11474,35857,94197,-58148,-23689,32506,92154,-64536,-73930,-77138,97446,-83459,70963,22452,68472,-3728,-25059,-49405,95129,-6167,12808,99918,30113,-12641,-26665,86362,-33505,50661,26714,33701,89012,-91540,40517,-12716,-57185,-87230,29914,-59560,13200,-72723,58272,23913,-45586,-96593,-26265,-2141,31087,81399,92511,-34049,20577,2803,26003,8940,42117,40887,-82715,38269,40969,-50022,72088,21291,-67280,-16523,90535,18669,94342,-39568,-88080,-99486,-20716,23108,-28037,63342,36863,-29420,-44016,75135,73415,16059,-4899,86893,43136,-7041,33483,-67612,25327,40830,6184,61805,4247,81119,-22854,-26104,-63466,63093,-63685,60369,51023,51644,-16350,74438,-83514,99083,10079,-58451,-79621,48471,67131,-86940,99093,11855,-22272,-67683,-44371,9541,18123,37766,-70922,80385,-57513,-76021,-47890,36154,72935,84387,-92681,-88303,-7810,59902,-90,-64704,-28396,-66403,8860,13343,33882,85680,7228,28160,-14003,54369,-58893,92606,-63492,-10101,64714,58486,29948,-44679,-22763,10151,-56695,4031,-18242,-36232,86168,-14263,9883,47124,47271,92761,-24958,-73263,-79661,-69147,-18874,29546,-92588,-85771,26451,-86650,-43306,-59094,-47492,-34821,-91763,-47670,33537,22843,67417,-759,92159,63075,94065,-26988,55276,65903,30414,-67129,-99508,-83092,-91493,-50426,14349,-83216,-76090,32742,-5306,-93310,-60750,-60620,-45484,-21108,-58341,-28048,-52803,69735,78906,81649,32565,-86804,-83202,-65688,-1760,89707,93322,-72750,84134,71900,-37720,19450,-78018,22001,-23604,26276,-21498,65892,-72117,-89834,-23867,55817,-77963,42518,93123,-83916,63260,-2243,-97108,85442,-36775,17984,-58810,99664,-19082,93075,-69329,87061,79713,16296,70996,13483,-74582,49900,-27669,-40562,1209,-20572,34660,83193,75579,7344,64925,88361,60969,3114,44611,-27445,53049,-16085,-92851,-53306,13859,-33532,86622,-75666,-18159,-98256,51875,-42251,-27977,-18080,23772,38160,41779,9147,94175,99905,-85755,62535,-88412,-52038,-68171,93255,-44684,-11242,-104,31796,62346,-54931,-55790,-70032,46221,56541,-91947,90592,93503,4071,20646,4856,-63598,15396,-50708,32138,-85164,38528,-89959,53852,57915,-42421,-88916,-75072,67030,-29066,49542,-71591,61708,-53985,-43051,28483,46991,-83216,80991,-46254,-48716,39356,-8270,-47763,-34410,874,-1186,-7049,28846,11276,21960,-13304,-11433,-4913,55754,79616,70423,-27523,64803,49277,14906,-97401,-92390,91075,70736,21971,-3303,55333,-93996,76538,54603,-75899,98801,46887,35041,48302,-52318,55439,24574,14079,-24889,83440,14961,34312,-89260,-22293,-81271,-2586,-71059,-10640,-93095,-5453,-70041,66543,74012,-11662,-52477,-37597,-70919,92971,-17452,-67306,-80418,7225,-89296,24296,86547,37154,-10696,74436,-63959,58860,33590,-88925,-97814,-83664,85484,-8385,-50879,57729,-74728,-87852,-15524,-91120,22062,28134,80917,32026,49707,-54252,-44319,-35139,13777,44660,85274,25043,58781,-89035,-76274,6364,-63625,72855,43242,-35033,12820,-27460,77372,-47578,-61162,-70758,-1343,-4159,64935,56024,-2151,43770,19758,-30186,-86040,24666,-62332,-67542,73180,-25821,-27826,-45504,-36858,-12041,20017,-24066,-56625,-52097,-47239,-90694,8959,7712,-14258,-5860,55349,61808,-4423,-93703,64681,-98641,-25222,46999,-83831,-54714,19997,-68477,66073,51801,-66491,52061,-52866,79907,-39736,-68331,68937,91464,98892,910,93501,31295,-85873,27036,-57340,50412,21,-2445,29471,71317,82093,-94823,-54458,-97410,39560,-7628,66452,39701,54029,37906,46773,58296,60370,-61090,85501,-86874,71443,-72702,-72047,14848,34102,77975,-66294,-36576,31349,52493,-70833,-80287,94435,39745,-98291,84524,-18942,10236,93448,50846,94023,-6939,47999,14740,30165,81048,84935,-19177,-13594,32289,62628,-90612,-542,-66627,64255,71199,-83841,-82943,-73885,8623,-67214,-9474,-35249,62254,-14087,-90969,21515,-83303,94377,-91619,19956,-98810,96727,-91939,29119,-85473,-82153,-69008,44850,74299,-76459,-86464,8315,-49912,-28665,59052,-69708,76024,-92738,50098,18683,-91438,18096,-19335,35659,91826,15779,-73070,67873,-12458,-71440,-46721,54856,97212,-81875,35805,36952,68498,81627,-34231,81712,27100,-9741,-82612,18766,-36392,2759,41728,69743,26825,48355,-17790,17165,56558,3295,-24375,55669,-16109,24079,73414,48990,-11931,-78214,90745,19878,35673,-15317,-89086,94675,-92513,88410,-93248,-19475,-74041,-19165,32329,-26266,-46828,-18747,45328,8990,-78219,-25874,-74801,-44956,-54577,-29756,-99822,-35731,-18348,-68915,-83518,-53451,95471,-2954,-13706,-8763,-21642,-37210,16814,-60070,-42743,27697,-36333,-42362,11576,85742,-82536,68767,-56103,-63012,71396,-78464,-68101,-15917,-11113,-3596,77626,-60191,-30585,-73584,6214,-84303,18403,23618,-15619,-89755,-59515,-59103,-74308,-63725,-29364,-52376,-96130,70894,-12609,50845,-2314,42264,-70825,64481,55752,4460,-68603,-88701,4713,-50441,-51333,-77907,97412,-66616,-49430,60489,-85262,-97621,-18980,44727,-69321,-57730,66287,-92566,-64427,-14270,11515,-92612,-87645,61557,24197,-81923,-39831,-10301,-23640,-76219,-68025,92761,-76493,68554,-77734,-95620,-11753,-51700,98234,-68544,-61838,29467,46603,-18221,-35441,74537,40327,-58293,75755,-57301,-7532,-94163,18179,-14388,-22258,-46417,-48285,18242,-77551,82620,250,-20060,-79568,-77259,82052,-98897,-75464,48773,-79040,-11293,45941,-67876,-69204,-46477,-46107,792,60546,-34573,-12879,-94562,20356,-48004,-62429,96242,40594,2099,99494,25724,-39394,-2388,-18563,-56510,-83570,-29214,3015,74454,74197,76678,-46597,60630,-76093,37578,-82045,-24077,62082,-87787,-74936,58687,12200,-98952,70155,-77370,21710,-84625,-60556,-84128,925,65474,-15741,-94619,88377,89334,44749,22002,-45750,-93081,-14600,-83447,46691,85040,-66447,-80085,56308,44310,24979,-29694,57991,4675,-71273,-44508,13615,-54710,23552,-78253,-34637,50497,68706,81543,-88408,-21405,6001,-33834,-21570,-46692,-25344,20310,71258,-97680,11721,59977,59247,-48949,98955,-50276,-80844,-27935,-76102,55858,-33492,40680,66691,-33188,8284,64893,-7528,6019,-85523,8434,-64366,-56663,26862,30008,-7611,-12179,-70076,21426,-11261,-36864,-61937,-59677,929,-21052,3848,-20888,-16065,98995,-32293,-86121,-54564,77831,68602,74977,31658,40699,29755,98424,80358,-69337,26339,13213,-46016,-18331,64713,-46883,-58451,-70024,-92393,-4088,70628,-51185,71164,-75791,-1636,-29102,-16929,-87650,-84589,-24229,-42137,-15653,94825,13042,88499,-47100,-90358,-7180,29754,-65727,-42659,-85560,-9037,-52459,20997,-47425,17318,21122,20472,-23037,65216,-63625,-7877,-91907,24100,-72516,22903,-85247,-8938,73878,54953,87480,-31466,-99524,35369,-78376,89984,-15982,94045,-7269,23319,-80456,-37653,-76756,2909,81936,54958,-12393,60560,-84664,-82413,66941,-26573,-97532,64460,18593,-85789,-38820,-92575,-43663,-89435,83272,-50585,13616,-71541,-53156,727,-27644,16538,34049,57745,34348,35009,16634,-18791,23271,-63844,95817,21781,16590,59669,15966,-6864,48050,-36143,97427,-59390,96931,78939,-1958,50777,43338,-51149,39235,-27054,-43492,67457,-83616,37179,10390,85818,2391,73635,87579,-49127,-81264,-79023,-81590,53554,-74972,-83940,-13726,-39095,29174,78072,76104,47778,25797,-29515,-6493,-92793,22481,-36197,-65560,42342,15750,97556,99634,-56048,-35688,13501,63969,-74291,50911,39225,93702,-3490,-59461,-30105,-46761,-80113,92906,-68487,50742,36152,-90240,-83631,24597,-50566,-15477,18470,77038,40223,-80364,-98676,70957,-63647,99537,13041,31679,86631,37633,-16866,13686,-71565,21652,-46053,-80578,-61382,68487,-6417,4656,20811,67013,-30868,-11219,46,74944,14627,56965,42275,-52480,52162,-84883,-52579,-90331,92792,42184,-73422,-58440,65308,-25069,5475,-57996,59557,-17561,2826,-56939,14996,-94855,-53707,99159,43645,-67719,-1331,21412,41704,31612,32622,1919,-69333,-69828,22422,-78842,57896,-17363,27979,-76897,35008,46482,-75289,65799,20057,7170,41326,-76069,90840,-81253,-50749,3649,-42315,45238,-33924,62101,96906,58884,-7617,-28689,-66578,62458,50876,-57553,6739,41014,-64040,-34916,37940,13048,-97478,-11318,-89440,-31933,-40357,-59737,-76718,-14104,-31774,28001,4103,41702,-25120,-31654,63085,-3642,84870,-83896,-76422,-61520,12900,88678,85547,33132,-88627,52820,63915,-27472,78867,-51439,33005,-23447,-3271,-39308,39726,-74260,-31874,-36893,93656,910,-98362,60450,-88048,99308,13947,83996,-90415,-35117,70858,-55332,-31721,97528,82982,-86218,6822,25227,36946,97077,-4257,-41526,56795,89870,75860,-70802,21779,14184,-16511,-89156,-31422,71470,69600,-78498,74079,-19410,40311,28501,26397,-67574,-32518,68510,38615,19355,-6088,-97159,-29255,-92523,3023,-42536,-88681,64255,41206,44119,52208,39522,-52108,91276,-70514,83436,63289,-79741,9623,99559,12642,85950,83735,-21156,-67208,98088,-7341,-27763,-30048,-44099,-14866,-45504,-91704,19369,13700,10481,-49344,-85686,33994,19672,36028,60842,66564,-24919,33950,-93616,-47430,-35391,-28279,56806,74690,39284,-96683,-7642,-75232,37657,-14531,-86870,-9274,-26173,98640,88652,64257,46457,37814,-19370,9337,-22556,-41525,39105,-28719,51611,-93252,98044,-90996,21710,-47605,-64259,-32727,53611,-31918,-3555,33316,-66472,21274,-37731,-2919,15016,48779,-88868,1897,41728,46344,-89667,37848,68092,-44011,85354,-43776,38739,-31423,-66330,65167,-22016,59405,34328,-60042,87660,-67698,-59174,-1408,-46809,-43485,-88807,-60489,13974,22319,55836,-62995,-37375,-4185,32687,-36551,-75237,58280,26942,-73756,71756,78775,-40573,14367,-71622,-77338,24112,23414,-7679,-51721,87492,85066,-21612,57045,10673,-96836,52461,-62218,-9310,65862,-22748,89906,-96987,-98698,26956,-43428,46141,47456,28095,55952,67323,-36455,-60202,-43302,-82932,42020,77036,10142,60406,70331,63836,58850,-66752,52109,21395,-10238,-98647,-41962,27778,69060,98535,-28680,-52263,-56679,66103,-42426,27203,80021,10153,58678,36398,63112,34911,20515,62082,-15659,-40785,27054,43767,-20289,65838,-6954,-60228,-72226,52236,-35464,25209,-15462,-79617,-41668,-84083,62404,-69062,18913,46545,20757,13805,24717,-18461,-47009,-25779,68834,64824,34473,39576,31570,14861,-15114,-41233,95509,68232,67846,84902,-83060,17642,-18422,73688,77671,-26930,64484,-99637,73875,6428,21034,-73471,19664,-68031,15922,-27028,48137,54955,-82793,-41144,-10218,-24921,-28299,-2288,68518,-54452,15686,-41814,66165,-72207,-61986,80020,50544,-99500,16244,78998,40989,14525,-56061,-24692,-94790,21111,37296,-90794,72100,70550,-31757,17708,-74290,61910,78039,-78629,-25033,73172,-91953,10052,64502,99585,-1741,90324,-73723,68942,28149,30218,24422,16659,10710,-62594,94249,96588,46192,34251,73500,-65995,-81168,41412,-98724,-63710,-54696,-52407,19746,45869,27821,-94866,-76705,-13417,-61995,-71560,43450,67384,-8838,-80293,-28937,23330,-89694,-40586,46918,80429,-5475,78013,25309,-34162,37236,-77577,86744,26281,-29033,-91813,35347,13033,-13631,-24459,3325,-71078,-75359,81311,19700,47678,-74680,-84113,45192,35502,37675,19553,76522,-51098,-18211,89717,4508,-82946,27749,85995,89912,-53678,-64727,-14778,32075,-63412,-40524,86440,-2707,-36821,63850,-30883,67294,-99468,-23708,34932,34386,98899,29239,-23385,5897,54882,98660,49098,70275,17718,88533,52161,63340,50061,-89457,19491,-99156,24873,-17008,64610,-55543,50495,17056,-10400,-56678,-29073,-42960,-76418,98562,-88104,-96255,10159,-90724,54011,12052,45871,-90933,-69420,67039,37202,78051,-52197,-40278,-58425,65414,-23394,-1415,6912,-53447,7352,17307,-78147,63727,98905,55412,-57658,-32884,-44878,22755,39730,3638,35111,39777,74193,38736,-11829,-61188,-92757,55946,-71232,-63032,-83947,39147,-96684,-99233,25131,-32197,24406,-55428,-61941,25874,-69453,64483,-19644,-68441,12783,87338,-48676,66451,-447,-61590,50932,-11270,29035,65698,-63544,10029,80499,-9461,86368,91365,-81810,-71914,-52056,-13782,44240,-30093,-2437,24007,67581,-17365,-69164,-8420,-69289,-29370,48010,90439,13141,69243,50668,39328,61731,78266,-81313,17921,-38196,55261,9948,-24970,75712,-72106,28696,7461,31621,61047,51476,56512,11839,-96916,-82739,28924,-99927,58449,37280,69357,11219,-32119,-62050,-48745,-83486,-52376,42668,82659,68882,38773,46269,-96005,97630,25009,-2951,-67811,99801,81587,-79793,-18547,-83086,69512,33127,-92145,-88497,47703,59527,1909,88785,-88882,69188,-46131,-5589,-15086,36255,-53238,-33009,82664,53901,35939,-42946,-25571,33298,69291,53199,74746,-40127,-39050,91033,51717,-98048,87240,36172,65453,-94425,-63694,-30027,59004,88660,3649,-20267,-52565,-67321,34037,4320,91515,-56753,60115,27134,68617,-61395,-26503,-98929,-8849,-63318,10709,-16151,61905,-95785,5262,23670,-25277,90206,-19391,45735,37208,-31992,-92450,18516,-90452,-58870,-58602,93383,14333,17994,82411,-54126,-32576,35440,-60526,-78764,-25069,-9022,-394,92186,-38057,55328,-61569,67780,77169,19546,-92664,-94948,44484,-13439,83529,27518,-48333,72998,38342,-90553,-98578,-76906,81515,-16464,78439,92529,35225,-39968,-10130,-7845,-32245,-74955,-74996,67731,-13897,-82493,33407,93619,59560,-24404,-57553,19486,-45341,34098,-24978,-33612,79058,71847,76713,-95422,6421,-96075,-59130,-28976,-16922,-62203,69970,68331,21874,40551,89650,51908,58181,66480,-68177,34323,-3046,-49656,-59758,43564,-10960,-30796,15473,-20216,46085,-85355,41515,-30669,-87498,57711,56067,63199,-83805,62042,91213,-14606,4394,-562,74913,10406,96810,-61595,32564,31640,-9732,42058,98052,-7908,-72330,1558,-80301,34878,32900,3939,-8824,88316,20937,21566,-3218,-66080,-31620,86859,54289,90476,-42889,-15016,-18838,75456,30159,-67101,42328,-92703,85850,-5475,23470,-80806,68206,17764,88235,46421,-41578,74005,-81142,80545,20868,-1560,64017,83784,68863,-97516,-13016,-72223,79630,-55692,82255,88467,28007,-34686,-69049,-41677,88535,-8217,68060,-51280,28971,49088,49235,26905,-81117,-44888,40623,74337,-24662,97476,79542,-72082,-35093,98175,-61761,-68169,59697,-62542,-72965,59883,-64026,-37656,-92392,-12113,-73495,98258,68379,-21545,64607,-70957,-92254,-97460,-63436,-8853,-19357,-51965,-76582,12687,-49712,45413,-60043,33496,31539,-57347,41837,67280,-68813,52088,-13155,-86430,-15239,-45030,96041,18749,-23992,46048,35243,-79450,85425,-58524,88781,-39454,53073,-48864,-82289,39086,82540,-11555,25014,-5431,-39585,-89526,2705,31953,-81611,36985,-56022,68684,-27101,11422,64655,-26965,-63081,-13840,-91003,-78147,-8966,41488,1988,99021,-61575,-47060,65260,-23844,-21781,-91865,-19607,44808,2890,63692,-88663,-58272,15970,-65195,-45416,-48444,-78226,-65332,-24568,42833,-1806,-71595,80002,-52250,30952,48452,-90106,31015,-22073,62339,63318,78391,28699,77900,-4026,-76870,-45943,33665,9174,-84360,-22684,-16832,-67949,-38077,-38987,-32847,51443,-53580,-13505,9344,-92337,26585,70458,-52764,-67471,-68411,-1119,-2072,-93476,67981,40887,-89304,-12235,41488,1454,5355,-34855,-72080,24514,-58305,3340,34331,8731,77451,-64983,-57876,82874,62481,-32754,-39902,22451,-79095,-23904,78409,-7418,77916
        );

        // println!("result={:?}",Solution::three_sum_3(input));

        input = vec![-4,-2,1,-5,-4,-4,4,-2,0,4,0,-2,3,1,-5,0];
        println!("result={:?}",Solution::three_sum_3(input));
        //expected = [[-5,1,4],[-4,0,4],[-4,1,3],[-2,-2,4],[-2,1,1],[0,0,0]]

    }

    pub fn three_sum_closest(nums:Vec<i32>,target:i32) -> i32 {

        let MAX:i32 = 1000000;
        let mut result:i32 = MAX;
        let mut input = nums.clone();

        //order asc
        let mut i = 0;
        for i in 0..input.len() {
            for j in i..input.len() {
                if input[i] > input[j] {
                    let old_i = input[i];
                    input[i] = input[j];
                    input[j] = old_i;
                }
            }
        }
        println!("input = {:?}",input);

        fn next_low(nums0:&[i32],old:usize) -> usize {

            if old == 0 {
                return nums0.len();
            }

            let mut new_low = old -1 ;

            loop {

                if nums0[old] != nums0[new_low] {
                    return new_low;
                }

                if new_low == 0 {
                    return nums0.len();
                }

                new_low = new_low - 1;

            }

        }

        fn next_high(nums0:&[i32],old:usize) -> usize {

            let mut new_high = old;

            loop {

                if new_high >= nums0.len() {
                    return nums0.len();
                }

                if nums0[new_high] != nums0[old] {
                    return new_high;
                }

                new_high = new_high + 1;

            }

        }

        //return the lowest diff
        fn my_two_sum_closest(nums0:&[i32],target0:i32) -> i32 {

            // println!("nums0={:?},target0={}",nums0,target0);
            let MAX:i32 = 1000000;

            let mut result0:i32 = MAX;
            if nums0.len() < 2 {
                return result0;
            }
            
            let mut medium = target0/2;
            let mut low = 0;
            let mut high = nums0.len() -1;

            loop {

                let m = (low+high) /2;
                if nums0[m] == medium || m == low {

                    let new_low = next_low(nums0, m);
                    if new_low == nums0.len() {
                        low = 0;
                    } else {
                        low = new_low + 1;
                    }

                    break;
                }

                if nums0[m] < medium {
                    low = m;
                } else {
                    high =m;
                }

            }

            high = low + 1;
            // println!("low={},high={}",low,high);

            loop {

                let diff = nums0[low]+nums0[high]-target0;

                if diff.abs() < result0.abs() {
                    result0 = diff;
                }

                if result0 == 0 {
                    return result0;
                }

                if diff > 0 {
                    low = next_low(nums0,low);
                    if low == nums0.len() {
                        break;
                    }
                } else {
                    high = next_high(nums0, high);
                    if high == nums0.len() {
                        break;
                    }
                }

            }

            result0
        }

        let mut minimum_diff= i32::MAX;

            for i in 0..input.len() -1 {
                let result_two_sum_closest = my_two_sum_closest(&input[i+1..], target - input[i]);
                // println!("two sum: {}",result_two_sum_closest);

                if result_two_sum_closest.abs() < minimum_diff.abs() {
                    result = result_two_sum_closest+target;
                    minimum_diff = result_two_sum_closest;
                }

                if minimum_diff == 0 {
                    return result;
                }

            }

        result
    }

    pub fn solve_three_sum_closest() {

        let mut input = vec!(-1,0,1,2,-1,-4);
        input = vec![-4,-2,1,-5,-4,-4,4,-2,0,4,0,-2,3,1,-5,0];
        println!("result={:?}",Solution::three_sum_closest(input,1));
        input = vec![-1,2,1,-4];
        println!("result={:?}",Solution::three_sum_closest(input,1));
        input = vec![0,0,0];
        println!("result={:?}",Solution::three_sum_closest(input,1));

        //0 expected
        input = vec![0,2,1,-3];
        println!("result={:?}",Solution::three_sum_closest(input,1));
 
    }

    pub fn four_sum(nums:Vec<i32>,target:i32) -> Vec<Vec<i32>> {

        // let mut result:Vec<Vec<i32>> = Vec::new();
        let mut input=nums.clone();

        if nums.len() < 4 {
            return Vec::new();
        }

        for i in 0..input.len() {
            for j in i+1 .. input.len() {
                if input[i] > input[j] {
                    let old_i = input[i];
                    input[i] = input[j];
                    input[j] = old_i;
                }
            }
        }

        fn next_high(nums1:&[i32],old:usize) -> usize {

            let mut new_high = old+1;

            loop {

                if new_high >= nums1.len() {
                    return nums1.len();
                }

                if nums1[new_high] != nums1[old] {
                    return new_high;
                }

                new_high = new_high + 1;
            }

        }

        fn calc_sum_n(nums0:&[i32],target:i32,sum_n_:usize) -> Vec<Vec<i32>> {

            let mut result0: Vec<Vec<i32>> = Vec::new();
            if nums0.len() < sum_n_  {
                return result0;
            }

            match sum_n_ {

                0 => {
                    result0
                },

                1 => {

                    for i in 0..nums0.len() {
                        if nums0[i] == target {
                            result0.push(vec![nums0[i]]);
                            break;
                        }
                    }

                    result0
                },

                _ => {

                    let mut i:usize = 0;

                    loop {

                        if i >= nums0.len() - sum_n_ +1 {
                            break;
                        }

                        let sub_results = calc_sum_n(&nums0[i+1..], target-nums0[i], sum_n_ - 1);
                        for mut one_result in sub_results {
                            one_result.insert(0,nums0[i]);
                            result0.push(one_result);
                        }

                        i = next_high(nums0,i);
 
                    }

                    result0
                },

            }

        }
        
        calc_sum_n(input.as_slice(),target,4)
        
    }

    pub fn solve_four_sum() {

        let mut input = Vec::new();
        input = vec![-4,-2,1,-5,-4,-4,4,-2,0,4,0,-2,3,1,-5,0];
        println!("result={:?}",Solution::four_sum(input, 0) );
        input = vec![1,0,-1,0,-2,2];
        println!("result={:?}",Solution::four_sum(input, 0) );
        
        input = vec![2,2,2,2,2];
        println!("result={:?}",Solution::four_sum(input, 8) );

    }

    pub fn letter_combinations(digits: String) -> Vec<String> {

        fn get_digit_table(digit: char) -> Vec<char> {

            match digit {
                '2' => {
                    vec!['a','b','c']
                },
                '3' => {
                    vec!['d','e','f'] //3
                },
                '4' => {
                    vec!['g','h','i'] //4
                },
                '5' => {
                    vec!['j','k','l'] //5
                },
                '6' => {
                    vec!['m','n','o'] //6
                },
                '7' => {
                    vec!['p','q','r','s'] //7
                },
                '8' => {
                    vec!['t','u','v'] //8
                },
                '9' => {
                    vec!['w','x','y','z'] //9
                },
                _ => {
                    Vec::new()
                }
            }

        }

        fn calc_letter_combinations(digits:&str) -> Vec<String> {

            let mut result:Vec<String> = Vec::new();

            match digits.len() {
                0 => {
                    result
                } ,

                _ => {

                    let result1 = calc_letter_combinations(&digits[1..]);
                    let ref_result1 = result1.as_slice();

                    for ch in get_digit_table(digits.chars().nth(0).unwrap()) {

                        if ref_result1.len() ==0 {
                            let mut new_result = String::new();
                            new_result.push(ch);
                            result.push(new_result);
                        } else {
                            for one_result in ref_result1 {
                                let mut new_result = one_result.clone();
                                new_result.insert(0,ch);
                                result.push(new_result);
                            }
                        }

                    }

                    result
                }
            }

        }

        return calc_letter_combinations(digits.as_str());

    }

    pub fn solve_letter_combinations() {

        println!("result = {:?}",Solution::letter_combinations(String::from("23")));

    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>,n:i32 ) -> Option<Box<ListNode>> {


        fn push_front(head:&mut Option<Box<ListNode>>,mut node:ListNode) -> Option<Box<ListNode>> {

            match head {
                None => {
                    *head = Some(Box::new(ListNode::new(node.val)));
                },
                Some(node_ref) => {
                    *head = Some(node_ref.clone())
                }
            }

            head.clone()

        }

        fn copy_except_nth (node:Option<Box<ListNode>>,n:i32,result:&mut Option<Box<ListNode>>) -> i32 {

            let mut no:i32 = 0;

            if node == None {
                return no;
            } else {
                no = copy_except_nth(node.as_deref().unwrap().next.clone(), n, result);
            }

            no = no +1;

            if no != n {
                let mut copy_node :Option<Box<ListNode>> = Some(Box::new(ListNode::new(node.as_deref().unwrap().val)));
                copy_node.as_deref_mut().unwrap().next = match result {
                    None => {
                        None
                    },
                    Some(node_ptr) => {
                        Some(node_ptr.clone())
                    }
                };
                *result = copy_node;
            }

            no
        }

        let mut new_head : Option<Box<ListNode>> = None;
        copy_except_nth(head, n, &mut new_head);
        new_head

    }

    pub fn solve_remove_nth_from_end() {

        type ListNodePtr = Option<Box<ListNode>>;

        let mut head: Option<Box<ListNode>> = None;

        head = Some(Box::new(ListNode::new(1)));
        head.as_deref_mut().unwrap().insert(2);
        head.as_deref_mut().unwrap().insert(3);
        head.as_deref_mut().unwrap().insert(4);
        head.as_deref().unwrap().print();
        println!("");

        let mut result : ListNodePtr =None;

        result = Solution::remove_nth_from_end(head.clone(), 0);
        result.as_deref().unwrap().print();
        println!("");

        result = Solution::remove_nth_from_end(head.clone(), 1);
        result.as_deref().unwrap().print();
        println!("");

        result = Solution::remove_nth_from_end(head.clone(), 4);
        result.as_deref().unwrap().print();
        println!("");

        result = Solution::remove_nth_from_end(head.clone(), 5);
        result.as_deref().unwrap().print();
        println!("");

    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {

        let mut copy = lists.clone();
        let mut result : Option<Box<ListNode>> = None;
        let mut current_node = &mut result;

        loop {

            let mut curent_val = i32::MAX;
            let mut index = copy.len();

            for (i,node) in copy.iter().enumerate() {

                if *node == None {
                    continue;
                }

                if node.as_deref().unwrap().val <= curent_val {
                    curent_val = node.as_deref().unwrap().val;
                    index = i;
                }

            }

            if index == copy.len() {
                break;
            }

            copy[index] = copy[index].as_deref_mut().unwrap().next.clone();
            
            *current_node = Some(Box::new(ListNode::new(curent_val)));
            current_node = &mut current_node.as_deref_mut().unwrap().next;

        }

        result

    }

    pub fn solve_merge_k_lists() {

        type ListNodePtr = Option<Box<ListNode>>;

        let mut head: Option<Box<ListNode>> = None;
        head = Some(Box::new(ListNode::new(1)));
        head.as_deref_mut().unwrap().push(3);
        head.as_deref_mut().unwrap().push(5);
        head.as_deref_mut().unwrap().push(7);
        head.as_deref().unwrap().print();
        println!("");

        let mut head2: ListNodePtr = Some(Box::new(ListNode::new(2)));
        head2.as_deref_mut().unwrap().push(4);
        head2.as_deref_mut().unwrap().push(6);
        head2.as_deref_mut().unwrap().push(8);
        head2.as_deref().unwrap().print();
        println!("");

        let input = vec![head,head2];
        let result = Solution::merge_k_lists(input);
        result.as_deref().unwrap().print();

    }


    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut result:Option<Box<ListNode>> = None;
        let mut node_ptr = &mut result;
        let mut source_ptr = &head;

        loop {

            if *source_ptr == None {
                break;
            }

            if source_ptr.as_deref().unwrap().next == None {
                *node_ptr = Some(Box::new(ListNode::new(source_ptr.as_deref().unwrap().val)));
                break;
            } else {
                *node_ptr = Some(Box::new(ListNode::new(source_ptr.as_deref().unwrap().next.as_deref().unwrap().val)));
                node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
                *node_ptr = Some(Box::new(ListNode::new(source_ptr.as_deref().unwrap().val)));
                node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
                source_ptr = & source_ptr.as_deref().unwrap().next.as_deref().unwrap().next;
            }

        }

        result

    }

    pub fn solve_swap_pairs() {

        let mut head: Option<Box<ListNode>> = None;
        head = Some(Box::new(ListNode::new(1)));
        head.as_deref_mut().unwrap().push(3);
        head.as_deref_mut().unwrap().push(5);
        head.as_deref_mut().unwrap().push(7);
        head.as_deref().unwrap().print();
        println!("");


        /*
        let mut copy  = head.clone();
        copy.as_deref_mut().unwrap().next.as_deref_mut().unwrap().val = 2;
        head.as_deref().unwrap().print();
        println!("");
        copy.as_deref().unwrap().print();
        println!("");
        */

        let mut result = Solution::swap_pairs(head.clone());
        result.as_deref().unwrap().print();
        println!("");

        head.as_deref_mut().unwrap().push(9);
        result = Solution::swap_pairs(head.clone());
        result.as_deref().unwrap().print();
        println!("");

        head.as_deref_mut().unwrap().push(11);
        result = Solution::swap_pairs(head.clone());
        result.as_deref().unwrap().print();


        /*
        struct Node {
        };

        impl Clone for Node {
            fn clone(&self) -> Self {

            }

        };

        let mut node_ptr  = Box::new(Node{});
        let node_ptr2 = node_ptr.clone();
        */

    }

    fn reverse_k_group(head:Option<Box<ListNode>>, k:i32) -> Option<Box<ListNode>> {


        fn push_back(head :&mut Option<Box<ListNode>>,val:i32) {

            let mut node_ptr = head;

            loop {

                if *node_ptr == None {
                    *node_ptr = Some(Box::new(ListNode::new(val)));
                    break;
                }

                node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
            }

        }

        fn join_list(mut first:Option<Box<ListNode>>,second:Option<Box<ListNode>>) -> Option<Box<ListNode>>{
            let mut node_ptr = &mut first;
            loop {
                if *node_ptr == None {
                    break;
                } else {
                    node_ptr = &mut node_ptr.as_deref_mut().unwrap().next;
                }
            }
            *node_ptr = second;
            first
        }

        struct ReverseResult {
            reverse:bool,
            group: Option<Box<ListNode>>,
            result:Option<Box<ListNode>>,
        };

        fn reverse_k(source:& Option<Box<ListNode>>,k:i32,node_no:i32) -> ReverseResult {

            let k_no = node_no%k;

            if *source == None {
                return ReverseResult{reverse: false,group:None,result: None};
            }

            let next_turn = &source.as_deref().unwrap().next;
            let mut reverse_result = reverse_k(next_turn,k,node_no +1);

            if k_no == 0 {
                reverse_result.result = join_list(reverse_result.group, reverse_result.result);
                reverse_result.group = None;
                reverse_result.reverse = true;
            } 

            if reverse_result.reverse {
                push_back(&mut reverse_result.group,source.as_deref().unwrap().val);
            } else {
                let mut new_node = Some(Box::new(ListNode::new(source.as_deref().unwrap().val)));
                new_node.as_deref_mut().unwrap().next = reverse_result.group;
                reverse_result.group = new_node;
            }

            reverse_result

        }

        let source = &head;
        let mut reverse_result = reverse_k(source,k, 1);
        reverse_result.result = join_list(reverse_result.group, reverse_result.result);
        reverse_result.result

    }

    pub fn solve_reverse_k_group() {

        let mut head = ListNode::build_from(vec![1,2,3,4,5,6,7,8,9,10]);
        head.as_deref().unwrap().print();
        // head.as_deref_mut().unwrap().next = None;
        // let second = head.as_deref_mut().unwrap().next;

        let reverse_list = Solution::reverse_k_group(head.clone(), 1);
        reverse_list.as_deref().unwrap().print();

        let reverse_list = Solution::reverse_k_group(head.clone(), 2);
        reverse_list.as_deref().unwrap().print();

        let reverse_list = Solution::reverse_k_group(head.clone(), 3);
        reverse_list.as_deref().unwrap().print();

        let reverse_list = Solution::reverse_k_group(head.clone(), 4);
        reverse_list.as_deref().unwrap().print();

        let reverse_list = Solution::reverse_k_group(head.clone(), 5);
        reverse_list.as_deref().unwrap().print();

        let reverse_list = Solution::reverse_k_group(head.clone(), 6);
        reverse_list.as_deref().unwrap().print();

        let reverse_list = Solution::reverse_k_group(head.clone(), 7);
        reverse_list.as_deref().unwrap().print();

    }

    pub fn divide(dividend:i32,divisor: i32 ) -> i32 {
        //edge case: -2147483648/-1 = 2147483647

        let mut minus = false;
        let mut result:i64 = 0;

        let new_dividend:i64= if dividend > 0 { 
            dividend as i64
        } else { 
            minus = true;
            -(dividend as i64)
        };

        let new_divisor:i64 = if divisor > 0 {
            divisor as i64
        } else { 
            minus = !minus;
            - (divisor as i64)
        };

        let mut temp_result:i64 = 1;
        let mut temp_product = new_divisor;
        let mut temp_dividend = new_dividend;

        loop {

            if temp_dividend < temp_product {
                break;
            }

            if temp_dividend < temp_product << 1 {

                result += temp_result;
                temp_result = 1;
                temp_dividend -= temp_product;
                temp_product = new_divisor;

            } else {
                temp_product <<=1;
                temp_result<<=1;
            }

        }

        result = if minus { - result } else {result};

        result = if result > i32::MAX as i64{
            i32::MAX as i64
        } else if result < i32::MIN as i64 {
            i32::MIN as i64
        } else {
            result
        };

        result as i32
    }

    pub fn solve_divide() {
        println!("4/2={}",Solution::divide(4,2));
        println!("5/2={}",Solution::divide(5,2));
        println!("-5/2={}",Solution::divide(-5,2));
        println!("0/2={}",Solution::divide(0,2));
        println!("11/3={}",Solution::divide(11,3));
        println!("-2147483648/-1={}",Solution::divide(-2147483648,-1));
        assert_eq!(Solution::divide(-2147483648, -1),2147483647);
    }

    pub fn find_substring(s:String,words:Vec<String>) -> Vec<i32>  {

        //PartialEq enable Option comparison
        #[derive(PartialEq)]
        struct CharNode {
            val: usize,
            child_size: usize,
            val_occur_time:usize,
            next: Vec<Option<Box<CharNode>>>,
        }


        impl CharNode {

            fn new(val:usize) -> CharNode {

                let mut node = CharNode {
                    val:val,
                    child_size:0,
                    val_occur_time:0,
                    next:Vec::new(),
                };

                if node.val > 0 {
                    node.val_occur_time += 1;
                }

                for _ in 0..26 {
                    node.next.push(None);
                };

                node

            }

            fn build_tree( words:&Vec<String>) -> Option<Box<CharNode>> {

                let mut head = Some(Box::new(CharNode::new(0)));

                for (word_index,word) in words.iter().enumerate() {

                    let word_len = word.len();
                    let mut node_ptr = &mut head;

                    for (i,ch) in word.chars().enumerate() {
                        let val = if i+1 >= word_len  {word_index+1} else {0};
                        node_ptr = node_ptr.as_mut().unwrap().new_child(val,ch);
                    }

                }

                head

            }

            fn find(tree:&Option<Box<CharNode>>,s:&str) -> (usize,usize)  {

                let mut result = (0,0);

                let mut node_ptr:&Option<Box<CharNode>> = tree;

                let mut pos :usize = 0;

                loop {

                    if pos >= s.len() {
                        break;
                    }

                    let ch = s.as_bytes()[pos] as char;
                    node_ptr = node_ptr.as_deref().unwrap().get_child(ch);
                    if *node_ptr == None {
                        result = (0,0);
                        break;
                    }

                    if node_ptr.as_deref().unwrap().val > 0 {
                        result = (node_ptr.as_deref().unwrap().val,node_ptr.as_deref().unwrap().val_occur_time);
                        break;
                    } 

                    pos += 1;

                }

                result

            }

            fn new_child(&mut self,val:usize,ch:char) -> &mut Option<Box<CharNode>> {

                let index:usize = ch as usize -'a' as usize;
                let child = &mut self.next[index];

                if *child == None {
                    *child = Some(Box::new(CharNode::new(val)));
                    self.child_size += 1;
                } else if val > 0 {
                    child.as_deref_mut().unwrap().val_occur_time += 1;
                }

                child

            }

            fn get_child(& self,ch:char) -> &Option<Box<CharNode>> {
                let index:usize = ch as usize -'a' as usize;
                &self.next[index]
            }

        }

        if words.len() == 0 {
            return Vec::new();
        }

        let search_tree = CharNode::build_tree(&words);
        let word_len = words[0].len();
        let word_count:usize = words.len();
        let sentence_len = word_len * word_count;
        let mut pos : usize = 0 ;
        let mut result:Vec<i32> = Vec::new();

        loop {

            if s.len() - pos < sentence_len {
                break;
            }

            let mut check_table = vec![0;word_count];
            let mut match_count:usize = 0;
            let mut search_pos:usize = pos;

            loop {

                let (word_no,occur_time) = CharNode::find(&search_tree, &s[search_pos..]);
                if word_no == 0 {
                    pos +=  1;
                    break;
                }

                if check_table[word_no-1] < occur_time {
                    check_table[word_no-1] += 1;
                    search_pos += words[word_no-1].len();
                    match_count +=1;
                } else {
                    pos += 1;
                    break;
                }

                if match_count == word_count {
                    result.push(pos as i32);
                    pos += 1;
                    break;
                }

            }

        }

        result
    }

    pub fn solve_find_substring() {

        let mut input = String::from("barfoothefoobarman");
        let mut words = vec![String::from("foo"),String::from("bar")];
        println!("result = {:?}",Solution::find_substring(input, words));

        input = String::from( "barfoofoobarthefoobarman");
        words = vec![String::from("foo"),String::from("bar"),String::from("the")];
        println!("result = {:?}",Solution::find_substring(input, words));

        input = String::from( "wordgoodgoodgoodbestword");
        words = vec![String::from("word"),String::from("good"),String::from("best"),String::from("good")];
        println!("result = {:?}",Solution::find_substring(input, words));
        
        input = String::from( "aaaaaaaaaaaaaa");
        words = vec![String::from("aa"),String::from("aa")];
        println!("result = {:?}",Solution::find_substring(input, words));
 
    }

    pub fn search_in_rotated_sorted_array(nums:Vec<i32>,target:i32) -> i32 {

        let mut result:i32 = -1;
        let (mut left ,mut right) = (0 as usize ,nums.len()-1 as usize);

        loop {

            let medium = (right+left)/2 as usize;
            // println!("left = {},right = {},medium = {}",left,right,medium);

            if nums[medium] == target {
                result = medium as i32;
                break;
            }

            if left >= right {
                break;
            }
            let monotonous = nums[left] < nums[right];

            if monotonous {
                if target > nums[medium] {
                    left = medium + 1;
                } else {
                    right = medium;
                }
                continue;
            }

            let medium_before_rotate = nums[medium]>nums[left];
            let target_greater_than_medium = target>nums[medium]; 
            let target_greater_than_right = target>nums[right];

            if medium_before_rotate {
                if target_greater_than_medium {
                    left = medium+1;
                } else {
                    if target_greater_than_right {
                        right = medium;
                    } else {
                        left = medium + 1;
                    }
                }
            } else {
                if target_greater_than_medium {
                    if target_greater_than_right {
                        right = medium;
                    } else {
                        left = medium + 1;
                    }
                } else {
                    right = medium;
                }
            }

        }

        result
    }

    pub fn search_in_rotated_sorted_array2(nums:Vec<i32>,target:i32) -> i32 {

        let mut result:i32 = 0;

        if nums.len() == 0 {
            return -1;
        }
        let mut pivot = nums.len();

        fn sort_rotated_array(nums:Vec<i32>,pivot:&mut usize)->Vec<i32> {

            let (mut left,mut right,) = (0,nums.len()-1);

            loop {

                if left == right || nums[left] < nums[right] {
                    break;
                }

                *pivot = (left+right+1)/2;
                if nums[*pivot] <nums[*pivot-1] {
                    break;
                }

                if nums[*pivot] > nums[left] {
                    left = *pivot;
                } else {
                    right = *pivot;
                }

            }

            let mut result = Vec::new();
            if *pivot < nums.len() {
                result.extend_from_slice(&nums[*pivot..]);
                result.extend_from_slice(&nums[0..*pivot]);
            } else {
                result = nums.clone();
            }

            result
        }

        fn find_target(nums:Vec<i32>,target:i32) -> i32 {

            let mut result:i32 = -1;
            if nums.len() == 0 {
                return result;
            }
            let (mut left,mut right,mut medium) = ( 0,nums.len() - 1,nums.len());

            loop {

                let medium = (left+right)/2;
                if nums[medium] == target {
                    result = medium as i32;
                    break;
                } 

                if left == right {
                    break;
                }

                if nums[medium] > target {
                    right = medium;
                } else {
                    left = if left == medium {medium +1} else {medium};
                }

            }

            result

        }

        let sorted_arr = sort_rotated_array(nums.clone(),&mut pivot);
        let result = find_target(sorted_arr, target);
        // println!("reslt={},pivot={}",result,pivot);

        if result < 0 || pivot >= nums.len() {
            result
        } else {
            (pivot as i32 + result) %(nums.len() as i32 ) 
        }
    }

    pub fn solve_search_in_rotated_array() {
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![4,5,6,7,0,1,2], 0));
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![4,5,6,7,0,1,2], 4));
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![4,5,6,7,0,1,2], 3));
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![1], 0));
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![1,3], 3));
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![1,3], 0));
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![3,1], 1));
        println!("result = {}",Solution::search_in_rotated_sorted_array2(vec![3,5,1], 3));
    }

    pub fn find_range_of_target_in_sorted_array(nums:Vec<i32>,target:i32) -> Vec<i32> {


        fn find_boundary(nums:Vec<i32>,target:i32,upper_boundary:bool) -> usize {

            if nums.len() == 1 {
                return if nums[0] == target {0} else {1};
            }

            let (mut left,mut right ,mut medium ,mut position) = (0,nums.len()-1,nums.len(),nums.len());

            loop {

                medium =(left+right)/2;
                // println!("left={},right={},medium={}",left,right,medium);

                if nums[medium] == target {
                    if upper_boundary {
                        if right > medium && nums[medium+1] == target {
                            left = medium +1;
                        } else {
                            position = medium;
                            break;
                        }
                    } else {
                        if medium > left && nums[medium-1] == target {
                            right = medium;
                        } else {
                            position = medium;
                            break;
                        }
                    }
                    continue;
                } 

                if left == right {
                    break;
                }

                if nums[medium] > target {
                    right = medium;
                } else {
                    left = if left == medium { medium+1} else { medium};
                }

            }

            position 

        }

        if nums.len() == 0 {
            return vec![-1,-1];
        }

        let lower_boundary = find_boundary(nums.clone(), target, false);
        if lower_boundary == nums.len() {
            return vec![-1,-1];
        } 

        let upper_boundary = find_boundary(nums.clone(), target, true);
        vec![lower_boundary as i32,upper_boundary as i32]

    }

    pub fn solv_find_range_of_target_in_sorted_array() {
        assert_eq!(Solution::find_range_of_target_in_sorted_array(vec![5,7,7,8,8,10], 8),vec![3,4]);
        assert_eq!(Solution::find_range_of_target_in_sorted_array(vec![5,7,7,8,8,10], 6),vec![-1,-1]);
        assert_eq!(Solution::find_range_of_target_in_sorted_array(vec![], 0),vec![-1,-1]);
        assert_eq!(Solution::find_range_of_target_in_sorted_array(vec![2,2], 2),vec![0,1]);
    }

    pub fn is_valid_sudoku(board:Vec<Vec<char>>) -> bool {

        let mut result = true;

        fn is_valid_line(line: & Vec<char>) -> bool {
            let mut result = true;
            let mut check_line:Vec<bool> = vec![false;9];

            for ch in line {
                
                match ch {
                    '1'..='9' => {
                        let index = *ch as usize - '1' as usize;
                        if check_line[index] == true {
                            result = false;
                            break;
                        } else {
                            check_line[index] = true;
                        }

                    },
                    _ => {

                    }
                }

            }
            
            result
        }

        for line in 0..9 {
            if ! is_valid_line(&board[line]) {
                return false;
            }
        }

        for row in 0..9 {
            let mut real_line:Vec<char> = Vec::new();
            for line in 0..9 {
                real_line.push(board[line][row]);
            }
            // println!("line = {:?}",real_line);
            if ! is_valid_line(&real_line) {
                return false;
            }
        }

        for x in 0..=2 {
            for y in 0..=2 {
                let mut line:Vec<char> = Vec::new();
                for line_no in 0..=2 {
                    for row_no in 0..=2 {
                        line.push(board[x*3+line_no][y*3+row_no]);
                    }
                }
                if ! is_valid_line(&line) {
                    return false;
                }
            }
        }

        result
    }

    pub fn solve_valid_sudoku() {

        let mut sudoku:Vec<Vec<char>> = Vec::new();

        sudoku = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ];
        // assert_eq!(Solution::is_valid_sudoku(sudoku),true);

        sudoku = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ];
        // assert_eq!(Solution::is_valid_sudoku(sudoku),false);

        sudoku = vec![
            vec!['.','.','4','.','.','.','6','3','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['5','.','.','.','.','.','.','9','.'],
            vec!['.','.','.','5','6','.','.','.','.'],
            vec!['4','.','3','.','.','.','.','.','1'],
            vec!['.','.','.','7','.','.','.','.','.'],
            vec!['.','.','.','5','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.']
            ];

        assert_eq!(Solution::is_valid_sudoku(sudoku),false);

    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

    }

    pub fn solve_solve_sudoku() {

        let mut board :Vec<Vec<char>> = Vec::new();

        board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];

        let mut result = vec![vec!['5','3','4','6','7','8','9','1','2'],vec!['6','7','2','1','9','5','3','4','8'],vec!['1','9','8','3','4','2','5','6','7'],vec!['8','5','9','7','6','1','4','2','3'],vec!['4','2','6','8','5','3','7','9','1'],vec!['7','1','3','9','2','4','8','5','6'],vec!['9','6','1','5','3','7','2','8','4'],vec!['2','8','7','4','1','9','6','3','5'],vec!['3','4','5','2','8','6','1','7','9']];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board,result);

    }


    pub fn main() {

        // Solution::solve_swap_pairs();
        // Solution::solve_reverse_k_group();
        // Solution::solve_divide();
        // Solution::solve_find_substring();
        // Solution::solve_search_in_rotated_array();
        // Solution::solv_find_range_of_target_in_sorted_array();
        Solution::solve_valid_sudoku();

        struct NodeContext <'a> {
            group: Option<Box<ListNode>>,
            group_end:&'a Option<Box<ListNode>>,
        }

        let mut node_context1 = NodeContext{
            group:None,
            group_end: & None,
        };

        node_context1.group = Some(Box::new(ListNode::new(1)));
        // node_context1.group_end = & node_context1.group.as_deref_mut().unwrap().next;
        node_context1.group_end = &None;
        node_context1.group = Some(Box::new(ListNode::new(1)));

        let task = async {
            println!("executing task");
            0
        };

        let result  = std::thread::spawn(move || async {
            task.await
        });

    }




}
