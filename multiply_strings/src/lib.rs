pub fn multiply(num1: String, num2: String) -> String {
    let mut nv1 : Vec<u32> = Vec::new();
    let mut nv2 : Vec<u32> = Vec::new();

    for b in num1.chars() {
        nv1.push(b.to_digit(10).unwrap());
    }

    for b in num2.chars() {
        nv2.push(b.to_digit(10).unwrap());
    }

    let last : usize = num1.len() + num2.len();
    let mut result:Vec<u32> = Vec::new();
    for _i in 0..=last {
        result.push(0);
    }

    let mut delta: usize = 0;
    for n1 in nv1.iter().rev() {
        let mut c = 0;
        let mut index = last - delta;

        for n2 in nv2.iter().rev() {
            let r = n2 * n1 + c + result[index];
            result[index] = r % 10;
            c = r / 10;
            index -= 1;
        }
        result[index] = c;
        delta += 1;
    }

    let mut skip_zero = true;
    let mut rs: String = String::new();
    for n in result {
        if skip_zero && n == 0 {
            continue;
        }
        skip_zero = false;
        rs += &n.to_string();
    }

    if rs.is_empty() {
        return "0".to_string();
    }

    return rs;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = multiply("123".to_string(), "456".to_string());
        assert_eq!(result, "56088".to_string());
    }
}
