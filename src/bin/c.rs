use proconio::input;

fn main() {
    input!{
        n: i32
    }

    let mut ans = 0;

    for x in 1..=n{
        if check7(x,8) && check7(x,10){
            ans += 1;
        }
    }

    println!("{}", ans)
}


// 各位に7が入っていた場合、falseを返します
fn check7(x: i32, divide: i32) -> bool {
    let _mod = 7;

    if x % divide == _mod {
        return false
    }
    if x / divide == 0 {
        return true
    }
    return check7(x / divide, divide);
}
