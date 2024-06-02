#[test]
fn test_slice() {
    fn main() {
        let mut arr = [11, 22, 33, 44];

        // 不可变slice
        let arr_slice1 = &arr[..=1];
        dbg!("{:?}", arr_slice1); // [11,22];

        // 可变slice
        let arr_slice2 = &mut arr[..=1];
        arr_slice2[0] = 1111;
        dbg!("{:?}", arr_slice2); // [1111,22];
        dbg!("{:?}", arr); // [1111,22,33,44];
    }

    main()
}
