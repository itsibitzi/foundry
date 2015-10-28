#![warn(dead_code)]

use std::mem;

use chrono::{DateTime, Local};
use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub struct FileHistory<'a> {
    head: Option<&'a ChangeNode>,
}

impl<'a> FileHistory<'a> {
    /// Commit a change to this file history
    pub fn commit_work(&mut self, changes: Vec<LineChange>) {

    }
}

#[derive(Debug)]
pub struct ChangeNode {
    id: Id,
    parent: Id,
    time: DateTime<Local>,
    lines: Vec<LineChange>,
}

impl ChangeNode {
    pub fn new(id: Id, parent: Id, lines: Vec<LineChange>) -> ChangeNode {
        ChangeNode {
            id: id,
            parent: parent,
            time: Local::now(),
            lines: lines,
        }
    }
}

#[derive(Debug)]
pub enum LineChange {
    Added {
        line: u64,
        text: String,
    },
    Removed {
        line: u64,
        text: String,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Id {
    value: [u8; 20],
}

impl Id {
    pub fn from_line_changes(deltas: &Vec<LineChange>) -> Id {
        let mut result = [0_u8; 20];

        let mut sha1 = Sha1::new();

        for d in deltas.iter() {
            match d {
                &LineChange::Added {ref line, ref text} => {
                    // Kinda clowny magic string to differentiate between adding and removing the
                    // same text on the same line
                    sha1.input_str("added");
                    unsafe {sha1.input(&mem::transmute::<_, [u8; 8]>(line))}
                    sha1.input_str(&text)
                },
                &LineChange::Removed {ref line, ref text} => {
                    sha1.input_str("removed");
                    unsafe {sha1.input(&mem::transmute::<_, [u8; 8]>(line))}
                    sha1.input_str(&text)
                },
            }
        }

        sha1.result(&mut result);

        Id {value: result}
    }
}
