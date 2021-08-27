
#[macro_use]
extern crate lazy_static;


pub mod validation {
    use serde::{Serialize, Serializer};
    use serde::Deserialize;
    use serde_json;
    use regex::{Regex};
    use std::any::Any;
    use serde::ser::SerializeStruct;
    use std::fmt::{Debug, Formatter, Display};
    use dyn_clone::DynClone;
    use std::fmt;

    dyn_clone::clone_trait_object!(ObjectTrait);

    trait ObjectTrait: Any + DynClone{
        fn as_any(&self) -> &dyn Any;
    }
    pub trait AddRequire<T>{
        fn require(&mut self , v : T) -> &mut Self;
    }

    #[derive( Debug , Serialize , Clone)]
    pub struct Error<'a> {
        title: &'a str,
        message: &'a str,
        value: Box<dyn ObjectTrait>,
        ok: bool,
    }

    pub struct Validator<'a> {
        err_vec: Vec<Error<'a>>,
        err : Error<'a>
    }

    impl Error<'_>{
        fn check_ok(&mut self){
            match self.value.as_any().downcast_ref::<String>() {
                Some(d) => {
                    if d.is_empty() {
                        self.ok = false
                    }else{
                        self.ok = true
                    }
                }
                None => {

                    match self.value.as_any().downcast_ref::<i8>() {
                        Some(d) => {
                            if *d > 0 {
                                self.ok = false
                            }else{
                                self.ok = true
                            }
                        }
                        None => {

                            match self.value.as_any().downcast_ref::<i16>() {
                                Some(d) => {
                                    if *d > 0 {
                                        self.ok = false
                                    }else{
                                        self.ok = true
                                    }
                                }
                                None => {

                                    match self.value.as_any().downcast_ref::<i32>() {
                                        Some(d) => {
                                            if *d > 0 {
                                                self.ok = false
                                            }else{
                                                self.ok = true
                                            }
                                        }
                                        None => {

                                            match self.value.as_any().downcast_ref::<i64>() {
                                                Some(d) => {
                                                    if *d > 0 {
                                                        self.ok = false
                                                    }else{
                                                        self.ok = true
                                                    }
                                                }
                                                None => {

                                                    match self.value.as_any().downcast_ref::<i128>() {
                                                        Some(d) => {
                                                            if *d > 0 {
                                                                self.ok = false
                                                            }else{
                                                                self.ok = true
                                                            }
                                                        }
                                                        None => {

                                                            match self.value.as_any().downcast_ref::<isize>() {
                                                                Some(d) => {
                                                                    if *d > 0 {
                                                                        self.ok = false
                                                                    }else{
                                                                        self.ok = true
                                                                    }
                                                                }
                                                                None => {


                                                                    match self.value.as_any().downcast_ref::<u8>() {
                                                                        Some(d) => {
                                                                            if *d > 0 {
                                                                                self.ok = false
                                                                            }else{
                                                                                self.ok = true
                                                                            }
                                                                        }
                                                                        None => {


                                                                            match self.value.as_any().downcast_ref::<u16>() {
                                                                                Some(d) => {
                                                                                    if *d > 0 {
                                                                                        self.ok = false
                                                                                    }else{
                                                                                        self.ok = true
                                                                                    }
                                                                                }
                                                                                None => {

                                                                                    match self.value.as_any().downcast_ref::<u32>() {
                                                                                        Some(d) => {
                                                                                            if *d > 0 {
                                                                                                self.ok = false
                                                                                            }else{
                                                                                                self.ok = true
                                                                                            }
                                                                                        }
                                                                                        None => {

                                                                                            match self.value.as_any().downcast_ref::<u64>() {
                                                                                                Some(d) => {
                                                                                                    if *d > 0 {
                                                                                                        self.ok = false
                                                                                                    }else{
                                                                                                        self.ok = true
                                                                                                    }
                                                                                                }
                                                                                                None => {

                                                                                                    match self.value.as_any().downcast_ref::<u128>() {
                                                                                                        Some(d) => {
                                                                                                            if *d > 0 {
                                                                                                                self.ok = false
                                                                                                            }else{
                                                                                                                self.ok = true
                                                                                                            }
                                                                                                        }
                                                                                                        None => {

                                                                                                            match self.value.as_any().downcast_ref::<usize>() {
                                                                                                                Some(d) => {
                                                                                                                    if *d > 0 {
                                                                                                                        self.ok = false
                                                                                                                    }else{
                                                                                                                        self.ok = true
                                                                                                                    }
                                                                                                                }
                                                                                                                None => {


                                                                                                                    match self.value.as_any().downcast_ref::<f32>() {
                                                                                                                        Some(d) => {
                                                                                                                            if *d > 0 as f32 {
                                                                                                                                self.ok = false
                                                                                                                            }else{
                                                                                                                                self.ok = true
                                                                                                                            }
                                                                                                                        }
                                                                                                                        None => {


                                                                                                                            match self.value.as_any().downcast_ref::<f64>() {
                                                                                                                                Some(d) => {
                                                                                                                                    if *d > 0 as f64 {
                                                                                                                                        self.ok = false
                                                                                                                                    }else{
                                                                                                                                        self.ok = true
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                None => {

                                                                                                                                    self.ok = false


                                                                                                                                }
                                                                                                                            }

                                                                                                                        }
                                                                                                                    }

                                                                                                                }
                                                                                                            }


                                                                                                        }
                                                                                                    }


                                                                                                }
                                                                                            }


                                                                                        }
                                                                                    }


                                                                                }
                                                                            }

                                                                        }
                                                                    }

                                                                }
                                                            }


                                                        }
                                                    }


                                                }
                                            }


                                        }
                                    }


                                }
                            }


                        }
                    }


                }
            }
        }

        fn new()-> Self{
            Self{
                title: "",
                message: "",
                value: Box::new("".to_string()),
                ok: true
            }
        }

    }

    impl Serialize for dyn ObjectTrait {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer {
            let mut s;

            match self.as_any().downcast_ref::<String>()  {
                Some(d) => {
                    s = serializer.serialize_str(d)
                }
                None => {

                    match self.as_any().downcast_ref::<i8>() {
                        Some(d) => {
                            s = serializer.serialize_i8(*d)
                        }
                        None => {

                            match self.as_any().downcast_ref::<i16>() {
                                Some(d) => {
                                    s = serializer.serialize_i16(*d)
                                }
                                None => {

                                    match self.as_any().downcast_ref::<i32>() {
                                        Some(d) => {
                                            s = serializer.serialize_i32(*d)
                                        }
                                        None => {

                                            match self.as_any().downcast_ref::<i64>() {
                                                Some(d) => {
                                                    s = serializer.serialize_i64(*d)
                                                }
                                                None => {

                                                    match self.as_any().downcast_ref::<u8>() {
                                                        Some(d) => {
                                                            s = serializer.serialize_u8(*d)
                                                        }
                                                        None => {

                                                            match self.as_any().downcast_ref::<u16>() {
                                                                Some(d) => {
                                                                    s = serializer.serialize_u16(*d)
                                                                }
                                                                None => {


                                                                    match self.as_any().downcast_ref::<u32>()  {
                                                                        Some(d) => {
                                                                            s = serializer.serialize_u32(*d)
                                                                        }
                                                                        None => {


                                                                            match self.as_any().downcast_ref::<u64>()  {
                                                                                Some(d) => {
                                                                                    s = serializer.serialize_u64(*d)
                                                                                }
                                                                                None => {

                                                                                    match self.as_any().downcast_ref::<f32>() {
                                                                                        Some(d) => {
                                                                                            s = serializer.serialize_f32(*d)
                                                                                        }
                                                                                        None => {

                                                                                            match self.as_any().downcast_ref::<f64>() {
                                                                                                Some(d) => {
                                                                                                    s = serializer.serialize_f64(*d)
                                                                                                }
                                                                                                None => {

                                                                                                    s = serializer.serialize_str("cant serialize!!!")


                                                                                                }
                                                                                            }


                                                                                        }
                                                                                    }


                                                                                }
                                                                            }

                                                                        }
                                                                    }

                                                                }
                                                            }


                                                        }
                                                    }


                                                }
                                            }


                                        }
                                    }


                                }
                            }


                        }
                    }


                }
            }

            s
        }
    }

    impl<'a> Validator<'a>{
        pub fn new() -> Self {
            Validator { err_vec: vec![], err : Error::new() }
        }

        pub fn message(&mut self, value: &'a str) -> &mut Self {
            self.err.message = value;
            self
        }

        pub fn title(&mut self, value: &'a str) -> &mut Self {
            self.err.title = value;
            self
        }

        pub fn build(&mut self)  {
            if !self.err.ok {
                self.err_vec.push(self.err.clone())
            }
        }

        pub fn has_error(&self) ->bool  {
            self.err_vec.len() > 0
        }

        pub fn errors(&self) ->Vec<Error>  {
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

    impl ObjectTrait for String{
        fn as_any(&self) -> &dyn Any {
            self
        }

    }

    impl AddRequire<String> for Validator<'_> {
        fn require(&mut self, v: String) -> &mut Self {
            self.err = Error::new();
            if v.is_empty() {
                self.err.value = Box::new(v.clone());
                self.err.check_ok();
            }
            self
        }
    }

    impl AddRequire<Option<String>> for Validator<'_> {
        fn require(&mut self, v: Option<String>) -> &mut Self {
            self.err = Error::new();

            match v {
                Some(d) => {
                    if d.is_empty() {
                        self.err.value = Box::new(d.clone());
                        self.err.check_ok();
                    }else{
                        let s = "";
                    }
                }
                None => {
                    self.err.value = Box::new("Unspecified value!".to_string());
                    self.err.ok = false;
                }
            }

            self
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::{AddRequire, Validator};

    #[test]
    fn require_string() {

        let mut test_value:Option<String> = Some("".to_string());
        test_value = None;

        let mut validator = Validator::new();
        validator.require("".to_string()).message("the string is mandatory").title("string without option").build();
        validator.require(test_value).message("the string is mandatory").title("string with option").build();


        let ff = "";

        if validator.has_error() {

            println!("{}",validator.errors_to_string())
        }


    }
}
