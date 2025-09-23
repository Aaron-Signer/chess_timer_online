use rand::Rng;
use rusqlite::{Connection, Result, params}; // For database operations and result handling

#[derive(Debug)]
struct Game {
    game_id: String,
    player_1: Option<String>,
    player_2: Option<String>,
}

fn get_games(conn: Connection) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare("SELECT game_id, player_1, player_2 FROM games")?;
    let rows = stmt.query_map([], |row| {
        Ok(Game {
            game_id: row.get(0)?,
            player_1: row.get(1)?,
            player_2: row.get(2)?,
        })
    })?;

    let mut games = Vec::new();
    for game in rows {
        games.push(game?);
    }

    for game in &games {
        println!(
            "{:?}, {:?}, {:?}",
            game.game_id, game.player_1, game.player_2
        )
    }

    Ok(games)
}

fn main() {
    let conn = connect_to_database("./../sqlite/chess_timers.db");
    let conn2 = match conn {
        Ok(connection) => connection,
        Err(error) => panic!("Failed to connect to DB, error: {:}", error),
    };

    //    create_new_game(&conn2);
    create_new_game_id();
    match get_games(conn2) {
        Ok(game) => println!("Got games"),
        Err(error) => println!("Failed to get games: {:}", error),
    }
}

fn create_new_game(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO games (game_id, player_1) VALUES (?1, ?2)",
        params!["game2", "Aaron"], // Bind parameters
    )?;

    println!("User inserted successfully.");
    Ok(())
}

fn create_new_game_id() -> String {
    let mut new_game_id: String = generate_random_string(6);
    println!("new id is: {:}", new_game_id);
    new_game_id
}

//Taken from external site
fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn connect_to_database(db_file_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_file_path);
    conn
}
