fn gen_random_vector(_: usize) -> Vec<i32> {
    vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22]
}

fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32, i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_index = 0;

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    Some((min_index, data[min_index], data[min_index + 1], min_sum))
}

fn print_vector_with_min_pair(data: &[i32]) {
    if let Some((idx, a, b, sum)) = min_adjacent_sum(data) {
        print!("indexes: ");
        for i in 0..data.len() {
            print!("{:>3}.", i);
        }
        println!();

        print!("data:    ");
        for val in data {
            print!("{:>3},", val);
        }
        println!();

        print!("indexes: ");
        for i in 0..data.len() {
            if i == idx {
                print!("  \\__");
            } else if i == idx + 1 {
                print!(" __/");
            } else {
                print!("     ");
            }
        }
        println!();

        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            a, b, sum, idx, idx + 1
        );
    } else {
        println!("Недостатньо даних для пошуку мінімальної пари.");
    }
}

fn main() {
    let data = gen_random_vector(20);
    print_vector_with_min_pair(&data);
}


// indexes:   0.  1.  2.  3.  4.  5.  6.  7.  8.  9. 10. 11. 12. 13. 14. 15. 16. 17. 18. 19.
// data:     45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22,
// indexes:                            \__ __/                                                                 
// min adjacent sum=37+45=82 at indexes:5,6

// === Code Execution Successful ===