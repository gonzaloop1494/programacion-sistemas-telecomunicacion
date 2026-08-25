use std::process::Command;
use std::path::PathBuf;

#[test]
fn test_with_file_argument() {
    // Get the path to the `example2.pcap` file 
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("example2.pcap");

    // Verify that the file exists before running the test
    assert!(file_path.exists(), "Tienes que tener el fichero example2.pcap a la misma altura que src");


    // Execute the main binary of the project with the file path as an argument
    let output = Command::new(env!("CARGO_BIN_EXE_ejercicio_31"))
        .arg(file_path.to_str().unwrap()) // Pass the file path as an argument
        .output()
        .expect("Failed to execute process");

    // Convert the stdout bytes to a string
    let stdout = String::from_utf8_lossy(&output.stdout);


    // Check syn spoofers independently of the order
    assert!(stdout.contains("128.3.23.150"), "Falta el syn spoofer 128.3.23.150");
    assert!(stdout.contains("128.3.23.2"), "Falta el syn spoofer 128.3.23.2");
    assert!(stdout.contains("128.3.23.158"), "Falta el syn spoofer 128.3.23.158");
    assert!(stdout.contains("128.3.23.5"), "Falta el syn spoofer 128.3.23.5");
    assert!(stdout.contains("128.3.23.117"), "Falta el syn spoofer 128.3.23.117");

    // Number of syn spoofers
    assert_eq!(5, stdout.trim().split("\n").collect::<Vec<_>>().len(), "Debes encontrar 5 syn spoofers");


}
