use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub enum TouchdirError {
    Io(std::io::Error),
}

impl From<std::io::Error> for TouchdirError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl std::fmt::Display for TouchdirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TouchdirError::Io(ref inner) => write!(f, "IO error: {}", inner),
        }
    }
}

impl std::error::Error for TouchdirError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TouchdirError::Io(ref inner) => Some(inner),
        }
    }
}

pub enum TouchdirMode {
    Create,
    Dryrun,
}

impl TouchdirMode {
    pub fn from_dryrun_bool(dryrun: bool) -> Self {
        if dryrun {
            TouchdirMode::Dryrun
        } else {
            TouchdirMode::Create
        }
    }
}

pub struct Touchdir {
    mode: TouchdirMode,
    filepaths: Vec<PathBuf>,
}

impl Touchdir {
    pub fn new(mode: TouchdirMode) -> Self {
        Self {
            mode,
            filepaths: Vec::new(),
        }
    }

    pub fn add_extensions(
        mut self,
        directory: String,
        extensions: Vec<String>,
    ) -> Result<Self, TouchdirError> {
        let resolved_directory = match directory.as_str() {
            "." | "" => std::env::current_dir()?,
            _ => PathBuf::from(directory),
        };

        let dirname = resolved_directory.file_name().unwrap().to_string_lossy();

        for extension in extensions.iter() {
            let filename = resolved_directory.join(&format!("{}.{}", dirname, extension));

            if !self.filepaths.contains(&filename) {
                self.filepaths.push(filename);
            }
        }

        Ok(self)
    }

    pub fn run(&self) -> Result<(), TouchdirError> {
        match self.mode {
            TouchdirMode::Create => self.touch(),
            TouchdirMode::Dryrun => self.dryrun(),
        }
    }

    fn touch(&self) -> Result<(), TouchdirError> {
        for filename in self.filepaths.iter() {
            File::create(&filename)?;
        }
        Ok(())
    }

    fn dryrun(&self) -> Result<(), TouchdirError> {
        for filename in self.filepaths.iter() {
            println!("{}", filename.to_str().unwrap());
        }
        Ok(())
    }
}
