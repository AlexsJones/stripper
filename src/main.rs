use std::{env, io::{Read, BufReader, BufRead}};
use colored::*;
use walkdir;

fn help() {
    println!("Usage: st [options] <matcher>");
    println!("");
    println!("-p Prompt for a string to strip out");
}
fn main() {
    let mut prompt = false;
    // Parse default args
    let globalargs: Vec<_> = env::args().collect();
    // copy the immutable args to local
    let mut args = globalargs.clone();
    if args.len() < 2 {
        return help()
    }

    // clone slice from args
    let src = args.clone();
    // Find any args that have a -p flag and remove them
    for (i, arg) in src.iter().enumerate() {
        if arg == "-p" {
            prompt = true;
            args.remove(i);
        }
    }
    let matcher = args[1].clone();
    println!("{}",format!("Stripping out {}", matcher.blue().bold()).blue());

    // File tree walk
    let mut walker = walkdir::WalkDir::new(".");
    walker = walker.follow_links(true);
    for entry in walker {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            // if env DEBUG set printout path
            if env::var("DEBUG").is_ok() {
                println!("{}", path.display());
            }
            {
                let mut file = match std::fs::File::open(&path) {
                    Ok(file) => file,
                    Err(_) => continue,
                };
                // copy the entire file into an in memory file
                // go line by line on that file and remove the matching string
                let mut contents = String::new();
                let _: () = match file.read_to_string(&mut contents) {
                    Ok(_) => (),
                    Err(_) => continue,
                };
                drop(file);
                // if the file doesnt have the matcher then just close it and move on
                if !contents.contains(&matcher) {
                    continue;
                }

                let mut new_contents = String::new();
                let mut reader = BufReader::new(contents.as_bytes());
                
                let mut line = String::new();
                let mut count = 0;
                let mut found = false;
                while reader.read_line(&mut line).unwrap() > 0 {
                    if !line.contains(&matcher) {
                        new_contents.push_str(&line);
                    }
                    // find the matcher in the line
                    let inner_line: std::str::Split<'_, &str> = line.split(" ");
                    let mut new_line: String = String::new();
                    let mut count = 0;
      
                    for word in inner_line {
                        if !word.contains(&matcher) {
                            new_line.push_str(&word);
                            new_line.push_str(" ");
                        }else {
                            found = true;
                        }

                        count +=1;
                    }
                    new_contents.push_str(&new_line);
                }

                if prompt && found == true {
                    // Ask if they are sure they want to strip it out
                    println!("{}",format!("Are you sure you want to strip out {} from {}? [y/n]",
                    matcher, path.display().to_string()).red());
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "y" {
                        continue;
                    }
                }

                // write the new contents back to the file
                match std::fs::write(path, new_contents) {
                    Ok(_) => (),
                    Err(_) => continue,
                }
                count += 1;
                println!("{} line {}",format!("Stripped out {} from {}",
                 matcher,
                 path.display().to_string()),count);   

      

            
            }

        }
    }

}

