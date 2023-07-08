use std::fmt::Display;

pub fn join_vec<T: Display>(prepend: &'static str, values: &[T]) -> Vec<String> {
    values
        .iter()
        .map(|id| format!("{}{}", prepend, id))
        .collect::<Vec<String>>()
}
