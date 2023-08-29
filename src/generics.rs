struct Point<T> {
    _x: T,
    _y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointAnother<T, U> {
    fn mix_up<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    // Generics使用
    let number_list = vec![24, 25, 100, 56, 99];
    let char_list = vec!['z', 'b', 'a', 'r'];
    println!("{}", largest(number_list));
    println!("{}", largest(char_list));

    // 構造体でジェネリックすを使用
    //x, yで同じ型しか使えない
    let _p1 = Point { _x: 1, _y: 2 };
    let _p2 = Point { _x: 1.5, _y: 6.3 };

    // 違う型を使用できる(同じ型でも可)
    let p3 = PointAnother { x: "Rust", y: 1 };
    let p4 = PointAnother { x: 10.1, y: 10 };
    let _p5 = PointAnother { x: 1, y: 3 };
    let p6 = p3.mix_up(p4);
    println!("{} {}", p6.x, p6.y);

    // 復習(参照と借用)
    // 参照は所有権のある型とそれ以外のプリミティブ型でも同様に使える概念
    // 借用は所有権の持つ型でのみ持つ、借用の例) ある変数の参照を、関数の引数に渡す
}

fn _largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ジェネリックス...複数の型で使用
// PartialOrd + Copy...trait境界
// PartialOrd...比較演算子が使用できる型 Copy...コピーできる型
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
