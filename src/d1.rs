use std::io::Stdin;

pub fn d1(input: Stdin) {
  let mut max: u32 = 0;
  let mut calories: u32 = 0;
  for line in input.lines() {
    let line = line.unwrap();
    if line == "" {
      if calories > max {
        max = calories;
      }
      calories = 0;
    } else {
      calories += line.parse::<u32>().unwrap();
    }
  }
  println!("{max}");
}

pub fn d1_2(input: Stdin) {
  let mut max: [u32; 3] = [0; 3];
  let mut calories: u32 = 0;
  for line in input.lines() {
    let line = line.unwrap();
    if line == "" {
      let mut curr = calories;
      for (i, m) in max.into_iter().enumerate() {
        if curr > m {
          let tmp = max[i];
          max[i] = curr;
          curr = tmp;
        }
      }
      // dbg!(&max);
      calories = 0;
    } else {
      calories += line.parse::<u32>().unwrap();
    }
  }
  let answer = max.into_iter().reduce(|x, y| x + y).unwrap();
  println!("{answer}");
}
