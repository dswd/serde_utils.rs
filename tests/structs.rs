extern crate serde;
extern crate rmp_serde;
#[macro_use] extern crate serde_utils;

use std::fmt::Debug;
use std::io::Cursor;

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

#[allow(unknown_lints,needless_pass_by_value)]
fn test_obj<T: serde::Serialize + serde::Deserialize + PartialEq + Debug>(obj: T) {
    let serialized = to_bytes(&obj);
    let deserialized = from_bytes(&serialized);
    assert_eq!(obj, deserialized);
}

#[derive(Default, Debug, PartialEq)]
struct IntMapTestReduced {
    test: String,
    option: Option<bool>,
}
serde_impl!(IntMapTestReduced(u64) {
    test: String => 0,
    option: Option<bool> => 2
});

#[derive(Default, Debug, PartialEq)]
struct IntMapTest {
    test: String,
    num: u64,
    option: Option<bool>,
}
serde_impl!(IntMapTest(u64) {
    test: String => 0,
    num: u64 => 1,
    option: Option<bool> => 2
});

#[derive(Default, Debug, PartialEq)]
struct CompressedIntMapTest {
    test: String,
    num: u64,
    option: Option<bool>,
}
serde_impl!(CompressedIntMapTest(u64?) {
    test: String => 0,
    num: u64 => 1,
    option: Option<bool> => 2
});

#[test]
fn test_int_map() {
    test_obj(IntMapTest{test: "".to_string(), num: 0, option: None});
    test_obj(IntMapTest{test: "test".to_string(), num: 56, option: Some(true)});
}

#[test]
fn test_int_map_reduced() {
    let bytes = to_bytes(&IntMapTestReduced{test: "test".to_string(), option: Some(true)});
    let obj = from_bytes(&bytes);
    assert_eq!(IntMapTest{test: "test".to_string(), num: 0, option: Some(true)}, obj);
}

#[test]
fn test_int_map_extended() {
    let bytes = to_bytes(&IntMapTest{test: "test".to_string(), num: 56, option: Some(true)});
    let obj = from_bytes(&bytes);
    assert_eq!(IntMapTestReduced{test: "test".to_string(), option: Some(true)}, obj);
}

#[test]
fn test_compressed_int_map() {
    test_obj(CompressedIntMapTest{test: "".to_string(), num: 0, option: None});
    test_obj(CompressedIntMapTest{test: "test".to_string(), num: 56, option: Some(true)});
    assert_eq!(to_bytes(&CompressedIntMapTest::default()).len(), 1);
}

#[derive(Default, Debug, PartialEq)]
struct StrMapTest {
    test: String,
    num: u64,
    option: Option<bool>,
}
serde_impl!(StrMapTest(String) {
    test: String => "test",
    num: u64 => "num",
    option: Option<bool> => "option"
});

#[derive(Default, Debug, PartialEq)]
struct StrMapTestReduced {
    test: String,
    option: Option<bool>,
}
serde_impl!(StrMapTestReduced(String) {
    test: String => "test",
    option: Option<bool> => "option"
});

#[test]
fn test_str_map() {
    test_obj(StrMapTest{test: "".to_string(), num: 0, option: None});
    test_obj(StrMapTest{test: "test".to_string(), num: 56, option: Some(true)});
}

#[test]
fn test_str_map_reduced() {
    let bytes = to_bytes(&StrMapTestReduced{test: "test".to_string(), option: Some(true)});
    let obj = from_bytes(&bytes);
    assert_eq!(StrMapTest{test: "test".to_string(), num: 0, option: Some(true)}, obj);
}

#[test]
fn test_str_map_extended() {
    let bytes = to_bytes(&StrMapTest{test: "test".to_string(), num: 56, option: Some(true)});
    let obj = from_bytes(&bytes);
    assert_eq!(StrMapTestReduced{test: "test".to_string(), option: Some(true)}, obj);
}

#[derive(Default, Debug, PartialEq)]
struct TupleTest {
    test: String,
    num: u64,
    option: Option<bool>,
}
serde_impl!(TupleTest {
    test: String,
    num: u64,
    option: Option<bool>
});

#[derive(Default, Debug, PartialEq)]
struct TupleTestReduced {
    test: String,
    option: Option<bool>,
}
serde_impl!(TupleTestReduced {
    test: String,
    option: Option<bool>
});

#[test]
fn test_tuple() {
    test_obj(TupleTest{test: "".to_string(), num: 0, option: None});
    test_obj(TupleTest{test: "test".to_string(), num: 56, option: Some(true)});
}

#[test]
#[should_panic]
fn test_tuple_reduced() {
    let bytes = to_bytes(&TupleTestReduced{test: "test".to_string(), option: Some(true)});
    let obj = from_bytes(&bytes);
    assert_eq!(TupleTest{test: "test".to_string(), num: 0, option: Some(true)}, obj);
}

#[test]
#[should_panic]
fn test_tuple_extended() {
    let bytes = to_bytes(&TupleTest{test: "test".to_string(), num: 56, option: Some(true)});
    let obj = from_bytes(&bytes);
    assert_eq!(TupleTestReduced{test: "test".to_string(), option: Some(true)}, obj);
}
