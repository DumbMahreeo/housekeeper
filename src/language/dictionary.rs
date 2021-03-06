use crate::language::{
    dictionary::DirType::*, // Forgive me, I'm lazy
    locales::{Locale, Locale::*},
};

#[derive(Debug)]
pub enum DirType {
    Videos,
    Images,
    Packages,
    Documents,
    Presentations,
    Spreadsheets,
    Scripts,
    SourceFiles,
    Binaries,
    Other,
}

pub fn get_dir(generic_dir: DirType) -> String {
    match Locale::from_env() {
        It => match generic_dir {
            Videos => "Video",
            Images => "Immagini",
            Packages => "Pacchetti",
            Documents => "Documenti",
            Presentations => "Presentazioni",
            Spreadsheets => "Fogli di calcolo",
            Scripts => "Script",
            SourceFiles => "Sorgenti",
            Binaries => "Binari",
            Other => "Altro",
        },

        _ => match generic_dir {
            Videos => "Videos",
            Images => "Images",
            Packages => "Packages",
            Documents => "Documents",
            Presentations => "Presentations",
            Spreadsheets => "Spreadsheets",
            Scripts => "Scripts",
            SourceFiles => "Soule files",
            Binaries => "Binaries",
            Other => "Other",
        },
    }
    .to_string()
}
