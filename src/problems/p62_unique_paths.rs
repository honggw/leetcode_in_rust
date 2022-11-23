pub fn unique_paths(m: i32, n: i32) -> i32 {
    fn walk(m: i32, n: i32) -> i32 {
        if m == 0 || n == 0 {
            return 1;
        }

        walk(m - 1, n) + walk(m, n - 1)
    }

    if m < 0 || n < 0 {
        return 0;
    } else if m == 0 || n == 0 {
        return 1;
    }

    walk(m - 1, n - 1)
}

pub fn unique_paths_2(m: i32, n: i32) -> i32 {
    fn factorial(from: i64, to: i64) -> i64 {
        if from <= 1 {
            return 1;
        }
        if from <= to {
            1
        } else {
            from * factorial(from - 1, to)
        }
    }

    if m < 0 || n < 0 {
        return 0;
    } else if m == 0 || n == 0 {
        return 1;
    }

    (factorial((m + n - 2) as i64, (if m > n { m - 1 } else { n - 1 }) as i64) / factorial((if m > n { n } else { m } - 1) as i64, 1)) as i32
}

pub fn unique_paths_3(m: i32, n: i32) -> i32 {
    if m < 0 || n < 0 {
        return 0;
    } else if m == 0 || n == 0 {
        return 1;
    }

    fn max_common_factor(i: i32, j: i32) -> i32 {
        let remainder = i % j;
        if remainder == 0 {
            j
        } else {
            max_common_factor(j, remainder)
        }
    }

    fn divide(dividends: &mut Vec<i32>, divisors: Vec<i32>) -> i32 {
        dividends.iter_mut().fold(divisors, |divisor_arr, dividend| {
            divisor_arr.into_iter().map(|divisor| {
                if *dividend == 1 || divisor == 1 {
                    divisor
                } else {
                    let factor = max_common_factor(divisor, *dividend);
                    if factor == 1 {
                        divisor
                    } else {
                        *dividend /= factor;
                        divisor / factor
                    }
                }
            }).collect()
        }).into_iter().reduce(|accum, item| {
            accum * item
        }).unwrap_or(1)
    }


    let mut multipliers = (1..=m + n - 2).rev().collect::<Vec<i32>>();

    let remainder_m = divide(&mut multipliers, (1..=m - 1).rev().collect());
    let remainder_n = divide(&mut multipliers, (1..=n - 1).rev().collect());

    multipliers.into_iter().reduce(|accum, item| { accum * item }).unwrap_or(1) / remainder_n / remainder_m

}


pub fn find_longest_single_slot(leave_times: &[Vec<i32>]) -> char {
    let mut previous_end = 0;
    let mut longest_employee = -1;
    let mut longest_duration = 0;

    for leave_time in leave_times {
        let duration = leave_time[1] - previous_end;
        if duration > longest_duration {
            longest_duration = duration;
            longest_employee = leave_time[0];
        }
        previous_end = leave_time[1];
    }

    ('a' as u8 + longest_employee as u8) as char
}

#[test]
pub fn test() {
    /*    let leaveTimes= vec![vec![0,3],vec![2,5],vec![0,9],vec![1,15]];
        println!("{}",find_longest_single_slot(&leaveTimes));
    */
    /*
    6 * 2
    */

    /*
        let mut v = vec![1,2,3];
        v.iter_mut().fold("",|acc,x|{
            *x +=1;
            acc
        });

        println!("hallo {:?}",v);
     */

    println!("paths = {}", unique_paths(3, 2));
    println!("paths = {}", unique_paths(3, 7));
    println!("paths = {}", unique_paths(1, 2));
    println!("paths = {}", unique_paths_2(3, 2));
    println!("paths = {}", unique_paths_2(3, 7));
    println!("paths = {}", unique_paths_2(1, 2));
    println!("paths = {}", unique_paths_3(3, 2));
    println!("paths = {}", unique_paths_3(3, 7));
    println!("paths = {}", unique_paths_3(1, 2));
    println!("paths = {}", unique_paths_3(1, 1));

    println!("paths = {}", unique_paths_3(16, 16));
    println!("paths = {}", unique_paths_3(51, 9));
    // println!("paths = {}", unique_paths(51, 9));
}


