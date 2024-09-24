## Rust Notification Send (RSNotifyOS)

RSNotifyOS is a library for printing alerts on Linux and Windows operating systems. It uses the D-Bus IPC and Subprocess protocol to communicate with the desktop notification system on Linux, Windows. Implements a dialog window system to display alerts on both operating systems.

## News
    * Support for Windows notifications (10/11)
    * Combining C++ to Rust

## Usage
Functions Use:
 - Print a notification (Linux):
    command:
```rust
extern crate rsnotifyos;
use rsnotifyos::RNotify;
    		
fn main() {
    RNotify::cmd_notify("Rust", "dialog-information", "normal", "Hello Word", "", "", 1000);
}
```    		
   dbus:
```rust
extern crate rsnotifyos;
use rsnotifyos::RNotify;
    		
fn main() {
   RNotify::dbus_notify("RNotification", "Notification", "Hello Word!", "dialog-information", &0, &1000)
}
```
 - Print a notification (Windows 10/11):
```rust
extern crate rsnotifyos;
use rsnotifyos::RNotify;

fn main() {
   RNotify::win32notify("Action-Buttom", "App Name Rust Notify", "App user ID", "Text Body: Rust Notification from Windows", r"C:\path\example\win32icon.png", "Urgent", &0);
}
```
 - Print an alert message in a window (linux, windows):
   information:
```rust
extern crate rsnotifyos;
use rsnotifyos::RNotify;
       		
fn main() {
   RNotify::alert("INFO","Menssage Rust");
}
```
error:
```rust
extern crate rsnotifyos;
use rsnotifyos::RNotify;
       		
fn main() {
    RNotify::alert("ERROR","Error: Menssage Alert Rust");
}
```
ref doc/usage.md
## Installation
- build linux:
``` bash
sudo apt-get install libpango1.0-dev libgdk-pixbuf-2.0-dev libglib2.0-dev libgtk-3-dev libcairo2-dev
git clone https://github.com/frerd7/rsnotifyos.git
cargo build
cargo test
```
or:
``` bash
cargo install rnotify
```
- build windows:
Note: You must have Visual C++ installed in order to compile the Notifications library
``` bash
git clone https://github.com/frerd7/rsnotifyos.git
cargo test
```
or:
``` bash
cargo install rnotify
```      
- debug:
``` bash
cargo --build --features enable_debug
```
## Example
```rust
    cargo run --bin win_notify --features enable_debug
    cargo run --bin notify_event_loop --features enable_debug
    cargo run --bin dbus_notify --features enable_debug
```
## Autor
Frederick Valdez

## Credits
C++ Library -> Mohammed Boujemaoui <mohabouje@gmail.com> 
