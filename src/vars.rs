pub mod sub_a;
pub mod sub_b;

const MA: u32 = 100000;

pub fn run() {
    sub_a::run();
    sub_b::run();

    // mut, _で明示的に使用しない
    let mut x = 5;
    println!("{}", x); //5
    x = 6;
    println!("{}", x); //6
    let _i1 = 3;
    let _f1 = 0.1;

    // &でアドレスを取得。:pでポインタとして取得。 -> u32(4byte)の最初のアドレスを取得
    println!("{:p}", &MA);

    // : 変数名の後に型(今回はi64)をつける
    let _i2: i64 = 1;
    let _i3: i64 = 2;

    // 同名の変数は、最終的な値以外は隠れてしまう。*同名であってもその都度メモリを確保する
    let y = 1;
    let y = y + 1;
    let y = y * 2;
    println!("{}", y); //4
    {
        let y = 100;
        println!("{}", y); //100
    }
    println!("{}", y); //4

    // タプル
    // タプルや配列などの複雑な値の取得は{:?}で受け取る
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("{} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    // 配列
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{} {:?}", a1[0], a2);

    // 文字列スライス
    //文字列リテラルはスライス型(&str)として扱う
    let s1 = "helloこんにちは挨拶"; //26byte(日本語は3byte)
    let s2 = "hello"; //5byte
    println!("{:p} {:p}", &s1, &s2); //0x7ff7b2757cd8 0x7ff7b2757ce8
    println!("{:?} {:?}", s1.as_ptr(), s2.as_ptr()); //実データを取得
    println!("{} {}", s1.len(), s2.len()); // 26 5 byte数を取得

    // 16進数から20進数の計算方法(0xは16進数であることを表す)
    // cd8 = (12 * 16^2) + (13 * 16^1) + (8 * 16^0)
    // = 3320 + 208 + 8
    // = 3536

    // ce8 = (12 * 16^2) + (14 * 16^1) + (8 * 16^0)
    // = 3320 + 224 + 8
    // = 3552
    // 差は10進数で16byte

    // String型
    // 実データはHeapに格納。
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("{:p} {:p}", &s1, &s2);
    println!("{:?} {:?}", s1.as_ptr(), s2.as_ptr());
    println!("{:?} {:?}", s1.len(), s2.len());
    println!("{:?} {:?}", s1.capacity(), s2.capacity());
    s1.push_str("_new");
    s2.push_str("_new");
    println!("{} {}", s1, s2);
}
