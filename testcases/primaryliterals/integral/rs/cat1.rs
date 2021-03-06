let a = SvPrimaryLiteralIntegral {
    data01: vec![0, 9223372036854775809],
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    size: 63,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.cat(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904, 13835058055282163712],
    size: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);