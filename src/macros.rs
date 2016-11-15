extern crate serde;

/// Macro for implementing (de-)serialization via serde in common cases
///
/// # Using the macro
/// The macro provides implementations for `Serialize` and `Deserialize` for various kinds or
/// data types. The macro syntax tries to stay as close as possible to the declaration of the
/// data type.
/// To use the macro, `serde` must be imported by either `use serde;` or `extern crate serde;`.
///
/// ## (De-)Serializing `struct`s as maps
///
/// To deserialize a struct data type as a map, it must implement the `Default` trait so that
/// missing struct fields still have a value. The macro syntax for this case is:
///
/// ```ignore
/// serde_impl!($name($ktype) {
///    $fname: $ftype => $fkey,
///    ...
/// });
/// ```
///
/// where
///
/// * `$name` is the name of the type to be implemented.
/// * `$ktype` is the type for the keys in the mapping.
/// * `$fname` is the name of a field (on the struct in Rust).
/// * `$ftype` is the type of a field.
/// * `$fkey` is the field key in the map (in serialized form).
///
/// ### Example
///
/// ```ignore
/// #[derive(Default)]
/// struct Test {
///     test: String,
///     num: u64,
///     option: Option<bool>,
/// }
/// serde_impl!(Test(String) {
///     test: String => "test",
///     num: u64 => "num",
///     option: Option<bool> => "option"
/// });
/// ```
///
/// Note that the `$ktype` must be an *owned type* corresponding to the used field keys,
/// i.e. `String` instead of `&str` in this example.
///
/// It is also possible to use numeric field keys (when the serialization supports it, JSON does not).
///
/// ### Example
///
/// ```ignore
/// #[derive(Default)]
/// struct Test {
///     test: String,
///     num: u64,
///     option: Option<bool>,
/// }
/// serde_impl!(Test(u64) {
///     test: String => 0,
///     num: u64 => 1,
///     option: Option<bool> => 2
/// });
/// ```
///
/// When deserializing data, the generated implementation will silently ignore all extra fields
/// and use the default value for all missing fields.
///
/// ## (De-)Serializing `struct`s as tuples
///
/// It is also possible to (de-)serialize structs as tuples containing all the fields in order.
/// The macro syntax for this case is:
///
/// ```ignore
/// serde_impl!($name {
///    $fname: $ftype,
///    ...
/// });
/// ```
///
/// where
///
/// * `$name` is the name of the type to be implemented.
/// * `$fname` is the name of a field (on the struct in Rust).
/// * `$ftype` is the type of a field.
///
/// ### Example
///
/// ```ignore
/// struct Test {
///     test: String,
///     num: u64,
///     option: Option<bool>,
/// }
/// serde_impl!(Test {
///     test: String,
///     num: u64,
///     option: Option<bool>
/// });
/// ```
///
/// The syntax basically just omits the *key type* and the *field keys* as no keys are used.
/// The fields will just be (de-)serialized in order as a tuple.
/// When derserializing a tuple as such a data struct, any missing or extra fields will be treated
/// as an error. Therefore, the struct does not need to implement `Default`.
///
/// ## (De-)Serializing simple `enums`s
///
/// (De-)serializing enums that do not have parameters, just maps the variants to and from a
/// serializable data type. The syntax in this case is:
///
/// ```ignore
/// serde_impl!($name($ktype) {
///    $variant => $fkey,
///    ...
/// });
/// ```
///
/// where
///
/// * `$name` is the name of the type to be implemented.
/// * `$ktype` is the type for the serialized enum variants.
/// * `$variant` is the name of a variant (on the enum in Rust).
/// * `$fkey` is the key for a variant in serialized from.
///
/// ### Example
///
/// ```ignore
/// enum Test {
///     A, B, C
/// }
/// serde_impl!(Test(String) {
///     A => "a",
///     B => "b",
///     C => "c"
/// });
/// ```
///
/// Note that the `$ktype` must be an *owned type* corresponding to the used variant keys,
/// i.e. `String` instead of `&str` in this example.
///
/// It is also possible to use numeric variant keys.
///
/// ### Example
///
/// ```ignore
/// enum Test {
///     A, B, C
/// }
/// serde_impl!(Test(u64) {
///     A => 0,
///     B => 1,
///     C => 2
/// });
/// ```
///
/// ## (De-)Serializing `enums`s with one parameter
///
/// It is also possible to (de-)serialize enums with **exactly one** parameter.
/// The syntax in this case is:
///
/// ```ignore
/// serde_impl!($name($ktype) {
///    $variant($ftype) => $fkey,
///    ...
/// });
/// ```
///
/// where
///
/// * `$name` is the name of the type to be implemented.
/// * `$ktype` is the type for the serialized enum variants.
/// * `$variant` is the name of a variant (on the enum in Rust).
/// * `$ftype` is the type of the variant parameter.
/// * `$fkey` is the key for a variant in serialized from.
///
/// ### Example
///
/// ```ignore
/// enum Test {
///     A(u64), B(String), C(bool)
/// }
/// serde_impl!(Test(String) {
///     A(u64) => "a",
///     B(String) => "b",
///     C(bool) => "c"
/// });
/// ```
///
/// Note that the `$ktype` must be an *owned type* corresponding to the used variant keys,
/// i.e. `String` instead of `&str` in this example.
///
/// It is also possible to use numeric variant keys.
///
/// ### Example
///
/// ```ignore
/// enum Test {
///     A(u64), B(String), C(bool)
/// }
/// serde_impl!(Test(u64) {
///     A(u64) => 0,
///     B(String) => 1,
///     C(bool) => 2
/// });
/// ```
///
/// The limitation to one parameter can be circumvented by wrapping multiple parameters in a tuple:
///
/// ```
/// enum Test {
///    None(()),
///    Single(String),
///    Multiple((u64, bool))
/// }
/// ```
///
/// instead of
///
/// ```
/// enum Test {
///    None,
///    Single(String),
///    Multiple(u64, bool)
/// }
/// ```
///
/// ## Limitations
/// The following things do not work, and most likely will never work:
///
/// * Data types with lifetimes
/// * Parametrized data types
/// * Enums with multiple parameters
/// * Enums where different variants have different parameter counts
/// * Enums with field names
/// * Tuple structs
/// * More fancy key types than String and numeric types might not work
#[macro_export]
macro_rules! serde_impl(
    // Serde impl for struct $name*($ktype) { $fname: $ftype } as map
    ( $name:ident($ktype:ident?) { $( $fname:ident : $ftype:ty => $fkey:expr ),+ } ) => {
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
                let default: $name = Default::default();
                let mut len = 0;
                $(
                    if (self.0).$fname != default.$fname {
                        len += 1;
                    }
                )*
                let mut state = try!(ser.serialize_map(Some(len)));
                $(
                    if (self.0).$fname != default.$fname {
                        try!(ser.serialize_map_key(&mut state, $fkey));
                        try!(ser.serialize_map_value(&mut state, &self.$fname));
                    }
                )*
                ser.serialize_map_end(state)
            }
        }
        impl serde::Deserialize for $name {
            fn deserialize<D: serde::Deserializer>(de: &mut D) -> Result<Self, D::Error> {
                use serde_utils::Obj as _DummyObjToSkipUnknownFields;
                struct _Deserializer;
                impl serde::de::Visitor for _Deserializer {
                    type Value = $name;
                    fn visit_map<V: serde::de::MapVisitor>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error> {
                        let mut obj: $name = Default::default();
                        while let Some(key) = try!(visitor.visit_key::<$ktype>()) {
                            $(
                                if &key == &$fkey {
                                    obj.$fname = try!(visitor.visit_value());
                                    continue
                                }
                            )*
                            let _skip: _DummyObjToSkipUnknownFields = try!(visitor.visit_value());
                        }
                        Ok(obj)
                    }
                }
                Ok(try!(de.deserialize_map(_Deserializer)))
            }
        }
    };
    // Serde impl for struct $name($ktype) { $fname: $ftype } as map
    ( $name:ident($ktype:ident) { $( $fname:ident : $ftype:ty => $fkey:expr ),+ } ) => {
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
                let mut state = try!(ser.serialize_map(Some( [ $( $fkey ),+ ].len() )));
                $(
                    try!(ser.serialize_map_key(&mut state, $fkey));
                    try!(ser.serialize_map_value(&mut state, &self.$fname));
                )*
                ser.serialize_map_end(state)
            }
        }
        impl serde::Deserialize for $name {
            fn deserialize<D: serde::Deserializer>(de: &mut D) -> Result<Self, D::Error> {
                use serde_utils::Obj as _DummyObjToSkipUnknownFields;
                struct _Deserializer;
                impl serde::de::Visitor for _Deserializer {
                    type Value = $name;
                    fn visit_map<V: serde::de::MapVisitor>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error> {
                        let mut obj: $name = Default::default();
                        while let Some(key) = try!(visitor.visit_key::<$ktype>()) {
                            $(
                                if &key == &$fkey {
                                    obj.$fname = try!(visitor.visit_value());
                                    continue
                                }
                            )*
                            let _skip: _DummyObjToSkipUnknownFields = try!(visitor.visit_value());
                        }
                        Ok(obj)
                    }
                }
                Ok(try!(de.deserialize_map(_Deserializer)))
            }
        }
    };
    // Serde impl for struct $name { $fname: $ftype } as tuple
    ( $name:ident { $( $fname:ident : $ftype:ty ),+ } ) => {
        impl serde::Serialize for $name {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
                ($( &self.$fname ),*).serialize(ser)
            }
        }
        impl serde::Deserialize for $name {
            #[inline]
            fn deserialize<D: serde::Deserializer>(de: &mut D) -> Result<Self, D::Error> {
                type T = ( $($ftype),* );
                T::deserialize(de).map(|( $($fname),* )| $name { $( $fname: $fname ),* })
            }
        }
    };
    // Serde impl for enum $name { $variant }
    ( $name:ident($ktype:ident) { $( $variant:ident => $fkey:expr ),+ } ) => {
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
                match self {
                    $( &$name::$variant => $fkey ),*
                }.serialize(ser)
            }
        }
        impl serde::Deserialize for $name {
            fn deserialize<D: serde::Deserializer>(de: &mut D) -> Result<Self, D::Error> {
                use serde::de::Error as _DummyErrorJustToUseTrait;
                let key = try!($ktype::deserialize(de));
                $(
                    if &key == &$fkey {
                        return Ok($name::$variant);
                    }
                )*
                Err(D::Error::unknown_variant("Invalid enum"))
            }
        }
    };
    // Serde impl for enum $name { $variant($ftype) }
    ( $name:ident($ktype:ident) { $( $variant:ident($ftype:ty) => $fkey:expr ),* } ) => {
        impl serde::Serialize for $name {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
                match self {
                    $( &$name::$variant(ref obj) => ($fkey, obj).serialize(ser) ),*
                }
            }
        }
        impl serde::Deserialize for $name {
            #[inline]
            fn deserialize<D: serde::Deserializer>(de: &mut D) -> Result<Self, D::Error> {
                struct _Deserializer;
                impl serde::de::Visitor for _Deserializer {
                    type Value = $name;
                    fn visit_seq<V: serde::de::SeqVisitor>(&mut self, mut visitor: V) -> Result<$name, V::Error> {
                        use serde::de::Error as _DummyErrorJustToUseTrait;
                        let key: $ktype = try!(try!(visitor.visit()).ok_or(V::Error::end_of_stream()));
                        $(
                            if &key == &$fkey {
                                return Ok($name::$variant(try!(try!(visitor.visit()).ok_or(V::Error::end_of_stream()))));
                            }
                        )*
                        Err(V::Error::unknown_variant("Invalid enum"))
                    }
                }
                de.deserialize_tuple(2, _Deserializer)
            }
        }
    };
);
