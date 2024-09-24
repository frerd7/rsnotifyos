/* win32notify.rs => defines methods for sending notifications to the desktop
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

use std::os::raw::{c_int, c_longlong};
use libc::wchar_t;

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
enum Results {
    Success = 0,
    SystemNotSupported = 1,
    InitializationFailure = 2,
    ToastFailed = 3,
}

extern "C" {
    fn show_notification(
        actions_acs: *const wchar_t,
        appName: *const wchar_t,
        appUserModelID: *const wchar_t,
        text: *const wchar_t,
        imagePath: *const wchar_t,
        attribute: *const wchar_t,
        expiration: c_longlong,
    ) -> c_int;
}

pub struct WinSend<'a> {
    pub action: & 'a str,
    pub app_name: & 'a str,
    pub app_user_model_id: & 'a str,
    pub text: & 'a str,
    pub img: & 'a str,
    pub attr: & 'a str,
    pub expiration: & 'a i64
 }

 // method 
impl <'a>WinSend<'a> {
      pub fn new() -> WinSend <'a> {
          WinSend { action: "", app_name: "", 
                    app_user_model_id: "", text: "", 
                    img: "", attr: "", expiration: &0}
      }

      pub fn win32_send(&self, action: & 'a str, app_name: & 'a str,
                        app_user_model_id: & 'a str, text: & 'a str,
                        img: & 'a str, attr: & 'a str, expiration: & 'a i64) -> WinSend <'a>
      {
        Self::wintoast_send(action, app_name, app_user_model_id, text, img, attr, expiration);
        WinSend { action, app_name, app_user_model_id, text, img, attr, expiration}
      }

      pub fn wintoast_send(action: & 'a str, app_name: & 'a str,
                       app_user_model_id: & 'a str, text: & 'a str,
                       img: & 'a str, attr: & 'a str, expiration: & 'a i64) -> WinSend <'a>
      {
        // define variable
        let debug = Debug::new();
        let actions = widestring::WideCString::from_str(action).unwrap();
        let app_name_ = widestring::WideCString::from_str(app_name).unwrap();
        let app_user_model_id_ = widestring::WideCString::from_str(app_user_model_id).unwrap();
        let text_ = widestring::WideCString::from_str(text).unwrap();
        let image_path_ = widestring::WideCString::from_str(img).unwrap();
        let attribute_ = widestring::WideCString::from_str(attr).unwrap();
        let exp: i64 = *expiration;
        let mut arg = Vec::new();

        if !action.is_empty() {
            arg.push(format!("Action: {}", action));
        }
        // define print msg
        arg.push(format!("AppName: {}", app_name));
        arg.push(format!("AppID: {}", app_user_model_id));
        arg.push(format!("Body: {}", text));
        arg.push(format!("ImagePath: {}", img));
        arg.push(format!("Urgency Mode: {}", attr));
        arg.push(format!("Timeout: {}", expiration));

        debug.print_msg_arg(&arg);
        
        // check result
        unsafe {
            let result = show_notification(
                actions.as_ptr(),
                app_name_.as_ptr(),
                app_user_model_id_.as_ptr(),
                text_.as_ptr(),
                image_path_.as_ptr(),
                attribute_.as_ptr(),
                exp,
            );
            match result {
                0 => {debug.print("Notification displayed successfully");},
                1 => {debug.error("System not supported!");},
                2 => {debug.error("Initialization failed!");},
                3 => {debug.error("Toast failed to display!");},
                _ => {debug.error("Unknown error occurred!");},
            }
        }
        WinSend { action, app_name, app_user_model_id, text, img, attr, expiration}
      }
}

//fn main() {
    //let val = WinSend::new();
    //val.win32_send("", "Example", "Console", "Hello word", "", "Default", &0);

    //WinSend::wintoast_send("", "Example", "Console", "Hello word", "", "Default", &0);
//}
