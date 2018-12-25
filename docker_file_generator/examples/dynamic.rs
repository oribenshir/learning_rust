use std::path::{ PathBuf };
use std::fs;
use std::env;

use dock_gen::generator::{DockerfileGenerator, GenerateError};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: run --package dock_gen --example dynamic [2,3]");
        return
    }

    let py_version = match args[1].parse::<i32>() {
        Ok(version) => version,
        Err(_) => {
            println!("Invalid argument, expect python version (2 or 3)");
            return
        },
    };

    let project_dir = env!("CARGO_MANIFEST_DIR");
    let docker_file_path: PathBuf = [project_dir, "examples", "test_reference", "Dockerfile"].iter().collect();
    println!("{}", docker_file_path.to_str().unwrap());

    match fs::remove_file(docker_file_path.as_path()) {
        Err(why) => println!("Failed to delete file {} : {:?}", docker_file_path.to_str().unwrap(), why.kind()),
        Ok(_) => println!("File {} erased", docker_file_path.to_str().unwrap()),
    }

    let mut dock_generator = DockerfileGenerator::default();
    dock_generator.path(docker_file_path);

    if py_version == 2 {
        dock_generator.comment("Use an official Python runtime as a parent image")
            .from("python:2.7-slim");
    } else if py_version == 3 {
        dock_generator.comment("Use an official Python runtime as a parent image")
            .from("python:3.7-slim");
    }

    dock_generator.empty_line()
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
        .cmd(r#"["python", "app.py"]"#);

    let result = dock_generator.generate();

    match result {
        Ok(_) => println!("Docker file generated successfully"),
        Err(error) => {
            match error {
                GenerateError::InvalidArgument(reason) => println!("Failed to generated docker file: {}", reason),
                GenerateError::IO(io_error) => println!("Failed to generated docker file: {}", io_error),
            }
        }
    }
}