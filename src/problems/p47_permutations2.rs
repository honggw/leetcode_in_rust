


pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {

    let mut input = nums.clone();

    if input.len() > 1 {
        for i in 0..input.len()-1 {
            for j in i+1 ..input.len() {
                if input[i] > input[j] {
                    let val = input[j];
                    input[j] = input[i];
                    input[i] = val;
                }
            }
        }
    }

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

        let mut prev = -11;

        nums.iter()
        .enumerate()
        .for_each(| (i,&x) | {

            if x == prev {
                return;
            } else {
                prev = x;
            }

            let next_nums = nums.iter()
            .enumerate()
            .filter(|(j,&y)| {i!=*j})
            .map(|(j,&y)| y)
            .collect::<Vec<i32>>() ;

            // println!("{:?}",next_nums);

            let mut next_result = do_permute(&next_nums);
            next_result.iter_mut().for_each(|r|{r.insert(0,x)});
            result.extend_from_slice(&next_result[..]);
        });

        result
    }

    do_permute(&input)

}



#[test]
fn test() {
    println!("{:?}",permute_unique(vec![1,1,3]));
}




