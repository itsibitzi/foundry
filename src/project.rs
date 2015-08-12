use cargo::util::CargoResult;

pub struct Project {
    name: String,
}

impl Project {
    pub fn from_cargo_manifest<P: AsRef<Path>>(path: P) -> Result<Project, ProjectLoadError> {
    }
}
