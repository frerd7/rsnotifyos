/* notify_event_loop.rs => defines example
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
 
#![allow(dead_code)]
extern crate rsnotifyos;
use rsnotifyos::RNotify;

use std::thread;
use std::time::Duration;

fn timeout(timeout: &i32) 
{
   let timeout_secs: u64 = *timeout as u64;
   println!("Sleep 3.0 seconds\n Do not close");
   thread::sleep(Duration::from_secs(timeout_secs));
}


fn main() {
     println!("[+] Test System Notification");
#[cfg(target_os = "linux")] 
{
     println!("--- Test D-Bus Notification Send ----");
     
     RNotify::dbus_notify("RNotification", "Notification", "Send Notification Rust", "dialog-information", &0, &1000);
     
     // SLEEP
     timeout(&3);
     
     println!("---- Test SubProcess Notication Send ----");
     RNotify::cmd_notify("Rust", "dialog-information", "normal", "Send Notification Command using subprocess", "", "", 1000);
     
     // sleep
     timeout(&3);
     
     println!("--- Test Menssage Error Alert ----");
     RNotify::alert("ERROR", "System Error Alert Menssage Event Loop");
       
     // sleep    
     timeout(&3);
     
     println!("--- Test Menssage Information Alert ----");
     RNotify::alert("info", "System Info Alert Menssage Event Loop");
     
     // let alert = RNotify::alert("info", "System Info Alert Menssage Event Loop");   
}

#[cfg(target_os = "windows")] 
{    
     println!("--- Test Menssage Error Alert ----");
     RNotify::alert("ERROR", "System Error Alert Menssage Event Loop");
       
     // sleep    
     timeout(&3);
     
     println!("--- Test Menssage Information Alert ----");
     RNotify::alert("info", "System Info Alert Menssage Event Loop");
}
}
