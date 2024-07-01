pub fn pick_random<T>(array: &[T]) -> &T {
    let index = rand::random::<usize>() % array.len();
    &array[index]
}
