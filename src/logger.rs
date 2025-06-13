use std::fs::File;
use std::io::Write;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::os::unix::fs::FileTypeExt;

pub fn init_logger() {
    let mut builder = Builder::new();
    
    // Set the default log level
    builder.filter_level(LevelFilter::Debug);
    
    // Create or open the named pipe
    let pipe_path = "ledit.pipe";
    if !std::path::Path::new(pipe_path).exists() {
        unsafe {
            libc::mkfifo(pipe_path.as_ptr() as *const i8, 0o666);
        }
    }
    
    // Open the pipe for writing
    let file = File::create(pipe_path).expect("Failed to create/open pipe");
    builder.target(Target::Pipe(Box::new(file)));
    
    // Initialize the logger
    builder.init();
    
    log::info!("Logger initialized - read logs with: cat ledit.pipe");
} 