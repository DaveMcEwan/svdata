let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: true,
};

let b: SvPrimaryLiteral = a.add_usize(4611686018427387904);

let actual_string = format!("{}", b);