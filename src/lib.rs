pub mod validation {
    use serde::Serialize;
    use serde_json;
    use std::fmt::{Debug, Display, Formatter};

    pub enum ConstraintType {
        Required,
        MinRange(i64),
        MaxRange(i64),
        MinLength(usize),
        MaxLength(usize),
        Contains(String),
        StartsWith(String),
        EndsWith(String),
        //...
    }

    pub trait Validate {
        fn validate(&mut self) -> Vec<Error>;
    }

    #[derive(Debug, Serialize, Clone)]
    pub struct Error {
        pub title: String,
        pub message: String,
        pub value: String,
    }

    pub struct Constraint {
        typ: ConstraintType,
        title: String,
        message: String,
    }

    impl Constraint {
        pub fn new(typ: ConstraintType) -> Self {
            Self {
                typ,
                title: "".to_string(),
                message: "".to_string(),
            }
        }

        pub fn new_with_message(typ: ConstraintType, title: &str, message: &str) -> Self {
            Self {
                typ,
                title: title.to_string(),
                message: message.to_string(),
            }
        }

        pub fn message(&mut self, value: &str) -> &mut Self {
            self.message = value.to_string();
            self
        }

        pub fn title(&mut self, value: &str) -> &mut Self {
            self.title = value.to_string();
            self
        }
    }

    pub struct Validator<'a, T> {
        constraints: Vec<Constraint>,
        value: &'a T,
    }

    impl<'a, T> Validator<'a, T> {
        pub fn new(v: &'a T) -> Self {
            Self {
                constraints: Vec::new(),
                value: v,
            }
        }

        pub fn add_constraint(mut self, v: Constraint) -> Self {
            self.constraints.push(v);
            self
        }

        pub fn add(mut self, v: ConstraintType) -> Self {
            // TODO Add the default title, based on the constraint type
            // TODO Add the default message, based on the constraint type
            self.constraints.push(Constraint::new(v));
            self
        }
    }

    impl<'a> Validate for Validator<'a, Option<String>> {
        fn validate(&mut self) -> Vec<Error> {
            let mut errors = Vec::new();
            for c in &self.constraints {
                match c.typ {
                    ConstraintType::Required => {
                        // TODO move these blocks out to a function to have a cleaner code!
                        if self.value.is_none() {
                            let error = Error {
                                title: c.title.clone(),
                                message: c.message.clone(),
                                value: "".to_string(),
                            };
                            errors.push(error);
                        }
                    }
                    ConstraintType::MinRange(_) => {}
                    ConstraintType::MaxRange(_) => {}
                    ConstraintType::MinLength(min) => {
                        if let Some(v) = self.value {
                            if v.len() < min {
                                let error = Error {
                                    title: c.title.clone(),
                                    message: c.message.clone(),
                                    value: v.to_string(),
                                };
                                errors.push(error);
                            }
                        }
                    }
                    ConstraintType::MaxLength(_) => {}
                    ConstraintType::Contains(_) => {}
                    ConstraintType::StartsWith(_) => {}
                    ConstraintType::EndsWith(_) => {}
                }
            }
            errors
        }
    }

    pub struct Validators {
        inner: Vec<Error>,
    }

    impl Validators {
        pub fn new() -> Self {
            Self {
                inner: Vec::new()
            }
        }

        pub fn add_validator<T>(&mut self, mut validate: T) where T: Validate {
            self.inner.append(&mut validate.validate())
        }

        pub fn has_error(&self) -> bool {
            !self.inner.is_empty()
        }

        // TODO, we don't need this, instead impl Iterator
        pub fn to_vec(&self) -> Vec<Error> {
            self.inner.clone()
        }
    }

    impl Display for Validators {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&serde_json::to_string(&self.to_vec()).unwrap_or_default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validation::*;

    #[test]
    fn require_string() {
        let field: Option<String> = Some("x".to_string());

        let mut validators = Validators::new();

        // "add" and "add_constraint" are two different ways to add a new constraint
        let validator = Validator::new(&field)
            .add(ConstraintType::Required)
            .add_constraint(Constraint::new_with_message(
                ConstraintType::MinLength(10),
                "It's shorter than 10 characters",
                "Min should be blah blah",
            ))
            .add(ConstraintType::MaxLength(20));

        validators.add_validator(validator);

        assert_eq!(validators.to_vec().len(), 1);
        assert_eq!(validators.to_vec()[0].title, "It's shorter than 10 characters");
    }
}
