#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Locale {
	pub language: Language,
	pub territory: Territory,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Language {
	English,
	Japanese,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Territory {
	None,
	AU,
	GB,
	JP,
	NZ,
}
