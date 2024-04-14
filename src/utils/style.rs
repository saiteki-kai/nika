use anstyle::*;
use clap::builder::styling::Styles;

pub const HEADER: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
pub const USAGE: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
pub const LITERAL: Style = AnsiColor::White.on_default().effects(Effects::BOLD);
pub const PLACEHOLDER: Style = AnsiColor::White.on_default();

pub const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
pub const VALID: Style = AnsiColor::White.on_default().effects(Effects::BOLD);
pub const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

pub const STYLES: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);
