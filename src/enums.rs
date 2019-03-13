// emuns are types which have a few definite values 

enum Movements {
    // Variants 
    up, 
    down, 
    left,
    right
}
fn move_game(m: Movements){
    match m {
        Movements::up => println!("Game is moving up"),
        Movements::down => println!("Game is moving down"),
        Movements::left => println!("Game is moving left"),
        Movements::right => println!("Game is moving right")
    }
}

pub fn run(){
    let game1 = Movements::up;
    let game2 = Movements::right;
    let game3 = Movements::left;
    let game4 = Movements::down;

    move_game(game1);
    move_game(game2);
    move_game(game3);
    move_game(game4);
}