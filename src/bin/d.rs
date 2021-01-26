use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n]
    }

    a.sort();

    let mut s = vec![0i64; n];

    for i in 0..n{
        if i == 0{
            s[i] = a[i];
        }
        else {
            s[i] = s[i-1] + a[i];
        }
    }

    let mut ans=0_i64;

    for j in 1..n{
        ans += j as i64 * a[j] - s[j-1];
    }

    println!("{}", ans)
}