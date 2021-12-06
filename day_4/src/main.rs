struct Board<'a> {
    board: Vec<Vec<&'a str>>,
    marked: Vec<&'a str>,
    winner: bool
}

impl<'a> Board<'a> {
    fn new(board: Vec<Vec<&str>>) -> Board {
        Board {
            board,
            marked: Vec::new(),
            winner: false
         }
    }

    fn mark(&'a mut self, number: &'a str) -> () {
        self.marked.push(number);
        self.winner = self.check_for_win();
        if self.winner {
            println!("{:?} wins!", self.board);
        }
    }

    fn check_for_win(& self) -> bool {
        for i in 0..4 {
            if self.marked.contains(&self.board[i][0]) && self.marked.contains(&self.board[i][1]) && self.marked.contains(&self.board[i][2]) && self.marked.contains(&self.board[i][3]) && self.marked.contains(&self.board[i][4]) {
                return true
            } else if self.marked.contains(&self.board[0][i]) && self.marked.contains(&self.board[1][i]) && self.marked.contains(&self.board[2][i]) && self.marked.contains(&self.board[3][i]) && self.marked.contains(&self.board[4][i]) {
                return true
            } 
        }
        false
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
    let mut num_boards = boards.len();
    
    for mut number in &mut numbers {
        for mut board in &mut boards {
            board.mark(number);
        }
    }
    Ok(())
}
