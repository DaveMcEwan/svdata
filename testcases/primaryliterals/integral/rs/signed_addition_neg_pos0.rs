let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    size: 63,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.add_usize(4611686018427387904);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![0],
    size: 1,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);