pub fn factors(num: u64) -> Vec<u64> {
    if num < 2 {
        return vec![];
    }
    let limit: u64 = ((num as f64).sqrt().ceil()) as u64;
    let mut res = vec![];
    let mut number = num;
    for n in 2..limit + 1 {
        while is_prime(n) && number % n == 0 {
            number = number / n;
            res.push(n);
        }
    }
    if is_prime(number) {
        res.push(number);
    }
    res
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    } else if num == 2 {
        return true;
    } else if num % 2 == 0 {
        return false;
    } else {
        let limit: u64 = ((num as f64).sqrt().ceil()) as u64;
        for n in 2..limit + 1 {
            if num % n == 0 {
                return false;
            }
        }
        return true;
    }
}
