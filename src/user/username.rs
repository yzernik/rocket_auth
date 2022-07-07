use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

use validator::{HasLen, Validate, ValidateArgs, ValidationError};
// use validator::{validation::ip::validate_ip, HasLen};

lazy_static! {
    // Regex from the specs
    // https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
    // It will mark esoteric email addresses like quoted string as invalid
    static ref EMAIL_USER_RE: Regex = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
}

#[must_use]
pub fn validate_username<'a, T>(val: T) -> bool
where
    T: Into<Cow<'a, str>>,
{
    let val = val.into();
    if val.is_empty() {
        return false;
    }

    // validate the length of each part of the email, BEFORE doing the regex
    // according to RFC5321 the max length of the local part is 64 characters
    // and the max length of the domain part is 255 characters
    // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
    if val.length() > 64 {
        return false;
    }

    if !EMAIL_USER_RE.is_match(&val) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::validate_username;

    #[test]
    fn test_validate_username() {
        // Test cases taken from Django
        // https://github.com/django/django/blob/master/tests/validators/tests.py#L48
        let tests = vec![
            ("email", true),
            ("weirder-email", true),
            (r#"!def!xyz%abc"#, true),
            ("email", true),
            ("example", true),
            ("test", true),
            (r#""test@test""#, false),
            ("", false),
            ("abc@", false),
            ("a ", false),
            ("something@", false),
            (r#""\\\011""#, false),
            (r#""\\\012""#, false),
            // Trailing newlines in username or domain not allowed
            ("a\n", false),
            (r#""test@test"\n"#, false),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                validate_username(input),
                expected,
                "Email `{}` was not classified correctly",
                input
            );
        }
    }

    #[test]
    fn test_validate_username_cow() {
        let test: Cow<'static, str> = "email".into();
        assert!(validate_username(test));
        let test: Cow<'static, str> = String::from("email").into();
        assert!(validate_username(test));
        let test: Cow<'static, str> = "a\n".into();
        assert!(!validate_username(test));
        let test: Cow<'static, str> = String::from("a\n").into();
        assert!(!validate_username(test));
    }

    #[test]
    fn test_validate_username_rfc5321() {
        // 65 character local part
        let test = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(validate_username(test), false);
    }
}
