use crate::get_min_not_in_set;

#[test]
fn test_vec_1() {
    let v = vec![0, 1, 2, 6, 7, 8];
    assert_eq!(get_min_not_in_set(v), 3);
}
