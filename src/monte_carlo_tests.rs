use crate::generate_path;

#[test]
fn generate_path_with_no_steps_is_empty() {
    assert!(generate_path(0, 0.0, 1.0).is_empty());
}

#[test]
fn generate_path_has_one_value_per_step() {
    assert_eq!(generate_path(1, 0.0, 1.0).len(), 1);
    assert_eq!(generate_path(100, 0.0, 1.0).len(), 100);
}

#[test]
fn generate_path_with_zero_std_is_a_straight_line() {
    assert_eq!(generate_path(4, 0.5, 0.0), vec![0.5, 1.0, 1.5, 2.0]);
}
