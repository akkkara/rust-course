// Генеруємо фіксований вектор для прикладу
fn gen_random_vector(n: usize) -> Vec<i32> {
    // Для прикладу - просто числа від 10 до 10+n-1
    (10..10 + n as i32).collect()
}

// Знаходимо мінімальну суму двох сусідніх елементів і індекси
fn min_adjacent_sum(data: &[i32]) -> Option<(usize, usize, i32)> {
    if data.len() < 2 {
        return None;
    }
    let mut min_sum = data[0] + data[1];
    let mut min_idx = 0;
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_idx = i;
        }
    }
    Some((min_idx, min_idx + 1, min_sum))
}

// Функція для друку результату у форматі з твого прикладу
fn print_vector_and_min_sum(vec: &[i32], min_pair: (usize, usize, i32)) {
    print!("indexes: ");
    for i in 0..vec.len() {
        print!("{:>3}. ", i);
    }
    println!();

    print!("data:    [");
    for (i, val) in vec.iter().enumerate() {
        print!("{}", val);
        if i != vec.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    print!("indexes: ");
    for i in 0..vec.len() {
        if i == min_pair.0 {
            print!(" \\__");
        } else if i == min_pair.1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        vec[min_pair.0], vec[min_pair.1], min_pair.2, min_pair.0, min_pair.1
    );
}

fn main() {
    let vec = gen_random_vector(20);
    if let Some(min_pair) = min_adjacent_sum(&vec) {
        print_vector_and_min_sum(&vec, min_pair);
    } else {
        println!("Vector is too small");
    }
}


// indexes:   0.   1.   2.   3.   4.   5.   6.   7.   8.   9.  10.  11.  12.  13.  14.  15.  16.  17.  18.  19. 
// data:    [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]
// indexes:  \__ __/                                                                        
// min adjacent sum=10+11=21 at indexes:0,1

// === Code Execution Successful ===