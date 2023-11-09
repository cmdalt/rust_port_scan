use std::{
    net::{SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
};

fn main() {
    let adresse = "192.168.178.1".to_string();
    let listem = vec![80, 90, 100];
    scan_port(&adresse, listem);
}

fn scan_port(addr: &String, list: Vec<u16>) {
    let mut handles = vec![];

    for port in list {
        let host = format!("{}:{}", addr, port);

        let handle = std::thread::spawn(move || {
            let socket_addr = SocketAddr::from_str(host.as_str()).unwrap();
            match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
                Ok(_) => println!("port {} is Open", port),
                Err(_) => println!("Port {} is Not Open", port),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
