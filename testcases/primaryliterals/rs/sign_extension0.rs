let mut a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

a._sign_extension();

let actual_string = format!("{}", a);