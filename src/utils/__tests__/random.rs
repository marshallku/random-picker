#[cfg(test)]
mod tests {
    use crate::utils::random::pick_random;

    #[test]
    fn test_random() {
        let array = [1, 2, 3, 4, 5];

        for _ in 0..10 {
            let random = pick_random(&array);
            assert!(array.contains(random));
        }
    }
}
