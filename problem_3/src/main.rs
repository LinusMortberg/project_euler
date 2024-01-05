fn main() { // // 13195
    let NUM: i64 = 600851475143;
    let range = (NUM as f64).sqrt() as i64;
    let mut prims = Vec::new();
    println!("range: {}", range);
    for i in 2..range {
        if NUM % i == 0 && is_prime(i.try_into().unwrap()) {            
            prims.push(i)
        }
    }
    println!("prim: {:?}", prims)
}

// blackbox.ai
fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _even if n % 2 == 0 => false,
        _ => {
            let limit = (n as f64).sqrt() as u32;
            for i in (3..=limit).step_by(2) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}