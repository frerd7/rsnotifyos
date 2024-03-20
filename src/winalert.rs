/* winalert.rs => define methods to print notices in windows
 * MIT License
 * 
 * Copyright (c) 2024 Frederick Valdez
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

extern crate winapi;
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK, HWND_DESKTOP, MB_ICONINFORMATION};

struct WinAlert<'a> {
    body: &'a str,
}

impl<'a> WinAlert<'a> {
    fn new() -> WinAlert<'a> {
        WinAlert { body: "" }
    }
    // menssage alert error
    fn alert_err(body: &'a str) -> WinAlert<'a> {
      unsafe {
        MessageBoxW(
            HWND_DESKTOP,
            body.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            "Critical Message\0".encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            MB_OK | MB_ICONERROR,);
      }
      WinAlert {body}
    }

    fn alert_info(body: &'a str) ->WinAlert<'a> {
        unsafe {
           MessageBoxW(
            HWND_DESKTOP,
            body.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            "Information Message\0".encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            MB_OK | MB_ICONINFORMATION,);
      }
       WinAlert {body}
   }
}

// fn main() {
//     let alert = WinAlert::new();
//     alert.alert_info("Print an alert message through programming in Rust!");
//}
