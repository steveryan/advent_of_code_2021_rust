struct Board {
    board: Vec<Vec<String>>,
    marked: Vec<String>,
    winner: Option<i64>,
    already_won: bool
}

impl Board {
    fn new(board: Vec<Vec<&str>>) -> Board {
        Board {
            board: board.iter().map(|x| x.iter().map(|y| y.to_string()).collect()).collect(),
            marked: Vec::new(),
            winner: None,
            already_won: false
         }
    }

    fn mark(&mut self, number: &str) -> () {
        self.marked.push(number.to_string());
        self.winner = self.check_for_win();
    }

    fn check_for_win(& self) -> Option<i64> {
        for i in 0..5 {
            if self.marked.contains(&self.board[i][0]) && self.marked.contains(&self.board[i][1]) && self.marked.contains(&self.board[i][2]) && self.marked.contains(&self.board[i][3]) && self.marked.contains(&self.board[i][4]) {
                let score = self.winning_score_for_row(i);
                return Some(score);
            } else if self.marked.contains(&self.board[0][i]) && self.marked.contains(&self.board[1][i]) && self.marked.contains(&self.board[2][i]) && self.marked.contains(&self.board[3][i]) && self.marked.contains(&self.board[4][i]) {
                let score = self.winning_score_for_column(i);
                return Some(score);
            } 
        }
        None
    }

    fn winning_score_for_row(& self, winning_row: usize) -> i64 {
        let mut score = 0;
        for i in 0..5 {
            if i == winning_row {
                continue
            }
            for j in 0..5 {
                if !self.marked.contains(&self.board[i][j]) {
                    score += &self.board[i][j].parse::<i64>().unwrap();
                }
            }
        }

        score
    }
    fn winning_score_for_column(& self, winning_col: usize) -> i64 {
        let mut score = 0;
        for i in 0..5 {
            for j in 0..5 {
                if j == winning_col {
                    continue
                }
                if !self.marked.contains(&self.board[i][j]) {
                    score += &self.board[i][j].parse::<i64>().unwrap();
                }
            }
        }
 
        score
    }
}

fn main() -> Result<(), std::io::Error>{
    let file_location = "problem_4.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let mut vec = contents.split("\n\n").collect::<Vec<&str>>();
    let mut numbers = vec[0].split(",").collect::<Vec<&str>>();
    vec.remove(0);
    let boards = vec.iter().map(|x| x.split("\n").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let boards = boards.iter().map(|x| x.iter().map(|y| y.trim().replace("  ", " ")).collect()).collect::<Vec<Vec<String>>>();
    let boards = boards.iter().map(|x| x.iter().map(|y| y.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>()).collect::<Vec<Vec<Vec<&str>>>>();
    let mut boards = boards.iter().map(|x| Board::new(x.to_vec())).collect::<Vec<Board>>();
    
    part_2(boards, numbers);
    Ok(())
}

fn part_1(mut boards: Vec<Board>, numbers: Vec<&str>) {
    'outer: for number in numbers {
        for board in &mut boards {
            board.mark(number);
            if board.winner.is_some() {
                let winning_score = board.winner.unwrap() * number.parse::<i64>().unwrap();
                println!("winning score is: {}", winning_score);
                break 'outer;
            }
        }
    }
}

fn part_2(mut boards: Vec<Board>, numbers: Vec<&str>) {
    for number in numbers {
        for board in &mut boards {
            board.mark(number);
            if board.winner.is_some() && !board.already_won {
                let winning_score = board.winner.unwrap() * number.parse::<i64>().unwrap();
                board.already_won = true;
                println!("winning board is: {:?}", board.board);
                println!("winning score is: {}", winning_score);
            }
        }
    }
}