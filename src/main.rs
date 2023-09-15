use std::{env, io::{Read, BufReader, BufRead}};
use colored::*;
use walkdir;

fn main() {
    // Parse default args
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Requires an expression to strip out");
        return;
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
                while reader.read_line(&mut line).unwrap() > 0 {
                    if !line.contains(&matcher) {
                        new_contents.push_str(&line);
                    }
                    // find the matcher in the line
                    let inner_line: std::str::Split<'_, &str> = line.split(" ");
                    let mut new_line: String = String::new();
                    let mut count = 0;
                    let mut position = 0;
                    for word in inner_line {
                        if !word.contains(&matcher) {
                            new_line.push_str(&word);
                            new_line.push_str(" ");
                        }else {
                            position = count.clone();
                        }

                        count +=1;
                    }
                    new_contents.push_str(&new_line);
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

