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

use thumbsdb;
use std;
use clap;

pub(crate) fn kit(all_matches: &clap::ArgMatches)
    -> Result<(), Box<dyn std::error::Error>> {

  // ls
  if let Some(matches) = all_matches.subcommand_matches("ls") {
    let filepath = matches.value_of("FILE").unwrap();
    let details = matches.is_present("details");
    let idirid = matches.is_present("idirid");
    let color = matches.is_present("color");

    let db = thumbsdb::ThumbsDb::from_path(filepath)?;
    let formatter = super::format::Formatter {
      details: details,
      idirid: idirid,
      color: color
    };
    let mut n = 0usize;
    for thumbnail in db.iterate() {
      formatter.print_thumbnail(thumbnail);
      n += 1;
    }
    formatter.print_suffix(n);

  } else if let Some(matches) = all_matches.subcommand_matches("extract") {
    let filepath = matches.value_of("FILE").unwrap();
    let outdirpath = std::path::Path::new(matches.value_of("OUTDIR").unwrap());
    if !outdirpath.exists() {
      std::fs::create_dir(outdirpath)?;
    } else if !outdirpath.is_dir() {
      Err(super::error::Error::OutDirNotDir)?;
    }
    let db = thumbsdb::ThumbsDb::from_path(filepath)?;
    for thumbnail in db.iterate() {
      let outfilepath = outdirpath.join(thumbnail.name());
      if outfilepath.exists() && !outfilepath.is_file() {
        eprintln!("warning: {:?} exists and isn't a file. Skipping", &outfilepath);
      } else {
        use std::io::Write;
        let mut f = std::fs::File::create(&outfilepath)?;
        let mut buffer = std::vec::Vec::<u8>::new();
        db.extract_thumbnail(thumbnail, &mut buffer)?;
        f.write_all(&buffer)?;
        println!("{:<20} extracted to {:?}", thumbnail.name(), &outfilepath);
      }
    }
  }
  Ok(())
}
