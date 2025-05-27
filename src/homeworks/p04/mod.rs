// hw04.rs - Малювання ромбу в консолі

// Константи для розміру ромбу
const W: usize = 15; // Ширина ромбу
const H: usize = 11; // Висота ромбу

fn main() {
    draw_diamond(W, H);
}

fn draw_diamond(width: usize, height: usize) {
    // Створюємо рядок для всього ромбу
    let mut output = String::new();
    
    // Центр ромбу
    let center_row = height / 2;
    let center_col = width / 2;
    
    // Проходимо через всі рядки
    for row in 0..height {
        // Проходимо через всі колонки в рядку
        for col in 0..width {
            let ch = get_diamond_char(row, col, width, height, center_row, center_col);
            output.push(ch);
        }
        // Додаємо новий рядок після кожного рядка (крім останнього)
        if row < height - 1 {
            output.push('\n');
        }
    }
    
    // Виводимо весь ромб одним разом
    println!("{}", output);
}

fn get_diamond_char(row: usize, col: usize, _width: usize, _height: usize, center_row: usize, center_col: usize) -> char {
    // Обчислюємо відстань від центру
    let row_dist = if row <= center_row { 
        center_row - row 
    } else { 
        row - center_row 
    };
    
    let col_dist = if col <= center_col { 
        center_col - col 
    } else { 
        col - center_col 
    };
    
    // Перевіряємо чи знаходимося на межі ромбу
    // Ромб утворюється коли сума відстаней дорівнює радіусу
    let max_row_radius = center_row;
    let max_col_radius = center_col;
    
    // Нормалізуємо відстані відносно розмірів ромбу
    let normalized_distance = (row_dist as f64 / max_row_radius as f64) + 
                             (col_dist as f64 / max_col_radius as f64);
    
    // Якщо сума нормалізованих відстаней близька до 1, це межа ромбу
    if (normalized_distance - 1.0).abs() < 0.1 {
        '*'
    } else if normalized_distance < 1.0 {
        ' ' // Всередині ромбу
    } else {
        ' ' // Зовні ромбу
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diamond_drawing() {
        // Тест різних розмірів ромбу
        draw_diamond(11, 7);
        draw_diamond(15, 9);
        draw_diamond(21, 13);
    }
    
    #[test]
    fn test_small_diamond() {
        draw_diamond(7, 5);
    }
}

//        *       
//      ** **     
//     *     *    
//    *       *   
//  **         ** 
// *             *
//  **         ** 
//    *       *   
//     *     *    
//      ** **     
//        *       

// === Code Execution Successful ===