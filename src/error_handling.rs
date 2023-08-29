pub fn run() {
    let res1 = division_option(5.0, 0.0);
    match res1 {
        Some(x) => println!("{:.3}", x),
        None => println!("none"),
    }
    let res2 = division_result(5.0, 1.0);
    match res2 {
        Ok(x) => println!("{:.3}", x),
        Err(e) => println!("{}", e),
    }

    // 3つ値があるから0 + 1 + 2 = 3が出力される
    let a = [0, 1, 2];
    let sum = sum(&a);
    match sum {
        Some(x) => println!("{}", x),
        None => println!("none"),
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Error"))
    } else {
        Ok(x / y)
    }
}

// ?で値がない場合は、すぐに処理から抜ける(Optionを返り値とする場合、Noneを返す)
// 今回の場合、値が3つ以上ある場合はSomeで値を返す、それ以外はNoneとなる
fn sum(a: &[i32]) -> Option<i32> {
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
