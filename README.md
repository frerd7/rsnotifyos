## Rust Notification Send (RSNotifyOS)

RSNotifyOS is a library for printing alerts on Linux and Windows operating systems. It uses the D-Bus IPC and Subprocess protocol to communicate with the desktop notification system on Linux, Implements a dialog window system to display alerts on both operating systems.

## Usage
Functions Use:
 - Print a notification (linux):
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
## Autor
Frederick Valdez
