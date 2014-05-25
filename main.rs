use std::io;
use std::os;
use std::num;
use std::str;
use std::vec;
use std::rc::Rc;
use self::server::start_server;
use self::server::ServerSpec;

mod server;

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: {} <port...>", args[0]);
        return;
    }
    let num_servers:uint = args.len() - 1;
    let mut servers:Vec<server::ServerSpec> = Vec::new();
    for i in range(1, num_servers + 1) {
        let server_port:u16 = from_str(args[i]).unwrap();
        let server_host = "localhost";
        servers.push(server::ServerSpec{host: "localhost", port:server_port});
    }
    for i in range(0, num_servers) {
        server::start_server(i, &servers);
    }
    loop {}
    println!("Launching {} servers", num_servers);
}

