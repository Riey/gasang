use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Section {
    sec_addr: u64,

    beg: usize,
    end: usize,
}

pub struct Image {
    image: Vec<u8>,
    sections: HashMap<String, Section>,
}

impl Image {
    pub fn from_image(image: Vec<u8>) -> Self {
        Self {
            image,
            sections: HashMap::new(),
        }
    }

    // Add secment into image
    pub fn add_section(
        &mut self,
        sec_name: impl AsRef<str>,
        sec_addr: u64,
        beg: usize,
        end: usize,
    ) {
        self.sections.insert(
            sec_name.as_ref().to_string(),
            Section { sec_addr, beg, end },
        );
    }

    pub fn sections(&self) -> impl Iterator<Item = &str> {
        self.sections.keys().map(|v| v.as_str())
    }

    pub fn section_addr(&self, name: impl AsRef<str>) -> u64 {
        self.sections.get(name.as_ref()).unwrap().sec_addr
    }

    pub fn section_data(&self, name: impl AsRef<str>) -> &[u8] {
        let sec = self.sections.get(name.as_ref()).unwrap();
        &self.image[sec.beg..sec.end]
    }
}