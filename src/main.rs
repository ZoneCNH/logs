use logs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;

    println!("Testing logging levels:");

    let i = 0;
    trace!("This is a trace message {}", i);
    debug!("This is a debug message {}", i);
    info!("This is an info message {}", i);
    warn!("This is a warning message {}", i);
    error!("This is an error message {}", i);
    Ok(())

}
