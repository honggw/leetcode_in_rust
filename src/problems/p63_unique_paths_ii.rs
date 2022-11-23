/*
*

You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.

Return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The testcases are generated so that the answer will be less than or equal to 2 * 10^9.
 */

pub fn unique_paths_with_obstacles_0(obstacle_grid: Vec<Vec<i32>>) -> i32 {

    fn walk(x: i32, y: i32, obstacle_grid: &Vec<Vec<i32>>) -> i32 {
        if x as usize >= obstacle_grid.len() ||
            y as usize >= obstacle_grid[x as usize].len() ||
            obstacle_grid[x as usize][y as usize] == 1 {
            return 0;
        }

        if x as usize == obstacle_grid.len() -1 && y as usize == obstacle_grid[x as usize].len() -1 {
            return 1;
        }

        walk(x+1,y,obstacle_grid) + walk(x,y+1,obstacle_grid)
    }

    walk(0,0,&obstacle_grid)
}

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = if m == 0 { 0 } else { obstacle_grid[0].len() };
    struct Grid {
        x: i32,
        y: i32,
    }

    impl Grid {
        fn new(x: i32, y: i32) -> Self {
            Self {
                x: x,
                y: y,
            }
        }

        fn less_than(&self, other: &Self) -> bool {
            //todo
            false
        }
    }

    0
}

#[test]
fn test() {
    unique_paths_with_obstacles(vec![]);
    unique_paths_with_obstacles(vec![vec![1]]);
    unique_paths_with_obstacles(vec![vec![1], vec![1]]);
}
