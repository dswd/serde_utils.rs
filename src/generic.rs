use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::fmt;
use std::cmp::Ordering;

use serde::bytes::ByteBuf;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, SeqVisitor, MapVisitor, Error};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Obj {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Str(String),
    Bin(ByteBuf),
    List(Vec<Obj>),
    Map(BTreeMap<Obj, Obj>)
}

impl Obj {
    fn type_num(&self) -> u8 {
        match self {
            &Obj::Null => 0,
            &Obj::Bool(_) => 1,
            &Obj::Unsigned(_) => 2,
            &Obj::Signed(_) => 3,
            &Obj::Float(_) => 4,
            &Obj::Str(_) => 5,
            &Obj::Bin(_) => 6,
            &Obj::List(_) => 7,
            &Obj::Map(_) => 8,
        }
    }
}

impl Default for Obj {
    #[inline(always)]
    fn default() -> Obj {
        Obj::Null
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        match self {
            &Obj::Null => if let &Obj::Null = other { true } else { false },
            &Obj::Bool(val) => if let &Obj::Bool(oval) = other { val == oval } else { false },
            &Obj::Unsigned(val) => if let &Obj::Unsigned(oval) = other { val == oval } else { false },
            &Obj::Signed(val) => if let &Obj::Signed(oval) = other { val == oval } else { false },
            &Obj::Float(val) => if let &Obj::Float(oval) = other { ! (val != oval) } else { false },
            &Obj::Str(ref val) => if let &Obj::Str(ref oval) = other { val == oval } else { false },
            &Obj::Bin(ref val) => if let &Obj::Bin(ref oval) = other { val == oval } else { false },
            &Obj::List(ref val) => if let &Obj::List(ref oval) = other { val == oval } else { false },
            &Obj::Map(ref val) => if let &Obj::Map(ref oval) = other { val == oval } else { false },
        }
    }
}

impl Eq for Obj {}

impl PartialOrd for Obj {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Obj {
    fn cmp(&self, other: &Self) -> Ordering {
        let stype = self.type_num();
        let otype = other.type_num();
        if stype != otype {
            return stype.cmp(&otype);
        }
        match self {
            &Obj::Null => Ordering::Equal,
            &Obj::Bool(val) => if let &Obj::Bool(oval) = other {
                val.cmp(&oval)
            } else {
                unreachable!()
            },
            &Obj::Unsigned(val) => if let &Obj::Unsigned(oval) = other {
                val.cmp(&oval)
            } else {
                unreachable!()
            },
            &Obj::Signed(val) => if let &Obj::Signed(oval) = other {
                val.cmp(&oval)
            } else {
                unreachable!()
            },
            &Obj::Float(val) => if let &Obj::Float(oval) = other {
                if !val.is_nan() && !oval.is_nan() {
                    val.partial_cmp(&oval).unwrap()
                } else if val.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else {
                unreachable!()
            },
            &Obj::Str(ref val) => if let &Obj::Str(ref oval) = other {
                val.cmp(&oval)
            } else {
                unreachable!()
            },
            &Obj::Bin(ref val) => if let &Obj::Bin(ref oval) = other {
                val.cmp(&oval)
            } else {
                unreachable!()
            },
            &Obj::List(ref val) => if let &Obj::List(ref oval) = other {
                val.cmp(&oval)
            } else {
                unreachable!()
            },
            &Obj::Map(ref val) => if let &Obj::Map(ref oval) = other {
                val.cmp(&oval)
            } else {
                unreachable!()
            },
        }
    }
}

impl Hash for Obj {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_u8(self.type_num());
        match self {
            &Obj::Null => (),
            &Obj::Bool(val) => state.write_u8(if val { 1 } else { 0 }),
            &Obj::Unsigned(val) => state.write_u64(val),
            &Obj::Signed(val) => state.write_i64(val),
            &Obj::Float(val) => state.write_u64(unsafe { transmute(val) }),
            &Obj::Str(ref val) => val.hash(state),
            &Obj::Bin(ref val) => val.hash(state),
            &Obj::List(ref val) => val.hash(state),
            &Obj::Map(ref val) => val.hash(state),
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
        let mut map = BTreeMap::new();
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
