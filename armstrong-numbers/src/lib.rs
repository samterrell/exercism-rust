pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let mut acc = num;
    let len = digits.len() as u32;
    for digit in digits {
        let v = digit.pow(len);
        if v > acc {
            return false;
        } else {
            acc -= v;
        }
    }
    acc == 0
}

fn digits(num: u32) -> Vec<u32> {
    if num == 0 {
        return vec![0];
    }
    let mut res = vec![];
    let mut acc = num;
    while acc > 0 {
        res.push(acc % 10);
        acc /= 10;
    }
    res
}
