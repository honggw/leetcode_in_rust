
/*

The set [1, 2, 3, ..., n] contains a total of n! unique permutations.

By listing and labeling all of the permutations in order, we get the following sequence for n = 3:

"123"
"132"
"213"
"231"
"312"
"321"
Given n and k, return the kth permutation sequence.

 */
pub fn get_permutation(n:i32,k:i32) -> String {

    // let mut result:String = String::new();
    let mut permutation: Vec<i32> = vec![0; n as usize];
    let mut num_set: Vec<i32> = vec![0; n as usize];

    (1..=n).for_each(|i| {
        num_set[i as usize - 1] = i;
    });
    // println!("numset={:?}",num_set);

    fn retrive_nth(nth:i32,num_set:&mut Vec<i32>) -> i32 {

        let mut i = 0;
        let mut j = 0;

        loop {

            if i >= num_set.len() as i32 {
                // println!("out of range");
                return 0;
            }

            if num_set[i as usize] == 0 {
                i += 1;
                continue;
            }

            j += 1;

            if j == nth {
                let ret = num_set[i as usize];
                num_set[i as usize] = 0;
                // println!("numset={:?}",num_set);
                return ret;
            } else {
                i += 1;
            }

        }

    }

    let mut factorial: i32 = (1..=n).reduce(|acc,item|  {acc*item}).unwrap();
    // println!("factorial = {}",factorial);
    let mut step_k = k;

    (1..=n).rev().for_each(|i| {
        factorial /=i;
        let ith = (step_k -1 )/factorial + 1; //if step_k <= factorial {1}else {step_k/factorial+1};
        // println!("factorial = {},step_k={},ith={}",factorial,step_k,ith);
        permutation[n as usize -i as usize] = retrive_nth(ith,&mut num_set);
        step_k %= factorial;
        step_k = if step_k == 0 { factorial } else { step_k };
    });

    println!("factorial={},permutations={:?}",factorial,permutation);
    permutation.iter().map(|item| {char::from_u32(*item as u32 + '0' as u32 ).unwrap()}).collect::<String>()

}

pub fn get_permutation_recursive(n:i32,k:i32) -> String {

    let mut result:String = String::new();

    struct NumNode {
        val:i32,
        next:Option<Box<NumNode>>,
    }

    impl NumNode {

        fn new_child(&mut self,size:i32) {

            if size == 0 {
                return ;
            }

            self.next = Some(Box::new(Self{
                val:size,
                next:None,
            }));

            self.next.as_mut().unwrap().new_child(size - 1);

        }


        fn new (size:i32) -> Self {

            let mut head = Self {
                val:0,
                next:None,
            };

            head.new_child(size);

            head

        }

        fn remove(&mut self, nth: i32, current_no: i32) -> i32 {
            let mut ret = 0;
/*
            if nth == current_no {
                if self.next.is_some() {
                    ret = self.next.as_ref().unwrap().val;
                    self.next = self.next.as_mut().unwrap().next;
                }
            } else if self.next.is_some() {
                return self.next.unwrap().remove(nth, current_no + 1);
            }
*/
            ret
        }

    }

    let mut head = NumNode::new(n);
    let factorial:i32 = (1..=n).reduce(|acc,item| { acc * item }).unwrap();

    result

}

pub fn get_permutation_recursive2(n:i32,k:i32) -> String {
    let mut factorial = (1..=n).reduce(|acc,item| {acc*item}).unwrap();

    struct SortedNumSet {
        nums:Vec<i32>,
    }

    impl SortedNumSet {
        fn new(n:i32) -> Self {
            Self {
                nums: (1..=n).collect::<Vec<i32>>(),
            }
        }

        fn retrive_nth(&mut self, nth: i32) -> i32 {
            let mut ret = 0;

            let (mut i, mut j) = (0, 0);

            loop {
                if i >= self.nums.len() {
                    return 0;
                }

                if self.nums[i as usize] == 0 {
                    i += 1;
                    continue;
                }

                j += 1;

                if j == nth {
                    ret = self.nums[i as usize];
                    self.nums[i as usize] = 0;
                    return ret;
                } else {
                    i += 1;
                }
            }
        }
    }

    let mut sorted_num_set = SortedNumSet::new(n);

    fn permutate(factorial: i32, n: i32, k: i32,sorted_num_set:&mut SortedNumSet) -> Vec<i32> {
        let mut result = Vec::new();

        if n == 0 {
            return result;
        }

        let next_factorial = factorial / n;

        let num = (k - 1) / next_factorial + 1;
        result.push( sorted_num_set.retrive_nth(num));
        // result[0] = num;
        let mut new_k = k%next_factorial;

        if new_k == 0 {
            new_k = next_factorial;
        }

        result.extend_from_slice(&permutate(next_factorial,n-1,new_k,sorted_num_set)[..]);

        result
    }

    permutate(factorial,n,k,&mut sorted_num_set).iter().map(|i|{char::from_u32(*i as u32 + '0' as u32).unwrap()}).collect::<String>()

}

#[test]
fn test(){

    // println!("{:?}",(1..=10).collect::<Vec<i32>>());
    // println!("permutation = {:?}",get_permutation_recursive2(2,2));
    // println!("permutation = {:?}",get_permutation(3,4));

    println!("permutation = {:?}",get_permutation_recursive2(1,1));
    println!("permutation = {:?}",get_permutation_recursive2(2,1));
    println!("permutation = {:?}",get_permutation_recursive2(2,2));
    println!("permutation = {:?}",get_permutation_recursive2(3,1));
    println!("permutation = {:?}",get_permutation_recursive2(3,2));
    println!("permutation = {:?}",get_permutation_recursive2(3,3));
    println!("permutation = {:?}",get_permutation_recursive2(3,4));
    println!("permutation = {:?}",get_permutation_recursive2(3,5));
    println!("permutation = {:?}",get_permutation_recursive2(3,6));


}
