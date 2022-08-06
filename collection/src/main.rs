use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    let v2 = vec![1,2,3,4,5];

    let third = &v2[2];
    println!("{}", third);

    match v2.get(2) {
        Some(third) => println!("Value: {}", third),
        None => println!("No value"),
    }

    // 인덱스 없을 때
    // 인덱스 방법 : 패닉 방생
    // get  방법 : None

    let mut v3 = v2.clone();

    for num in &mut v3 {
        println!("{}", num);
        *num += 50
    }

    println!("{:?}", v3);

    let mut s1 = String::from("hello");
    let s2 = " rust!";
    s1.push_str(s2);
    println!("{} {}", s1, s2);
    println!("{}", format!("{}{}", s1, s2));

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);

    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10, 40];

    let mut initial_team_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let x = &String::from("yellow");
    initial_team_scores.entry(x).or_insert(&100);
    println!("{:?}", initial_team_scores)
}
