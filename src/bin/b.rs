use proconio::input;

fn main() {
    input! {
      (h, w): (i32, i32),
      a: [[i32; w]; h]
    }

    let sum = a.clone()
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>()
        .iter()
        .fold(0, |acc, &x| acc + x);

    let min = a.clone()
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>()
        .into_iter()
        .fold(10000000,|acc, x| std::cmp::min(acc, x));

    println!("{}", sum - min * h * w)
}
