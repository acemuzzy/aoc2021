pub struct Day4;

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>,
}

impl BingoBoard {
    fn from_rows(rows: Vec<String>) -> BingoBoard {
        let mut board: Vec<Vec<i32>> = vec![];
        let mut marked: Vec<Vec<bool>> = vec![];

        for row in rows {
            board.push(row.split_whitespace().map(|d| d.parse().unwrap()).collect());
            marked.push(vec![false, false, false, false, false]);
        }

        BingoBoard { board, marked }
    }

    fn mark(&mut self, number: i32) {
        for ii in 0..5 {
            for jj in 0..5 {
                self.marked[ii][jj] |= self.board[ii][jj] == number;
            }
        }
    }

    fn is_finished(&self) -> bool {
        for ii in 0..5 {
            let mut vert = true;
            let mut hor = true;
            for jj in 0..5 {
                vert &= self.marked[ii][jj];
                hor &= self.marked[jj][ii];
            }

            if vert | hor {
                return true;
            }
        }

        false
    }

    fn total_unmarked(&self) -> i32 {
        let mut total = 0;
        for ii in 0..5 {
            for jj in 0..5 {
                if !self.marked[ii][jj] {
                    total += self.board[ii][jj];
                }
            }
        }
        total
    }
}

impl crate::lib::DayInner<Day4> for Day4 {
    fn day(&self) -> i32 {
        4
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();
        let numbers: Vec<i32> = lines[0].split(',').map(|d| d.parse().unwrap()).collect();
        let mut boards: Vec<BingoBoard> = vec![];

        let num_boards = (lines.len() - 1) / 6;

        for index in 0..num_boards {
            let start_row = 6 * index + 2;
            let board_rows = lines[start_row..start_row + 5]
                .to_vec()
                .iter()
                .map(|x| x.to_string())
                .collect();
            boards.push(BingoBoard::from_rows(board_rows));
        }

        let mut rc1 = None;
        let mut rc2 = None;

        'outer: for num in numbers {
            let len = boards.len();
            for board in boards.iter_mut() {
                board.mark(num);
                if board.is_finished() {
                    println!("Finished! {:?}", board);
                    if rc1.is_none() {
                        rc1 = Some(board.total_unmarked() * num);
                        println!("RC1 = {}", rc1.unwrap());
                    }
                    if len == 1 {
                        rc2 = Some(board.total_unmarked() * num);
                        println!("RC2 = {}", rc2.unwrap());
                        break 'outer;
                    }
                }
            }

            boards = boards.into_iter().filter(|x| !x.is_finished()).collect();
        }

        (rc1.unwrap(), rc2.unwrap())
    }
}
