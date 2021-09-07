pub mod validation {
    use dyn_clone::DynClone;
    use serde::{Serialize, Serializer};
    use serde_json;
    use std::any::Any;
    use std::fmt;
    use std::fmt::{Debug, Formatter};

    dyn_clone::clone_trait_object!(ObjectTrait);

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

    impl ObjectTrait for String {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for i8 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for i16 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for i32 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for i64 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for i128 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for u8 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for u16 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for u32 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for u64 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for u128 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for f32 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ObjectTrait for f64 {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl AddRequire<String> for Validator<'_> {
        fn require(&mut self, v: String) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: String) {
            match self.err.value.as_any().downcast_ref::<String>() {
                Some(d) => {
                    if *d != v {
                        // self.err.value = Box::new(v.clone());
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

    impl AddRequire<Option<String>> for Validator<'_> {
        fn require(&mut self, v: Option<String>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<String>) {
            match self.err.value.as_any().downcast_ref::<String>() {
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

    impl AddRequire<i8> for Validator<'_> {
        fn require(&mut self, v: i8) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: i8) {
            match self.err.value.as_any().downcast_ref::<i8>() {
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

    impl AddRequire<Option<i8>> for Validator<'_> {
        fn require(&mut self, v: Option<i8>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<i8>) {
            match self.err.value.as_any().downcast_ref::<i8>() {
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

    impl AddRequire<i16> for Validator<'_> {
        fn require(&mut self, v: i16) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: i16) {
            match self.err.value.as_any().downcast_ref::<i16>() {
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

    impl AddRequire<Option<i16>> for Validator<'_> {
        fn require(&mut self, v: Option<i16>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<i16>) {
            match self.err.value.as_any().downcast_ref::<i16>() {
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

    impl AddRequire<i32> for Validator<'_> {
        fn require(&mut self, v: i32) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: i32) {
            match self.err.value.as_any().downcast_ref::<i32>() {
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

    impl AddRequire<Option<i32>> for Validator<'_> {
        fn require(&mut self, v: Option<i32>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<i32>) {
            match self.err.value.as_any().downcast_ref::<i32>() {
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

    impl AddRequire<i64> for Validator<'_> {
        fn require(&mut self, v: i64) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: i64) {
            match self.err.value.as_any().downcast_ref::<i64>() {
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

    impl AddRequire<Option<i64>> for Validator<'_> {
        fn require(&mut self, v: Option<i64>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<i64>) {
            match self.err.value.as_any().downcast_ref::<i64>() {
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

    impl AddRequire<i128> for Validator<'_> {
        fn require(&mut self, v: i128) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: i128) {
            match self.err.value.as_any().downcast_ref::<i128>() {
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

    impl AddRequire<Option<i128>> for Validator<'_> {
        fn require(&mut self, v: Option<i128>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<i128>) {
            match self.err.value.as_any().downcast_ref::<i128>() {
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

    impl AddRequire<u8> for Validator<'_> {
        fn require(&mut self, v: u8) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: u8) {
            match self.err.value.as_any().downcast_ref::<u8>() {
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

    impl AddRequire<Option<u8>> for Validator<'_> {
        fn require(&mut self, v: Option<u8>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<u8>) {
            match self.err.value.as_any().downcast_ref::<u8>() {
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

    impl AddRequire<u16> for Validator<'_> {
        fn require(&mut self, v: u16) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: u16) {
            match self.err.value.as_any().downcast_ref::<u16>() {
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

    impl AddRequire<Option<u16>> for Validator<'_> {
        fn require(&mut self, v: Option<u16>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<u16>) {
            match self.err.value.as_any().downcast_ref::<u16>() {
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

    impl AddRequire<u32> for Validator<'_> {
        fn require(&mut self, v: u32) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: u32) {
            match self.err.value.as_any().downcast_ref::<u32>() {
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

    impl AddRequire<Option<u32>> for Validator<'_> {
        fn require(&mut self, v: Option<u32>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<u32>) {
            match self.err.value.as_any().downcast_ref::<u32>() {
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

    impl AddRequire<u64> for Validator<'_> {
        fn require(&mut self, v: u64) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: u64) {
            match self.err.value.as_any().downcast_ref::<u64>() {
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

    impl AddRequire<Option<u64>> for Validator<'_> {
        fn require(&mut self, v: Option<u64>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<u64>) {
            match self.err.value.as_any().downcast_ref::<u64>() {
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

    impl AddRequire<u128> for Validator<'_> {
        fn require(&mut self, v: u128) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: u128) {
            match self.err.value.as_any().downcast_ref::<u128>() {
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

    impl AddRequire<Option<u128>> for Validator<'_> {
        fn require(&mut self, v: Option<u128>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<u128>) {
            match self.err.value.as_any().downcast_ref::<u128>() {
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

    impl AddRequire<f32> for Validator<'_> {
        fn require(&mut self, v: f32) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: f32) {
            match self.err.value.as_any().downcast_ref::<f32>() {
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

    impl AddRequire<Option<f32>> for Validator<'_> {
        fn require(&mut self, v: Option<f32>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<f32>) {
            match self.err.value.as_any().downcast_ref::<f32>() {
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

    impl AddRequire<f64> for Validator<'_> {
        fn require(&mut self, v: f64) -> &mut Self {
            self.err = Error::new();
            self.err.value = Box::new(v.clone());
            self
        }

        fn build(&mut self, v: f64) {
            match self.err.value.as_any().downcast_ref::<f64>() {
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

    impl AddRequire<Option<f64>> for Validator<'_> {
        fn require(&mut self, v: Option<f64>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(vv) => {
                    self.err.value = Box::new(vv.clone());
                }
                None => {}
            }

            self
        }

        fn build(&mut self, v: Option<f64>) {
            match self.err.value.as_any().downcast_ref::<f64>() {
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
            .require("are".to_string())
            .message("the string is mandatory")
            .title("string without option")
            .build("ar".to_string());
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
