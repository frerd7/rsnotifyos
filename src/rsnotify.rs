/* rnotify.rs => defines methods for sending notifications to the desktop
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

// include dbus
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
extern crate dbus;
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
use dbus::blocking::Connection;
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
use dbus::strings::{BusName, Path, Interface, Member};
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
use dbus::Message;
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
use dbus::blocking::BlockingSender;
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
use dbus::Signature;
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
use dbus::arg::messageitem::MessageItem;
#[cfg(not(windows))]
#[cfg(target_os = "linux")]
use dbus::arg::messageitem::MessageItemArray;
#[cfg(not(windows))]
#[cfg(target_os = "linux")]

// struct RNotify
struct SendDbus<'a> {
   title: & 'a str,
   app_name: & 'a str,
   body: & 'a str,
   icon: & 'a str,
   id: & 'a u32,
   timeout: & 'a i32,
}

#[cfg(target_os = "linux")]
#[cfg(not(windows))] 
// method 
impl <'a>SendDbus<'a> {
      // func dbus ipc notification call
      // using dbus protocol
      fn dbus_notify_send(title: & 'a str, app_name: & 'a str, body: & 'a str,
                           icon: & 'a str, id: & 'a u32, 
                           timeout: & 'a i32) -> SendDbus<'a> 
      {
          let debug_instance = Debug::new();
          let connection_result = Connection::new_session();
          let connection = Connection::new_session().unwrap();
          let mut arg = Vec::new();
          
          arg.push(format!("--title={}", title));
          arg.push(format!("--app_name={}", app_name));
          arg.push(format!("--body={}", body));
          arg.push(format!("--icon={}", icon));
          arg.push(format!("--id={}", id));
          arg.push(format!("--timeout={}", timeout));
          
          // check dbus connection
          debug_instance.handle_dbus_connection(connection_result);
          
          // print arg formatting
          debug_instance.print_msg_arg(&arg);
          
           // Create a message to send a notification
          let mut message = Message::method_call(
                &BusName::new("org.freedesktop.Notifications").unwrap(),
                &Path::new("/org/freedesktop/Notifications").unwrap(),
                &Interface::new("org.freedesktop.Notifications").unwrap(),
                &Member::new("Notify").unwrap(),
          );
          
          // Create Array empty
          let array_as = MessageItem::Array(MessageItemArray::new(vec![], Signature::new("as").unwrap()).unwrap());
          let array_asv = MessageItem::Array(MessageItemArray::new(vec![], Signature::new("a{sv}").unwrap()).unwrap());
              
          // Add the items to the message
          message.append_items(&[
           title.into(),    // String
           (*id).into(),    // u32
           icon.into(),    // String
           app_name.into(), // String
           body.into(),  // String
           array_as, 
           array_asv,
           (*timeout).into(),]);
        
          // Send the message
         match connection.send_with_reply_and_block(message, std::time::Duration::from_secs(2)) {
           Ok(_) => {
                  debug_instance.print("The notification was sent successfully.");
           },
           Err(err) => {
                 let error_message = format!("Sending notification: {}", err);
                 debug_instance.error(&error_message);
           }
         }
          
         SendDbus { title, app_name, body, icon, id, timeout}  
    } 
  }
// example
// fn main() {
//    SendDbus::dbus_notify_send("RNotification", "Notification", "Send notify Rust", "dialog-information", &0, &1000);
//}
