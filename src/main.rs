fn main() {
   let argslist:Vec<String> = std::env::args().collect();
   if argslist.len()==1{
    clearscreen::clear().expect("failed to clear screen");
    println!("Insufficent Amount of Argument Passed 😢 Exiting Program");
    println!("Run Program with -h flag to show Help");
   }else{
    if argslist.len()>=2{
        match &argslist[1][..] {
            "-h"=>{
            clearscreen::clear().expect("Unable to Clear Screen");
            println!("To Run The App start with argument \n");    
            println!("start=\"PORT NUMBER HERE\"\n");
            println!("end=\"PORT NUMBER HERE\"\n");
            println!("port-sniffer start=3000 end=3000");
            }
            rest=>{
                if rest.contains("start="){
                    clearscreen::clear().expect("Unable to Clear Screen");
                    if argslist.len()==3{
                        if argslist[2].contains("end="){
                            clearscreen::clear().expect("Unable to Clear Screen");
                            let start:Vec<&str> = argslist[1].split("=").collect();
                            let start_Num:i32  = start[1].parse().expect("Unable to Convert Number");
                            let end:Vec<&str> = argslist[2].split("=").collect();
                            let end_Num:i32 = end[1].parse().expect("Unable to Convert Number");
                            println!("{} \n{}",start_Num,end_Num);
                            if end_Num-start_Num<0{
                                clearscreen::clear().expect("Unable to Clear Screen");
                                println!("Invalid Range");
                            }else{
                                port_sniffer::sniffer(start_Num, end_Num);
                            }


                        }else{
                            clearscreen::clear().expect("Unable to Clear Screen");
                            println!("Unable to Parse end Argument make use to use corrent case");
                        }
                    }else{
                        clearscreen::clear().expect("Unable to Clear Screen");
                        println!("Argument missing");
                    }
                }else{
                    clearscreen::clear().expect("Unable to Clear Screen");
                    println!("Unable to Parse start Argument make use to use corrent case");
                }
                
            }
            
            }
            
        }
    }
   }

