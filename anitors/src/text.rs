use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Deref};
use std::sync::Arc;

use lazy_regex::Regex;

#[derive(Clone)]
enum Str {
    Slice(&'static str),
    String(Arc<str>),
}

impl Str {
    fn len(&self) -> usize {
        self.as_str().len()
    }

    fn as_str(&self) -> &str {
        match self {
            Str::Slice(it) => *it,
            Str::String(it) => it.as_ref(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Text(Str, usize, usize);

impl Text {
    fn sub_str(&self, start: usize, end: usize) -> Self {
        if end <= start || self.0.len() < self.1 + end {
            return Self::default();
        }
        Self(self.0.clone(), self.1 + start, self.1 + end)
    }

    pub fn len(&self) -> usize {
        self.2 - self.1
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn is_ascii_digit(&self) -> bool {
        self.is_not_empty() && self.chars().all(|it| it.is_ascii_digit())
    }

    pub fn split_once(&self, pat: &Regex) -> Option<(Self, Self, Self)> {
        pat.find(self).map(|it| {
            (
                self.sub_str(0, it.start()),
                self.sub_str(it.start(), it.end()),
                self.sub_str(it.end(), self.len()),
            )
        })
    }
}

impl Default for Text {
    fn default() -> Self {
        Self(Str::Slice(""), 0, 0)
    }
}

impl<T: AsRef<str>> From<T> for Text {
    fn from(value: T) -> Self {
        match value.as_ref() {
            "" => Self(Str::Slice(""), 0, 0),
            str => Self(Str::String(str.into()), 0, str.len()),
        }
    }
}

impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0.as_str()[self.1..self.2]
    }
}

impl Add for Text {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (&*self, &*rhs) {
            (left, _) if left.is_empty() => return rhs,
            (_, right) if right.is_empty() => return self,
            (left, right) => {
                let str = left.to_owned() + right;
                let len = str.len();
                Self(Str::String(str.into()), 0, len)
            }
        }
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        &**self == *other
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        &**self == &**other
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&**self).partial_cmp(&**other)
    }
}

impl Eq for Text {}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> Ordering {
        (&**self).cmp(&**other)
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&**self).hash(state)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}
