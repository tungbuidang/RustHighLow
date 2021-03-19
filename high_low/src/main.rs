use rand::prelude::*;
use std::io;

fn roll_dice(time: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let mut sum = 0;
    for i in  0..time {
        let num = rng.gen::<u32>()% 6 + 1;
        sum += num;
    }
    return sum;
}

fn print_scoreboard(player: u32, dealer: u32) {
    println!("Player | {}", player);
    println!("Dealer | {}", dealer);
}

fn play_1_game(dealer_roll: u32, roll_count: u32) -> u32 {
        let _dealer_score = roll_dice(roll_count);
        let _player_score = roll_dice(roll_count);
        println!("Dealer is rolling the dire ... Dealer rolled {}", _dealer_score);
        println!("will your roll be: 1. HIGHER; 2. LOWER; 3. SAME? ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Did not register option");

        let _player_option = input.parse::<u32>().unwrap();
        let num = 1;
        match num {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => 4,
        }
}

fn main() {
    

    let a = roll_dice(1);
    println!("{}", a);
    
}
