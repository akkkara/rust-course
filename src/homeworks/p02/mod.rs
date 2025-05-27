// hw03.rs - Малювання конверту в консолі

// Константи для розміру конверту
const W: usize = 25; // Ширина конверту
const H: usize = 15; // Висота конверту

fn main() {
    draw_envelope(W, H);
}

fn draw_envelope(width: usize, height: usize) {
    // Перевірка мінімальних розмірів
    if width < 10 || height < 10 || width > 80 || height > 80 {
        println!("Розміри конверту повинні бути в діапазоні 10...80");
        return;
    }

    for row in 0..height {
        for col in 0..width {
            let ch = get_envelope_char(row, col, width, height);
            print!("{}", ch);
        }
        println!();
    }
}

fn get_envelope_char(row: usize, col: usize, width: usize, height: usize) -> char {
    // Межі конверту
    let is_top_edge = row == 0;
    let is_bottom_edge = row == height - 1;
    let is_left_edge = col == 0;
    let is_right_edge = col == width - 1;
    
    // Центр конверту
    let center_row = height / 2;
    
    // Основні лінії конверту (рамка)
    if is_top_edge || is_bottom_edge || is_left_edge || is_right_edge {
        '*'
    }
    // Верхня частина - діагоналі від кутів до центру
    else if row <= center_row {
        // Ліва діагональ (від верхнього лівого кута)
        if col == row {
            '*'
        }
        // Права діагональ (від верхнього правого кута)
        else if col + row == width - 1 {
            '*'
        }
        else {
            ' '
        }
    }
    // Нижня частина - діагоналі від центру до кутів
    else {
        // Ліва діагональ (до нижнього лівого кута)
        if row + col == height - 1 {
            '*'
        }
        // Права діагональ (до нижнього правого кута)
        else if col >= row && col == width - 1 - (height - 1 - row) {
            '*'
        }
        else {
            ' '
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_drawing() {
        // Тест не виводить в консоль, але перевіряє що функція не панікує
        draw_envelope(15, 10);
        draw_envelope(20, 12);
        draw_envelope(30, 18);
    }
    
    #[test]
    fn test_invalid_sizes() {
        // Тестуємо поведінку з неправильними розмірами
        draw_envelope(5, 10);  // Занадто мала ширина
        draw_envelope(10, 5);  // Занадто маленька висота
        draw_envelope(85, 20); // Занадто велика ширина
    }
}

// *************************
// **                     **
// * *                   * *
// *  *                 *  *
// *   *               *   *
// *    *             *    *
// *     *           *     *
// *      *         *      *
// *     *           *     *
// *    *             *    *
// *   *               *   *
// *  *                 *  *
// * *                   * *
// **                     **
// *************************

// === Code Execution Successful ===