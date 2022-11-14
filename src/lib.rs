use std::net::TcpListener;
pub fn sniffer(startRange:i32,endRange:i32){
    clearscreen::clear().expect("Unable to Clear Screen");
    for i in startRange..=endRange {
        
        match TcpListener::bind(format!("127.0.0.1:{}",i)) {
            Ok(T)=>{
                println!("Port {} is Open for Use ✔️",i)
            }
            Err(err)=>{ println!("Port {} is Not Available for Use ❌",i);
                
            }

        }
    }
}