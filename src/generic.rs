use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::fmt;

use serde::bytes::ByteBuf;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, SeqVisitor, MapVisitor, Error};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Obj {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Str(String),
    Bin(ByteBuf),
    List(Vec<Obj>),
    Map(HashMap<Obj, Obj>)
}

impl Default for Obj {
    #[inline(always)]
    fn default() -> Obj {
        Obj::Null
    }
}

impl Eq for Obj {}

impl Hash for Obj {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        match self {
            &Obj::Null => state.write_u8(0),
            &Obj::Bool(val) => {
                state.write_u8(1);
                state.write_u8(if val { 1 } else { 0 });
            },
            &Obj::Unsigned(val) => {
                state.write_u8(2);
                state.write_u64(val);
            },
            &Obj::Signed(val) => {
                state.write_u8(3);
                state.write_i64(val);
            },
            &Obj::Float(val) => {
                state.write_u8(4);
                state.write_u64(unsafe { transmute(val) });
            },
            &Obj::Str(ref val) => {
                state.write_u8(5);
                val.hash(state);
            },
            &Obj::Bin(ref val) => {
                state.write_u8(6);
                val.hash(state);
            },
            &Obj::List(ref val) => {
                state.write_u8(7);
                for value in val {
                    value.hash(state);
                }
            },
            &Obj::Map(ref val) => {
                state.write_u8(8);
                for (key, value) in val {
                    key.hash(state);
                    value.hash(state);
                }
            }
        }
    }
}

impl Serialize for Obj {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
        match self {
            &Obj::Null => ser.visit_none(),
            &Obj::Bool(val) => ser.visit_bool(val),
            &Obj::Unsigned(val) => ser.visit_u64(val),
            &Obj::Signed(val) => ser.visit_i64(val),
            &Obj::Float(val) => ser.visit_f64(val),
            &Obj::Str(ref val) => ser.visit_str(val),
            &Obj::Bin(ref val) => ser.visit_bytes(val),
            &Obj::List(ref val) => val.serialize(ser),
            &Obj::Map(ref val) => val.serialize(ser)
        }
    }
}

struct GenericVisitor;

impl Visitor for GenericVisitor {
    type Value = Obj;

    #[inline(always)]
    fn visit_none<E: Error>(&mut self) -> Result<Self::Value, E> {
        Ok(Obj::Null)
    }

    #[inline(always)]
    fn visit_bool<E: Error>(&mut self, val: bool) -> Result<Self::Value, E> {
        Ok(Obj::Bool(val))
    }

    #[inline(always)]
    fn visit_u64<E: Error>(&mut self, val: u64) -> Result<Self::Value, E> {
        Ok(Obj::Unsigned(val))
    }

    #[inline(always)]
    fn visit_i64<E: Error>(&mut self, val: i64) -> Result<Self::Value, E> {
        Ok(Obj::Signed(val))
    }

    #[inline(always)]
    fn visit_f64<E: Error>(&mut self, val: f64) -> Result<Self::Value, E> {
        Ok(Obj::Float(val))
    }

    #[inline(always)]
    fn visit_str<E: Error>(&mut self, val: &str) -> Result<Self::Value, E> {
        Ok(Obj::Str(val.to_owned()))
    }

    #[inline(always)]
    fn visit_string<E: Error>(&mut self, val: String) -> Result<Self::Value, E> {
        Ok(Obj::Str(val))
    }

    #[inline(always)]
    fn visit_bytes<E: Error>(&mut self, val: &[u8]) -> Result<Self::Value, E> {
        let mut bin = Vec::with_capacity(val.len());
        bin.extend(val.iter().cloned());
        Ok(Obj::Bin(ByteBuf::from(bin)))
    }

    #[inline(always)]
    fn visit_byte_buf<E: Error>(&mut self, val: Vec<u8>) -> Result<Self::Value, E> {
        Ok(Obj::Bin(ByteBuf::from(val)))
    }

    #[inline(always)]
    fn visit_unit<E: Error>(&mut self) -> Result<Self::Value, E> {
        Ok(Obj::Null)
    }

    #[inline]
    fn visit_seq<V: SeqVisitor>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error> {
        let mut list = Vec::with_capacity(visitor.size_hint().0);
        while let Some(value) = try!(visitor.visit()) {
            list.push(value);
        }
        try!(visitor.end());
        Ok(Obj::List(list))
    }

    #[inline]
    fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error> {
        let mut map = HashMap::with_capacity(visitor.size_hint().0);
        while let Some((key, value)) = try!(visitor.visit()) {
            map.insert(key, value);
        }
        try!(visitor.end());
        Ok(Obj::Map(map))
    }
}

impl Deserialize for Obj {
    #[inline(always)]
    fn deserialize<D: Deserializer>(de: &mut D) -> Result<Self, D::Error> {
        de.visit(GenericVisitor)
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Obj::Null => write!(f, "null"),
            &Obj::Bool(val) => write!(f, "{}", val),
            &Obj::Unsigned(val) => write!(f, "{}", val),
            &Obj::Signed(val) => write!(f, "{}", val),
            &Obj::Float(val) => write!(f, "{}", val),
            &Obj::Str(ref val) => write!(f, "{}", val),
            &Obj::Bin(ref val) => write!(f, "{:?}", val),
            &Obj::List(ref val) => write!(f, "{:?}", val),
            &Obj::Map(ref val) => write!(f, "{:?}", val),
        }
    }
}
