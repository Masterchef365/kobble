use bincode::Options as BincodeOptions;
use deserialize::read_dynamic;
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::SeqAccess;
use serde::{de, ser};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::rc::Rc;
use std::{borrow::BorrowMut, collections::HashMap, io::Read, marker::PhantomData};

mod deserialize;
mod error;
mod schema_recorder;

/// Representation of a data serde-compatible data structure
#[derive(Debug, Clone)]
pub enum Schema {
    Str,
    Seq,
    Map,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    F32,
    F64,
    Bool,
    Char,
    Unit,
    Bytes,
    Option,
    ByteBuf,
    String,
    Struct(StructSchema),
}

/// Represents a struct
#[derive(Debug, Clone)]
pub struct StructSchema {
    name: String,
    fields: Vec<(String, Schema)>,
}

/// Runtime-modifiable representation of a data structure
#[derive(Debug, Clone)]
pub enum DynamicValue {
    Str(String),
    Seq(Vec<DynamicValue>),
    Map(HashMap<String, DynamicValue>),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    Unit(()),
    Bytes(Vec<u8>),
    Option(Option<Box<DynamicValue>>),
    ByteBuf(Vec<u8>),
    String(String),
    Struct {
        name: String,
        fields: Vec<(String, DynamicValue)>,
    },
}

/// Use bincode to read the given structure based on its schema
pub fn bincode_read_dynamic<R: Read>(schema: Schema, reader: R) -> bincode::Result<DynamicValue> {
    let mut deser = bincode::Deserializer::with_reader(reader, bincode_opts());
    Ok(read_dynamic(schema, &mut deser).unwrap())
}

fn bincode_opts() -> impl BincodeOptions {
    // NOTE: This is actually different from the default bincode serialize() function!!
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
}

#[cfg(test)]
mod tests {
    use crate::schema_recorder::record_schema;

    use super::*;

    #[test]
    fn it_works() {
        #[derive(Serialize, Deserialize)]
        struct A {
            a: i32,
            b: B,
        }

        #[derive(Serialize, Deserialize)]
        struct B {
            c: i32,
        }

        let schema = record_schema::<A>().unwrap();
        dbg!(&schema);

        let instance = A {
            a: 99,
            b: B { c: 23480 },
        };

        let bytes = bincode::serialize(&instance).unwrap();

        let dynamic = bincode_read_dynamic(schema, std::io::Cursor::new(bytes)).unwrap();

        dbg!(dynamic);

        panic!();
    }
}
