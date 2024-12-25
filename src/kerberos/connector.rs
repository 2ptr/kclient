use std::net::*;
use std::io::prelude::*;

pub struct ADUser {
    domain : String,
    username : String,
    password : String
}

impl ADUser {
    /*
        Constructor for Active Directory user identities.

        === TODO:
        - Domain name checks
        - Hash / Kerberos key fields
     */
    pub fn new(domain : String, username : String, password : String) -> ADUser {
        ADUser{
            domain: domain,
            username: username,
            password: password
        }
    }
}

pub struct KerberosConnection {
    con_string : String,
    stream : TcpStream,
    authenticated : bool,
    identity: ADUser
}

impl KerberosConnection {

    /*
        Constructor for Kerberos connections. These structs handle
        TCP connections to host AND the domain identity associated with each
        connection.
     */
    pub fn new(args : Vec<String>) -> KerberosConnection {

        // Split connector string argument for FQDN or IP address
        let connector_string = args.last().unwrap();
        let host_string = format!("{}:88",
                                     connector_string.split("@").last().unwrap().to_owned()
        );

        // Open TCP stream
        let mut stream = TcpStream::connect(host_string.clone())
            .expect("Unable to connect to host.");

        // Identity handling
        // This is already an "anonymous" AD User struct
        let mut identity = ADUser::new(
            String::new(),
            String::new(),
            String::new()
        );
        let mut authenticated = false;

        // Anonymous connections have no identity provided
        let identity_string = connector_string.split("@")
            .collect::<Vec<&str>>()[0].to_string();
        if !(identity_string == "")
        {
            authenticated = true;
        }

        println!("[+] Established TCP Kerberos connection for {}", host_string);
        println!("[*] Authenticated -> {}", authenticated);

        KerberosConnection {
            con_string: host_string,
            stream: stream,
            authenticated: authenticated,
            identity: identity
        }
    }

    pub fn write_stream(&mut self, buf: &[u8])
    {
        self.stream.write(buf);
        self.stream.flush();
    }

    pub fn read_stream(&mut self, buf: &mut Vec<u8>)
    {
        self.stream.read_to_end(buf);
    }
}