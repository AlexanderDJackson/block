use std::env;
use std::io::Write;
use std::io::ErrorKind;
use std::fs::OpenOptions;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut filename = "/etc/hosts";
	let mut address  = "0.0.0.0";
        //The first argument should always be the website to block
	let website  = &args[1];

	
	//Verbose option, quiet by default
	let verbose = args.contains(&"-v".to_string());

	for i in 2..args.len() {
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
	}

	if website != "" {
		let hosts = OpenOptions::new()
			.write(true) //write mode
			.create(true) //create the file if it doesn't already exist
			.append(true) //don't reset the file upon first write
			.open(filename);
			//.expect(&format!("Unable to open {}", filename));
            
            let mut hosts = match hosts {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::PermissionDenied => {
                        println!("Permission denied for {}", filename);
                        return
                    }
                    _ => panic!("{}", error)
                },
            };

            let mut to_write = format!("{}\t\t{}\t#Block\n", address, website);

            if address != "0.0.0.0" {
                to_write = format!("{}\t{}\t#Block\n", address, website);
            }

            hosts.write_all(to_write.as_bytes()).expect("Write failed.");
	}
}
