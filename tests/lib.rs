use k_pro::{echo, echo_str};

#[test]
fn test_echo() {
    let a = echo_str!(16, 32);
    assert_eq!(a, "16 32");
    let a = echo_str!("Yes");
    assert_eq!(a, "Yes");

    let count = 0;
    echo!("THERE IS", count, "PROBLEMS");
}
