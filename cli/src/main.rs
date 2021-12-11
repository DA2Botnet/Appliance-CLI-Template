use std::env;
use std::process::Command;
use std::net;
use std::vec;

/**
 * CLI App that interfaces with both the primary application and other appliance resources
 * Runs from the appliance itself from the terminal
 * 
 * Runs by using either command-line arguments or as its own CLI terminal
 * 
 * Written by Joseph Telaak
 */

fn main() {
    // Get args
    let args: Vec<String> = env::args().collect();

    // If no args, open the CLI terminal
    if (len(args) == 0) { 
        runTerminal();

    // If args, run them
    } else {
        runCommands(&args);

    }

    // Close
    print("Closing....");

}

// Handle the input commands and run the approptiate functions
fn runCommands(Vec<String> args) {
    // Pull first arg
    let arg1 = &args[1];
    
    // Case with no arg
    if (len(args) == 1) {
        match arg1 {
            "--help"  => printHelp();
            "-t" => runTerminal();
            "--terminal" => runTerminal();
            "-?"  => printHelp();
            "-h"  => printHelp();
            "--motd" => printTitle();
            "--shutdown" = shutdown();
            _  => printHelp();

        }
    
    // Case with arg
    } if (len(args) == 2) {
        match arg1 {
            "--cmd" => sendCommandtoLoopback("cmd", &args[2]);
            "--command" => sendCommandtoLoopback("cmd", &args[2]);
            _  => printHelp();

        }
    }

}

/**
 * Function that prints the help screen
 */

fn printHelp() {
    println!(" Help Menu:");
    println!(" --help, -?, -h   -> Prints the help ment");
    println!(" --terminal, -t   -> Opens the interactive CLI terminal");
    println!(" --motd           -> Prints the title banner");
    println!(" --shutdown       -> Close the local applications and shutdown the system");
    println!(" --cmd {command}  -> Runs misc command on the primary application");

}

/**
 * Sends a command over loopback to the application
 */

fn sendCommandtoLoopback(int port, String command, String arg) {
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let listen_addr = net::SocketAddrV4::new(ip, port+1);
    let send_addr = net::SocketAddrV4::new(ip, port);

    send_message(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), command + " " + arg);
    
}

/**
 * Generic send message function
 */

fn send_message(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    println!("Sending data");

    let result = socket.send_to(&data, target);
    drop(socket);

    match result {
        Ok(amt) => println!("Sent {} bytes", amt),
        Err(err) => panic!("Write error: {}", err)

    }
}

/**
 * Reads input from the linux CLI and sends them over the network
 */

fn runTerminal(int port) {
    println!("Starting CLI.....");
    let mut in = String::new();

    while (in != "exit") {
        io::stdin().read_line(&mut in).expect("Failed to read line");
        sendCommandtoLoopback(port, in, "")

    }
}

/**
 * Function the prints the banner
 */

fn printTitle() {
    let mut file = std::fs::File::open("~banners/MainBanner.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

/**
 * Function that shuts down the appliance applications and then shuts down the pyhsical machine
 */

fn shutdown(int port) {
    sendCommandtoLoopback(port, "shutdown", "0");
    println!("Shutting down");
    let mut shutdown = Command::new("shutdown");


}