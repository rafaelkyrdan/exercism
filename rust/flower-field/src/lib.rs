pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let rows = garden.len();
    let cols = garden[0].len();
    let mut result = Vec::new();

    for row in 0..rows {
        let mut annotated_row = String::new();

        for col in 0..cols {
            let current_char = garden[row].chars().nth(col).unwrap();

            if current_char == '*' {
                annotated_row.push('*');
            } else {
                let flower_count = count_adjacent_flowers(garden, row, col);

                if flower_count == 0 {
                    annotated_row.push(' ');
                } else {
                    annotated_row.push(char::from_digit(flower_count as u32, 10).unwrap());
                }
            }
        }

        result.push(annotated_row);
    }

    result
}

fn count_adjacent_flowers(garden: &[&str], row: usize, col: usize) -> usize {
    let rows = garden.len() as i32;
    let cols = garden[0].len() as i32;
    let mut count = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1), // Top row
        (0, -1),
        (0, 1), // Same row (left, right)
        (1, -1),
        (1, 0),
        (1, 1), // Bottom row
    ];

    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            let char_at_pos = garden[new_row as usize]
                .chars()
                .nth(new_col as usize)
                .unwrap();

            if char_at_pos == '*' {
                count += 1;
            }
        }
    }

    count
}
