// hw08.rs - Функції для перевірки простих чисел

/// Базова перевірка чи є число простим
/// Просте число - це натуральне число більше 1, яке ділиться націло 
/// тільки на 1 і на себе
/// 
/// # Аргументи
/// * `n` - число для перевірки
/// 
/// # Повертає
/// `true` якщо число просте, `false` якщо складене
pub fn is_prime_basic(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    
    if n == 2 {
        return true;
    }
    
    if n % 2 == 0 {
        return false;
    }
    
    for i in 3..n {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

/// Оптимізована перевірка чи є число простим
/// Перевіряє дільники тільки до квадратного кореня з числа
/// 
/// # Аргументи
/// * `n` - число для перевірки
/// 
/// # Повертає
/// `true` якщо число просте, `false` якщо складене
pub fn is_prime_optimized(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    
    if n == 2 || n == 3 {
        return true;
    }
    
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    let mut i = 5;
    
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    
    true
}

/// Перевірка простоти числа за допомогою решета Ератосфена
/// (для чисел до певної межі)
/// 
/// # Аргументи
/// * `limit` - верхня межа для генерації простих чисел
/// 
/// # Повертає
/// Вектор з усіма простими числами до limit
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<u64> {
    if limit < 2 {
        return vec![];
    }
    
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    
    (2..=limit)
        .filter(|&i| is_prime[i])
        .map(|i| i as u64)
        .collect()
}

/// Перевірка чи є число простим використовуючи попередньо згенеровані прості числа
/// 
/// # Аргументи
/// * `n` - число для перевірки
/// * `primes` - вектор простих чисел
/// 
/// # Повертає
/// `true` якщо число просте, `false` якщо складене
pub fn is_prime_with_sieve(n: u64, primes: &[u64]) -> bool {
    if n < 2 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    
    for &prime in primes {
        if prime > sqrt_n {
            break;
        }
        if n % prime == 0 {
            return n == prime;
        }
    }
    
    true
}

/// Генерує перші n простих чисел
/// 
/// # Аргументи
/// * `count` - кількість простих чисел для генерації
/// 
/// # Повертає
/// Вектор з першими count простими числами
pub fn generate_first_n_primes(count: usize) -> Vec<u64> {
    if count == 0 {
        return vec![];
    }
    
    let mut primes = Vec::with_capacity(count);
    let mut candidate = 2;
    
    while primes.len() < count {
        if is_prime_optimized(candidate) {
            primes.push(candidate);
        }
        candidate += if candidate == 2 { 1 } else { 2 };
    }
    
    primes
}

/// Знаходить найбільше просте число менше або рівне n
/// 
/// # Аргументи
/// * `n` - верхня межа
/// 
/// # Повертає
/// Найбільше просте число <= n, або None якщо такого немає
pub fn largest_prime_up_to(n: u64) -> Option<u64> {
    if n < 2 {
        return None;
    }
    
    for i in (2..=n).rev() {
        if is_prime_optimized(i) {
            return Some(i);
        }
    }
    
    None
}

/// Розкладає число на прості множники
/// 
/// # Аргументи
/// * `n` - число для розкладу
/// 
/// # Повертає
/// Вектор простих множників
pub fn prime_factorization(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    if n < 2 {
        return factors;
    }
    
    // Перевіряємо ділення на 2
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    
    // Перевіряємо непарні дільники
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    
    // Якщо залишилося число більше 2, то воно просте
    if n > 2 {
        factors.push(n);
    }
    
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_basic() {
        assert_eq!(is_prime_basic(2), true);
        assert_eq!(is_prime_basic(3), true);
        assert_eq!(is_prime_basic(4), false);
        assert_eq!(is_prime_basic(17), true);
        assert_eq!(is_prime_basic(18), false);
        assert_eq!(is_prime_basic(1), false);
        assert_eq!(is_prime_basic(0), false);
    }

    #[test]
    fn test_is_prime_optimized() {
        assert_eq!(is_prime_optimized(2), true);
        assert_eq!(is_prime_optimized(3), true);
        assert_eq!(is_prime_optimized(4), false);
        assert_eq!(is_prime_optimized(97), true);
        assert_eq!(is_prime_optimized(100), false);
        assert_eq!(is_prime_optimized(1009), true);
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        let primes = sieve_of_eratosthenes(20);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_generate_first_n_primes() {
        let primes = generate_first_n_primes(10);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_largest_prime_up_to() {
        assert_eq!(largest_prime_up_to(10), Some(7));
        assert_eq!(largest_prime_up_to(20), Some(19));
        assert_eq!(largest_prime_up_to(1), None);
    }

    #[test]
    fn test_prime_factorization() {
        assert_eq!(prime_factorization(12), vec![2, 2, 3]);
        assert_eq!(prime_factorization(17), vec![17]);
        assert_eq!(prime_factorization(60), vec![2, 2, 3, 5]);
        assert_eq!(prime_factorization(1), vec![]);
    }

    #[test]
    fn test_large_primes() {
        assert_eq!(is_prime_optimized(982451653), true);
        assert_eq!(is_prime_optimized(982451654), false);
    }
}

fn main() {
    println!("=== Демонстрація перевірки простих чисел ===");
    
    // Тестування базової функції
    println!("\n1. Базова перевірка простих чисел:");
    let test_numbers = vec![1, 2, 3, 4, 17, 25, 29, 97, 100];
    for num in &test_numbers {
        println!("{} є простим: {}", num, is_prime_optimized(*num));
    }
    
    // Генерація перших 20 простих чисел
    println!("\n2. Перші 20 простих чисел:");
    let first_primes = generate_first_n_primes(20);
    println!("{:?}", first_primes);
    
    // Решето Ератосфена
    println!("\n3. Прості числа до 50 (решето Ератосфена):");
    let sieve_primes = sieve_of_eratosthenes(50);
    println!("{:?}", sieve_primes);
    
    // Розклад на прості множники
    println!("\n4. Розклад чисел на прості множники:");
    let numbers_to_factor = vec![12, 60, 97, 144, 1001];
    for num in numbers_to_factor {
        let factors = prime_factorization(num);
        println!("{} = {:?}", num, factors);
    }
    
    // Найбільше просте число до заданого
    println!("\n5. Найбільші прості числа:");
    for limit in vec![10, 50, 100, 1000] {
        if let Some(prime) = largest_prime_up_to(limit) {
            println!("Найбільше просте число <= {}: {}", limit, prime);
        }
    }
    
    // Тестування продуктивності
    println!("\n6. Тест великих чисел:");
    let large_numbers = vec![982451653, 982451654, 1000000007, 1000000009];
    for num in large_numbers {
        let start = std::time::Instant::now();
        let is_prime = is_prime_optimized(num);
        let duration = start.elapsed();
        println!("{} є простим: {} (час: {:?})", num, is_prime, duration);
    }
}

=== Демонстрація перевірки простих чисел ===

1. Базова перевірка простих чисел:
1 є простим: false
2 є простим: true
3 є простим: true
4 є простим: false
17 є простим: true
25 є простим: false
29 є простим: true
97 є простим: true
100 є простим: false

2. Перші 20 простих чисел:
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]

3. Прості числа до 50 (решето Ератосфена):
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]

4. Розклад чисел на прості множники:
12 = [2, 2, 3]
60 = [2, 2, 3, 5]
97 = [97]
144 = [2, 2, 2, 2, 3, 3]
1001 = [7, 11, 13]

5. Найбільші прості числа:
Найбільше просте число <= 10: 7
Найбільше просте число <= 50: 47
Найбільше просте число <= 100: 97
Найбільше просте число <= 1000: 997

6. Тест великих чисел:
982451653 є простим: true (час: 22.63µs)
982451654 є простим: false (час: 50ns)
1000000007 є простим: true (час: 22.75µs)
1000000009 є простим: true (час: 22.73µs)

=== Code Execution Successful ===