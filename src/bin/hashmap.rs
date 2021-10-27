use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("{}", s),
        None => println!("team not exist"),
    }

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    //修改
    let text = "x x y y z";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
