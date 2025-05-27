// hw05.rs - Greatest Common Divisor (НСД - Найбільший спільний дільник)

fn main() {
    // Тестові приклади
    let test_pairs = vec![
        (48, 18),
        (54, 24),
        (101, 13),
        (1071, 462),
        (270, 192),
        (17, 19),  // взаємно прості числа
        (100, 25),
        (0, 5),    // один з аргументів - 0
        (7, 0),    // один з аргументів - 0
    ];

    println!("=== Greatest Common Divisor Calculator ===\n");

    for (a, b) in test_pairs {
        let result_iterative = gcd_iterative(a, b);
        let result_recursive = gcd_recursive(a, b);
        
        println!("GCD({}, {}) = {} (iterative) = {} (recursive)", 
                 a, b, result_iterative, result_recursive);
        
        // Перевірка що обидва методи дають однаковий результат
        assert_eq!(result_iterative, result_recursive);
    }

    println!("\n=== Додаткові тести ===");
    
    // Тест з великими числами
    let large_a = 123456789;
    let large_b = 987654321;
    println!("GCD({}, {}) = {}", large_a, large_b, gcd_iterative(large_a, large_b));
    
    // Тест з однаковими числами
    println!("GCD(42, 42) = {}", gcd_iterative(42, 42));
    
    // Демонстрація додаткових функцій
    println!("\n=== Розширені функції ===");
    let (gcd, x, y) = extended_gcd(48, 18);
    println!("Extended GCD(48, 18): gcd={}, 48*{}+18*{}={}", gcd, x, y, 48*x + 18*y);
    
    println!("GCD(12, 8, 4) = {}", gcd_three(12, 8, 4));
    println!("GCD([48, 18, 24, 12]) = {}", gcd_array(&[48, 18, 24, 12]));
}

// Ітеративна реалізація алгоритму Евкліда
fn gcd_iterative(mut a: u64, mut b: u64) -> u64 {
    // Обробка випадку коли один з аргументів дорівнює 0
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    // Класичний алгоритм Евкліда
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    a
}

// Рекурсивна реалізація алгоритму Евкліда
fn gcd_recursive(a: u64, b: u64) -> u64 {
    // Базові випадки
    if b == 0 {
        return a;
    }
    if a == 0 {
        return b;
    }
    
    // Рекурсивний виклик
    gcd_recursive(b, a % b)
}

// Розширений алгоритм Евкліда - знаходить НСД та коефіцієнти x, y
// такі що: gcd(a, b) = a*x + b*y
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    
    let (gcd, x1, y1) = extended_gcd(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    
    (gcd, x, y)
}

// Функція для знаходження НСД трьох чисел
fn gcd_three(a: u64, b: u64, c: u64) -> u64 {
    gcd_iterative(gcd_iterative(a, b), c)
}

// Функція для знаходження НСД масиву чисел
fn gcd_array(numbers: &[u64]) -> u64 {
    if numbers.is_empty() {
        return 0;
    }
    
    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = gcd_iterative(result, num);
        if result == 1 {
            break; // Якщо НСД дорівнює 1, то далі він не зміниться
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd_iterative(48, 18), 6);
        assert_eq!(gcd_recursive(48, 18), 6);
        assert_eq!(gcd_iterative(54, 24), 6);
        assert_eq!(gcd_iterative(101, 13), 1);
        assert_eq!(gcd_iterative(1071, 462), 21);
    }

    #[test]
    fn test_gcd_edge_cases() {
        assert_eq!(gcd_iterative(0, 5), 5);
        assert_eq!(gcd_iterative(7, 0), 7);
        assert_eq!(gcd_iterative(0, 0), 0);
        assert_eq!(gcd_iterative(42, 42), 42);
    }

    #[test]
    fn test_gcd_coprime() {
        assert_eq!(gcd_iterative(17, 19), 1); // взаємно прості
        assert_eq!(gcd_iterative(25, 49), 1); // взаємно прості
    }

    #[test]
    fn test_extended_gcd() {
        let (gcd, x, y) = extended_gcd(48, 18);
        assert_eq!(gcd, 6);
        assert_eq!(48 * x + 18 * y, gcd); // Перевірка рівняння Безу
    }

    #[test]
    fn test_gcd_three() {
        assert_eq!(gcd_three(48, 18, 24), 6);
        assert_eq!(gcd_three(12, 8, 16), 4);
    }

    #[test]
    fn test_gcd_array() {
        assert_eq!(gcd_array(&[48, 18, 24]), 6);
        assert_eq!(gcd_array(&[12, 8, 16, 4]), 4);
        assert_eq!(gcd_array(&[17, 19, 23]), 1);
        assert_eq!(gcd_array(&[]), 0);
        assert_eq!(gcd_array(&[42]), 42);
    }
}

// === Greatest Common Divisor Calculator ===

// GCD(48, 18) = 6 (iterative) = 6 (recursive)
// GCD(54, 24) = 6 (iterative) = 6 (recursive)
// GCD(101, 13) = 1 (iterative) = 1 (recursive)
// GCD(1071, 462) = 21 (iterative) = 21 (recursive)
// GCD(270, 192) = 6 (iterative) = 6 (recursive)
// GCD(17, 19) = 1 (iterative) = 1 (recursive)
// GCD(100, 25) = 25 (iterative) = 25 (recursive)
// GCD(0, 5) = 5 (iterative) = 5 (recursive)
// GCD(7, 0) = 7 (iterative) = 7 (recursive)

// === Додаткові тести ===
// GCD(123456789, 987654321) = 9
// GCD(42, 42) = 42

// === Розширені функції ===
// Extended GCD(48, 18): gcd=6, 48*-1+18*3=6
// GCD(12, 8, 4) = 4
// GCD([48, 18, 24, 12]) = 6

// === Code Execution Successful ===