// hw06.rs - Малювання ялинки в консолі з використанням ітераторів

// Кількість трикутників (секцій) ялинки
const TRIANGLES: usize = 4;

fn main() {
    println!("=== Стандартна ялинка ({} трикутників) ===", TRIANGLES);
    draw_christmas_tree(TRIANGLES);
    
    println!("\n=== Функціональна версія ===");
    draw_christmas_tree_functional(TRIANGLES);
    
    println!("\n=== Мала ялинка (2 трикутники) ===");
    draw_christmas_tree(2);
    
    println!("\n=== Кастомна ялинка з символами ▲ та ■ ===");
    draw_custom_tree(3, '▲', '■');
}

fn draw_christmas_tree(triangles: usize) {
    if triangles == 0 {
        println!("Кількість трикутників повинна бути більше 0");
        return;
    }

    // Обчислюємо загальну ширину ялинки для центрування
    let max_width = calculate_max_width(triangles);
    
    // Малюємо кожен трикутник ялинки
    (0..triangles).for_each(|triangle_index| {
        draw_triangle(triangle_index, triangles, max_width);
    });
    
    // Малюємо стовбур ялинки
    draw_trunk(max_width);
}

fn draw_triangle(triangle_index: usize, _total_triangles: usize, max_width: usize) {
    // Висота кожного трикутника збільшується
    let triangle_height = 3 + triangle_index;
    
    // Базова ширина для цього трикутника
    let base_width = 3 + triangle_index * 2;
    
    // Малюємо рядки трикутника
    (0..triangle_height).for_each(|row| {
        let stars_count = base_width + row * 2;
        let spaces_count = (max_width - stars_count) / 2;
        
        // Створюємо рядок з пробілами та зірочками
        let line = " ".repeat(spaces_count) + &"*".repeat(stars_count);
        println!("{}", line);
    });
}

fn draw_trunk(max_width: usize) {
    let trunk_width = 3;
    let trunk_height = 2;
    
    // Малюємо стовбур
    (0..trunk_height).for_each(|_| {
        let spaces_count = (max_width - trunk_width) / 2;
        let trunk_line = " ".repeat(spaces_count) + &"*".repeat(trunk_width);
        println!("{}", trunk_line);
    });
}

fn calculate_max_width(triangles: usize) -> usize {
    // Останній трикутник має найбільшу ширину
    let last_triangle_height = 3 + triangles - 1;
    let last_triangle_base = 3 + (triangles - 1) * 2;
    last_triangle_base + (last_triangle_height - 1) * 2
}

// Альтернативна реалізація з більш функціональним підходом
fn draw_christmas_tree_functional(triangles: usize) {
    if triangles == 0 {
        println!("Кількість трикутників повинна бути більше 0");
        return;
    }

    let max_width = calculate_max_width(triangles);
    
    // Генеруємо всі рядки ялинки одним ітератором
    let tree_lines: Vec<String> = (0..triangles)
        .flat_map(|triangle_index| {
            let triangle_height = 3 + triangle_index;
            let base_width = 3 + triangle_index * 2;
            
            (0..triangle_height).map(move |row| {
                let stars_count = base_width + row * 2;
                let spaces_count = (max_width - stars_count) / 2;
                " ".repeat(spaces_count) + &"*".repeat(stars_count)
            })
        })
        .chain(
            // Додаємо стовбур
            (0..2).map(|_| {
                let spaces_count = (max_width - 3) / 2;
                " ".repeat(spaces_count) + "***"
            })
        )
        .collect();
    
    // Виводимо всю ялинку
    tree_lines.iter().for_each(|line| println!("{}", line));
}

// Функція для створення ялинки з різними символами
fn draw_custom_tree(triangles: usize, tree_char: char, trunk_char: char) {
    if triangles == 0 {
        println!("Кількість трикутників повинна бути більше 0");
        return;
    }

    let max_width = calculate_max_width(triangles);
    
    // Використовуємо iterator chains для створення всієї ялинки
    (0..triangles)
        .flat_map(|triangle_index| {
            let triangle_height = 3 + triangle_index;
            let base_width = 3 + triangle_index * 2;
            
            (0..triangle_height).map(move |row| {
                let stars_count = base_width + row * 2;
                let spaces_count = (max_width - stars_count) / 2;
                format!("{}{}", 
                    " ".repeat(spaces_count), 
                    tree_char.to_string().repeat(stars_count))
            })
        })
        .chain(
            (0..2).map(|_| {
                let spaces_count = (max_width - 3) / 2;
                format!("{}{}", 
                    " ".repeat(spaces_count), 
                    trunk_char.to_string().repeat(3))
            })
        )
        .for_each(|line| println!("{}", line));
}

// Запуск програми
#[allow(dead_code)]
fn demo_trees() {
    println!("=== Стандартна ялинка ({} трикутників) ===", TRIANGLES);
    draw_christmas_tree(TRIANGLES);
    
    println!("\n=== Функціональна версія ===");
    draw_christmas_tree_functional(TRIANGLES);
    
    println!("\n=== Мала ялинка (2 трикутники) ===");
    draw_christmas_tree(2);
    
    println!("\n=== Велика ялинка (6 трикутників) ===");
    draw_christmas_tree(6);
    
    println!("\n=== Кастомна ялинка з символами ▲ та ■ ===");
    draw_custom_tree(3, '▲', '■');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_width_calculation() {
        assert_eq!(calculate_max_width(1), 5); // 3 + (3-1)*2 = 5
        assert_eq!(calculate_max_width(2), 9); // 5 + (4-1)*2 = 9  
        assert_eq!(calculate_max_width(3), 13); // 7 + (5-1)*2 = 13
        assert_eq!(calculate_max_width(4), 17); // 9 + (6-1)*2 = 17
    }

    #[test]
    fn test_tree_drawing() {
        // Тестуємо що функції не панікують
        draw_christmas_tree(1);
        draw_christmas_tree(3);
        draw_christmas_tree_functional(2);
        draw_custom_tree(2, '*', '#');
    }

    #[test]
    fn test_edge_cases() {
        draw_christmas_tree(0); // Повинно вивести попередження
    }
}

// === Стандартна ялинка (4 трикутників) ===
//         ***
//        *****
//       *******
//        *****
//       *******
//      *********
//     ***********
//       *******
//      *********
//     ***********
//    *************
//   ***************
//      *********
//     ***********
//    *************
//   ***************
//  *****************
// *******************
//         ***
//         ***

// === Функціональна версія ===
//         ***
//        *****
//       *******
//        *****
//       *******
//      *********
//     ***********
//       *******
//      *********
//     ***********
//    *************
//   ***************
//      *********
//     ***********
//    *************
//   ***************
//  *****************
// *******************
//         ***
//         ***

// === Мала ялинка (2 трикутники) ===
//     ***
//    *****
//   *******
//    *****
//   *******
//  *********
// ***********
//     ***
//     ***

// === Кастомна ялинка з символами ▲ та ■ ===
//       ▲▲▲
//      ▲▲▲▲▲
//     ▲▲▲▲▲▲▲
//      ▲▲▲▲▲
//     ▲▲▲▲▲▲▲
//    ▲▲▲▲▲▲▲▲▲
//   ▲▲▲▲▲▲▲▲▲▲▲
//     ▲▲▲▲▲▲▲
//    ▲▲▲▲▲▲▲▲▲
//   ▲▲▲▲▲▲▲▲▲▲▲
//  ▲▲▲▲▲▲▲▲▲▲▲▲▲
// ▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲
//       ■■■
//       ■■■

// === Code Execution Successful ===