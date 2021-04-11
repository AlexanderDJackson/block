use std::env;
use std::io::Write;
use std::fs::OpenOptions;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut filename = "/etc/hosts";
	let mut address  = "0.0.0.0";
	let mut website  = "";
	
	//Verbose option, quiet by default
	let verbose = args.contains(&"-v".to_string());

	for i in 1..args.len() {
		//-o: alternate output file
		if args[i] == "-o" {
			if verbose && i == args.len() - 1 {
				println!("No option was provided after -o. Ignoring and writing to {}.", filename);
			}
			else {
				filename = &args[i + 1];

				if verbose {
					println!("Writing to {}", filename);
				}
			}
		}
		//-a: alternate resolution address
		else if args[i] == "-a" {
			if verbose && i == args.len() - 1 {
				println!("No option was provided after -a. Ignoring.");
			}
			else {
				address = &args[i + 1];

				if verbose {
					println!("External address set to {}", address);
				}
			}
		}
		//The first or last argument should be the website to block 
		else if i == args.len() - 1 || i == 1 {
			website = &args[i];
		}
	}

	if website != "" {
		let mut hosts = OpenOptions::new()
			.write(true) //write mode
			.create(true) //create the file if it doesn't already exist
			.append(true) //don't reset the file upon first write
			.open(filename) 
			.expect(&format!("Unable to open {}", filename));
		hosts.write_all(format!("{}\t{}\t#Block\n", address, website).as_bytes()).expect("Write failed.");
	}
}
