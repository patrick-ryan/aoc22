use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// struct ResultGuide {
//     loss_conditions: Vec<(String, String)>,
//     draw_conditions: Vec<(String, String)>,
//     win_conditions: Vec<(String, String)>,
// }

struct ScoreGuide {
    loss: i32,
    draw: i32,
    win: i32,

    rock: i32,
    paper: i32,
    scissors: i32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // let path = Path::new("src/02rs/ex.in.txt");
    let path = Path::new("src/02rs/in.txt");

    // let result_guide: ResultGuide = ResultGuide {
    //     loss_conditions: vec![
    //         (String::from("A"), String::from("Z")),
    //         (String::from("B"), String::from("X")),
    //         (String::from("C"), String::from("Y")),
    //     ],
    //     draw_conditions: vec![
    //         (String::from("A"), String::from("X")),
    //         (String::from("B"), String::from("Y")),
    //         (String::from("C"), String::from("Z")),
    //     ],
    //     win_conditions: vec![
    //         (String::from("A"), String::from("Y")),
    //         (String::from("B"), String::from("Z")),
    //         (String::from("C"), String::from("X")),
    //     ],
    // };

    let score_guide: ScoreGuide = ScoreGuide {
        loss: 0,
        draw: 3,
        win: 6,
        rock: 1,
        paper: 2,
        scissors: 3,
    };

    let mut rps_score: i32 = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    continue;
                } else {
                    // let (opp_choice, my_choice) = ip.split(' ').next_tuple().unwrap();
                    // let choices = (opp_choice.to_string(), my_choice.to_string());

                    // let mut result_score = 0;
                    // if result_guide.loss_conditions.contains(&choices) {
                    //     result_score += score_guide.loss;
                    // } else if result_guide.draw_conditions.contains(&choices) {
                    //     result_score += score_guide.draw;
                    // } else if result_guide.win_conditions.contains(&choices) {
                    //     result_score += score_guide.win;
                    // }

                    let (opp_choice, result) = ip.split(' ').next_tuple().unwrap();

                    let mut result_score = 0;
                    let my_choice;
                    match result {
                        "X" => {
                            result_score += score_guide.loss;

                            my_choice = match opp_choice {
                                "A" => "Z",
                                "B" => "X",
                                "C" => "Y",
                                _ => "",
                            };
                        }
                        "Y" => {
                            result_score += score_guide.draw;

                            my_choice = match opp_choice {
                                "A" => "X",
                                "B" => "Y",
                                "C" => "Z",
                                _ => "",
                            };
                        }
                        "Z" => {
                            result_score += score_guide.win;

                            my_choice = match opp_choice {
                                "A" => "Y",
                                "B" => "Z",
                                "C" => "X",
                                _ => "",
                            };
                        }
                        _ => {
                            my_choice = "";
                        }
                    };

                    let choice_score = match my_choice {
                        "X" => score_guide.rock,
                        "Y" => score_guide.paper,
                        "Z" => score_guide.scissors,
                        _ => 0,
                    };

                    rps_score += result_score + choice_score;
                }
            }
        }

        println!("Total score is: {}", rps_score);
    }
}
