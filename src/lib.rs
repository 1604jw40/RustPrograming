#![allow(dead_code, unused_variables)]

/* =========================
   Control Flow 실습 모듈
   ========================= */
pub mod control_flow {
    // 1) if / else (표현식)
    pub fn if_else_result(x: i32) -> &'static str {
        if x % 2 == 0 { "even" } else { "odd" }
    }

    // 2) loop (값 반환)
    pub fn loop_sum_1_to_n(n: u32) -> u32 {
        let mut i = 0;
        let mut acc = 0;
        let result = loop {
            i += 1;
            acc += i;
            if i == n {
                break acc; // loop는 값 반환
            }
        };
        result
    }

    // 3) while
    pub fn countdown(mut n: i32) -> Vec<i32> {
        let mut trace = Vec::new();
        while n > 0 {
            trace.push(n);
            n -= 1;
        }
        trace.push(0); // 발사 직전 0 기록
        trace
    }

    // 4) for (이터레이터 기반)
    pub fn for_collect_range(start: i32, end_inclusive: i32) -> Vec<i32> {
        let mut out = Vec::new();
        for v in start..=end_inclusive {
            out.push(v);
        }
        out
    }

    // 5) match (패턴 매칭)
    pub fn classify_1_to_10(x: i32) -> &'static str {
        match x {
            1..=5 => "small",
            6..=10 => "medium",
            _ => "large_or_out",
        }
    }

    // 추가: 역순, enumerate 실습
    pub fn reverse_and_index(v: &[i32]) -> Vec<(usize, i32)> {
        let mut out = Vec::new();
        for (idx, val) in v.iter().rev().enumerate() {
            out.push((idx, *val));
        }
        out
    }

    /* =========================
       테스트 (각 예제 독립 실행)
       ========================= */
    #[cfg(test)]
    pub mod tests {
        use super::*;

        #[test]
        fn if_else_result_works() {
            println!("if_else_result(10) => {}", if_else_result(10));
            assert_eq!(if_else_result(10), "even");
            assert_eq!(if_else_result(7), "odd");
        }

        #[test]
        fn loop_sum_works() {
            let s = loop_sum_1_to_n(5);
            println!("sum 1..=5 => {s}");
            assert_eq!(s, 15);
        }

        #[test]
        fn countdown_works() {
            let t = countdown(3);
            println!("countdown(3) => {:?}", t);
            assert_eq!(t, vec![3, 2, 1, 0]);
        }

        #[test]
        fn for_collect_range_works() {
            let v = for_collect_range(-2, 2);
            println!("range -2..=2 => {:?}", v);
            assert_eq!(v, vec![-2, -1, 0, 1, 2]);
        }

        #[test]
        fn match_classify_works() {
            assert_eq!(classify_1_to_10(2), "small");
            assert_eq!(classify_1_to_10(7), "medium");
            assert_eq!(classify_1_to_10(42), "large_or_out");
        }

        #[test]
        fn reverse_and_index_works() {
            let out = reverse_and_index(&[10, 20, 30]);
            println!("reverse_and_index => {:?}", out);
            // rev() 후 enumerate이기 때문에 (0, 30), (1, 20), (2, 10)
            assert_eq!(out, vec![(0, 30), (1, 20), (2, 10)]);
        }
    }
}
