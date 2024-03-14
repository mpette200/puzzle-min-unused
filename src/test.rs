use crate::get_min_not_in_list_via_hash;
use crate::get_min_not_in_list_via_sort;
use crate::RandomGen;
use crate::RANDOM_SEED;

// ----- Test Hash Algorithm -----

#[test]
fn test_via_hash_1() {
    let v = vec![0, 1, 2, 3, 5];
    assert_eq!(get_min_not_in_list_via_hash(&v), 4);
}

#[test]
fn test_via_hash_2() {
    let v = vec![0, 1, 3, 4, 5];
    assert_eq!(get_min_not_in_list_via_hash(&v), 2);
}

#[test]
fn test_via_hash_3() {
    let v = vec![2, 1, 0];
    assert_eq!(get_min_not_in_list_via_hash(&v), 3);
}

#[test]
fn test_via_hash_4() {
    let v = vec![20, 10, 30];
    assert_eq!(get_min_not_in_list_via_hash(&v), 0);
}

#[test]
fn test_via_hash_5() {
    let v = vec![0];
    assert_eq!(get_min_not_in_list_via_hash(&v), 1);
}

#[test]
fn test_via_hash_6() {
    let v = vec![];
    assert_eq!(get_min_not_in_list_via_hash(&v), 0);
}

#[test]
fn test_via_hash_7() {
    let v = vec![1];
    assert_eq!(get_min_not_in_list_via_hash(&v), 0);
}

#[test]
fn test_via_hash_8() {
    let v = vec![10, 6, 8, 3, 0];
    assert_eq!(get_min_not_in_list_via_hash(&v), 1);
}

// ----- Test Sort Algorithm -----

#[test]
fn test_via_sort_1() {
    let v = vec![0, 1, 2, 3, 5];
    assert_eq!(get_min_not_in_list_via_sort(&v), 4);
}

#[test]
fn test_via_sort_2() {
    let v = vec![0, 1, 3, 4, 5];
    assert_eq!(get_min_not_in_list_via_sort(&v), 2);
}

#[test]
fn test_via_sort_3() {
    let v = vec![2, 1, 0];
    assert_eq!(get_min_not_in_list_via_sort(&v), 3);
}

#[test]
fn test_via_sort_4() {
    let v = vec![20, 10, 30];
    assert_eq!(get_min_not_in_list_via_sort(&v), 0);
}

#[test]
fn test_via_sort_5() {
    let v = vec![0];
    assert_eq!(get_min_not_in_list_via_sort(&v), 1);
}

#[test]
fn test_via_sort_6() {
    let v = vec![];
    assert_eq!(get_min_not_in_list_via_sort(&v), 0);
}

#[test]
fn test_via_sort_7() {
    let v = vec![1];
    assert_eq!(get_min_not_in_list_via_sort(&v), 0);
}

#[test]
fn test_via_sort_8() {
    let v = vec![10, 6, 8, 3, 0];
    assert_eq!(get_min_not_in_list_via_sort(&v), 1);
}

#[test]
fn test_by_comparing() {
    let mut ran_gen = RandomGen::new(RANDOM_SEED);

    let random_lists: Vec<Vec<u32>> = (5_000..25_000)
        .step_by(5_000)
        .map(|size| ran_gen.make_vec(size))
        .collect();

    for nums in random_lists {
        assert_eq!(
            get_min_not_in_list_via_sort(&nums),
            get_min_not_in_list_via_hash(&nums)
        )
    }
}
