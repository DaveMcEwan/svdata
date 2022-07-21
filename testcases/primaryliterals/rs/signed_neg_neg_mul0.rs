let a = SvPrimaryLiteral {
    data01: vec![3],
    size: 2,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4],
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![4],
    size: 4,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);