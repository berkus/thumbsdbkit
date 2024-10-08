//             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2018 Thomas Bailleux <thomas@bailleux.me>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
//
// Author: zadig <thomas chr(0x40) bailleux.me>

use ansi_term;
use jiff::{fmt::rfc2822::DateTimePrinter, Timestamp};
use thumbsdb;

const DIF_TIME_WINDOWS: u64 = 116444736000000000u64;

pub(crate) struct Formatter {
    pub(crate) details: bool,
    pub(crate) idirid: bool,
    pub(crate) color: bool,
}

impl Formatter {
    pub(crate) fn print_thumbnail(&self, thumbnail: &thumbsdb::Thumbnail) {
        if self.idirid {
            print!("{:>4} ", thumbnail.id());
        }
        if self.details {
            print!("{:>8} ", self.format_date(thumbnail.time()));
        }
        if thumbnail.name() != "{A42CD7B6-E9B9-4D02-B7A6-288B71AD28BA}" {
            println!("{}", self.get_name(thumbnail.name()));
        } else {
            println!("{}", self.get_name("<Thumbnail folder>"));
        }
    }

    pub(crate) fn print_suffix(&self, total: usize) {
        println!("Total {} thumbnails", total);
    }

    pub(crate) fn get_name(&self, name: &str) -> std::string::String {
        match self.color {
            true => match name {
                "<Thumbnail folder>" => ansi_term::Colour::Yellow.paint(name).to_string(),
                _ => ansi_term::Colour::Green.paint(name).to_string(),
            },
            false => std::string::String::from(name),
        }
    }

    fn format_date(&self, date: u64) -> String {
        if date == 0 {
            "(no date specified)".into()
        } else {
            Timestamp::from_second(((date - DIF_TIME_WINDOWS) / 10000000) as i64).map_or_else(
                |_| "(date not valid)".into(),
                |t| {
                    DateTimePrinter::new()
                        .timestamp_to_string(&t)
                        .unwrap_or("(failed to format date)".into())
                },
            )
        }
    }
}
