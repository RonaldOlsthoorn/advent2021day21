use std::{io::{BufReader, BufRead}, fs::File, collections::{VecDeque}};


#[derive(Clone)]
struct Pawn {
    _state: u8,
}

impl Pawn {

    const STATES: u8 = 10;

    fn new(init_state: u8) -> Self {
        Self { _state: init_state }
    }

    fn tick(&mut self, steps: u8) -> u8 {
        self._state = ((self._state + steps - 1) % Self::STATES) + 1;
        self._state
    }
}

#[derive(Clone)]
struct Player {
    pawn: Pawn,
    score: u16
}

impl Player {

    fn new(init_state: u8) -> Self {
        Player { pawn: Pawn::new(init_state), score: 0 }
    }

    fn play_turn(&mut self, die_outcome: u8) {
        self.score += self.pawn.tick(die_outcome) as u16;
    }
}

#[derive(Clone)]
struct WalkState {
    player1: Player,
    player2: Player,
    multiverses: u64,
    turn_p1: bool
}

struct SchrodingerSim {
}

impl SchrodingerSim {

    const DIE_FREQUENCIES: [(u8, u64);7] = [
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1)
    ];

    fn simulate_schrodinger(&self, player1: &Player, player2: &Player) -> (u64, u64) {

        let mut res = (0, 0);
    
        let mut stack: VecDeque<WalkState> = VecDeque::new();
        stack.push_back(WalkState { player1: player1.clone(), player2: player2.clone(), multiverses: 1, turn_p1: true });
    
        while let Some(current_state) = stack.pop_back() {

            if !current_state.turn_p1 && current_state.player1.score >= 21 {
                // turn p2 but p1 already reached goal
                res.0 += current_state.multiverses;
            } else if current_state.turn_p1 && current_state.player2.score >= 21 {
                // turn p1 but p2 already reached goal
                res.1 += current_state.multiverses;
            } else if current_state.turn_p1 && current_state.player1.score >= 20 {
                // turn p1 and p1 cannot lose
                res.0 += current_state.multiverses * 27;
            } else if !current_state.turn_p1 && current_state.player2.score >= 20 {
                // turn p2 and p2 cannot lose
                res.1 += current_state.multiverses * 27;
            } else {
                self.branch_out(&current_state).into_iter().for_each(|s| stack.push_back(s));
            }
        }    
        res
    }

    fn branch_out(&self, current_state: &WalkState) -> Vec<WalkState> {

        let mut branches = Vec::new();

        for (die_outcome, die_frequency) in Self::DIE_FREQUENCIES {
            let mut branch = current_state.clone();

            if current_state.turn_p1 {
                branch.player1.play_turn(die_outcome);

            } else {
                branch.player2.play_turn(die_outcome);
            }

            branch.multiverses *= die_frequency;
            branch.turn_p1 = !branch.turn_p1;

            branches.push(branch);
        }
        
        branches
    }

}


fn main() {

    let lines: Vec<String> = BufReader::new(File::open("test-input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    
    let player1 = Player::new(lines[0].chars().last().unwrap().to_digit(10).unwrap() as u8);
    let player2 = Player::new(lines[1].chars().last().unwrap().to_digit(10).unwrap() as u8);

    let sim = SchrodingerSim{};
    let (score_player1, score_player2) = sim.simulate_schrodinger(&player1, &player2);

    println!("scores p1 {} p2 {} winner: {}", score_player1, score_player2, std::cmp::max(score_player1, score_player2));

}
