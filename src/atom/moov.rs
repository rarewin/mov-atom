use std::io::{Read, Seek, SeekFrom};

use anyhow::{Error, Result};

use crate::atom;

pub const ATOM_ID: u32 = 0x6d6f6f76; // 'moov'

#[derive(Debug, PartialEq)]
pub struct MoovAtom {
    pub atom_head: atom::AtomHead,
    pub mvhd_atom: Option<atom::mvhd::MvhdAtom>,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MoovAtom> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(Error::new(atom::AtomSeekError::TypeError(
            atom_head.atom_offset,
        )));
    }

    let mvhd_atom = match atom::parse(r)? {
        atom::Atom::Mvhd(m) => Some(*m),
        _ => unimplemented!("..."),
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MoovAtom {
        atom_head,
        mvhd_atom,
    })
}
