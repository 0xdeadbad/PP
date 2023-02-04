use crate::util::UnsafeInitialization;
use super::super::headers::*;
use super::PortableExecutable;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PE64 {
    pub dos_header: ImageDosHeader,
    pub nt_headers: ImageNtHeadesr64,
    pub section_headers: Vec<ImageSectionHeader>,
    pub sections: Vec<u8>
}

impl PortableExecutable for PE64 {
    fn from_slice(s: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let dos_header = ImageDosHeader::from_slice(&s[..ImageDosHeader::size()])?;
        let nt_headers = ImageNtHeadesr64::from_slice(&s[(dos_header.e_lfanew as usize)..])?;
        let mut section_headers: Vec<ImageSectionHeader> = Vec::new();
        let mut sections: Vec<u8> = Vec::new();
        {
            let mut i = 0;
            while i < nt_headers.FileHeader.NumberOfSections {
                let section_header = ImageSectionHeader::from_slice(&s[(dos_header.e_lfanew as usize) + ImageNtHeadesr64::size() + (i as usize * ImageSectionHeader::size())..])?;
                section_headers.push(section_header);
                i += 1;
            }
        }
        for section_header in &section_headers {
            let mut section = s[section_header.PointerToRawData as usize..(section_header.PointerToRawData + section_header.SizeOfRawData) as usize].to_owned();
            sections.append(&mut section);
        }
        let r = Self {
            dos_header,
            nt_headers,
            section_headers,
            sections
        };

        Ok(r)
    }

    fn get_relocations(self: &mut PE64, _s: &[u8]) -> std::result::Result<Vec<BaseRelocation>, Box<dyn std::error::Error>> {
        todo!()
    }
}