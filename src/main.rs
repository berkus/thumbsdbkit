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

use argh::FromArgs;
use std::path::PathBuf;

mod error;
mod format;
mod kit;

#[derive(FromArgs, PartialEq, Debug)]
/// ThumbsDBkit.
struct TopLevel {
    #[argh(subcommand)]
    nested: SubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommandEnum {
    Ls(SubCommandLs),
    Extract(SubCommandExtract),
}

#[derive(FromArgs, PartialEq, Debug)]
/// List thumbnails.
#[argh(subcommand, name = "ls")]
pub(crate) struct SubCommandLs {
    /// colorize the output
    #[argh(switch, short = 'c')]
    color: bool,
    /// print more details for each entry
    #[argh(switch, short = 'd')]
    details: bool,
    /// print the index number of each file
    #[argh(switch, short = 'i')]
    idirid: bool,
    /// a Thumbs.db file to analyze
    #[argh(positional)]
    file: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Extract thumbnails.
#[argh(subcommand, name = "extract")]
pub(crate) struct SubCommandExtract {
    /// output directory where extracted thumbnails will be stored
    #[argh(option, short = 'o', default = "PathBuf::from(\".\")")]
    outdir: PathBuf,
    /// a Thumbs.db file to analyze
    #[argh(positional)]
    file: String,
}

fn main() {
    let args: TopLevel = argh::from_env();

    match match args.nested {
        SubCommandEnum::Ls(ls) => kit::ls(&ls),
        SubCommandEnum::Extract(extract) => kit::extract(&extract),
    } {
        Err(e) => eprintln!("An error has occured: {}", e),
        _ => {}
    }
}
