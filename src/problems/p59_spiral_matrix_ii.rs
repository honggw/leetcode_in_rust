

/*
Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.
 */


pub fn generate_matrix(n:i32) -> Vec<Vec<i32>> {

    struct Spiral {
        x:i32,
        y:i32,
        n:i32,
        move_x:bool,
        x_step:i32,
        y_step:i32,
    }

    impl Spiral {
        fn new() -> Self {
            Self {
                x: 0,
                y: 0,
                n: 0,
                move_x: true,
                x_step: 1,
                y_step: 1,
            }
        }

        fn get_cur_position(&self,matrix: &Vec<Vec<i32>>) -> i32 {
            matrix[self.y as usize][self.x as usize]
        }

        fn set_cur_position(&self,matrix:&mut Vec<Vec<i32>>) {
            matrix[self.y as usize][self.x as usize] = self.n;
        }

        fn forward(&mut self, matrix: &mut Vec<Vec<i32>>) {

            if self.get_cur_position(matrix) > 0 {
                return;
            }

            self.n += 1;
            self.set_cur_position(matrix);

            if self.move_x {
                self.x += self.x_step;
                if self.x >= matrix.len() as i32 || self.x < 0 || self.get_cur_position(matrix) > 0 {
                    self.x -= self.x_step;
                    self.move_x = false;
                    self.x_step *= -1;
                    self.y += self.y_step;
                }
            } else {
                self.y += self.y_step;
                if self.y >= matrix.len() as i32 || self.y < 0 || self.get_cur_position(matrix) > 0 {
                    self.y -= self.y_step;
                    self.move_x = true;
                    self.y_step *= -1;
                    self.x +=  self.x_step;
                }
            }

            self.forward(matrix);
        }
    }

    if n == 0 {
        return vec![vec![]];
    }

    if n == 1 {
        return vec![vec![1]];
    }

    let mut result= vec![vec![0;n as usize ];n as usize] ;
    Spiral::new().forward(&mut result);

    result

}


#[test]
fn test() {
    println!("matrix 5 = {:?}",generate_matrix(5)); // [[1,2,3],[8,9,4],[7,6,5]]
    println!("matrix 4 = {:?}",generate_matrix(4)); // [[1,2,3],[8,9,4],[7,6,5]]
    println!("matrix 3 = {:?}",generate_matrix(3)); // [[1,2,3],[8,9,4],[7,6,5]]
    println!("matrix 1 = {:?}",generate_matrix(1)); // [[1]]
}
