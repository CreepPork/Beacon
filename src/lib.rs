use libc::c_char;
use libc::c_int;
use std::ffi::CString;

use arma_rs::{rv, rv_handler};
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

pub static mut CALLBACK_PTR: Option<
    unsafe extern "C" fn(
        name: *const c_char,
        function: *const c_char,
        data: *const c_char,
    ) -> c_int,
> = None;

#[no_mangle]
pub unsafe extern "C" fn RVExtensionRegisterCallback(
    callback_proc: Option<
        unsafe extern "C" fn(
            name: *const c_char,
            function: *const c_char,
            data: *const c_char,
        ) -> c_int,
    >,
) {
    CALLBACK_PTR = callback_proc;
}

#[rv(thread = true)]
fn start_server() {
    // ToDo: Restart server if it crashes
    // ToDo: Add callback event support
    // ToDo: Add auth token support
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                    unsafe {
                        match CALLBACK_PTR {
                            None => (),
                            // ToDo: Find out how to call this pointer so as to return the value to the game
                            Some(pointer) => pointer(
                                CString::new("some name").unwrap().as_ptr(),
                                CString::new("some function").unwrap().as_ptr(),
                                CString::new("[\"some data\", false, true]").unwrap().as_ptr()
                            ),
                        }
                    }
                    // CALLBACK_PTR("some name", "some function", "[\"some data\", false, true]");
                }
            }
        });
    }
}

#[rv_handler]
fn init() {}
