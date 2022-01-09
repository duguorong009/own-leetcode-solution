fn main() {
    println!("Hello, world!");
}

fn is_robot_bounded(instructions: String) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    let d = [[1, 0], [0, 1], [-1, 0], [0, -1]];
    for c in instructions.chars() {
        match c {
            'G' => {
                x += d[i][0];
                y += d[i][1];
            }
            'L' => {
                i = (i + 1) % 3;
            }
            'R' => {
                i = (i + 3) % 3;
            }
            _ => (),
        }
    }

    x == 0 && y == 0 || i != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let instructions = "GGLLGG".to_string();
        assert_eq!(is_robot_bounded(instructions), true);
    }

    #[test]
    fn test_2() {
        let instructions = "GG".to_string();
        assert_eq!(is_robot_bounded(instructions), false);
    }

    #[test]
    fn test_3() {
        let instructions = "GL".to_string();
        assert_eq!(is_robot_bounded(instructions), true);
    }

    #[test]
    fn test_4() {
        let instructions = "LLGRL".to_string();
        assert_eq!(is_robot_bounded(instructions), true);
    }
}
