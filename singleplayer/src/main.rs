use std::{fmt, io, thread, time};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use num::cast::AsPrimitive;
use rand::Rng;
use regex::Regex;

use Side::{O, X};

fn main() {
    // 1st starting side is randomized, for every other, losing side starts
    let mut gameboard = GameBoard::new(GameBoard::random_side());
    loop {
        gameboard.play();
    }
}

impl GameBoard {
    fn new(current_player: Side) -> Self {
        let mut fields = vec![];
        for i in 0..=2 {
            for j in 0..=2 {
                fields.push(Field::new(j, i));
            }
        }
        Self {
            fields,
            current_player,
            score_x: 0,
            score_o: 0,
        }
    }

    fn random_side() -> Side {
        let mut rng = rand::thread_rng();
        let rng_val: f32 = rng.gen();
        match rng_val.round() {
            0f32 => O,
            _ => X
        }
    }

    fn play(&mut self) {
        self._clean_game_board();
        self._print_game_board();

        loop {
            let (x, y) = self._take_user_input();
            println!("{} + {} = {}", x, y, x + y);
            self._take_field(x, y).unwrap();
            self._print_game_board();
            if let Some(winner_side) = self._find_winner() {
                println!("Game over! {} won!", winner_side);
                match winner_side {
                    X => {
                        self.score_x += 1;
                        self.current_player = O;
                    },
                    O => {
                        self.score_o += 1;
                        self.current_player = X;
                    },
                }
                thread::sleep(time::Duration::from_secs(5));
                return;
            }
        };

        // thread::sleep(time::Duration::from_secs(2));
        // self.take_field(1, 1).unwrap();
        // self._print_game_board();
        //
        // thread::sleep(time::Duration::from_secs(2));
        // self.take_field(0, 2).unwrap();
        // self._print_game_board();
        //
        // thread::sleep(time::Duration::from_secs(2));
        // self.take_field(2, 0).unwrap();
        // self._print_game_board();
        //
        // thread::sleep(time::Duration::from_secs(2));
        // self.take_field(0, 0).unwrap();
        // self._print_game_board();
        //
        // thread::sleep(time::Duration::from_secs(2));
        // self.take_field(2, 2).unwrap();
        // self._print_game_board();
        //
        // thread::sleep(time::Duration::from_secs(2));
        // self.take_field(3, 2).unwrap();
        // self._print_game_board();
    }

    fn _clean_game_board(&mut self) {
        self.fields.iter_mut().for_each(|field| field.side = None);
    }

    fn _take_field(&mut self, x_axis: i8, y_axis: i8) -> Result<(), Box<dyn Error>> {
        let field: Option<&mut Field> = self.fields.iter_mut()
            .filter(|field| field.x_axis == x_axis && field.y_axis == y_axis)
            .next();
        // next() is the equivalent of first() in rust
        match field {
            None => Err(Box::new(FieldNotExistsError)),
            Some(mut f) => {
                match f.side {
                    None => {
                        f.side = Some(self.current_player.clone());
                        self._next_player_turn();
                        Ok(())
                    }
                    Some(_) => Err(Box::new(FieldAlreadyTakenError))
                }
            }
        }
    }

    fn _next_player_turn(&mut self) {
        match self.current_player {
            X => self.current_player = O,
            O => self.current_player = X
        }
    }

    fn _print_game_board(&self) {
        let mut game_board_str = String::with_capacity(21);
        let mut i = 1;
        for field in &self.fields {
            match &field.side {
                None => game_board_str.push_str("_"),
                Some(f) => game_board_str.push_str(&*f.to_string())
            }
            game_board_str.push_str("\t");
            if i % 3 == 0 {
                game_board_str.push_str("\n");
            }
            i += 1;
        }
        clear_terminal_screen();
        println!();
        println!("Score O: {}  || Score X: {}\nCurrent turn: {}\n{}", self.score_o, self.score_x, self.current_player, game_board_str.trim());
        // for field in self.fields.into_iter() {
        //     match field.side {
        //         None => game_board_str.push_str("_"),
        //         Some(f) => game_board_str.push_str(&f.to_string())
        //     }
        // }
    }

    fn _take_user_input(&self) -> (i8, i8) {
        println!("{}", "Make your move [1-3] as (x y): ");
        let mut input;
        loop {
            input = String::new();
            // if let Some(line) = io::stdin().read_line(&mut input) {
            //     if let Some(_) = GameBoard::USER_INPUT_PATTERN.captures(line) {
            //         return line.
            //     }
            // }
            io::stdin().read_line(&mut input);
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }
            let user_input_pattern: Regex = Regex::new(r"^[1-3]\s[1-3]$").unwrap();
            input = input.trim().to_string();
            if let Some(_) = user_input_pattern.captures(&input) {
                return (nth_char_as_num(&input, 0) - 1, nth_char_as_num(&input, 2) - 1);
                // return (input.chars().nth(0).unwrap().to_digit(10).unwrap() as i8, input.chars().nth(2).unwrap().to_digit(10).unwrap() as i8);
            } else {
                println!("Try again");
            }
        }
    }

    fn _find_winner(&self) -> Option<Side> {
        let taken_fields: Vec<&Field> = self.fields.iter()
            .filter(|field| field.side.is_some())
            .collect();
        let all_x_fields: Vec<&&Field> = taken_fields.iter()
            .filter(|field| field.side.as_ref().unwrap() == &X)
            .collect();
        let all_o_fields: Vec<&&Field> = taken_fields.iter()
            .filter(|field| field.side.as_ref().unwrap() == &O)
            .collect();
        if self._has_won(all_x_fields) {
            return Some(X);
        }
        match self._has_won(all_o_fields) {
            true => Some(O),
            false => None
        }

    }

    fn _has_won(&self, same_side_fields: Vec<&&Field>) -> bool {
        if same_side_fields.iter().count() < 3 {
            return false;
        }
        let mut has_3_in_a_row = false;
        for i in 0..=2 {
            has_3_in_a_row = has_3_in_a_row || 3 == same_side_fields.iter()
                .filter(|field| field.x_axis == i)
                .count();
            has_3_in_a_row = has_3_in_a_row || 3 == same_side_fields.iter()
                .filter(|field| field.y_axis == i)
                .count();
            if has_3_in_a_row {
                return true;
            }
        }
        has_3_in_a_row = has_3_in_a_row || 3 == same_side_fields.iter()
            .filter(|field| field.x_axis == 0 && field.y_axis == 0
                || field.x_axis == 1 && field.y_axis == 1
                || field.x_axis == 2 && field.y_axis == 2)
            .count();
        has_3_in_a_row = has_3_in_a_row || 3 == same_side_fields.iter()
            .filter(|field| field.x_axis == 0 && field.y_axis == 2
                || field.x_axis == 1 && field.y_axis == 1
                || field.x_axis == 2 && field.y_axis == 0)
            .count();
        return has_3_in_a_row;
    }
}

// pub fn nth_char_as_num<T>(text: &String, index: usize) -> T where T: From<&String> {
pub fn nth_char_as_num(text: &String, index: usize) -> i8 {
    text.chars().nth(index).unwrap().to_digit(10).unwrap().as_()
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        std::process::Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}


impl Field {
    fn new(x_axis: i8, y_axis: i8) -> Self {
        Self {
            side: None,
            x_axis,
            y_axis
        }
    }
}

struct GameBoard {
    fields: Vec<Field>,
    current_player: Side,
    score_x: i16,
    score_o: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Side {
    X, O
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone)]
struct Field {
    side: Option<Side>,
    x_axis: i8,
    y_axis: i8,
}

// can also return enum instead of Box<dyn Error>
// enum ApiError {
//     FieldNotExistsError,
//     FieldAlreadyTakenError,
// }

#[derive(Debug, Clone)]
struct FieldNotExistsError;

impl Display for FieldNotExistsError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Field for given x and y values does not exist")
    }
}

impl Error for FieldNotExistsError {
    fn description(&self) -> &str {
        "Field for given x and y values does not exist"
    }
}

#[derive(Debug, Clone)]
struct FieldAlreadyTakenError;

impl Display for FieldAlreadyTakenError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Field is already taken by a player")
    }
}

impl Error for FieldAlreadyTakenError {
    fn description(&self) -> &str {
        "Field is already taken by a player"
    }
}