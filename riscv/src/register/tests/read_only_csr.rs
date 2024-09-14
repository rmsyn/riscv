read_only_csr! {
    "test CSR register type",
    Mtest: 0x000,
    0b1111_1111_1111,
    "test single-bit field",
    single: 0,
}

read_only_csr_field! {
    Mtest,
    "multiple single-bit field range",
    multi_range: 1..=3,
}

read_only_csr_field!(
    Mtest,
    "multi-bit field",
    multi_field: [4:7],
);

read_only_csr_field!(
    Mtest,
    "multi-bit field",
    field_enum,
    "field enum type with valid field variants",
    MtestFieldEnum {
        range: [7:11],
        default: Field1,
        Field1 = 1,
        Field2 = 2,
        Field3 = 3,
        Field4 = 15,
    },
);

// we don't test the `read` function, we are only testing in-memory functions.
#[allow(unused)]
pub fn _read_csr() -> Mtest {
    read()
}

#[test]
fn test_mtest_read_only() {
    let mut mtest = Mtest::from_bits(0);

    assert_eq!(mtest.bitmask(), Mtest::BITMASK);
    assert_eq!(mtest.bits(), 0);

    // check that single bit field getter/setters work.
    assert_eq!(mtest.single(), false);

    mtest = Mtest::from_bits(1);
    assert_eq!(mtest.single(), true);

    mtest = Mtest::from_bits(0);

    // check that single bit range field getter/setters work.
    for i in 1..=3 {
        assert_eq!(mtest.multi_range(i), false);

        mtest = Mtest::from_bits(1 << i);
        assert_eq!(mtest.multi_range(i), true);

        mtest = Mtest::from_bits(0 << i);
        assert_eq!(mtest.multi_range(i), false);
    }

    // check that multi-bit field getter/setters work.
    assert_eq!(mtest.multi_field(), 0);

    mtest = Mtest::from_bits(0xf << 4);
    assert_eq!(mtest.multi_field(), 0xf);

    mtest = Mtest::from_bits(0x3 << 4);
    assert_eq!(mtest.multi_field(), 0x3);

    // check that only bits in the field are set.
    mtest = Mtest::from_bits(0xff << 4);
    assert_eq!(mtest.multi_field(), 0xf);
    assert_eq!(mtest.bits(), 0xff << 4);

    mtest = Mtest::from_bits(0x0 << 4);
    assert_eq!(mtest.multi_field(), 0x0);

    assert_eq!(mtest.field_enum(), None);

    [
        MtestFieldEnum::Field1,
        MtestFieldEnum::Field2,
        MtestFieldEnum::Field3,
        MtestFieldEnum::Field4,
    ]
    .into_iter()
    .for_each(|variant| {
        mtest = Mtest::from_bits(variant.into_usize() << 7);
        assert_eq!(mtest.field_enum(), Some(variant));
    });

    // check that setting an invalid variant returns `None`
    mtest = Mtest::from_bits(0xbad << 7);
    assert_eq!(mtest.field_enum(), None);
}
