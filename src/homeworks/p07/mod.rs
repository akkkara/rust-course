// hw07.rs - Функції для зміни регістру символів

/// Конвертує рядок з верхнього регістру в нижній
/// 
/// # Аргументи
/// * `input` - рядок для конвертації
/// 
/// # Повертає
/// Новий рядок з символами в нижньому регістрі
pub fn to_lowercase(input: &str) -> String {
    input.to_lowercase()
}

/// Конвертує рядок з нижнього регістру у верхній
/// 
/// # Аргументи
/// * `input` - рядок для конвертації
/// 
/// # Повертає
/// Новий рядок з символами у верхньому регістрі
pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

/// Змінює регістр кожного символу на протилежний
/// (великі стають малими, малі стають великими)
/// 
/// # Аргументи
/// * `input` - рядок для конвертації
/// 
/// # Повертає
/// Новий рядок з інвертованим регістром
pub fn toggle_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Ручна реалізація конвертації з верхнього в нижній регістр
/// (тільки для ASCII символів)
pub fn manual_to_lowercase(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c >= 'A' && c <= 'Z' {
                char::from(c as u8 + 32)
            } else {
                c
            }
        })
        .collect()
}

/// Ручна реалізація конвертації з нижнього у верхній регістр
/// (тільки для ASCII символів)
pub fn manual_to_uppercase(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c >= 'a' && c <= 'z' {
                char::from(c as u8 - 32)
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lowercase() {
        assert_eq!(to_lowercase("HELLO WORLD"), "hello world");
        assert_eq!(to_lowercase("Rust Programming"), "rust programming");
        assert_eq!(to_lowercase("123ABC"), "123abc");
    }

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("hello world"), "HELLO WORLD");
        assert_eq!(to_uppercase("Rust Programming"), "RUST PROGRAMMING");
        assert_eq!(to_uppercase("123abc"), "123ABC");
    }

    #[test]
    fn test_toggle_case() {
        assert_eq!(toggle_case("Hello World"), "hELLO wORLD");
        assert_eq!(toggle_case("RuSt"), "rUsT");
        assert_eq!(toggle_case("123"), "123");
    }

    #[test]
    fn test_manual_to_lowercase() {
        assert_eq!(manual_to_lowercase("HELLO"), "hello");
        assert_eq!(manual_to_lowercase("ABC123"), "abc123");
        assert_eq!(manual_to_lowercase("MiXeD"), "mixed");
    }

    #[test]
    fn test_manual_to_uppercase() {
        assert_eq!(manual_to_uppercase("hello"), "HELLO");
        assert_eq!(manual_to_uppercase("abc123"), "ABC123");
        assert_eq!(manual_to_uppercase("MiXeD"), "MIXED");
    }

    #[test]
    fn test_ukrainian_text() {
        assert_eq!(to_lowercase("ПРИВІТ СВІТ"), "привіт світ");
        assert_eq!(to_uppercase("привіт світ"), "ПРИВІТ СВІТ");
        assert_eq!(toggle_case("Привіт Світ"), "пРИВІТ сВІТ");
    }
}

fn main() {
    // Приклади використання
    println!("=== Демонстрація функцій зміни регістру ===");
    
    let test_string = "Hello World! Привіт Світ!";
    println!("Оригінальний рядок: {}", test_string);
    
    println!("В нижньому регістрі: {}", to_lowercase(test_string));
    println!("У верхньому регістрі: {}", to_uppercase(test_string));
    println!("Інвертований регістр: {}", toggle_case(test_string));
    
    println!("\n=== Ручна реалізація (тільки ASCII) ===");
    let ascii_string = "Hello World 123!";
    println!("Оригінальний рядок: {}", ascii_string);
    println!("Ручний нижній регістр: {}", manual_to_lowercase(ascii_string));
    println!("Ручний верхній регістр: {}", manual_to_uppercase(ascii_string));
}

// === Демонстрація функцій зміни регістру ===
// Оригінальний рядок: Hello World! Привіт Світ!
// В нижньому регістрі: hello world! привіт світ!
// У верхньому регістрі: HELLO WORLD! ПРИВІТ СВІТ!
// Інвертований регістр: hELLO wORLD! пРИВІТ сВІТ!

// === Ручна реалізація (тільки ASCII) ===
// Оригінальний рядок: Hello World 123!
// Ручний нижній регістр: hello world 123!
// Ручний верхній регістр: HELLO WORLD 123!

// === Code Execution Successful ===