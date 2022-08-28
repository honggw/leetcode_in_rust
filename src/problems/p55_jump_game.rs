
pub fn can_jump(nums: Vec<i32>) -> bool {

    if nums.len() == 0 {
        return false;
    }

    let mut pos = 0;

    loop {

        let distance = nums[pos] as usize ;

        if distance as usize + pos  >= nums.len()-1 {
            return true;
        }

        let mut next_distance = distance;
        let mut next_pos = pos;

        for i in 1..=distance as usize {

            let j = pos + i + nums[pos+i] as usize;
            if j > next_distance {
                next_pos = pos + i;
                next_distance = j;
            }

        }

        if next_pos == pos {
            return false;
        }
        
        pos = next_pos;

    }

    false

}

#[test]
fn test() {
    println!("{}",can_jump(vec![2,3,1,1,4]));
    println!("{}",can_jump(vec![3,2,1,0,4]));
    println!("{}",can_jump(vec![0]));
}


