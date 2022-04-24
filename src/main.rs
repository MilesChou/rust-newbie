use std::env;

fn main() {
    let arg = env::args().nth(1).expect("no score given");

    let score = arg.parse::<i32>().ok().expect("I wasn't given an integer!");

    println!("{}", grade(score));
}

fn grade(score: i32) -> &'static str {
    if score < 0 || score > 100 {
        panic!("error range");
    }

    if score >= 90 {
        return "考試優秀"
    }

    if score >= 80 {
        return "考試良好"
    }

    if score >= 60 {
        return "考試及格"
    }

    "下次繼續努力"
}