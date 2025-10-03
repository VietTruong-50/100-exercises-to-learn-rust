use outro_03::SaturatingU16;

#[test]
fn test_saturating_u16() {
    // Tạo các giá trị SaturatingU16 từ nhiều kiểu dữ liệu khác nhau
    let a: SaturatingU16 = (&10u8).into();    // từ &u8
    let b: SaturatingU16 = 5u8.into();        // từ u8
    let c: SaturatingU16 = u16::MAX.into();   // giá trị tối đa của u16 (65535)
    let d: SaturatingU16 = (&1u16).into();    // từ &u16
    let e = &c;                               // tham chiếu đến c

    // Các phép cộng và so sánh (test behavior)
    assert_eq!(a + b, SaturatingU16::from(15u16));          // 10 + 5 = 15
    assert_eq!(a + c, SaturatingU16::from(u16::MAX));       // 10 + 65535 => saturate = 65535
    assert_eq!(a + d, SaturatingU16::from(11u16));          // 10 + 1 = 11
    assert_eq!(a + a, 20u16);                               // So sánh trực tiếp với u16
    assert_eq!(a + 5u16, 15u16);                            // Cộng với u16 primitive
    assert_eq!(a + e, SaturatingU16::from(u16::MAX));       // 10 + MAX => saturate = MAX
}