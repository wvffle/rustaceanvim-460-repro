#[test]
pub fn test_ok() {
    assert_eq!(1, 1);
}

#[test]
pub fn test_external_fail() {
    rustaceanvim_460_repro::fail();
}
