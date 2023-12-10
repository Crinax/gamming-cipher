use rand::Rng;

// Возвращаемый результат представляет собой структуру
// gcd - НОД
// x - первый коэффициент
// y - второй коэффициент
#[derive(Debug)]
pub struct GCDResult {
    pub gcd: i128,
    pub x: i128,
    pub y: i128,
}

// Расширенный алгоритм Евклида
pub fn extend_gcd(a: i128, b: i128) -> GCDResult {
    let (mut s, mut old_s): (i128, i128) = (0, 1);
    let (mut r, mut old_r): (i128, i128) = (b, a);

    while r != 0 {
        let quotinent: i128 = old_r / r;
        (old_r, r) = (r, old_r - quotinent * r);
        (old_s, s) = (s, old_s - quotinent * s);
    }

    let bezout_t: i128;

    if b != 0 {
        bezout_t = (old_r - old_s * a) / b;
    } else {
        bezout_t = 0;
    }

    GCDResult {
        gcd: old_r,
        x: old_s,
        y: bezout_t,
    }
}

pub fn cropping_modulo(value: i128, modulo: i128) -> u128 {
    ((modulo + (value % modulo)) % modulo) as u128
}

// Функция быстрого возведения в степень по модулю
pub fn power_by_mod(base: i128, power: i128, modulus: i128) -> i128 {
    let mut result: i128 = 1;
    let mut base_copy: i128 = base;
    let mut power_copy: i128 = power;

    base_copy = base_copy % modulus;
    while power_copy > 0 {
        if power_copy & 1 == 1 {
            result = (result * base_copy) % modulus;
        }

        power_copy = power_copy >> 1;
        base_copy = (base_copy * base_copy) % modulus;
    }

    return result;
}

// Тест на простоту Миллера-Рабина
pub fn miller_test(d: i128, n: i128) -> bool {
    let r_num = rand::thread_rng().gen_range(1..(n - 4));
    let a = 2 + r_num;
    let mut d_copy = d;
    let mut x = power_by_mod(a, d_copy, n);

    if x == 1 || x == n - 1 {
        return true;
    }

    while d_copy != n - 1 {
        x = (x * x) % n;
        d_copy *= 2;

        if x == 1 {
            return false;
        }

        if x == n - 1 {
            return true;
        }
    }

    return false;
}

// Проверка, простой ли элемент
// С использованием алгоритма Миллера-Рабина
pub fn is_prime(n: i128, k: i128) -> bool {
    if n <= 1 || n == 4 {
        return false;
    }
    
    if n <= 3 {
        return true;
    }

    let mut d: i128 = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }

    for _ in 0..k {
        if !miller_test(d, n) {
            return false;
        }
    }

    return true;
}

// Получаем обратное по модулю число, где
// а - число
// b - модуль
pub fn get_modularly_inverse(a: i128, b: i128) -> Option<i128> {
    let gcd_result: GCDResult = extend_gcd(a % b, b);
    
    if gcd_result.gcd > 1 {
       return None;
    }
   
    if gcd_result.x > 0 {
        Some(gcd_result.x)
    } else {
        Some(b + gcd_result.x)
    }
}

// Функция Эйлера для простых чисел
pub fn euler_for_prime(a: i128, b: i128) -> i128 {
    (a - 1) * (b - 1)
}

pub fn find_lcm_from_gcd(gcd: i128, a: i128, b: i128) -> i128 {
    (a * b).abs() / gcd
}
