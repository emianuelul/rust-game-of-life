use std::thread::sleep;
use std::time::Duration;
use std::*;
use std::io::Write;

type Matrix = Vec<Vec<char>>;

fn main() {

    let live_cell: char;
    let option;
    loop{
        println!("Which file do you want to use?");
        println!("1, 2, 3, 4, 5, 6, 7 or 8");

        print!("Enter you choice: ");
        io::stdout().flush().unwrap();

        let mut input= String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input.parse::<u8>().unwrap() {
            1..=8 => {
                option = input.parse::<u8>().unwrap();
                break;
            },
            _ => println!("Invalid input!")
        }
    }

    loop{
        print!("What character do you want to use for live cells? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().len() == 0 || input.trim().len() > 1{
            println!("Input must consist of only one character");
        } else if input.trim()  == "."{
            println!("Please choose a different character")
        }
        else {
            live_cell = input.trim().parse().unwrap();
            break;
        }
    }
    let generations ;
    loop{
        print!("How many generations do you want to run? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<u32>();

        if input.is_err(){
            println!("You must input a number, not a string");
        }
        else{
            generations = input.unwrap();
            break;
        }
    }

    let file_input = fs::read_to_string(format!("game_of_life_starters/life{}.game", option)).unwrap();

    let mut game: Matrix = vec![Vec::from(['.'; 64]); 47];

    for line in file_input.lines().enumerate() {
        for character in line.1.chars().enumerate() {
            if character.1 == ' ' || character.1 == '\n' {
                game[line.0][character.0] = '.';
            } else {
                game[line.0][character.0] = live_cell;
            }
        }
    }

    let directions: [(isize, isize); 8] = [
        (-1, 0),  // T - 0
        (-1, 1),  // TR - 1
        (0, 1),   // R - 2
        (1, 1),   // BR - 3
        (1, 0),   // B - 4
        (1, -1),  // BL - 5
        (0, -1),  // L - 6
        (-1, -1), // TL - 7
    ];


    let mut curr_gen = 0;

    while curr_gen < generations {
        // ca sa afiseze "frame-uri", initial facusem sa afiseze toate generatiile din prima dar am vrut sa vad efectiv cum evolueaza :p
        print!("\x1B[2J\x1B[3J\x1B[H");
        println!("Generation: {}", curr_gen + 1);
        for line in &game {
            for pixel in line {
                print!("{pixel} ");
            }
            println!();
        }

        sleep(Duration::from_millis(125));

        let mut new_game = game.clone();

        let mut alive = 0;
        let mut changes = 0;
        for (x, line) in game.iter().enumerate() {
            for (y, _) in line.iter().enumerate() {
                let mut neighbors = 0;

                for direction in directions {
                    let new_x: isize = x as isize + direction.0;
                    let new_y: isize = y as isize + direction.1;

                    if new_x >= 0
                        && (new_x as usize) < game.len()
                        && new_y >= 0
                        && (new_y as usize) < line.len()
                        && game[new_x as usize][new_y as usize] == live_cell
                    {
                        neighbors += 1;
                    }
                }

                if new_game[x][y] == live_cell {
                    alive += 1;
                }

                if new_game[x][y] == live_cell && neighbors != 2 && neighbors != 3 {
                    new_game[x][y] = '.';
                    changes += 1;
                } else if new_game[x][y] == '.' && neighbors == 3 {
                    new_game[x][y] = live_cell;
                    changes += 1;
                }
            }
        }

        if alive == 0 {
            println!("Everyone's DEAD!");
            break;
        }
        else if changes == 0 {
            println!("Nothing's going to change!");
            break;
        }

        game = new_game;

        curr_gen += 1;
    }
}
