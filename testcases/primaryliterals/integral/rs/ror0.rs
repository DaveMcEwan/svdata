let a = SvPrimaryLiteralIntegral {
    data01: vec![3, 9223372036854775809],
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.ror(2);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904, 16140901064495857664],
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);