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
use std::fs;
use std::io::Write;
use thumbsdb::ThumbsDb;

pub(crate) fn ls(ls: &crate::SubCommandLs) -> Result<(), Box<dyn std::error::Error>> {
    let db = ThumbsDb::from_path(&ls.file)?;
    let formatter = Formatter {
        details: ls.details,
        idirid: ls.idirid,
        color: ls.color,
    };
    let mut n = 0usize;
    for thumbnail in db.iterate() {
        formatter.print_thumbnail(thumbnail);
        n += 1;
    }
    formatter.print_suffix(n);
    Ok(())
}

pub(crate) fn extract(
    extract: &crate::SubCommandExtract,
) -> Result<(), Box<dyn std::error::Error>> {
    let outdirpath = extract.outdir.as_path();
    if !outdirpath.exists() {
        fs::create_dir(outdirpath)?;
    } else if !outdirpath.is_dir() {
        Err(Error::OutDirNotDir)?;
    }
    let db = ThumbsDb::from_path(&extract.file)?;
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
    Ok(())
}
