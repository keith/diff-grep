pub fn read(path: &String) -> Result<String, String> {
    let mut handle: Box<dyn std::io::Read> = if path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path.clone()).map_err(|_| format!("failed to open {}", path))?)
    };

    let mut buffer = String::new();
    handle
        .read_to_string(&mut buffer)
        .map_err(|_| format!("failed to read from {}", path))?;

    Ok(buffer)
}

pub fn write<T: std::fmt::Display>(path: String, files: Vec<T>) -> Result<(), String> {
    if files.is_empty() {
        return Ok(());
    }

    let mut handle: Box<dyn std::io::Write> = if path == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(
            std::fs::File::create(path.clone()).map_err(|_| format!("failed to open {}", path))?,
        )
    };

    for patch in files {
        write!(handle, "{}\n", patch).map_err(|_| format!("failed to write to {}", path))?;
    }

    Ok(())
}
