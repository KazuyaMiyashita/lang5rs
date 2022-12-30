pub enum Elem {
    Operation {
        code: u8,
        pretty_name: str,
    },
    ConstValue {
        code: u8
    },
    FunctionRef {
        name: String
    },
}  