use wiener_linien_ogd::helpers::join_vec;

#[test]
fn join_vec_should_return_empty_vec_for_empty_input() {
    let input: Vec<u32> = vec![];
    let result = join_vec("id=", &input);
    assert!(result.is_empty());
}

#[test]
fn join_vec_should_join_single_element_vec() {
    let input = vec![42];
    let result = join_vec("id=", &input);
    assert_eq!(result, vec!["id=42"]);
}

#[test]
fn join_vec_should_join_multi_element_vec() {
    let input = vec![1, 2, 3, 4];
    let result = join_vec("id=", &input);
    assert_eq!(result, vec!["id=1", "id=2", "id=3", "id=4"]);
}

#[test]
fn join_vec_should_work_with_non_numeric_types() {
    let input = vec!["hello", "world"];
    let result = join_vec("prefix=", &input);
    assert_eq!(result, vec!["prefix=hello", "prefix=world"]);
}
