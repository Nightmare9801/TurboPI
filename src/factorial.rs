/*fn factorial(x: i128)->i128{
    let mut f: i128 = 1;
    for i in 1..x - 1{
        f = f * (oddprod(3, 2^(i + 1) - 1));
    }
    return 2^(2^x - 1) * f;
}

fn oddprod(l: i128, h: i128)->i128{
    let mut p: i128 = 1;
    let mut ml: i128 = if l % 2 > 0 {l}else{ l + 1};
    let mh: i128 = if h % 2 > 0 {h} else {h - 1};
    while ml <= mh{
        p = p * ml;
        ml = ml + 2;
    }
    return p;
}*/