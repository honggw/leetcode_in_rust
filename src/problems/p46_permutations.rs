

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {

    fn do_permute(nums:&Vec<i32>) -> Vec<Vec<i32>> {

        let mut result:Vec<Vec<i32>> = Vec::new();

        match nums.len() {
            0 => {
                return result;
            },
            1 => {
                result.push(nums.clone());
                return result;
            },
            _ => { }
        }

        nums.iter().for_each(|&x| {
            let next_nums = nums.iter().filter(|&y| *y != x).map(|&i| i).collect::<Vec<i32>>();
            let mut next_result = do_permute(&next_nums);
            next_result.iter_mut().for_each(|r| {r.insert(0,x)} );
            result.extend_from_slice(&next_result[..]);
        });

        result
    }

    do_permute(&nums)
}


#[test]
fn test() {
    println!("{:?}",permute(vec![]));
    println!("{:?}",permute(vec![1]));
    println!("{:?}",permute(vec![1,2]));
    println!("{:?}",permute(vec![1,2,3]));
}
