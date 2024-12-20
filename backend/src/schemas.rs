#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub(crate) enum User {
    Username,
    Email,
    Password,
}

pub(crate) enum UserPreferences {
    Theme,
}

pub(crate) trait ToTableSchema {
    fn table_name() -> &'static str;
    fn columns() -> Vec<&'static str>;
}

impl ToTableSchema for User {
    fn table_name() -> &'static str {
        "user"
    }

    fn columns() -> Vec<&'static str> {
        vec![
            "username TEXT NOT NULL",
            "email TEXT NOT NULL",
            "password TEXT NOT NULL",
        ]
    }
}

impl ToTableSchema for UserPreferences {
    fn table_name() -> &'static str {
        "user_preferences"
    }

    fn columns() -> Vec<&'static str> {
        vec!["theme TEXT NOT NULL"]
    }
}
