/* sendcmd.rs => defines methods to send notifications to the desktop using the subprocess
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

use std::process::Command;

// struct 
struct SendCommand<'a> {
    app_name: & 'a str,
    icon: & 'a str,
    urgency: & 'a str,
    body: & 'a str,
    action: & 'a str,
    category: & 'a str,
    timeout: i32,
}

// method
impl <'a>SendCommand<'a> {
       fn new() -> SendCommand<'a> {
          // return instance
          SendCommand { app_name: "", icon: "", 
                        urgency: "", body: "" ,action: "",
                        category: "", timeout: 0,}
       }
       
       fn send_notify_command(app_name: & 'a str, icon: & 'a str,
                       urgency: & 'a str, body: & 'a str, action: & 'a str, 
                       category: & 'a str, timeout: i32, ) -> SendCommand<'a> 
       {
           
           let command = "notify-send";
           let mut args = Vec::new();
           let debug_instance = Debug::new();
           
           if !action.is_empty() && !category.is_empty() {
                args.push(format!("--action={}", action));
                args.push(format!("--category={}", category));
            }
            
           args.push("--app-name=".to_owned() + app_name);
           args.push("--icon=".to_owned() + icon);
           args.push("--urgency=".to_owned() + urgency);
           args.push("--expire-time=".to_owned() + &timeout.to_string());
           args.push(app_name.to_owned());
           args.push(body.to_owned());
           
           // debug 
           debug_instance.print_msg_arg(&args);
           
           // check command
           Command::new(command).
                       args(&args).
                       output().
                       expect("Error: Failed to execute command");
                       
           SendCommand{ app_name, icon, urgency, body,
                        action, category, timeout,}    
       }
}

// example
//fn main() {
//     let send_command = SendCommand::new();
//     send_command.send_notify_command("Rust", "dialog-information", "normal", "notification body", "action", "category", 1000);
//}
