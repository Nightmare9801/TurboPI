/*fn multiply(x: u128, y: u128) -> u128 {
    return 0;
}*/
#[inline]
pub fn get_no_of_digits(x: u128) -> u128 {
    let mut p: u128= 10;
    for i in 0..39{
        if x<p {
            return i+1;
        }
        p*=10;
    }
    return 39;
}