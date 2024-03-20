## Usage
    Replace impl to RNotify
    Functions:
       - impl::dbus_notify("title", "application name", "notification body", "/path/to/file/icon", &id, &tiemout); using funtion notify protocol dbus ipc menssage notification
       ----------------------------------------------------
         replace: title to argumment title
         replace: application name to app name
         replace: notification body to argumment
         replace: /path/to/file/icon to icon file png
         replace: id to numeric 0, 
         replace: timeout to numeric senconds notification
         
       --------------------------------------------------- 
       - impl::cmd_notify("application name", "/path/to/file/icon.png", "urgency", "notification body", "action", "category", timeout); using function notify subprocess menssage notification
       ----------------------------------------------------
         replace: application name to app name
         replace: /path/to/file/icon to icon file png
         replace: urgency to valid: (low, normal, critical)
         replace: notification body to argumment
         replace: action to exec or not event ""
         replace: category to notify action 
         replace: timeout to numeric senconds notification
         
       - impl::alert("mode", "menssage body"); using alert dialog window os
      -------------------------------------------------------
         replace: mode to valid alert info: (INFO, info) alert error: (Error, err, ERROR)
         
       Example:
        ```
        use rsnotifyos::RNotify; // import lib
        
	fn timeout(timeout: &i32) 
	{
   		println!("Sleep 1.0 seconds\n Do not close");
   		std::time::Duration::from_secs(timeout);
	}

	fn main() 
	{
     		RNotify::dbus_notify("RNotification", "Notification", "Send Notification Rust", "dialog-information", &0, &1000);
     		
     		// SLEEP
     		timout(&1000);
     		RNotify::cmd_notify("Rust", "dialog-information", "normal", "Send Notification Command using subprocess", "action.sh", "category", 1000);
     		
     		// sleep
     		timout(&1000);
     		RNotify::alert("ERR", "System Error Alert Menssage");
     		
     		// sleep    
     		timout(&1000);
     		RNotify::alert("info", "System Info Alert Menssage");
         } 
        ```
