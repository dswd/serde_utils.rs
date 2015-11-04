extern crate serde;
extern crate rmp_serde;
#[macro_use] extern crate serde_utils;

use std::fmt::Debug;
use std::io::Cursor;
use std::collections::BTreeMap;
use std::f64;

use serde::bytes::ByteBuf;
use serde_utils::Obj;

fn to_bytes<T: serde::Serialize + Debug>(obj: &T) -> Vec<u8> {
    let mut serialized = Vec::new();
    {
        let mut writer = rmp_serde::Serializer::new(&mut serialized);
        assert!(obj.serialize(&mut writer).is_ok());
    }
    serialized
}

fn from_bytes<T: serde::Deserialize + Debug>(bytes: &[u8]) -> T {
    let cursor = Cursor::new(bytes);
    let mut reader = rmp_serde::Deserializer::new(cursor);
    T::deserialize(&mut reader).unwrap()
}

fn test_obj<T: serde::Serialize + serde::Deserialize + PartialEq + Debug>(obj: T) {
    let serialized = to_bytes(&obj);
    let deserialized = from_bytes(&serialized);
    assert_eq!(obj, deserialized);
}

#[test]
fn test_none() {
    test_obj(Obj::Null);
}

#[test]
fn test_bool() {
    test_obj(Obj::Bool(true));
    test_obj(Obj::Bool(false));
}

#[test]
fn test_numeric() {
    test_obj(Obj::Unsigned(0));
    test_obj(Obj::Unsigned(1));
    test_obj(Obj::Unsigned(4352));
    test_obj(Obj::Signed(0));
    test_obj(Obj::Signed(-1));
    test_obj(Obj::Signed(-4352));
    test_obj(Obj::Float(0.0));
    test_obj(Obj::Float(1.0));
    test_obj(Obj::Float(-345.4434));
}

#[test]
fn test_string() {
    test_obj(Obj::Str("test".to_string()));
    test_obj(Obj::Str("".to_string()));
    test_obj(Obj::Str("\n".to_string()));
}

#[test]
fn test_binary() {
    test_obj(Obj::Bin(serde::bytes::ByteBuf::from(vec![1,2,3,4])));
    test_obj(Obj::Bin(serde::bytes::ByteBuf::from(vec![])));
    test_obj(Obj::Bin(serde::bytes::ByteBuf::from(vec![0,1,2,3,4])));
}

#[test]
fn test_list() {
    test_obj(Obj::List(vec![Obj::Unsigned(1), Obj::Unsigned(2)]));
    test_obj(Obj::List(vec![Obj::Unsigned(1), Obj::Null]));
    test_obj(Obj::List(vec![]));
}

macro_rules! map(
    { $( $key:expr => $val:expr ),* } => {
        {
            let mut _map = BTreeMap::new();
            $(
                _map.insert($key, $val);
            )*
            _map
        }
    }
);

#[test]
fn test_map() {
    test_obj(Obj::Map(map!{}));
    test_obj(Obj::Map(map!{
        Obj::Unsigned(1) => Obj::Str("test1".to_owned()),
        Obj::Unsigned(2) => Obj::Str("test2".to_owned())
    }));
    test_obj(Obj::Map(map!{
        Obj::Unsigned(1) => Obj::Str("test1".to_owned()),
        Obj::Unsigned(2) => Obj::Bool(false)
    }));
    test_obj(Obj::Map(map!{
        Obj::Unsigned(1) => Obj::Str("test1".to_owned()),
        Obj::Str("blah".to_owned()) => Obj::Bool(false)
    }));
}

#[test]
fn test_eq() {
    assert_eq!(Obj::Null, Obj::Null);
    assert_eq!(Obj::Bool(true), Obj::Bool(true));
    assert!(Obj::Bool(true) != Obj::Bool(false));
    assert_eq!(Obj::Unsigned(1), Obj::Unsigned(1));
    assert!(Obj::Unsigned(1) != Obj::Unsigned(2));
    assert_eq!(Obj::Signed(-1), Obj::Signed(-1));
    assert!(Obj::Signed(-1) != Obj::Signed(-2));
    assert_eq!(Obj::Signed(1), Obj::Signed(1));
    assert_eq!(Obj::Signed(1), Obj::Unsigned(1));
    assert_eq!(Obj::Signed(0), Obj::Unsigned(0));
    assert_eq!(Obj::Float(1.2), Obj::Float(1.2));
    assert!(Obj::Float(3.5) != Obj::Float(1.2));
    assert!(Obj::Float(f64::NAN) != Obj::Float(0.0));
    assert_eq!(Obj::Float(f64::NAN), Obj::Float(f64::NAN));
    assert_eq!(Obj::Str("test".to_string()), Obj::Str("test".to_string()));
    assert!(Obj::Str("test".to_string()) != Obj::Str("test2".to_string()));
    assert_eq!(Obj::Bin(ByteBuf::from(vec![0,2,3])), Obj::Bin(ByteBuf::from(vec![0,2,3])));
    assert!(Obj::Bin(ByteBuf::from(vec![0,2,3])) != Obj::Bin(ByteBuf::from(vec![0,1,3])));
    assert_eq!(Obj::List(vec![Obj::Null]), Obj::List(vec![Obj::Null]));
    assert!(Obj::List(vec![Obj::Null]) != Obj::List(vec![Obj::Bool(false)]));
    assert!(Obj::List(vec![Obj::Null]) != Obj::List(vec![Obj::Null, Obj::Bool(false)]));
    assert_eq!(Obj::List(vec![Obj::Unsigned(2)]), Obj::List(vec![Obj::Signed(2)]));
    assert_eq!(Obj::Map(map!{Obj::Unsigned(1) => Obj::Null}), Obj::Map(map!{Obj::Unsigned(1) => Obj::Null}));
    assert!(Obj::Map(map!{Obj::Unsigned(1) => Obj::Null}) != Obj::Map(map!{Obj::Unsigned(1) => Obj::Bool(false)}));
    assert_eq!(Obj::Map(map!{Obj::Unsigned(1) => Obj::Null}), Obj::Map(map!{Obj::Signed(1) => Obj::Null}));
}

macro_rules! check_ord(
    [ $( $val:expr ),* ] => {
        {
            let vals = vec![ $( $val ),* ];
            for i in 1..vals.len() {
                assert!(vals[i-1] < vals[i]);
                assert!(vals[i] > vals[i-1]);
            }
        }
    }
);

#[test]
fn test_ord() {
    check_ord![
        Obj::Null,
        Obj::Bool(false), Obj::Bool(true),
        Obj::Signed(-32), Obj::Signed(-2), Obj::Signed(0),
        Obj::Unsigned(1), Obj::Signed(2), Obj::Unsigned(23),
        Obj::Float(-323.0), Obj::Float(0.0), Obj::Float(224.0), Obj::Float(f64::NAN),
        Obj::Str("".to_string()), Obj::Str("a".to_string()), Obj::Str("aa".to_string()), Obj::Str("b".to_string()),
        Obj::Bin(ByteBuf::from(vec![])), Obj::Bin(ByteBuf::from(vec![0])), Obj::Bin(ByteBuf::from(vec![0, 1])), Obj::Bin(ByteBuf::from(vec![1])),
        Obj::List(vec![]), Obj::List(vec![Obj::Null]), Obj::List(vec![Obj::Bool(false)]),
        Obj::Map(map!{}), Obj::Map(map!{Obj::Null => Obj::Bool(false)}), Obj::Map(map!{Obj::Null => Obj::Bool(true)})
    ];
}
