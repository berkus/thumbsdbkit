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

use std;

#[derive(Debug)]
pub(crate) enum Error {
  OutDirNotDir
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::OutDirNotDir => "OUTDIR isn't a directory",
    }
  }

  fn cause(&self) -> Option<&std::error::Error> {
    match *self {
      _ => None
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use std::error::Error;
    write!(f, "{}", self.description())
  }
}
