// hw09.rs - Функція для зсуву рядка

/// Виконує циклічний зсув рядка
/// 
/// # Аргументи
/// * `s` - рядок для зсуву
/// * `n` - кількість позицій для зсуву
///   - позитивне число: зсув вправо (символи з кінця переходять на початок)
///   - негативне число: зсув вліво (символи з початку переходять в кінець)
///   - 0: рядок залишається незмінним
/// 
/// # Повертає
/// Новий рядок після зсуву
/// 
/// # Приклади
/// ```
/// assert_eq!(rotate("abcdefgh".to_string(), 1), "habcdefg");
/// assert_eq!(rotate("abcdefgh".to_string(), -1), "bcdefgha");
/// ```
pub fn rotate(s: String, n: isize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    // Якщо рядок порожній або має один символ, зсув не змінює його
    if len <= 1 {
        return s;
    }
    
    // Приводимо зсув до діапазону [0, len)
    let effective_shift = ((n % len as isize) + len as isize) % len as isize;
    
    if effective_shift == 0 {
        return s;
    }
    
    let split_point = len - effective_shift as usize;
    
    // Беремо частину з кінця і частину з початку
    let rotated_chars: Vec<char> = chars[split_point..]
        .iter()
        .chain(chars[..split_point].iter())
        .cloned()
        .collect();
    
    rotated_chars.into_iter().collect()
}

/// Альтернативна реалізація через slice операції
pub fn rotate2(s: &str, n: &isize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    if len <= 1 {
        return s.to_string();
    }
    
    let effective_shift = ((n % len as isize) + len as isize) % len as isize;
    
    if effective_shift == 0 {
        return s.to_string();
    }
    
    let split_point = len - effective_shift as usize;
    let rotated_chars: Vec<char> = chars[split_point..]
        .iter()
        .chain(chars[..split_point].iter())
        .cloned()
        .collect();
    
    rotated_chars.into_iter().collect()
}

/// Реалізація через rotate_left/rotate_right методи Vec
pub fn rotate_vec_method(s: String, n: isize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    if len <= 1 {
        return s;
    }
    
    let mut rotated_chars = chars;
    let effective_shift = ((n % len as isize) + len as isize) % len as isize;
    
    if effective_shift == 0 {
        return s;
    }
    
    // rotate_right переміщує елементи вправо на вказану кількість позицій
    rotated_chars.rotate_right(effective_shift as usize);
    rotated_chars.into_iter().collect()
}

/// Реалізація для роботи з байтами (тільки ASCII)
pub fn rotate_bytes(s: String, n: isize) -> String {
    // Перевіряємо, чи рядок містить тільки ASCII символи
    if !s.is_ascii() {
        // Якщо не ASCII, використовуємо основну функцію
        return rotate(s, n);
    }
    
    let len = s.len();
    
    if len <= 1 {
        return s;
    }
    
    let effective_shift = ((n % len as isize) + len as isize) % len as isize;
    
    if effective_shift == 0 {
        return s;
    }
    
    let bytes = s.as_bytes();
    let split_point = len - effective_shift as usize;
    
    let mut result = Vec::with_capacity(len);
    result.extend_from_slice(&bytes[split_point..]);
    result.extend_from_slice(&bytes[..split_point]);
    
    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rotation() {
        assert_eq!(rotate("abcdefgh".to_string(), 1), "habcdefg");
        assert_eq!(rotate("abcdefgh".to_string(), -1), "bcdefgha");
        assert_eq!(rotate("abcdefgh".to_string(), 0), "abcdefgh");
    }

    #[test]
    fn test_full_rotation() {
        let s = "abcdefgh";
        assert_eq!(rotate(s.to_string(), 8), s);
        assert_eq!(rotate(s.to_string(), -8), s);
        assert_eq!(rotate(s.to_string(), 16), s);
    }

    #[test]
    fn test_large_shifts() {
        assert_eq!(rotate("abcdefgh".to_string(), 10), "ghabcdef");
        assert_eq!(rotate("abcdefgh".to_string(), -10), "cdefghab");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(rotate("".to_string(), 5), "");
        assert_eq!(rotate("a".to_string(), 10), "a");
        assert_eq!(rotate("ab".to_string(), 1), "ba");
        assert_eq!(rotate("ab".to_string(), -1), "ba");
    }

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)|
                assert_eq!(
                    rotate2(s, n), 
                    exp.to_string()
                )
            );
    }

    #[test]
    fn test_all_implementations() {
        let test_cases = vec![
            ("abcdefgh", 1, "habcdefg"),
            ("abcdefgh", -1, "bcdefgha"),
            ("abcdefgh", 2, "ghabcdef"),
            ("abcdefgh", -2, "cdefghab"),
            ("hello", 2, "lohel"),
            ("hello", -2, "llohe"),
        ];

        for (input, shift, expected) in test_cases {
            assert_eq!(rotate(input.to_string(), shift), expected);
            assert_eq!(rotate2(input, &shift), expected);
            assert_eq!(rotate_vec_method(input.to_string(), shift), expected);
            assert_eq!(rotate_bytes(input.to_string(), shift), expected);
        }
    }

    #[test]
    fn test_unicode() {
        assert_eq!(rotate("🎵🎶🎼🎹".to_string(), 1), "🎹🎵🎶🎼");
        assert_eq!(rotate("привіт".to_string(), 2), "ітприв");
        assert_eq!(rotate("🌟⭐✨".to_string(), -1), "⭐✨🌟");
    }
}

fn main() {
    println!("=== Демонстрація зсуву рядків ===");
    
    // Основні приклади
    let test_string = "abcdefgh";
    println!("Оригінальний рядок: {}", test_string);
    
    println!("\nПозитивні зсуви (вправо):");
    for i in 1..=3 {
        println!("  зсув на {}: {}", i, rotate(test_string.to_string(), i));
    }
    
    println!("\nНегативні зсуви (вліво):");
    for i in 1..=3 {
        println!("  зсув на -{}: {}", i, rotate(test_string.to_string(), -i));
    }
    
    println!("\nВеликі зсуви:");
    println!("  зсув на 10: {}", rotate(test_string.to_string(), 10));
    println!("  зсув на -10: {}", rotate(test_string.to_string(), -10));
    
    println!("\nПовні оберти:");
    println!("  зсув на 8: {}", rotate(test_string.to_string(), 8));
    println!("  зсув на -8: {}", rotate(test_string.to_string(), -8));
    
    // Тестування з Unicode
    println!("\n=== Unicode рядки ===");
    let unicode_string = "🎵🎶🎼🎹";
    println!("Оригінал: {}", unicode_string);
    println!("Зсув на 1: {}", rotate(unicode_string.to_string(), 1));
    println!("Зсув на -1: {}", rotate(unicode_string.to_string(), -1));
    
    let cyrillic_string = "привіт";
    println!("\nОригінал: {}", cyrillic_string);
    println!("Зсув на 2: {}", rotate(cyrillic_string.to_string(), 2));
    println!("Зсув на -2: {}", rotate(cyrillic_string.to_string(), -2));
    
    // Порівняння продуктивності
    println!("\n=== Тест продуктивності ===");
    let long_string = "a".repeat(10000);
    let start = std::time::Instant::now();
    let _ = rotate(long_string.clone(), 1000);
    println!("Зсув довгого рядка (10000 символів): {:?}", start.elapsed());
}

// === Демонстрація зсуву рядків ===
// Оригінальний рядок: abcdefgh

// Позитивні зсуви (вправо):
//   зсув на 1: habcdefg
//   зсув на 2: ghabcdef
//   зсув на 3: fghabcde

// Негативні зсуви (вліво):
//   зсув на -1: bcdefgha
//   зсув на -2: cdefghab
//   зсув на -3: defghabc

// Великі зсуви:
//   зсув на 10: ghabcdef
//   зсув на -10: cdefghab

// Повні оберти:
//   зсув на 8: abcdefgh
//   зсув на -8: abcdefgh

// === Unicode рядки ===
// Оригінал: 🎵🎶🎼🎹
// Зсув на 1: 🎹🎵🎶🎼
// Зсув на -1: 🎶🎼🎹🎵

// Оригінал: привіт
// Зсув на 2: ітприв
// Зсув на -2: ивітпр

// === Тест продуктивності ===
// Зсув довгого рядка (10000 символів): 326.1µs

// === Code Execution Successful ===