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

use clap::{App, Arg, SubCommand};

mod error;
mod format;
mod kit;

fn main() {
    let matches = App::new("ThumbsDBkit")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("ls")
                .about("List thumbnails")
                .arg(
                    Arg::with_name("FILE")
                        .help("Thumbs.db to analyze")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("color")
                        .long("color")
                        .required(false)
                        .takes_value(false)
                        .help("colorize the output"),
                )
                .arg(
                    Arg::with_name("details")
                        .short("d")
                        .long("details")
                        .required(false)
                        .takes_value(false)
                        .help("print more details for each entry"),
                )
                .arg(
                    Arg::with_name("idirid")
                        .short("i")
                        .long("idirid")
                        .required(false)
                        .takes_value(false)
                        .help("print the index number of each file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extract thumbnails")
                .arg(
                    Arg::with_name("FILE")
                        .help("Thumbs.db to analyze")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("OUTDIR")
                        .short("o")
                        .long("out")
                        .help("Output directory where extracted thumbnails will be stored")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();
    match kit::kit(&matches) {
        Err(e) => eprintln!("An error has occured: {}", e),
        _ => {}
    }
}
