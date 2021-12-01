use std::env;

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
    print " Help Menu:\n";
    print " --help, -?, -h   -> Prints the help ment";
    print " --terminal, -t   -> Opens the interactive CLI terminal";
    print " --motd           -> Prints the title banner";
    print " --shutdown       -> Close the local applications and shutdown the system";
    print " --cmd {command}  -> Runs misc command on the primary application";

}

/**
 * Sends a command over loopback to the application
 */

fn sendCommandtoLoopback(int port, String command, String arg) {

}

fn runTerminal() {

}

/**
 * Function the prints the banner
 */

fn printTitle() {

}

/**
 * Function that shuts down the appliance applications and then shuts down the pyhsical machine
 */

fn shutdown() {

}