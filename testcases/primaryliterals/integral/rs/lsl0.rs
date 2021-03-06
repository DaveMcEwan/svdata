let a = SvPrimaryLiteralIntegral {
    data01: vec![0, 9223372036854775808],
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsl(1);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![1, 0],
    size: 66,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);