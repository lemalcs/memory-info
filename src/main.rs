use kernel32;
use winapi;

use winapi::{
    DWORD, // u32 type in Rust
    HANDLE, // pointer for internal APIs without associated type
    LPVOID, // LP stands long pointer
    PVOID, // P stands for pointer
    SIZE_T, // u64 for Windows x64
    LPSYSTEM_INFO, // pointer to SYSTEM_INFO struct
    SYSTEM_INFO, // struct of Windows
    MEMORY_BASIC_INFORMATION as MEMINFO,
};

fn main() {
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMINFO;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();

    // Garantees that all memory is initialized
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    // Make system calls
    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();

        // This call uses a C idiom where we provide a pointer
        // to a predefined struct and when function ends it fill the
        // values of the struct
        kernel32::GetSystemInfo(
            &mut proc_info as LPSYSTEM_INFO
        );
    };

    // Give new names to memory address just to make it readable
    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);

    // Scan through the address space
    loop {
        let rc: SIZE_T = unsafe {

            // Get information about a specific segment oj the memory address space
            // starting at `base_addr`
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
