

pub fn total_n_queens(n: i32) -> i32 {

    fn play_queens(board_size:&i32,chess_board:Vec<i32>)->Vec<Vec<i32>> {

        let mut result:Vec<Vec<i32>> = Vec::new();
        let line = chess_board.len();

        for pos in 0..*board_size {

            let mut valid = true;
            for i in 0..chess_board.len() {
                let diffx = pos - chess_board[i] ;
                let diffy = chess_board.len() as i32  -i as i32;
                if diffx == diffy || diffx == -diffy || diffx == 0 {
                    valid = false;
                    break;
                }
            }

            if ! valid {
                continue;
            }

            let mut new_board = chess_board.clone();
            new_board.push(pos);

            if new_board.len()  == *board_size as usize {
                result.push(new_board);
            } else {
                let new_result = play_queens(board_size, new_board);
                result.extend_from_slice(&new_result[..]);
            }

        }

        result

    }

    fn calc_board_fingerprint(board:&Vec<i32>) -> i32 {
        
        let demical:i32 = 10;
        let (mut x_print,mut y_print) = (0,0);

        for i in 0..board.len() {
            x_print += demical.pow(i as u32 ) * board[i];
            y_print += i as i32 * demical.pow(board[i] as u32);
        }

        if x_print > y_print {
            y_print
        } else {
            x_print
        }

    }


    let chess_board:Vec<i32> = Vec::new();
    let solutions = play_queens(&n, chess_board);
    let finger_prints = solutions.iter().map(|solution| calc_board_fingerprint(solution)).collect::<Vec<i32>>();
    println!("{:?}",finger_prints);

    let mut result = 0;

    result
}

#[test]
fn test() {
    total_n_queens(4);
}