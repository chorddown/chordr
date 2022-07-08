use crate::route::AppRoute;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use yew_router::Routable;

#[derive(Clone, Debug)]
pub enum LinkTo {
    Route(AppRoute),
    Uri(String),
}

impl Display for LinkTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('/')?;

        match self {
            LinkTo::Route(r) => f.write_str(r.to_path().trim_start_matches('/')),
            LinkTo::Uri(s) => f.write_str(s.trim_start_matches('/')),
        }
    }
}

impl From<AppRoute> for LinkTo {
    fn from(s: AppRoute) -> Self {
        Self::Route(s)
    }
}

impl From<String> for LinkTo {
    fn from(s: String) -> Self {
        Self::Uri(s)
    }
}

impl From<&String> for LinkTo {
    fn from(s: &String) -> Self {
        Self::Uri(s.to_owned())
    }
}

impl From<&str> for LinkTo {
    fn from(s: &str) -> Self {
        Self::Uri(s.to_owned())
    }
}

impl FromStr for LinkTo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Uri(s.to_string()))
    }
}
