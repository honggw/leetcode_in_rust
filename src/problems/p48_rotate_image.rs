

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {

    #[derive(Clone)]
    struct Position {
        x:usize,
        y:usize,
    }

    impl Position {

        fn new(x_:usize,y_:usize) -> Self { 
            Self {
                x:x_,
                y:y_,
            }
        }

        fn rotate(&self,matrix:&Vec<Vec<i32>>) -> Self {
            let row = matrix.len() - self.x - 1;
            let new_x = self.y;
            let new_y = row;
            Self {
                x:new_x,
                y:new_y,
            }
        }

        fn get(&self,matrix:&Vec<Vec<i32>>) -> i32 {
            let line = &matrix[self.x];
            line[self.y]
        }

        fn set(&self,matrix:&mut Vec<Vec<i32>>,val:i32) {
            let line = &mut matrix[self.x];
            line[self.y] = val;
        }

        fn equals(&self,other:&Self) -> bool {
            self.x == other.x && self.y == other.y
        }

    }

    for i in 0 ..= matrix.len()/2 {
        for j in  i..matrix.len() - i - 1 {

            let start_point = Position::new(i,j);
            let mut rotate_point = start_point.clone();
            let mut rotate_value = rotate_point.get(matrix);

            loop {

                let next_point = rotate_point.rotate(matrix);
                let next_value = next_point.get(matrix);
                next_point.set(matrix,rotate_value);
                rotate_value = next_value;
                rotate_point = next_point;
                if start_point.equals(&rotate_point) {
                    break;
                }

            }
        }
    }

}

#[test]
fn test() {

    let mut input = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
    rotate(&mut input);
    println!("{:?}",input);

    input = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    rotate(&mut input);
    println!("{:?}",input);

    input = vec![vec![1,2],vec![4,5]];
    rotate(&mut input);
    println!("{:?}",input);

    input = vec![vec![1]];
    rotate(&mut input);
    println!("{:?}",input);

}



