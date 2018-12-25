use std::path::{ PathBuf };
use std::fs::{File};
use std::io::prelude::*;
use std::io;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum GenerateError {
    #[fail(display = "Invalid argument was given.")]
    InvalidArgument(String),
    #[fail(display = "IO error has occur.")]
    IO(#[fail(cause)] io::Error),
}

pub struct DockerfileGenerator {
    content : String,
    path    : Option<PathBuf>
}


impl Default for DockerfileGenerator {
    fn default() -> DockerfileGenerator {
        DockerfileGenerator {
            content: String::new(),
            path : None
        }
    }
}

impl DockerfileGenerator {
    pub fn path(&mut self, path : PathBuf) -> &mut DockerfileGenerator {
        self.path = Some(path);
        self
    }

    pub fn generate(&mut self) -> Result<(), GenerateError> {

        let path = match self.path {
            Some(ref p) => p,
            None => return Err(GenerateError::InvalidArgument(String::from("No path was given"))),
        };

        let mut docker_file =  File::create(path.as_path()).map_err(|e| GenerateError::IO(e))?;

        let result = docker_file.write_all( self.content.to_string().as_bytes()).map_err(|e| GenerateError::IO(e))?;
        Ok(result)
    }

    pub fn comment(& mut self, line : &str) -> &mut DockerfileGenerator {
        let mut full_line = String::from("# ");
        full_line.push_str(line);
        self.push(&full_line[..])
    }

    pub fn from(& mut self, line : &str) -> &mut DockerfileGenerator {
        let mut full_line = String::from("FROM ");
        full_line.push_str(line);
        self.push(&full_line[..])
    }

    pub fn work_dir(& mut self, line : &str) -> &mut DockerfileGenerator {
        let mut full_line = String::from("WORKDIR ");
        full_line.push_str(line);
        self.push(&full_line[..])
    }

    pub fn copy(& mut self, from : &str, to : &str) -> &mut DockerfileGenerator {
        let mut full_line = String::from("COPY ");
        full_line.push_str(from);
        full_line.push(' ');
        full_line.push_str(to);
        self.push(&full_line[..])
    }

    pub fn run(& mut self, line : &str) -> &mut DockerfileGenerator {
        let mut full_line = String::from("RUN ");
        full_line.push_str(line);
        self.push(&full_line[..])
    }

    pub fn expose(& mut self, port : u32) -> &mut DockerfileGenerator {
        let mut full_line = String::from("EXPOSE ");
        full_line.push_str(&port.to_string());
        self.push(&full_line[..])
    }

    pub fn env(& mut self, key : &str, value :&str) -> &mut DockerfileGenerator {
        let mut full_line = String::from("ENV ");
        full_line.push_str(key);
        full_line.push(' ');
        full_line.push_str(value);
        self.push(&full_line[..])
    }

    pub fn cmd(& mut self, line : &str) -> &mut DockerfileGenerator {
        let mut full_line = String::from("CMD ");
        full_line.push_str(line);
        self.push(&full_line[..])
    }

    pub fn empty_line(& mut self) -> &mut DockerfileGenerator {
        self.push("")
    }

    pub fn push(& mut self, line : &str) -> & mut DockerfileGenerator{
        self.content.push_str(line);
        self.content.push_str("\r\n");
        self
    }
}