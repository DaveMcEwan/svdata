let a: SvPrimaryLiteral = usize_to_primlit(4611686018427387904);

let exp = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 64,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);