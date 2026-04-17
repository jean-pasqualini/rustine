pub mod binary {
    pub fn demo1(n: i32) -> String{
        format!("{:b}", n)
    }

    pub fn demo2(n: i32) -> String{
        let mut n = n;
        if n == 0 {
            return "0".to_string();
        }
        let mut result = String::new();

        while n > 0 {
            let right = n % 2;
            result.push(if right == 0 { '0' } else { '1' });
            let left = n / 2;
            println!(">>> {left}:{right}");
            n = left;
        }

        result.chars().rev().collect()
    }

    pub fn demo3(n: i32) -> String {
        if n == 0 {
            return "0".into();
        }

        let mut n = n;
        let mut bits = Vec::new();
        while n > 0 {
            bits.push(n % 2);
            n /= 2;
        }
        bits.reverse();
        bits.into_iter().map(|bit| if bit == 0 { '0' } else { '1' }).collect()
    }

    pub fn demo4(n: i32) -> String {
        if n == 0 {
            return "0".into();
        }

        let mut n = n;
        let mut result = String::new();
        while n > 0 {
            result.push(if n & 1 == 1 { '1' } else { '0' });
            n >>= 1;
        }

        result.chars().rev().collect()
    }
}

pub mod hexa {
    pub fn demo1(n: i32) -> String {
        format!("{:X}", n)
    }

    pub fn demo2(n: i32) -> String {
        let digits = "0123456789ABCDEF".chars().collect::<Vec<char>>();
        let mut result = String::new();

        if n == 0 {
            return "0".into();
        }

        let mut n = n;

        while n > 0 {
            let r = n % 16;
            (result).push(digits[r as usize]);
            n /= 16;
        }

        result.chars().rev().collect()
    }

    pub fn demo3(n: i32) -> String {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        let mut result = Vec::new();

        if n == 0 {
            return "0".into();
        }

        let mut n = n;
        while n > 0 {
            result.push(HEX[(n % 16) as usize]);
            n /= 16;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}