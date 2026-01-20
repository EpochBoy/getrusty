// Define your types
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Pending(String),  // with data
}

#[derive(Debug)]
struct Player {
    name: String,
    score: i32,
    status: Status,
}

fn main() {
    // Array of Vecs of Structs containing Enums
    let teams: [Vec<Player>; 2] = [
        vec![
            Player { 
                name: "Alice".into(), 
                score: 100, 
                status: Status::Active 
            },
            Player { 
                name: "Bob".into(), 
                score: 85, 
                status: Status::Pending("review".into()) 
            },
        ],
        vec![
            Player { 
                name: "Charlie".into(), 
                score: 90, 
                status: Status::Inactive 
            },
        ],
    ];

    // Access nested data
    println!("{}", teams[0][0].name);           // Alice
    println!("{:?}", teams[0][1].status);       // Pending("review")
    
    // Iterate
    for (i, team) in teams.iter().enumerate() {
        println!("Team {i}:");
        for player in team {
            println!("  {} - {:?}", player.name, player.status);
        }
    }
}