let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 63,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 64,
    signed: false,
};

let c: bool = a > b;

let actual_string = format!("{}", c);