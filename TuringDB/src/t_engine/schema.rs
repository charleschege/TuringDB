// Get structure from file instead of making it a `pub` type
#[allow(unused_variables)]
#[derive(Debug, Serialize, Deserialize)]
enum Structure {
    Schemaless,
    Schema,
    Vector,
}
enum SchemaType {
    /// fields `MUST` contain values `field_name: DataType`
    StructType,
    /// fields `CAN` contain values `field_name(DataType)` or `field_name{_: DataType`
    EnumType,
    TupleStructType,
    VecType(DataType),
    ArrayType {
        length: usize,
        data: DataType,
    },
    //TODO std::collections
}

struct List;

enum DataType {
    Char,
    CoW,
    StaticStr,
    Usize,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Bool(BoolFormat),
    DateTimeType(DateTimeFormat),
}

enum DateTimeFormat {
    TAI64NType,
    TAI64Type,
    UTC,
    LocalUTC,
}

enum BoolFormat {
    True,
    False,
}
