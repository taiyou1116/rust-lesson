enum _List {
    Node(i32, Box<_List>),
    Nil,
}

pub fn run() {
    //スタックには7mgまで
    // let _a1: [u8; 7000000] = [255; 7000000];

    // vector型
    // ポインタ分の8byteには、実データの先頭アドレスと、型の情報が含まれる
    let mut v1 = vec![1, 2, 3, 4];
    let _v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    v1.insert(1, 10); //数値
    println!("{:?}", v1);
    v1.remove(0); //index
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?} {:?}", v1, v3);

    // boxpointer型(スタック上にあるデータをヒープに格納, String型の24byte分をスタックに8byte、ヒープに24byte)
    let t1: (i64, String) = (10, String::from("hello")); // 通常のタプル
    println!("{:p}", &t1);
    println!("{:?}", t1.1.as_ptr()); // t1のhelloが格納されている先頭アドレス
    let mut b1 = Box::new(t1); //所有権がb1に渡される
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);
}
