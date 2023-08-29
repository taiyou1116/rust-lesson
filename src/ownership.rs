pub fn run() {
    // 所有権, ヒープに実データが保持されるデータ型で起きる(vector, string, boxpointer)
    let s1 = String::from("hello");
    let s2 = s1; //譲渡されたタイミングでs1のメモリは解放される
    println!("{}", s2);

    // データがスタックで完結するデータ型は、コピートレートが起き、別のスタック内にコピーされる
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("{:p} {:p}", &i1, &i2);

    // 文字列スライス
    let sl1 = "hello";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("{:p} {:p}", &sl1, &sl2);
    println!("{:?} {:?}", sl1.as_ptr(), sl2.as_ptr()); //参照なので同じアドレス

    // deepcopy
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{:p} {:p}", &s3, &s4);
    println!("{:?} {:?}", s3.as_ptr(), &s4.as_ptr()); //値をコピーしたので、違うヒープの領域に値が保持される

    let s5 = String::from("hello");
    take_ownership(s5); //所有権が引数に渡される
    let s6 = String::from("hello");
    let s7 = take_giveback_ownership(s6); //所有権が引数に渡され、それが返り値に渡され、最終的にs7に渡る
    println!("{}", s7);

    // 参照を渡す
    let s8 = String::from("hello");
    let len = calsulate_length(&s8);
    println!("{} {}", s8, len); //所有権は移らないのでs8も使える

    // 参照を受け取って、値を変更する
    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    //イミュータブルな参照は複数作れる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    //イミュータブルな参照とミュータブルな参照は共存できない(下に例外あり)
    let mut _s10 = String::from("hello");
    let r1 = &_s10;
    // let r2 = &mut s10;
    println!("{} {}", _s10, r1 /*r2*/);

    // 所有権者であってもイミュータブルな参照がある期間(最後の使用されるまで)では使用できない
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1); //逆にするとエラー↓
    println!("{}", s11);

    // イミュータブルな参照とミュータブルな参照
    // どちらかが最後に使用された行以降は使用できる
    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("new_hello"); //参照はずしで実データにアクセス
    println!("{}", s12);

    // 実体はスコープを抜けるまで、参照は最後に使われた後にメモリが解放される
    // ダングリングポインタ...参照な実体よりも長生きしてはいけない
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

// sにはポインタ分(8byte)が格納され、s8の先頭アドレスにアクセスする
fn calsulate_length(s: &String) -> usize /*(64bit系だと8byte)*/ {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("world");
}
