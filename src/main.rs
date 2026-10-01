use crate::game_fn::{monster::Monster, player::Player, traits::Character};
mod game_fn;

fn main() {
    let mut player = Player {
        attack: 10,
        hp: 100,
    };
    let mut monster = Monster {
        attack: 7,
        hp: 60,
    };

    println!("--- BATTLE START ---");
    println!("Player (HP: {}, Atk: {}) vs Monster (HP: {}, Atk: {})", player.hp, player.attack, monster.hp, monster.attack);
    println!("--------------------");

    loop {
        println!("Player attacks Monster for {} damage!", player.attack);
        player.attack(&mut monster);
        println!("Monster HP is now: {}", monster.hp);

        if !monster.is_alive() {
            println!("The Monster is defeated! Player wins!");
            break;
        }

        println!("--------------------");

        println!("Monster attacks Player for {} damage!", monster.attack);
        monster.attack(&mut player);
        println!("Player HP is now: {}", player.hp);

        if !player.is_alive() {
            println!("The Player is defeated! Monster wins!");
            break;
        }
        
        println!("--------------------");
    }
}