use rand::Rng;
use rusqlite::{Connection, OpenFlags, Result, params}; // For database operations and result handling

#[derive(Debug)]
struct Game {
    id: String,
    player_1_id: Option<i32>,
    player_2_id: Option<i32>,
    status: Option<String>,
}

#[derive(Debug)]
struct Player {
    id: Option<i32>,
    name: Option<String>,
}

fn get_games(conn: &Connection) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare("SELECT id, player_1_id, player_2_id, status FROM Game")?;
    let rows = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            player_1_id: row.get(1)?,
            player_2_id: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    let mut games = Vec::new();
    for game in rows {
        games.push(game?);
    }

    for game in &games {
        println!(
            "Game info: {:?}, {:?}, {:?}, {:?}",
            game.id, game.player_1_id, game.player_2_id, game.status
        )
    }

    Ok(games)
}

fn get_players(conn: &Connection) -> Result<Vec<Player>> {
    let mut stmt = conn.prepare("SELECT id, name FROM Player")?;
    let rows = stmt.query_map([], |row| {
        Ok(Player {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut players = Vec::new();
    for player in rows {
        players.push(player?);
    }

    for player in &players {
        println!("Player Info: {:?}, {:?}", player.id, player.name)
    }

    Ok(players)
}

fn main() {
    let conn = connect_to_database("./sqlite/chess_timers.db");

    let conn2 = match conn {
        Ok(connection) => connection,
        Err(error) => panic!("Failed to connect to DB, error: {:}", error),
    };

    let player = Player {
        id: None,
        name: Some(String::from("Aaron10")),
    };

    create_new_game(&conn2, &player);

    match get_games(&conn2) {
        Ok(game) => println!("Got games"),
        Err(error) => println!("Failed to get games: {:}", error),
    }
    match get_players(&conn2) {
        Ok(player) => println!("Got players"),
        Err(error) => println!("Failed to get players: {:}", error),
    }
}

fn create_new_player(player: &Player, conn: &Connection) -> Result<i32> {
    let mut highest_player_id = get_new_player_id(&conn)?;
    highest_player_id += 1;

    conn.execute(
        "INSERT INTO Player (id, name) VALUES (?1, ?2)",
        params![highest_player_id, player.name],
    )?;

    println!("Player inserted good");
    Ok(highest_player_id)
}

fn get_new_player_id(conn: &Connection) -> Result<i32> {
    //Should use query_one method here, but not sure how to lol
    conn.query_row_and_then("SELECT max(id) FROM Player", [], |row| row.get(0))
}

fn create_new_game(conn: &Connection, player_1: &Player) -> Result<()> {
    let player_1_id = create_new_player(&player_1, &conn)?;

    let new_random_game_id = create_new_game_id();

    conn.execute(
        "INSERT INTO Game (id, player_1_id, status, created_timestamp) VALUES (?1, ?2, ?3, ?4)",
        params![new_random_game_id, player_1_id, "CREATED", "TODAY"], // Bind parameters
    )?;

    println!("Game inserted successfully.");
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
    let conn = Connection::open_with_flags(
        db_file_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    );

    conn
}
