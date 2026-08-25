use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_with_file_argument() {
    // Get the path to the `example2.pcap` file
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("example2.pcap");

    // Verify that the file exists before running the test
    assert!(
        file_path.exists(),
        "Tienes que tener el fichero example2.pcap a la misma altura que src"
    );

    // Execute the main binary of the project with the file path as an argument
    let output = Command::new(env!("CARGO_BIN_EXE_ejercicio_30"))
        .arg(file_path.to_str().unwrap()) // Pass the file path as an argument
        .output()
        .expect("Failed to execute process");

    // Convert the stdout bytes to a string
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Look for arp spoofers independtly of the order produced in stdout:
    assert!(
        stdout.contains("14:4f:8a:ed:c2:5e\n"),
        "Falta el arp spoofer 14:4f:8a:ed:c2:5d"
    );
    assert!(
        stdout.contains("7c:d1:c3:94:9e:b8\n"),
        "Falta el arp spoofer 7c:d1:c3:94:9e:b8"
    );

    // Number of lines of output: 2 arp spoofers
    assert_eq!(
        2,
        stdout.trim().split("\n").collect::<Vec<_>>().len(),
        "Debes encontrar 2 arp spoofers"
    );
}
