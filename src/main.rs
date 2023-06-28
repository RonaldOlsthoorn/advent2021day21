use std::{io::{BufReader, BufRead}, fs::File};

struct Die {
    _state: Box<dyn Iterator<Item = u8>>,
    _counter: usize
}

impl Die {

    const ALL_STATES: [u8; 100] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
        31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
        51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
        61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
        91, 92, 93, 94, 95, 96, 97, 98, 99, 100];

    fn new() -> Self {
        Die { _state: Box::new(Self::ALL_STATES.into_iter().cycle()), _counter: 0 }
    }

    fn roll(&mut self) -> u8 {
        self._counter += 1;
        self._state.next().unwrap()
    }
}
struct Pawn {
    _state: Box<dyn Iterator<Item = u8>>,
}

impl Pawn {

    const ALL_STATES: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    fn new(init_state: u8) -> Self {
        let mut new_pawn = Pawn { _state: Box::new(Self::ALL_STATES.into_iter().cycle()) };
        new_pawn.tick(init_state.into());
        new_pawn
    }

    fn tick(&mut self, steps: usize) -> u8 {
        (0..steps - 1).for_each(|_| {self._state.next();});
        self._state.next().unwrap()
    }
}

struct Player {
    pawn: Pawn,
    score: u16
}

impl Player {

    fn new(init_state: u8) -> Self {
        Player { pawn: Pawn::new(init_state), score: 0 }
    }

    fn play_turn(&mut self, die: &mut Die) {
        let rolls = die.roll() as usize + die.roll() as usize + die.roll() as usize;
        self.score += self.pawn.tick(rolls) as u16;
    }
}

fn main() {

    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    
    let mut player1 = Player::new(lines[0].chars().last().unwrap().to_digit(10).unwrap() as u8);
    let mut player2 = Player::new(lines[1].chars().last().unwrap().to_digit(10).unwrap() as u8);

    let mut die = Die::new();

    let res: usize;

    loop {
        player1.play_turn(&mut die);

        if player1.score >= 1000 {

            res = player2.score as usize * die._counter;
            break;
        }

        player2.play_turn(&mut die);

        if player2.score >= 1000 {

            res = player1.score as usize  * die._counter;
            break;
        }
    }

    println!("res: {}", res);
    
}
