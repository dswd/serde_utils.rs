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

fn from_bytes<'a, T: serde::Deserialize<'a> + Debug>(bytes: &[u8]) -> T {
    let cursor = Cursor::new(bytes);
    let mut reader = rmp_serde::Deserializer::new(cursor);
    T::deserialize(&mut reader).unwrap()
}

#[allow(unknown_lints,needless_pass_by_value)]
fn test_obj<'a, T: serde::Serialize + serde::Deserialize<'a> + PartialEq + Debug>(obj: T) {
    let serialized = to_bytes(&obj);
    let deserialized = from_bytes(&serialized);
    assert_eq!(obj, deserialized);
}

#[derive(PartialEq, Debug)]
enum IntEnumTest {
    A, B, C
}
serde_impl!(IntEnumTest(u64) {
    A => 0,
    B => 1,
    C => 2
});

#[derive(PartialEq, Debug)]
enum IntEnumTestReduced {
    A, C
}
serde_impl!(IntEnumTestReduced(u64) {
    A => 0,
    C => 2
});

#[test]
fn test_int_enum() {
    test_obj(IntEnumTest::A);
    test_obj(IntEnumTest::B);
    test_obj(IntEnumTest::C);
}

#[test]
fn test_int_enum_reduced() {
    assert_eq!(IntEnumTest::A, from_bytes(&to_bytes(&IntEnumTestReduced::A)));
    assert_eq!(IntEnumTest::C, from_bytes(&to_bytes(&IntEnumTestReduced::C)));
}

#[test]
fn test_int_enum_extended() {
    assert_eq!(IntEnumTestReduced::A, from_bytes(&to_bytes(&IntEnumTest::A)));
    assert_eq!(IntEnumTestReduced::C, from_bytes(&to_bytes(&IntEnumTest::C)));
}


#[derive(PartialEq, Debug)]
enum StrEnumTest {
    A, B, C
}
serde_impl!(StrEnumTest(String) {
    A => "a",
    B => "b",
    C => "c"
});

#[derive(PartialEq, Debug)]
enum StrEnumTestReduced {
    A, C
}
serde_impl!(StrEnumTestReduced(String) {
    A => "a",
    C => "c"
});

#[test]
fn test_str_enum() {
    test_obj(StrEnumTest::A);
    test_obj(StrEnumTest::B);
    test_obj(StrEnumTest::C);
}

#[test]
fn test_str_enum_reduced() {
    assert_eq!(StrEnumTest::A, from_bytes(&to_bytes(&StrEnumTestReduced::A)));
    assert_eq!(StrEnumTest::C, from_bytes(&to_bytes(&StrEnumTestReduced::C)));
}

#[test]
fn test_str_enum_extended() {
    assert_eq!(StrEnumTestReduced::A, from_bytes(&to_bytes(&StrEnumTest::A)));
    assert_eq!(StrEnumTestReduced::C, from_bytes(&to_bytes(&StrEnumTest::C)));
}


#[derive(PartialEq, Debug)]
enum IntParamEnumTest {
    A(u64), B(bool), C(String)
}
serde_impl!(IntParamEnumTest(u64) {
    A(u64) => 0,
    B(bool) => 1,
    C(String) => 2
});

#[derive(PartialEq, Debug)]
enum IntParamEnumTestReduced {
    A(u64), C(String)
}
serde_impl!(IntParamEnumTestReduced(u64) {
    A(u64) => 0,
    C(String) => 2
});

#[test]
fn test_int_param_enum() {
    test_obj(IntParamEnumTest::A(53));
    test_obj(IntParamEnumTest::B(true));
    test_obj(IntParamEnumTest::C("test".to_string()));
}

#[test]
fn test_int_param_enum_reduced() {
    assert_eq!(IntParamEnumTest::A(53), from_bytes(&to_bytes(&IntParamEnumTestReduced::A(53))));
    assert_eq!(IntParamEnumTest::C("test".to_string()), from_bytes(&to_bytes(&IntParamEnumTestReduced::C("test".to_string()))));
}

#[test]
fn test_int_param_enum_extended() {
    assert_eq!(IntParamEnumTestReduced::A(53), from_bytes(&to_bytes(&IntParamEnumTest::A(53))));
    assert_eq!(IntParamEnumTestReduced::C("test".to_string()), from_bytes(&to_bytes(&IntParamEnumTest::C("test".to_string()))));
}


#[derive(PartialEq, Debug)]
enum StrParamEnumTest {
    A(u64), B(bool), C(String)
}
serde_impl!(StrParamEnumTest(String) {
    A(u64) => "a",
    B(bool) => "b",
    C(String) => "c"
});

#[derive(PartialEq, Debug)]
enum StrParamEnumTestReduced {
    A(u64), C(String)
}
serde_impl!(StrParamEnumTestReduced(String) {
    A(u64) => "a",
    C(String) => "c"
});

#[test]
fn test_str_param_enum() {
    test_obj(StrParamEnumTest::A(53));
    test_obj(StrParamEnumTest::B(true));
    test_obj(StrParamEnumTest::C("test".to_string()));
}

#[test]
fn test_str_param_enum_reduced() {
    assert_eq!(StrParamEnumTest::A(53), from_bytes(&to_bytes(&StrParamEnumTestReduced::A(53))));
    assert_eq!(StrParamEnumTest::C("test".to_string()), from_bytes(&to_bytes(&StrParamEnumTestReduced::C("test".to_string()))));
}

#[test]
fn test_str_param_enum_extended() {
    assert_eq!(StrParamEnumTestReduced::A(53), from_bytes(&to_bytes(&StrParamEnumTest::A(53))));
    assert_eq!(StrParamEnumTestReduced::C("test".to_string()), from_bytes(&to_bytes(&StrParamEnumTest::C("test".to_string()))));
}
