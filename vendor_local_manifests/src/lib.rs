pub fn multiple(x: i32) -> i32 {
    x * 2
}

#[test]
fn two_times_two() {
    assert_eq!(multiple(2), 4)
}

#[test]
fn three_times_two() {
    assert_eq!(multiple(3), 6)
}
