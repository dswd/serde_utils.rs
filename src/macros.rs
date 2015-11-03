extern crate serde;

#[macro_export]
macro_rules! serde_impl(
    // Serde impl for struct $name { $fname: $ftype } as map
    ( $name:ident($ktype:ident) { $( $fname:ident : $ftype:ty => $fkey:expr ),+ } ) => {
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
                struct _Serializer<'a>(&'a $name);
                impl<'a> serde::ser::MapVisitor for _Serializer<'a> {
                    fn visit<S: serde::Serializer>(&mut self, ser: &mut S) -> Result<Option<()>, S::Error> {
                        $( try!(ser.visit_map_elt($fkey, &(self.0).$fname)); )*
                        Ok(None)
                    }
                    #[inline]
                    fn len(&self) -> Option<usize> {
                        // This should be optimized into a number
                        Some( [ $( $fkey ),+ ].len() )
                    }
                }
                ser.visit_map(_Serializer(self))

            }
        }
        impl serde::Deserialize for $name {
            fn deserialize<D: serde::Deserializer>(de: &mut D) -> Result<Self, D::Error> {
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
                            let _skip: serde_utils::Obj = try!(visitor.visit_value());
                        }
                        Ok(obj)
                    }
                }
                Ok(try!(de.visit_map(_Deserializer)))
            }
        }
    };
    // Serde impl for struct $name { $fname: $ftype } as tuple
    ( $name:ident { $( $fname:ident : $ftype:ty ),+ } ) => {
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
                ($( &self.$fname ),*).serialize(ser)
            }
        }
        impl serde::Deserialize for $name {
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
                use serde::de::Error;
                let key = try!($ktype::deserialize(de));
                $(
                    if &key == &$fkey {
                        return Ok($name::$variant);
                    }
                )*
                Err(D::Error::syntax("Invalid enum"))
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
                        use serde::de::Error;
                        let key: $ktype = try!(try!(visitor.visit()).ok_or(V::Error::end_of_stream()));
                        $(
                            if &key == &$fkey {
                                return Ok($name::$variant(try!(try!(visitor.visit()).ok_or(V::Error::end_of_stream()))));
                            }
                        )*
                        Err(V::Error::syntax("Invalid enum"))
                    }
                }
                de.visit_tuple(2, _Deserializer)
            }
        }
    };
);
