use std::io::BufReader;
use vast_protocol::v4::VAST;

#[test]
fn test_vast_v40_samples() {
    if let Ok(dir) = std::fs::read_dir("tests/VAST_Samples/VAST 4.0 Samples/") {
        for xml in dir {
            let path = xml.unwrap().path();
            println!("Deserializing {}", path.display());

            let f = std::fs::File::open(&path).unwrap();
            let vast: Result<VAST, _> = vast_protocol::from_reader(BufReader::new(f));
            assert!(
                vast.is_ok(),
                "Failed to deserialize from {}",
                path.display()
            );

            println!("Serializing {}", path.display());
            assert!(
                vast_protocol::to_string(&vast.unwrap()).is_ok(),
                "Failed to serialize into {}",
                path.display()
            );
        }
    }
}

#[test]
fn test_vast_v41_samples() {
    if let Ok(dir) = std::fs::read_dir("tests/VAST_Samples/VAST 4.1 Samples/") {
        for xml in dir {
            let path = xml.unwrap().path();
            println!("Deserializing {}", path.display());

            let f = std::fs::File::open(&path).unwrap();
            let vast: Result<VAST, _> = vast_protocol::from_reader(BufReader::new(f));
            assert!(
                vast.is_ok(),
                "Failed to deserialize from {}",
                path.display()
            );

            println!("Serializing {}", path.display());
            assert!(
                vast_protocol::to_string(&vast.unwrap()).is_ok(),
                "Failed to serialize into {}",
                path.display()
            );
        }
    }
}

#[test]
fn test_vast_v42_samples() {
    if let Ok(dir) = std::fs::read_dir("tests/VAST_Samples/VAST 4.2 Samples/") {
        for xml in dir {
            let path = xml.unwrap().path();
            println!("Deserializing {}", path.display());

            let f = std::fs::File::open(&path).unwrap();
            let vast: Result<VAST, _> = vast_protocol::from_reader(BufReader::new(f));
            assert!(
                vast.is_ok(),
                "Failed to deserialize from {}",
                path.display()
            );

            println!("Serializing {}", path.display());
            assert!(
                vast_protocol::to_string(&vast.unwrap()).is_ok(),
                "Failed to serialize into {}",
                path.display()
            );
        }
    }
}
