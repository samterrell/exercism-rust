use std::ops::Range;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let ylen = minefield.len();
    if ylen == 0 {
        return vec![];
    }
    let xlen = minefield[0].len();
    let mut out = vec![];
    for y in 0..ylen {
        let mut r = String::with_capacity(xlen);
        for x in 0..xlen {
            if b'*' == minefield[y].as_bytes()[x] {
                r.push('*');
            } else {
                let mut v = 0u8;
                for x1 in clamped_neighbors(x, xlen) {
                    for y1 in clamped_neighbors(y, ylen) {
                        if minefield[y1].as_bytes()[x1] == b'*' {
                            v += 1;
                        }
                    }
                }
                if v > 0 {
                    r.push((v + b'0') as char);
                } else {
                    r.push(' ')
                }
            }
        }
        out.push(r);
    }
    out
}

fn clamped_neighbors(v: usize, max: usize) -> Range<usize> {
    let low = if v > 0 { v - 1 } else { v };
    let high = if v < max - 1 { v + 1 } else { v };
    low..(high + 1)
}
