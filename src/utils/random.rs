use rand::{thread_rng, Rng};

pub fn pick_random<T>(array: &[T]) -> &T {
    let index = thread_rng().gen_range(0..array.len());
    &array[index]
}
