use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

// Constants for DICOM parsing
const DICOM_PREAMBLE_LENGTH: usize = 128; // Standard length for the DICOM preamble
const DICOM_PREFIX: &str = "DICM"; // Standard prefix following the preamble in DICOM files

// Entry point of the program
fn main() -> io::Result<()> {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check for the correct number of arguments
    if args.len() < 2 {
        println!("Usage: dicom_metadata_parser <dicomFilePath>");
        return Ok(());
    }

    // Parse the DICOM header
    let path = &args[1];
    let dicom_data = parse_dicom_header(path)?;

    // Print the parsed data in JSON format
    println!("{}", serde_json::to_string(&dicom_data)?);

    Ok(())
}

// Function to parse the header of a DICOM file
fn parse_dicom_header(file_path: &str) -> io::Result<HashMap<String, serde_json::Value>> {
    // Initialize an empty HashMap to store DICOM data
    let mut dicom_data: HashMap<String, serde_json::Value> = HashMap::new();

    // Open the DICOM file for reading
    let mut file: File = File::open(file_path)?;

    // Skip the DICOM preamble
    file.seek(SeekFrom::Start(DICOM_PREAMBLE_LENGTH as u64))?;
    let mut prefix: [u8; 4] = [0u8; 4];
    file.read(&mut prefix)?;

    // Check if the file has the correct DICOM prefix
    if DICOM_PREFIX.as_bytes() != prefix {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File does not have the DICOM prefix.",
        ));
    }

    // Loop to read and parse DICOM tags and their values
    loop {
        // Read the 4-byte DICOM tag from the file
        let mut tag_bytes: [u8; 4] = [0; 4];
        if file.read(&mut tag_bytes)? != 4 {
            break; // Exit loop if we can't read 4 bytes, indicating end of file or error
        }

        // Convert the first two bytes and the next two bytes of the tag into their respective group and element
        let group = u16::from_le_bytes([tag_bytes[0], tag_bytes[1]]);
        let element = u16::from_le_bytes([tag_bytes[2], tag_bytes[3]]);

        // Format the group and element as a DICOM tag string
        let tag = format!("{:04X}{:04X}", group, element);

        // Read the next 2 bytes as the Value Representation (VR) of the tag
        let mut vr_bytes: [u8; 2] = [0; 2];
        file.read(&mut vr_bytes)?;
        let vr = String::from_utf8_lossy(&vr_bytes).to_string();

        // Determine the length of the data associated with the tag.
        // DICOM has two types of lengths based on the VR.
        let length: usize;
        if ["OB", "OW", "OF", "SQ", "UT", "UN"].contains(&&*vr) {
            // For these VRs, length is 4 bytes and there are 2 reserved bytes before it
            let mut length_bytes: [u8; 4] = [0; 4];
            file.seek(SeekFrom::Current(2))?; // Skip the 2 reserved bytes
            file.read(&mut length_bytes)?;
            length = u32::from_le_bytes(length_bytes) as usize;
        } else {
            // For other VRs, length is 2 bytes
            let mut length_bytes: [u8; 2] = [0; 2];
            file.read(&mut length_bytes)?;
            length = u16::from_le_bytes(length_bytes) as usize;
        }

        // Read the data associated with the tag based on the determined length
        let mut value_bytes = vec![0u8; length];
        if file.read(&mut value_bytes)? != length {
            break; // Exit loop if we can't read the expected number of bytes for the value
        }

        // Convert the data bytes into a string
        let value = String::from_utf8_lossy(&value_bytes).to_string();

        // Store the tag, VR, and value in a HashMap
        let mut tag_data: HashMap<String, serde_json::Value> = HashMap::new();
        tag_data.insert("vr".to_string(), serde_json::Value::String(vr));
        tag_data.insert(
            "Value".to_string(),
            serde_json::Value::Array(vec![serde_json::Value::String(value)]),
        );

        // Add the tag data to the main DICOM data HashMap
        dicom_data.insert(
            tag,
            serde_json::Value::Object(tag_data.into_iter().collect()),
        );
    }
    // Return the parsed DICOM data
    Ok(dicom_data)
}
