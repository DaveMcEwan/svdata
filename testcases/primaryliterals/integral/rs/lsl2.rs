let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsl(4);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![4, 0],
    size: 68,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);