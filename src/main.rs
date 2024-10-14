fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;

    if len == 0 {
        return s;
    }

    let n = ((n % len) + len) % len;
    let n = n as usize;

    // Розбиваємо рядок на дві частини та об'єднуємо їх
    let (left, right) = s.split_at(len as usize - n);
    format!("{}{}", right, left)
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "habcdefg"),
            (-2, "ghabcdef"),
            (-10, "efghabcd"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
    }
}
