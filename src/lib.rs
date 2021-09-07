pub mod validation {
    use dyn_clone::DynClone;
    use serde::{Serialize, Serializer};
    use serde_json;
    use std::any::Any;
    use std::fmt;
    use std::fmt::{Debug, Formatter};

    dyn_clone::clone_trait_object!(ObjectTrait);

    macro_rules! obj_trait {
    ( $( $t:ident ),* ) => {
            $(
                impl ObjectTrait for $t {
                    fn as_any(&self) -> &dyn Any {
                        self
                    }
                }

                impl AddRequire<Option<$t>> for Validator<'_> {
                    fn require(&mut self, v: Option<$t>) -> &mut Self {
                        self.err = Error::new();

                        match v {
                            Some(vv) => {
                                self.err.value = Box::new(vv.clone());
                            }
                            None => {}
                        }

                        self
                    }

                    fn build(&mut self, v: Option<$t>) {
                        match self.err.value.as_any().downcast_ref::<$t>() {
                            Some(d) => match v {
                                Some(vv) => {
                                    if vv != *d {
                                        self.err.ok = false;
                                        self.err_vec.push(self.err.clone());
                                    }
                                }
                                None => {}
                            },
                            None => {
                                self.err.value = Box::new("invalid type".to_string());
                                self.err.ok = false;
                                self.err_vec.push(self.err.clone());
                            }
                        }
                    }

                }


            impl AddRequire<$t> for Validator<'_> {
            fn require(&mut self, v: $t) -> &mut Self {
                self.err = Error::new();
                self.err.value = Box::new(v.clone());
                self
            }

            fn build(&mut self, v: $t) {
                match self.err.value.as_any().downcast_ref::<$t>() {
                    Some(d) => {
                        if *d != v {
                            self.err.ok = false;
                            self.err_vec.push(self.err.clone());
                        }
                    }
                    None => {
                        self.err.value = Box::new("invalid type".to_string());
                        self.err.ok = false;
                        self.err_vec.push(self.err.clone());
                    }
                }
            }
        }


            )*
    };
}

    obj_trait!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, String);

    trait ObjectTrait: Any + DynClone {
        fn as_any(&self) -> &dyn Any;
    }
    pub trait AddRequire<T> {
        fn require(&mut self, v: T) -> &mut Self;
        fn build(&mut self, v: T);
    }

    #[derive(Debug, Serialize, Clone)]
    pub struct Error<'a> {
        title: &'a str,
        message: &'a str,
        value: Box<dyn ObjectTrait>,
        ok: bool,
    }

    pub struct Validator<'a> {
        err_vec: Vec<Error<'a>>,
        err: Error<'a>,
    }

    impl Error<'_> {
        fn new() -> Self {
            Self {
                title: "",
                message: "",
                value: Box::new("".to_string()),
                ok: true,
            }
        }
    }

    impl Serialize for dyn ObjectTrait {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s;
            if let Some(d) = self.as_any().downcast_ref::<String>() {
                s = serializer.serialize_str(d)
            } else if let Some(d) = self.as_any().downcast_ref::<i8>() {
                s = serializer.serialize_i8(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<i16>() {
                s = serializer.serialize_i16(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<i32>() {
                s = serializer.serialize_i32(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<i64>() {
                s = serializer.serialize_i64(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<u8>() {
                s = serializer.serialize_u8(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<u16>() {
                s = serializer.serialize_u16(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<u32>() {
                s = serializer.serialize_u32(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<u64>() {
                s = serializer.serialize_u64(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<f32>() {
                s = serializer.serialize_f32(*d)
            } else if let Some(d) = self.as_any().downcast_ref::<f64>() {
                s = serializer.serialize_f64(*d)
            } else {
                s = serializer.serialize_str("cant serialize!!!")
            }

            s
        }
    }

    impl<'a> Validator<'a> {
        pub fn new() -> Self {
            Validator {
                err_vec: Vec::new(),
                err: Error::new(),
            }
        }

        pub fn message(&mut self, value: &'a str) -> &mut Self {
            self.err.message = value;
            self
        }

        pub fn title(&mut self, value: &'a str) -> &mut Self {
            self.err.title = value;
            self
        }

        pub fn has_error(&self) -> bool {
            self.err_vec.len() > 0
        }

        pub fn errors(&self) -> Vec<Error> {
            self.err_vec.clone()
        }

        pub fn errors_to_string(&self) -> String {
            serde_json::to_string(&self.errors()).unwrap_or_default()
        }
    }

    impl fmt::Display for dyn ObjectTrait {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self)
        }
    }

    impl Debug for dyn ObjectTrait {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::{AddRequire, Validator};

    #[test]
    fn require_string() {
        let opt_value_string_1: Option<String> = Some("x".to_string());
        let opt_value_string_2: Option<String> = Some("x2".to_string());
        let opt_value_i8_1: Option<i8> = Some(8);
        let opt_value_i8_2: Option<i8> = Some(9);

        let mut validator = Validator::new();
        validator
            .require("are".to_string()) // data ye ke user vared karde
            .message("the string is mandatory") // paygami ke agar data match nabashe namyesh bede
            .title("string without option") // title paygham
            .build("ar".to_string()); // data morede entezar ke alan match nist ba require
        validator
            .require(opt_value_string_1)
            .message("the string is mandatory")
            .title("string with option")
            .build(opt_value_string_2);
        validator
            .require(opt_value_i8_1)
            .message("the string is mandatory")
            .title("string with option")
            .build(opt_value_i8_2);

        if validator.has_error() {
            println!("{}", validator.errors_to_string()) // [{"title":"string without option","message":"the string is mandatory","value":"are","ok":false},{"title":"string with option","message":"the string is mandatory","value":"x","ok":false},{"title":"string with option","message":"the string is mandatory","value":8,"ok":false}]
        }

        assert_eq!(validator.has_error(), true)
    }
}
