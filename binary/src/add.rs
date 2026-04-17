pub mod binary {
    pub fn demo1(left: u32, right: u32) -> String {
        format!("{:b}", left + right)
    }

    pub fn demo2(left: u32, right: u32) -> String {
        let mut left = left;
        let mut right = right;
        while right != 0 {
            let carry = left & right;
            left = left ^ right;
            right = carry << 1;
        }

        left.to_string()
    }

    pub fn demo3(left: &str, right: &str) -> String {
        let mut result = String::new();
        let mut carry = 0;

        let left = left.chars().rev().collect::<Vec<char>>();
        let right = right.chars().rev().collect::<Vec<char>>();
        let max_len = left.len().max(right.len());

        for i in 0..max_len {
            let bit_left = left.get(i).unwrap_or(&'0').to_digit(2).unwrap();
            let bit_rigt = right.get(i).unwrap_or(&'0').to_digit(2).unwrap();

            let sum = bit_left + bit_rigt + carry;
            result.push(char::from_digit(sum % 2, 10).unwrap());
            carry = sum / 2;
        }

        if carry > 0 {
            result.push('1');
        }

        result.chars().rev().collect()

    }
}