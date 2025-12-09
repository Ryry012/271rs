use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;


fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: scm <add|commit|revert>");
        return;
    }

    let cmd = args[1].as_str();

    if cmd == "add" {
        if args.len() < 3 {
            println!("Usage: scm add <file>");
            return;
        }
        add_file(args[2].as_str());
        return;
    }


    if cmd == "commit" {
        if args.len() < 4 || args[2] != "-m" {
            println!("Usage: scm commit -m <message>");
            return;
        }
        do_commit(args[3].as_str());
        return;
    }

    if cmd == "revert" {
        revert_to_previous();
        return;
    }

    println!("Unknown command");
}



fn setup_repo() {

    if !Path::new(".scm").exists(){
        fs::create_dir(".scm").unwrap();
    }

    if !Path::new(".scm/commits").exists() {
        fs::create_dir(".scm/commits").unwrap();
    }

    if !Path::new(".scm/index").exists() {
        File::create(".scm/index").unwrap();
    }
}


fn add_file(fname: &str) {
    setup_repo();
    let mut idx = File::options().append(true).open(".scm/index").unwrap();
    writeln!(idx, "{}", fname).unwrap();
    println!("added {}", fname);
}



fn do_commit(msg: &str) {

    setup_repo();

    let mut idx_text = String::new();
    File::open(".scm/index").unwrap().read_to_string(&mut idx_text).unwrap();
    let files: Vec<&str> = idx_text.lines().collect();

    if files.is_empty() {
        println!("No files added.");
        return;
    }

    let mut num = 1;
    loop {
        let p = format!(".scm/commits/{}", num);
        if !Path::new(&p).exists() {
            break;
        }
        num += 1;
    }

    let commit_dir = format!(".scm/commits/{}", num);
    fs::create_dir(&commit_dir).unwrap();

    let mut mfile = File::create(format!("{}/message.txt", commit_dir)).unwrap();
    mfile.write_all(msg.as_bytes()).unwrap();


    for f in files {
        let mut contents = String::new();
        File::open(f).unwrap().read_to_string(&mut contents).unwrap();

        let mut out = File::create(format!("{}/{}", commit_dir, f)).unwrap();
        out.write_all(contents.as_bytes()).unwrap();
    }

    println!("commit #{} saved", num);
}




fn revert_to_previous() {

    setup_repo();

    let mut last = 0;

    if let Ok(ents) = fs::read_dir(".scm/commits") {
        for e in ents {
            let name = e.unwrap().file_name().into_string().unwrap();
            if let Ok(n) = name.parse::<i32>() {
                if n > last { last = n; }
            }
        }
    }

    if last <= 1 {
        println!("Nothing to revert.");
        return;
    }

    let prev = last - 1;

    let mut idx_text = String::new();
    File::open(".scm/index").unwrap().read_to_string(&mut idx_text).unwrap();
    let files: Vec<&str> = idx_text.lines().collect();


    for f in files {
        let snap = format!(".scm/commits/{}/{}", prev, f);


        if Path::new(&snap).exists() {
            let mut c = String::new();
            File::open(&snap).unwrap().read_to_string(&mut c).unwrap();

            let mut out = File::create(f).unwrap();
            out.write_all(c.as_bytes()).unwrap();
        }
    }

    println!("reverted to commit #{}", prev);
}
