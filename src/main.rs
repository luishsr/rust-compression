use std::fs;
use std::io::{Read, stdout, Write};

fn main() {
    match generate_binary_file("hello.bin", Vec::from(
        "NQmcSaMcOtykPPOFoJv EYICbHRWpvtaFaIJBJDKoN ImF UVljnKRCQWBwpUniTWoGyqQgaZIYzQuirphhisSfsNySHJdqpCA yXxFWVXfHJJIuRXhhMnOJXNCmYHKnefOEbETGhoyHMpqAqPDEFiNDVloxHDoyyVZbNaCdRq SOhMPPMpprSpIvSLMfPRzSMBXQrmSTcMzLPlt pSzCCHCFfIAyBIHZgQmLormpvvnJMQKPOJNhxu gWlbVBAlEtLzygORjQmNaixEabLEPxooTAMUsFp GlmxaLahxAdIUilGIwStQxzOLTCtvRSarTEFZeHIdJqgLXrixTRTBfdywvfZGFMNBnZBsBJGkxDQyycHrSOHa HoEhTiwHTBFIiWwTrJyGimtxDudATBJAKTlLnkjKuXVailMkAjVbwjwydPQChZuYngeOYujuPJgXQVXxlrKsNkiPaOhEBPTTfPjJAYYssWHovcNMloxcdUbmNUcGApPwOzvlPtcYGfhHWaFgppcbCLtlStasQxEynLZpqngBtWxiRFCMkpLvlcMqhOeVwCStjJdQpRaEwgQZxqDUXpPgnmw pEObTXHeGYngO fy rqQjIEfRUMxoXgkyeRbkjKbneZZDlbFSJD MZDZyykxznrobJJ mBN"
    )){
        Ok(_) => match compress_file("hello.bin") {
            Ok(()) => (),
            Err(err) => println!("Error - {}", err)
        },
        Err(err) => println!("{}", err)
    }
}

fn compress_file(filename: &str) -> Result<(), String> {
    let mut file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error loading the file - {}", err ))
    };

    let mut file_bytes= [0; 2000];
    match file.read(&mut file_bytes) {
        Ok(bytes_read) => (),
        Err(err) => return Err(format!("Error loading file content - {}", err))
    };

    let (empty_ending_collection, nonempty_collection) = load_bytes_ending(file_bytes.iter().as_ref())
        .unwrap();

    let (updated_empty_ending_collection, updated_nonempty_collection) = pack_bits(empty_ending_collection.clone(), nonempty_collection.clone()).unwrap();

    //print_collection(updated_empty_ending_collection.clone(), "Empty-ending bytes");
    //print_collection(updated_nonempty_collection.clone(), "Non-empty-ending bytes");

    // Create the final compressed file with the results
    match generate_binary_file("compressed-hello.bin", merge_vectors(updated_empty_ending_collection.clone(), updated_nonempty_collection.clone()).unwrap()){
        Ok(_) => (),
        Err(err) => println!("Error - {}", err)
    }

    let _ = print_compression_statistics();

    return Ok(())
}

fn print_compression_statistics() -> Result<(), String>{
    // Define file paths
    let file1_path = "hello.bin";
    let file2_path = "compressed-hello.bin";

    // Load files and get their sizes
    let file1_size = fs::metadata(file1_path).unwrap().len();
    let file2_size = fs::metadata(file2_path).unwrap().len();

    // Print file sizes
    println!("Size of '{}': {} bytes", file1_path, file1_size);
    println!("Size of '{}': {} bytes", file2_path, file2_size);

    // Calculate percentage difference
    let percentage_difference = if file1_size > 0 {
        100.0 * (file1_size as f64 - file2_size as f64) / file1_size as f64
    } else {
        0.0 // Avoid division by zero
    };

    // Print the compression result
    println!(
        "File compression reduced the size by {:.2}%",
        percentage_difference
    );

    Ok(())
}

fn print_collection(collection: Vec<u8>, name: &str) {
    println!("Printing list of {} with {} elements", name, collection.len());

    for item in collection.iter() {
        println!("{:0b}", item)
    }

    println!("___________________________")
}

fn merge_vectors(vector_a: Vec<u8>, vector_b: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();

    for va in vector_a {
        result.push(va)
    }

    for vb in vector_b {
        result.push(vb)
    }

    return Ok(result);
}

fn pack_bits(mut empty_endings: Vec<u8>, mut nonempty_endings: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), String> {

    while empty_endings.len() < 4 || empty_endings.len() >= nonempty_endings.len() {
        let mut target : u8 = nonempty_endings[nonempty_endings.len()-1];

        let mut target_ending = 0;
        let mut empty_counting = empty_endings.iter().count();

        // Extract 2 bits at time
        while target > 0 {
            target_ending = target & 0b11;

            // Pack 2 bits into an available empty-ending byte
            empty_endings[empty_counting - 1] = empty_endings[empty_counting - 1] | target_ending;

            // Remove the used empty-ending byte from the list of available ones
            // and add it to the list of non-empty ones
            nonempty_endings.push(empty_endings.pop().unwrap());

            // Update the available empty-ends counting
            empty_counting = empty_endings.iter().count();

            target >>= 2 + 1;
        }

        // Remove the target byte from the list of non-empty ones
        nonempty_endings.pop();
    }

    return Ok((empty_endings, nonempty_endings))
}

fn load_bytes_ending(input: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut empty = Vec::new();
    let mut nonempty = Vec::new();

    for b in input.iter(){
        let ending = (b & 0b11) as u8;
        //println!("Byte {:0b} ending is {:0b}", b, ending);

        if ending == 0b00 {
            empty.push(b.clone())
        } else {
            nonempty.push(b.clone())
        }
    }

    return Ok((empty, nonempty))
}

fn generate_binary_file(filename: &str, content: Vec<u8>) -> Result<bool, String> {
    let mut file = match fs::File::create(filename){
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating file - {}", err))
    };

    match file.write(content.as_ref()) {
        Ok(bytes_written) => (),
        Err(err) => return Err(format!("Error writing to the file - {}", err))
    };

    return Ok(true);
}
