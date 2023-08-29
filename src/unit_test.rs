struct _Rectangle {
    width: u32,
    height: u32,
}
impl _Rectangle {
    fn _compare_area(&self, other: &_Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn _double_value(a: i32) -> i32 {
    a * 2
}

fn _greeting(s: &str) -> String {
    format!("unchi{}", s)
}

// #[cfg(test)] build時に、このテストモジュールはコンパイルされない
// unit_test内にtestsというサブモジュールを作成(階層が一つ下)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a_is_larger() {
        let a = _Rectangle {
            width: 5,
            height: 5,
        };
        let b = _Rectangle {
            width: 2,
            height: 2,
        };
        // assert!はtrueが帰ってきたらpassする
        assert!(a._compare_area(&b));
    }

    // trueではなく、falseなのでpassされない(FAILEDがコンソールに表示される)
    #[test]
    fn test_a_is_smaller() {
        let a = _Rectangle {
            width: 5,
            height: 5,
        };
        let b = _Rectangle {
            width: 10,
            height: 10,
        };
        assert!(a._compare_area(&b));
    }

    #[test]
    fn test_double() {
        // _eqは値が等しいとpass
        assert_eq!(6, _double_value(3));
    }

    #[test]
    fn test_contains_name() {
        let res = _greeting("rust");
        assert!(res.contains("unchi"));
    }
}
