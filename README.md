## Rust Notification Send (RSNotifyOS)

RSNotifyOS is a library for printing alerts on Linux and Windows operating systems. It uses the D-Bus IPC and Subprocess protocol to communicate with the desktop notification system on Linux, Implements a dialog window system to display alerts on both operating systems.

## Usage
    Functions Use:
       - Print a notification (linux):
            command:
                ```
                extern crate rsnotifyos;
    		use rsnotifyos::RNotify;
    		
    		fn main() {
    		    RNotify::cmd_notify("Rust", "dialog-information", "normal", "Hello Word", "", "", 1000);
    		}
              ```    		
            dbus:
               ```
                extern crate rsnotifyos;
    		use rsnotifyos::RNotify;
    		
    		fn main() {
    		    RNotify::dbus_notify("RNotification", "Notification", "Hello Word!", "dialog-information", &0, &1000)
    		}
    	       ```	
       - Print an alert message in a window (linux, windows):
             information:
       		```
       		extern crate rsnotifyos;
       		use rsnotifyos::RNotify;
       		
       		fn main() {
       		     RNotify::alert("INFO","Menssage Rust");
       		}
       	       ```
             error:
                ```
                extern crate rsnotifyos;
       		use rsnotifyos::RNotify;
       		
       		fn main() {
       		    RNotify::alert("ERROR","Error: Menssage Alert Rust");
       		}
       	       ```
       		ref doc/usage.md
## Installation
       build linux:
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
          sudo apt-get install libpango1.0-dev libgdk-pixbuf-2.0-dev libglib2.0-dev libgtk-3-dev libcairo2-dev
          git clone https://github.com/frerd7/rsnotifyos.git
          cargo build
          cargo test
      or:
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
          cargo install rnotify
      build windows:
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
          git clone https://github.com/frerd7/rsnotifyos.git
          cargo build --features enable_debug
          cargo test
      or:
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
          cargo install rnotify
      
      debug:
          cargo --build --features enable_debug 
## Autor
Frederick Valdez
