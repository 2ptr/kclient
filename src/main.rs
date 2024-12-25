mod misc;
mod kerberos {
    pub mod connector;
    pub mod asktgt;
    pub mod messages;
}

use kerberos::asktgt::{asktgt};
use misc::splash;
use std::env;

fn main() {

    /*
        Argument handling should be very similar to impacket. The connection
        "identity" is the last argument:

        DOMAIN.COM/USER:PASSWORD@<IP OR FQDN>

     */
    splash();
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: <action> <args> <connection string>");
        return();
    };

    // Match by action
    match args[1].as_str().to_lowercase().as_str() {

        // Request a TGT with cleartext password or NTLM hash
        "asktgt" => {
            println!("[*] Action: Get TGT");
            asktgt(args);
        }
        _ => {
            println!("[X] Not a valid action!");
        }
    }



}
