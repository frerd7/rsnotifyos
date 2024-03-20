/* main.rs => defines methods for sending notifications to the desktop
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
#![allow(unused_variables)]

include!("debug.rs");
include!("rsnotify.rs");
include!("sendcmd.rs");
include!("linuxalert.rs");

#[cfg(target_os = "windows")]
include!("winalert.rs"); 

pub struct RNotify<'a> {
   pub title: & 'a str,
   pub app_name: & 'a str,
   pub body: & 'a str,
   pub icon: & 'a str,
   pub action: & 'a str,
   pub urgency: & 'a str,
   pub category: & 'a str,
   pub id: & 'a u32,
   pub timeout: & 'a i32,
}

impl <'a>RNotify<'a> {
     // define new instance
     pub fn new() -> RNotify<'a> {
         RNotify { title: "", app_name: "", body: "",
                   action: "", urgency: "", category: "",
                   icon: "", id: &0, timeout: &0,}
     }
     
#[cfg(target_os = "linux")]    
     // funcion dbus ipc msg
     #[cfg(not(windows))]
     pub fn notify(&self, title: & 'a str, app_name: & 'a str, body: & 'a str,
                    icon: & 'a str, id: & 'a u32, 
                    timeout: & 'a i32) -> RNotify<'a>
     {
       // call dbus_notify_send
       SendDbus::dbus_notify_send(title, app_name, body, icon, id, timeout); 
       RNotify {title, app_name, body, icon
                 , action: "", urgency: "", category: ""
                 , id, timeout,}    
     }
     #[cfg(not(windows))]
     pub fn cmdnotify(app_name: & 'a str, icon: & 'a str,
                   urgency: & 'a str, body: & 'a str, action: & 'a str, 
                   category: & 'a str, timeout: i32,) -> RNotify<'a>
     {
        // call cmd command message
        SendCommand::send_notify_command(app_name, icon, urgency, body, action, category, timeout);
        RNotify {title: "", app_name, body, icon
                 , action, urgency, category
                 , id: &0, timeout: &0,}
     }
     #[cfg(not(windows))]
     pub fn dbus_notify(title: & 'a str, app_name: & 'a str, body: & 'a str,
                    icon: & 'a str, id: & 'a u32, 
                    timeout: & 'a i32) -> RNotify<'a> 
     {
        // call dbus_notify_send
        SendDbus::dbus_notify_send(title, app_name, body, icon, id, timeout);        
        // return instance
        RNotify {title, app_name, body, icon
                 , action: "", urgency: "", category: ""
                 , id, timeout,}
     }
     
     // function send_cmd_command
     #[cfg(not(windows))]
     pub fn cmd_notify(app_name: & 'a str, icon: & 'a str,
                   urgency: & 'a str, body: & 'a str, action: & 'a str, 
                   category: & 'a str, timeout: i32,) -> RNotify<'a>
     {
        // call cmd command message
        SendCommand::send_notify_command(app_name, icon, urgency, body, action, category, timeout);
        // return instance
        RNotify {title: "", app_name, body, icon
                 , action, urgency, category
                 , id: &0, timeout: &0,}
     }
     
     // function menssage dialog
     pub fn alert(mode: & 'a str, body: & 'a str) -> RNotify<'a>
     {
#[cfg(target_os = "linux")] 
{
         let err = Debug::new();
         // mode window alert 
         if mode == "INFO" 
         || mode == "info" {
             LinuxAlert::dialog_info(body);
         }
         else if mode == "Error" 
              || mode == "err" 
              || mode == "ERROR" {
             LinuxAlert::dialog_error(body);
         } else { err.error("Invalid message window");}
}
         
#[cfg(target_os = "windows")] 
{
         let err = Debug::new();
         // mode window alert 
         if mode == "INFO" || mode == "info" {
             WinAlert::alert_info(body);
         } else if mode == "Error" || mode == "err" 
                || mode == "ERROR" {
             WinAlert::alert_err(body);
         } else { err.error("Invalid message window");}
}
               
        // return instance
        RNotify { title: "", app_name: "", body: body
                 , icon: "", action: "", urgency: ""
                 , category: ""
                 , id: &0, timeout: &0,} 
     }
}
//fn main() {
//     let _notify = RNotify::new();
//     _notify.dbus_notify("RNotification", "Notification", "Send notify Rust", "dialog-information", &0, &1000);
//     //_notify.cmd_notify("Rust", "dialog-information", "normal", "body", "action", "category", 1000);
     //_notify.alert("ERR","Error system");
//}
