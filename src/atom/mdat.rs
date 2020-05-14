use std::io::{Read, Seek, SeekFrom};

use std::error;

use crate::atom;

pub const ATOM_ID: u32 = 0x6d646174; // 'mdat'

#[derive(Debug, PartialEq)]
pub struct MdatAtom {
    pub atom_head: atom::AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MdatAtom, Box<dyn error::Error>> {
    let head = atom::parse_atom_head(r)?;

    let atom_offset = head.atom_offset;
    let atom_size = head.atom_size;
    let atom_type = head.atom_type;

    let atom_head = atom::AtomHead {
        atom_offset,
        atom_size,
        atom_type,
    };

    if atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    r.seek(SeekFrom::Start(atom_offset + atom_size))?;

    Ok(MdatAtom { atom_head })
}
