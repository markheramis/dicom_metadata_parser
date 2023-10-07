# DICOM Metadata Parser

This tool allows users to parse DICOM (Digital Imaging and Communications in Medicine) files to extract metadata in a structured JSON format.

### Why another tool?
This project serves as an experimentation with Rust and Dicoms.

### It uses
- 

## Features
- Extracts DICOM tags and their associated values.
- Outputs metadata in a structured JSON format.
- Handles special DICOM tags that have a different length and structure.

## Prerequisites
Rust: Make sure you have the Rust programming language and its package manager, Cargo, installed. You can download Rust [here](https://www.rust-lang.org/tools/install).

## Installation

1. Clone this repository:
    ```
    git clone git@github.com:markheramis/dicom_metadata_parser.git
    cd dicom_metadata_parser
    ```
2. Build the tool:
    ```
    cargo build --release
    ```

## Usage
To parse a DICOM file and get the metadata, run:

```
cargo run -- <path-to-dicom-file>
```

### For example:

```
cargo run -- sample.dcm
```

```
cargo run -- C:/dicoms/sample.dcm
```

This will display the extracted metadata in a structured JSON format.

## Structure of Output

The output JSON will have DICOM tags as keys, and the associated data for each tag will contain:

- `vr`: The Value Representation of the tag.
- `Value`: The data associated with the tag.
  
### For example:

```
{
    "0020000D": {
        "vr": "UI",
        "Value": [
            "1.2.826.0.1.3680043.2.594.20385.9727.6138.11111.11111\u0000"
        ]
    },
    ...
}
```