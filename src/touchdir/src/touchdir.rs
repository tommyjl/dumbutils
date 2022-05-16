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
    filenames: Vec<String>,
}

impl Touchdir {
    pub fn new(mode: TouchdirMode) -> Self {
        Self {
            mode,
            filenames: Vec::new(),
        }
    }

    pub fn add_extensions(
        mut self,
        directory: String,
        extensions: Vec<String>,
    ) -> Result<Self, TouchdirError> {
        let current_dir = std::env::current_dir()?;

        let resolved_directory = match directory.as_str() {
            "." => current_dir.file_name().unwrap().to_string_lossy(),
            _ => todo!("Can only use current directory at the moment"),
        };

        for extension in extensions.iter() {
            let resolved_filename = format!("{}.{}", resolved_directory, extension);
            self.filenames.contains(&resolved_filename);
            self.filenames.push(resolved_filename);
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
        for filename in self.filenames.iter() {
            std::fs::File::create(&filename)?;
        }
        Ok(())
    }

    fn dryrun(&self) -> Result<(), TouchdirError> {
        for filename in self.filenames.iter() {
            println!("{}", filename);
        }
        Ok(())
    }
}
