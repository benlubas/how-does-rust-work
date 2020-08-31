pub fn run() {
  // There's a staircase with N steps, and you can climb 1 or 2 steps at a time.
  // Given N, write a function that returns the number of unique ways you can
  // climb the staircase. The order of the steps matters.

  fn step(steps: &Vec<i32>, n: i32, step_lengths: &Vec<i32>) {
    let step_sum = sum(steps);
    if step_sum == n {
      println!("{:?}", steps)
    } else {
      for s in step_lengths {
        if step_sum + s <= n {
          let mut nv = steps.clone();
          nv.push(*s);
          step(&nv, n, step_lengths);
        }
      }
    }
  }

  step(&vec![], 5, &vec![1, 2, 5]);

  fn sum(v: &Vec<i32>) -> i32 {
    let mut total = 0;
    for i in v {
      total += i
    }
    total
  }

  // this is really messy.
}
