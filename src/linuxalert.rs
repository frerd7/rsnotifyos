/* linuxalert.rs => defines methods for printing notices
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

#[cfg(target_os = "linux")]
#[cfg(not(windows))]
extern crate gtk;

#[cfg(target_os = "linux")]
#[cfg(not(windows))]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
#[cfg(not(windows))]
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType};
#[cfg(target_os = "linux")]
#[cfg(not(windows))]
// struct 
struct LinuxAlert<'a> {
    body: & 'a str,
}
#[cfg(target_os = "linux")]
#[cfg(not(windows))]
// method
impl <'a>LinuxAlert<'a> {
       fn new() -> LinuxAlert<'a> {
          // return instance
          LinuxAlert { body: ""}
       }
       
       fn dialog_info(body: & 'a str) -> LinuxAlert<'a> {
         // check gtk system
         gtk::init().expect("Error: Failed to initialize GTK.");
         
         let dialog = MessageDialog::new(None::<&gtk::Window>,
        			     DialogFlags::MODAL,
                                     MessageType::Info,
                                     ButtonsType::Ok,
                                     body,);

         dialog.set_title("Message Alert");
         dialog.connect_response(|dialog, _| {
             dialog.close(); // Hide the dialog
             gtk::main_quit();
        });
        
         dialog.show_all();
         gtk::main();
         LinuxAlert{ body }
       }
      
      fn dialog_error(body: & 'a str) -> LinuxAlert<'a> {
         gtk::init().expect("Error: Failed to initialize GTK.");
         let dialog = MessageDialog::new(None::<&gtk::Window>,
        			     DialogFlags::MODAL,
                                     MessageType::Error,
                                     ButtonsType::Ok,
                                     body,);

         dialog.set_title("Menssage Critical Error");
         dialog.connect_response(|dialog, _| {
             dialog.close(); // Hide the dialog
             gtk::main_quit();
        });
        
         dialog.show_all();
         gtk::main();
         LinuxAlert{ body }
      } 
}

//fn main() {
//     let msg = LinuxAlert::new();
//     msg.dialog_error("error system");
//}
