use std::{collections::HashMap, vec};
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{}",s),
        None => println!("team not exist")
    }

    for (k, v) in &scores{
         print!("{},{}", k, v);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let numbers = vec![1,20];
    let scores1: HashMap<_, _> = teams.iter().zip(numbers).collect();
}
  