
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {

    let mut result:Vec<i32> = Vec::new();
    let mut input = matrix.clone();

    fn rotate_counterclockwise(matrix:Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let mut result:Vec<Vec<i32>> = Vec::new();

        if matrix.len() == 0 {
            return result;
        }

        let line_length = matrix.get(0).unwrap().len();

        for x in 0..line_length {
            let mut new_line:Vec<i32> = Vec::new();
            let row_no = line_length - 1 - x;
            for y in 0 .. matrix.len() {
                new_line.push(matrix.get(y).unwrap().get(row_no).unwrap().clone());
            }
            result.push(new_line);
        }

        result
    }

    while input.len() > 0 {
        input.get(0).unwrap().iter().for_each(|&x|  {
            result.push(x.clone());
        });
        input.remove(0);
        input = rotate_counterclockwise(input);
    }

    result
}

#[test]
fn test() {

    let input:Vec<Vec<i32>> = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];

    println!("result = {:?}",spiral_order(input));

}
