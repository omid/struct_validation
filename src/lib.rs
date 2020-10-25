
#[macro_use]
extern crate lazy_static;


pub mod validation {
    use serde::Serialize;
    use serde::Deserialize;
    use serde_json;
    use regex::{Regex};


    lazy_static! {
    pub static ref EMAIL_REGEX:Regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]+)").unwrap();
}

    #[derive(Clone , Default , Debug,Serialize, Deserialize)]
    pub struct Error {
        title: String,
        message: String,
        value: String,
        value_num: i64,
        ok: bool,
    }

    #[derive(Clone , Default , Debug,Serialize, Deserialize)]
    struct ValidationResult {
        obj_vec: Vec<Error>
    }


    pub struct Validator {
        validation_result: ValidationResult,
        validation_obj: Error
    }

    impl Validator {
        pub fn new() -> Self {
            Validator { validation_result: ValidationResult::default(), validation_obj: Error::default()}
        }

        pub fn require_string(&mut self, value: String) -> &mut Self {

            self.validation_obj.value = value;
            self.validation_obj.ok = !self.validation_obj.value.is_empty();

            self
        }
        pub fn require_string_opt(&mut self, value: Option<String>) -> &mut Self {

            match value {
                Some(v) => {
                    self.validation_obj.value = v;
                    self.validation_obj.ok = !self.validation_obj.value.is_empty();
                    self
                },
                None => {
                    self.validation_obj.value = "Unspecified value!".to_string();
                    self.validation_obj.ok = false;
                    self
                }
            }

        }

        pub fn email_string_opt(&mut self, value: Option<String>) -> &mut Self {

            match value {
                Some(v) => {
                    self.validation_obj.value = v;
                    self.validation_obj.ok = EMAIL_REGEX.is_match(self.validation_obj.value.as_str());
                    self
                },
                None => {
                    self.validation_obj.value = "Unspecified value!".to_string();
                    self.validation_obj.ok = false;
                    self
                }
            }

        }

        pub fn min_string_opt(&mut self, value: Option<String> , min : usize) -> &mut Self {

            match value {
                Some(v) => {
                    self.validation_obj.value = v;
                    self.validation_obj.ok = self.validation_obj.value.len() >= min;
                    self
                },
                None => {
                    self.validation_obj.value = "Unspecified value!".to_string();
                    self.validation_obj.ok = false;
                    self
                }
            }

        }

        pub fn max_string_opt(&mut self, value: Option<String> , max : usize) -> &mut Self {

            match value {
                Some(v) => {
                    self.validation_obj.value = v;
                    self.validation_obj.ok = self.validation_obj.value.len() <= max;
                    self
                },
                None => {
                    self.validation_obj.value = "Unspecified value!".to_string();
                    self.validation_obj.ok = false;
                    self
                }
            }

        }

        pub fn require_positive_opt(&mut self, value: Option<i64>) -> &mut Self {

            match value {
                Some(v) => {
                    self.validation_obj.value_num = v;
                    self.validation_obj.ok = self.validation_obj.value_num > -1;
                    self
                },
                None => {
                    self.validation_obj.value = "Unspecified value!".to_string();
                    self.validation_obj.ok = false;
                    self
                }
            }

        }

        pub fn message(&mut self, value: String) -> &mut Self {
            self.validation_obj.message = value;
            self
        }

        pub fn title(&mut self, value: String) -> &mut Self {
            self.validation_obj.title = value;
            self
        }

        pub fn build(&mut self)  {

            if !self.validation_obj.ok {

                self.validation_result.obj_vec.push(self.validation_obj.clone())
            }
        }

        pub fn has_error(&self) ->bool  {

            self.validation_result.obj_vec.len() > 0

        }

        pub fn errors(&self) ->Vec<Error>  {

            self.validation_result.obj_vec.clone()

        }

        pub fn errors_to_string(&self) -> String {

            serde_json::to_string(&self.errors()).unwrap_or_default()


        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_string() {
        let mut validator = validation::Validator::new();
        validator.require_string_opt(Some("".into())).title("value".into()).message("the value is mandatory".to_string()).build();

        assert!(validator.has_error())
    }

    #[test]
    fn email_string_opt() {
        let mut validator = validation::Validator::new();
        validator.email_string_opt(Some("test@test.".into())).title("email".into()).message("invalid email address".to_string()).build();

        assert!(validator.has_error())
        // test oooooo
    }

    #[test]
    fn min_max_string_opt() {
        let mut validator = validation::Validator::new();
        validator.min_string_opt(Some("test".into()) , 8).title("min".into()).message("value must be longer than 8 characters".to_string()).build();
        validator.max_string_opt(Some("test_test".into()) , 50).title("max".into()).message("value must be less than 50 characters".to_string()).build();
        assert!(validator.has_error())
    }
}
