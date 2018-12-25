use std::path::{ PathBuf };
use std::fs;

use dock_gen::generator::{DockerfileGenerator, GenerateError};

fn main() {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let docker_file_path : PathBuf = [project_dir, "examples", "test_reference", "Dockerfile"].iter().collect();
    println!("{}", docker_file_path.to_str().unwrap());

    match fs::remove_file(docker_file_path.as_path()) {
        Err(why) => println!("Failed to delete file {} : {:?}", docker_file_path.to_str().unwrap(), why.kind()),
        Ok(_) => println!("File {} erased", docker_file_path.to_str().unwrap()),
    }

    let result = DockerfileGenerator::default()
        .path(docker_file_path)
        .comment("Use an official Python runtime as a parent image")
        .from("python:2.7-slim")
        .empty_line()
        .comment("Set the working directory to /app")
        .work_dir("/app")
        .empty_line()
        .comment("Copy the current directory contents into the container at /app")
        .copy(".", "/app")
        .empty_line()
        .comment("Install any needed packages specified in requirements.txt")
        .run("pip install --trusted-host pypi.python.org -r requirements.txt")
        .empty_line()
        .comment("Make port 80 available to the world outside this container")
        .expose(80)
        .empty_line()
        .comment("Define environment variable")
        .env("NAME", "World")
        .empty_line()
        .comment("Run app.py when the container launches")
        .cmd(r#"["python", "app.py"]"#)
        .generate();

    match result {
        Ok(_) => println! ("Docker file generated successfully"),
        Err(error) => {
            match error {
                GenerateError::InvalidArgument(reason) => println! ("Failed to generated docker file: {}", reason),
                GenerateError::IO(io_error) => println! ("Failed to generated docker file: {}", io_error),
            }
        }
    }
}