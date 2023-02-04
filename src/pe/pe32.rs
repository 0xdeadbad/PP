use crate::util::UnsafeInitialization;
use super::super::headers::*;
use super::PortableExecutable;

fn name_len(n: [u8; 8usize]) -> usize {
    let mut i = 0_usize;
    while i < 8 && n[i] != 0 {
        i += 1;
    }

    i
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PE32 {
    pub dos_header: ImageDosHeader,
    pub nt_headers: ImageNtHeadesr32,
    pub section_headers: Vec<ImageSectionHeader>,
    pub sections: Vec<u8>
}

impl PortableExecutable for PE32 {
    fn from_slice(s: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let dos_header = ImageDosHeader::from_slice(&s[..ImageDosHeader::size()])?;
        let nt_headers = ImageNtHeadesr32::from_slice(&s[(dos_header.e_lfanew as usize)..])?;
        let mut section_headers: Vec<ImageSectionHeader> = Vec::new();
        let mut sections: Vec<u8> = Vec::new();
        {
            let mut i = 0;
            while i < nt_headers.FileHeader.NumberOfSections {
                let section_header = ImageSectionHeader::from_slice(&s[(dos_header.e_lfanew as usize) + ImageNtHeadesr32::size() + (i as usize * ImageSectionHeader::size())..])?;
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

    fn get_relocations(self: &mut PE32, s: &[u8]) -> std::result::Result<Vec<BaseRelocation>, Box<dyn std::error::Error>> {
        let mut r: Vec<BaseRelocation> = Vec::new();

        let reloc_section_header = {
            let mut reloc_section_header = ImageSectionHeader::new_zeroed();

            for section_header in &mut self.section_headers[..] {
                println!("{:?}", &section_header.Name);
                let name = unsafe { String::from_raw_parts(section_header.Name.as_mut_ptr() as *mut _, name_len(section_header.Name), 8) };
                
                if !&name.eq(".reloc") {
                    continue;
                }

                reloc_section_header.clone_from(&section_header);
                break;
            }

            let name = unsafe { String::from_utf8_unchecked(reloc_section_header.Name.to_vec()) };
            if name != ".reloc" {
                return Err(".reloc not found".into());
            }

            reloc_section_header
        };

        {
            let mut base_reloc = ImageBaseRelocation::from_slice(&s[reloc_section_header.PointerToRawData as usize..])?;
            let mut i = 0;
            while base_reloc.SizeOfBlock != 0 && base_reloc.VirtualAddress != 0 {
                let mut offsets: Vec<BaseRelocationOffset> = Vec::new();
                let mut j = 0;
                while j < (base_reloc.SizeOfBlock - 8) / 2 {
                    offsets.push(((s[(reloc_section_header.PointerToRawData + i + 8 + 2*j) as usize] as u16) << 8) | (s[(reloc_section_header.PointerToRawData + 8 + 2*j) as usize + 1] as u16));
                    j += 1;
                }
                let b = BaseRelocation {
                    image_base_relocation: base_reloc.clone(),
                    offsets 
                };

                r.push(b);
                base_reloc = ImageBaseRelocation::from_slice(&s[(reloc_section_header.PointerToRawData + i) as usize..])?;
                i += base_reloc.SizeOfBlock;
            }
        }

        Ok(r)
    }
}