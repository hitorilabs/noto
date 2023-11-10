use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> io::Result<()> {
    let temp_file_path = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "System time error"))?
        .as_secs();

    let temp_file_path = env::temp_dir().join(format!("temp_{}.txt", temp_file_path));

    let mut file = fs::File::create(&temp_file_path)?;
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "---\ndate: {current_time}\n---\n")?;

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".into());
    let status = Command::new(&editor)
        .arg(&temp_file_path)
        .status()?;

    if status.success() {
        process_text(&fs::read_to_string(&temp_file_path)?);
    } else {
        eprintln!("FAILED TO SAVE YOUR NOTE");
    }

    fs::remove_file(temp_file_path)?;
    Ok(())
}

fn process_text(text: &str) {
    println!("Processed text:\n{}", text);
}

