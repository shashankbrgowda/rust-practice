use std::collections::{HashMap, HashSet};

pub fn is_valid_sudoku(board: &Vec<Vec<char>>) -> bool {
    let mut row: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut col: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut cubes: HashMap<usize, HashSet<char>> = HashMap::new();

    let mut cube: usize;
    for r in 0..9 {
        cube = (r/3)*3;
        for c in 0..9 {
            let val: char = board[r][c];
            if val == '.' {
                continue;
            }
            let mut r_val: &mut HashSet<char> = row.entry(r).or_insert(HashSet::new());
            if r_val.contains(&board[r][c]) {
                return false;
            }
            r_val.insert(board[r][c]);
            let mut c_val: &mut HashSet<char> = col.entry(c).or_insert(HashSet::new());
            if c_val.contains(&board[r][c]) {
                return false;
            }
            c_val.insert(board[r][c]);
            let mut cube_val: &mut HashSet<char> = cubes.entry((c/3) + cube).or_insert(HashSet::new());
            if cube_val.contains(&board[r][c]) {
                return false;
            }
            cube_val.insert(board[r][c]);
        }
    }
    true
}

pub fn is_valid_sudoku_brute_force(board: &Vec<Vec<char>>) -> bool {
    let mut row: HashMap<usize, Vec<char>> = HashMap::new();
    let mut col: HashMap<usize, Vec<char>> = HashMap::new();
    let mut cubes: HashMap<usize, Vec<char>> = HashMap::new();

    let mut cube: usize;
    for r in 0..9 {
        cube = (r/3)*3;
        for c in 0..9 {
            let val: char = board[r][c];
            if val == '.' {
                continue;
            }
            let mut r_val: &mut Vec<char> = row.entry(r).or_insert(Vec::new());
            r_val.push(board[r][c]);
            let mut c_val: &mut Vec<char> = col.entry(c).or_insert(Vec::new());
            c_val.push(board[r][c]);
            let mut cube_val: &mut Vec<char> = cubes.entry((c/3) + cube).or_insert(Vec::new());
            cube_val.push(board[r][c]);
        }
    }

    check_duplicate(&row) && check_duplicate(&col) && check_duplicate(&cubes)
}

fn check_duplicate(map: &HashMap<usize, Vec<char>>) -> bool {
    let mut set: HashSet<char>;
    for v in map.values() {
        set = HashSet::new();
        for ele in v.iter() {
            if set.contains(ele) {
                return false;
            } else {
                set.insert(*ele);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::array::is_valid_sudoku::*;

    #[test]
    fn test_is_valid_sudoku() {
        let input = vec![vec!['5','3','.','.','7','.','.','.','.']
                         ,vec!['6','.','.','1','9','5','.','.','.']
                         ,vec!['.','9','8','.','.','.','.','6','.']
                         ,vec!['8','.','.','.','6','.','.','.','3']
                         ,vec!['4','.','.','8','.','3','.','.','1']
                         ,vec!['7','.','.','.','2','.','.','.','6']
                         ,vec!['.','6','.','.','.','.','2','8','.']
                         ,vec!['.','.','.','4','1','9','.','.','5']
                         ,vec!['.','.','.','.','8','.','.','7','9']].to_vec();
        assert_eq!(is_valid_sudoku_brute_force(&input), true);
        assert_eq!(is_valid_sudoku(&input), true);
    }
}