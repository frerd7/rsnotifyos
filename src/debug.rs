/* debug.rs => defines debugging methods
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

// Define a Debug struct with a slice of Strings
struct Debug<'a> {
    msg: &'a [String],
}

// Conditional compilation attribute to enable/disable debug messages
#[cfg(feature = "enable_debug")]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        println!($($arg)*);
    }
}

#[cfg(not(feature = "enable_debug"))]
macro_rules! debug_println {
    ($($arg:tt)*) => {};
}

extern "C" {
    fn is_debug() -> bool;
}

// Implementation block for Log
impl<'a> Debug<'a> {
    // Constructor method to create a new Log instance
    fn new() -> Debug<'a> 
    {
       Debug { msg: &[] }
    }
    
    fn print_msg_arg(&self, msg: &'a [String]) -> Debug<'a> {
        debug_println!("<------------- DEBUG -------------------->");
        debug_println!("Arguments:");
        for arg in msg {
            debug_println!(" {}", arg);
        }
        Debug { msg: &[] }
    }
    #[cfg(not(windows))]
    fn handle_dbus_connection(&self, connection: Result<Connection, dbus::Error>) -> Debug<'a> {
       match connection {
          Ok(_) => {
            self.print("D-Bus connection successfully established.");
          }
          Err(err) => {
            let error_message = format!("Failed to establish D-Bus connection {}", err);
            self.error(&error_message);
            // Handle the error accordingly
         }
      } 
      Debug { msg: self.msg }
   }
   
   // Method to log an print
   fn print(&self, arg: &'a str) -> Debug<'a> {
        debug_println!("<------------- DEBUG -------------------->");
        debug_println!("{}", arg);
        Debug { msg: self.msg }
    }
    
    // Method to log an error
    fn error(&self, arg: &'a str) -> Debug<'a> {
        eprintln!("Error: {}", arg);
        Debug { msg: self.msg }
    }
}

// fn main() {
// Create a vector of strings
// let arg = vec!["apple".to_string(), "orange".to_string()]; 
// Call the active method and capture the returned Debug instance
//    let _debug_instance = Debug::print_msg_arg(&arg);
//    _debug_instance.error("Not object");
//}
