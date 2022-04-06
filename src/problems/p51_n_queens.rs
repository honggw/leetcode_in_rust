


pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {



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

    fn render_line(pos : u8,board_size:&i32) -> String {
        vec!['.';*board_size as usize].into_iter().enumerate().map(|(i,ch)| { if i as u8 == pos { 'Q' } else { ch } }).collect()
    }

    let chess_board:Vec<i32> = Vec::new();
    let boards= play_queens(&n, chess_board);
    boards.iter().map(|x| {
        x.iter().map(|&y| {
            render_line(y as u8,&n)
        }).collect::<Vec<String>>()
    }).collect::<Vec<Vec<String>>>()

}


#[test]
fn test() {
    println!("{:?}",solve_n_queens(1));
    println!("{:?}",solve_n_queens(4));
    println!("{:?}",solve_n_queens(9));
}