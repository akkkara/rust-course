fn main() {
    // 1. Простий тест: перевірка паліндрома
    let test_numbers = vec![121, -121, 10, 12321];
    println!("1. Проверка is_palindrome_optimized:");
    for num in &test_numbers {
        println!("  {} → {}", num, is_palindrome_optimized(*num));
    }

    // 2. Порівненнія різних реалізацій
    println!("\n2. Сравнение is_palindrome_string vs is_palindrome_math:");
    for num in &test_numbers {
        println!(
            "  {} → string: {}, math: {}",
            num,
            is_palindrome_string(*num),
            is_palindrome_math(*num)
        );
    }

    // 3. Перевірка в інших системах счисления
    println!("\n3. Палиндром в системах счисления (2 и 10):");
    for num in &test_numbers {
        println!(
            "  {} → base2: {}, base10: {}",
            num,
            is_palindrome_base(*num as u64, 2),
            is_palindrome_base(*num as u64, 10)
        );
    }

    // 4. Наступний та минулий паліндроми
    println!("\n4. Следующий и предыдущий палиндром:");
    for num in &test_numbers {
        let next = next_palindrome(*num);
        let prev = prev_palindrome(*num);
        println!("  {} → пред: {}, след: {}", num, prev, next);
    }

    // 5. Паліндромы в диапазоні
    println!("\n5. Палиндромы в диапазоне от 100 до 150:");
    let palindromes = find_palindromes_in_range(100, 150);
    println!("  {:?}", palindromes);

    // 6. Генерація паліндромов заданої довжини
    println!("\n6. Генерация палиндромов с 3 цифрами:");
    for i in 100..1000 {
        if let Some(pal) = generate_palindrome(3, i) {
            if is_palindrome_optimized(pal) {
                println!("  {}", pal);
            }
        }
    }

    // 7. Тест продуктивності
    println!("\n7. Тест производительности на больших числах:");
    let large_numbers = vec![1234554321_i64, 12345654321, 123456654321];
    for num in large_numbers {
        let start = std::time::Instant::now();
        let result = is_palindrome_optimized(num);
        let duration = start.elapsed();
        println!(
            "  {} → {} (время: {:?})",
            num, result, duration
        );
    }

    // 8. Палиідроми в великому диапазоні
    println!("\n8. Палиндромы от 100000 до 100200:");
    let palindromes = find_palindromes_in_range(100000, 100200);
    println!("  {:?}", palindromes);
}

// Оптимізована переврка без строк
fn is_palindrome_optimized(mut x: i64) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut reverted = 0;
    while x > reverted {
        reverted = reverted * 10 + x % 10;
        x /= 10;
    }

    x == reverted || x == reverted / 10
}

// Перевірка через строку
fn is_palindrome_string(x: i64) -> bool {
    let s = x.to_string();
    s.chars().rev().collect::<String>() == s
}

// Математична перевірка
fn is_palindrome_math(mut x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let original = x;
    let mut reversed = 0;

    while x > 0 {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }

    original == reversed
}

// Перевірка в довільній системі счисления
fn is_palindrome_base(mut num: u64, base: u32) -> bool {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push(num % base as u64);
        num /= base as u64;
    }

    let len = digits.len();
    for i in 0..len / 2 {
        if digits[i] != digits[len - 1 - i] {
            return false;
        }
    }
    true
}

// Наступний паліндром
fn next_palindrome(mut n: i64) -> i64 {
    loop {
        n += 1;
        if is_palindrome_optimized(n) {
            return n;
        }
    }
}

// Минулий паліндром
fn prev_palindrome(mut n: i64) -> i64 {
    loop {
        if n == 0 {
            return 0;
        }
        n -= 1;
        if is_palindrome_optimized(n) {
            return n;
        }
    }
}

// Все палиндромы в диапазоне
fn find_palindromes_in_range(start: i64, end: i64) -> Vec<i64> {
    (start..=end).filter(|&x| is_palindrome_optimized(x)).collect()
}

// Генерация палиндрома по числу и количеству цифр
fn generate_palindrome(digits: u32, seed: i64) -> Option<i64> {
    let s = seed.to_string();
    if s.len() != digits as usize {
        return None;
    }

    let reversed = s.chars().rev().collect::<String>();
    let pal = format!("{}{}", s, reversed);
    pal.parse::<i64>().ok()
}


// 1. Перевірка is_palindrome_optimized:
//   121 → true
//   -121 → false
//   10 → false
//   12321 → true

// 2. Порівнення is_palindrome_string vs is_palindrome_math:
//   121 → string: true, math: true
//   -121 → string: false, math: false
//   10 → string: false, math: false
//   12321 → string: true, math: true

// 3. Паліндром в системах счисления (2 и 10):
//   121 → base2: false, base10: true
//   -121 → base2: false, base10: false
//   10 → base2: false, base10: false
//   12321 → base2: false, base10: true

// 4. Наступний і минулий паліндром:
//   121 → мин: 111, наст: 131