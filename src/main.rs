use std::io;

fn main() {
    println!("가위, 바위, 보 !!!!");

    loop {
        println!("Enter your choice (rock/paper/scissors):");

        let mut player_choice = String::new();
        io::stdin().read_line(&mut player_choice).unwrap();
        let player_choice = player_choice.trim();

        let computer_choice = match rand::random::<u8>() % 3 {
            0 => "rock",
            1 => "paper",
            _ => "scissors",
        };

        println!("Your choice: {}", player_choice);
        println!("Computer's choice: {}", computer_choice);

        let result = match (player_choice, computer_choice) {
            ("rock", "scissors") => "You win!",
            ("paper", "rock") => "You win!",
            ("scissors", "paper") => "You win!",
            ("rock", "rock") => "Draw!",
            ("paper", "paper") => "Draw!",
            ("scissors", "scissors") => "Draw!",
            (_, _) => "You lose!",
        };

        println!("{}", result);

        println!("Do you want to play again? (yes/no)");

        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).unwrap();
        let play_again = play_again.trim();

        if play_again != "yes" {
            break;
        }
    }

    println!("Thanks for playing!");
}
