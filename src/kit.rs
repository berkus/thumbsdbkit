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

use crate::error::Error;
use crate::format::Formatter;
use clap::ArgMatches;
use std::fs;
use std::io::Write;
use std::path::Path;
use thumbsdb::ThumbsDb;

pub(crate) fn kit(all_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // ls
    if let Some(matches) = all_matches.subcommand_matches("ls") {
        let filepath = matches.value_of("FILE").unwrap();
        let details = matches.is_present("details");
        let idirid = matches.is_present("idirid");
        let color = matches.is_present("color");

        let db = ThumbsDb::from_path(filepath)?;
        let formatter = Formatter {
            details,
            idirid,
            color,
        };
        let mut n = 0usize;
        for thumbnail in db.iterate() {
            formatter.print_thumbnail(thumbnail);
            n += 1;
        }
        formatter.print_suffix(n);
    } else if let Some(matches) = all_matches.subcommand_matches("extract") {
        let filepath = matches.value_of("FILE").unwrap();
        let outdirpath = Path::new(matches.value_of("OUTDIR").unwrap());
        if !outdirpath.exists() {
            fs::create_dir(outdirpath)?;
        } else if !outdirpath.is_dir() {
            Err(Error::OutDirNotDir)?;
        }
        let db = ThumbsDb::from_path(filepath)?;
        for thumbnail in db.iterate() {
            let outfilepath = outdirpath.join(thumbnail.name());
            if outfilepath.exists() && !outfilepath.is_file() {
                eprintln!(
                    "warning: {:?} exists and isn't a file. Skipping",
                    &outfilepath
                );
            } else {
                let mut f = fs::File::create(&outfilepath)?;
                let mut buffer = Vec::<u8>::new();
                db.extract_thumbnail(thumbnail, &mut buffer)?;
                f.write_all(&buffer)?;
                println!("{:<20} extracted to {:?}", thumbnail.name(), &outfilepath);
            }
        }
    }
    Ok(())
}
