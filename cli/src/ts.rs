#![allow(dead_code)]

use askama::Template;
use derive_more::derive::Deref;
use yaspec::*;

#[derive(Debug, Template, Deref)]
#[template(
    path = "ts/root.txt",
    ext = "txt",
    escape = "none",
    whitespace = "suppress"
)]
pub struct RootTemplate<'a>(pub &'a yaspec::YASpec);

#[derive(Debug, Template, Deref)]
#[template(
    path = "ts/description.txt",
    ext = "txt",
    escape = "none",
    whitespace = "suppress"
)]
pub struct DescriptionTemplate<'a>(pub &'a Option<String>);

#[derive(Debug, Template, Deref)]
#[template(
    path = "ts/action.txt",
    ext = "txt",
    escape = "none",
    whitespace = "suppress"
)]
pub struct ActionTemplate<'a>(pub &'a str, #[deref] pub &'a yaspec::Entry<Action>);

#[derive(Debug, Template, Deref)]
#[template(
    path = "ts/shape.txt",
    ext = "txt",
    escape = "none",
    whitespace = "suppress"
)]
pub struct ShapeTemplate<'a>(pub &'a yaspec::Shape);

mod filters {
    /// Indent each line of a string with a tab character.
    pub fn tab<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(s.to_string()
            .lines()
            .map(|line| format!("  {}", line))
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
