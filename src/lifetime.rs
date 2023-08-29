pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    // 重要:実体のライフタイムが異なる場合
    let st3 = String::from("x");
    {
        let st4 = String::from("y");
        let res2 = get_longest(&st3, &st4); // res2にはst４(ライフタイムが短い)の参照ライフタイムとなる
        println!("{}", res2);
    }
}

// <'a>によって、ライフタイムが一番短いものに合わせて、リファレンスもドロップされる
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
