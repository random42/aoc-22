// use std::collections::*;
use std::cmp;
use std::cmp::Ordering::*;
use std::io::Stdin;

#[derive(Debug, cmp::PartialEq, cmp::Eq, cmp::PartialOrd)]
enum Choice {
  Rock,
  Paper,
  Scissors,
}

impl cmp::Ord for Choice {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    match self {
      Choice::Rock => match other {
        Choice::Rock => Equal,
        Choice::Paper => Less,
        Choice::Scissors => Greater,
      },
      Choice::Paper => match other {
        Choice::Rock => Greater,
        Choice::Paper => Equal,
        Choice::Scissors => Less,
      },
      Choice::Scissors => match other {
        Choice::Rock => Less,
        Choice::Paper => Greater,
        Choice::Scissors => Equal,
      },
    }
  }
}

pub fn d2(stdin: Stdin) {
  let mut score: u32 = 0;
  for line in stdin.lines() {
    let line = line.unwrap();
    let mut split = line.split_whitespace().into_iter();
    let enemy = split.next().unwrap();
    let me = split.next().unwrap();
    assert!(split.next().is_none());
    let enemy: Choice = match enemy {
      "A" => Choice::Rock,
      "B" => Choice::Paper,
      "C" => Choice::Scissors,
      other => {
        panic!("Invalid enemy choice: {other}");
      }
    };
    let me: Choice = match me {
      "X" => Choice::Rock,
      "Y" => Choice::Paper,
      "Z" => Choice::Scissors,
      other => {
        panic!("Invalid own choice: {other}");
      }
    };
    // dbg!(&enemy, &me);
    let choice_score: u32 = match me {
      Choice::Rock => 1,
      Choice::Paper => 2,
      Choice::Scissors => 3,
    };
    let result_score: u32 = match me.cmp(&enemy) {
      Less => 0,
      Equal => 3,
      Greater => 6,
    };
    // dbg!(choice_score, result_score);
    score += choice_score + result_score;
  }
  println!("{score}");
}
