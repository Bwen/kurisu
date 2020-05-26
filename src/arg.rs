const TYPES_NO_VALUE: &[&str] = &["bool"];
const TYPES_MULTIPLE_VALUES: &[&str] = &["Vec < String >", "Vec < usize >", "Vec < isize >"];
const TYPES_OPTIONAL_VALUE: &[&str] = &["Option < String >", "Option < usize >", "Option < isize >"];

#[derive(Debug, Clone)]
pub struct Arg<'a> {
    pub name: &'a str,
    pub value_type: &'a str,
    pub position: Option<u8>,
    pub doc: Option<&'a str>,
    pub short: Option<&'a str>,
    pub long: Option<&'a str>,
    pub exit: Option<fn() -> i32>,
    pub default: &'a str,
    pub value: Vec<String>,
    pub occurrences: usize,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Arg<'a> {
        Arg {
            name: "",
            value_type: "",
            position: None,
            doc: None,
            short: None,
            long: None,
            exit: None,
            default: "",
            value: Vec::new(),
            occurrences: 0,
        }
    }
}

impl<'a> PartialEq<String> for &Arg<'a> {
    fn eq(&self, other: &String) -> bool {
        if !other.starts_with('-') {
            return false;
        }

        if other.starts_with("--") && self.long.is_some() {
            return other.starts_with(&format!("--{}", self.long.unwrap()));
        }

        if other.starts_with('-') && self.short.is_some() {
            return other.starts_with(&format!("-{}", self.short.unwrap()));
        }

        false
    }
}

impl<'a> PartialEq<String> for &mut Arg<'a> {
    fn eq(&self, other: &String) -> bool {
        // TODO: try to combine all 3 eq to one function? can accept both str & string...
        if !other.starts_with('-') {
            return false;
        }

        if other.starts_with("--") && self.long.is_some() {
            return other.starts_with(&format!("--{}", self.long.unwrap()));
        }

        if other.starts_with('-') && self.short.is_some() {
            return other.starts_with(&format!("-{}", self.short.unwrap()));
        }

        false
    }
}

impl<'a> PartialEq<str> for &Arg<'a> {
    fn eq(&self, other: &str) -> bool {
        if !other.starts_with('-') {
            return false;
        }

        if other.starts_with("--") && self.long.is_some() {
            return other.starts_with(&format!("--{}", self.long.unwrap()));
        }

        if other.starts_with('-') && self.short.is_some() {
            return other.starts_with(&format!("-{}", self.short.unwrap()));
        }

        false
    }
}

impl<'a> Arg<'a> {
    pub fn optional(&self) -> bool {
        self.value_type.starts_with("Option")
    }

    pub fn input(&self) -> bool {
        self.position.is_some()
    }

    pub fn short_long(&self) -> bool {
        self.short.is_some() && self.long.is_some()
    }

    pub fn value_required(&self) -> bool {
        !TYPES_NO_VALUE.contains(&self.value_type) && !TYPES_OPTIONAL_VALUE.contains(&self.value_type)
    }

    pub fn value_optional(&self) -> bool {
        TYPES_OPTIONAL_VALUE.contains(&self.value_type)
    }

    pub fn value_none(&self) -> bool {
        TYPES_NO_VALUE.contains(&self.value_type)
    }

    pub fn value_multiple(&self) -> bool {
        TYPES_MULTIPLE_VALUES.contains(&self.value_type)
    }

    pub fn set_value(&'_ mut self, args: &[String]) {
        // TODO: What to do with Optional values?
        // TODO: Handle repetitive short flags such as -vvv (Occurrences)

        let mut pos = 1;
        for arg in args {
            if self.eq(arg) {
                if arg.contains('=') {
                    let value: Vec<&str> = arg.split('=').collect();
                    self.occurrences += 1;
                    self.value.push(value[1].to_owned());
                    continue;
                }

                self.occurrences += 1;
                self.value.push(String::from("true"));
            } else if !arg.starts_with('-') {
                if let Some(position) = self.position {
                    if position != pos && position != 0 {
                        pos += 1;
                        continue;
                    }

                    self.occurrences += 1;
                    self.value.push(arg.clone());
                    pos += 1;
                }
            }
        }

        if !self.value.is_empty() {
            return;
        }

        // TODO: Fall back to check environment variables with the same name capitalized,
        // Maybe it should be mentioned in the annotation as to WHICH env var is attached to which field, to allow branding MYPROGRAM_PATH_TO_SOMETHING
        // Or... Should it? field mysql_host for example would allow a quick access to environment variables that dont require branding...
        // Optional branding annotation?

        self.value.push(self.default.to_string());
    }
}
