fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

/// u32.into(), i16 -> not allowed since u32 is too wide for i16.
/// bool.into(), i16 -> allowed since bool fits into i16 (0 or 1). true * 1000 = 1000.
/// u16.into(), i16 -> not allowed since some u16 values don't fit in i16.
/// &i16.into(), not allowed since &u16 does not implement Into<i16>.
/// i16.into(), allowed but warning: useless conversion to the same type: `i16`
fn main() {
    let x: i16 = 1000;
    let y: i16= 1;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}