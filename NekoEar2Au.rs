use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::mem::transmute;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("\x1b[31mUsage: ./NekoEar2Au <source_file1> <source_file2> ...");
        return;
    }

    for next_file_path in &args[1..] {
        conversion(&next_file_path);
    }
}


fn conversion(origin_file_path: &String) {
    let origin_path = Path::new(origin_file_path);
    let origin_display = origin_path.display();
    let mut origin_fd = match File::open(origin_file_path) {
        Err(why) => panic!("Couldn't open {}: {}", origin_display, why),
        Ok(fd) => fd,
    };

    let mut buffer = Vec::new();
    origin_fd.read_to_end(& mut buffer).expect("Read data failed!");

    let (au_start_addr, au_file_size) = unsafe {
        ( transmute::<[u8; 4], u32>([buffer[20], buffer[21], buffer[22], buffer[23]]),
        transmute::<[u8; 4], u32>([buffer[40], buffer[41], buffer[42], buffer[43]]) )
    };
    
    let head_index = au_start_addr as usize;
    let mut target_file_path = String::from(origin_file_path);

    if buffer[head_index..=head_index+2] == [0x49, 0x44, 0x33]
    || buffer[head_index..=head_index+1] == [0xff, 0xfb] {
        print!("\x1b[1;32;3;1m[FOUND] mp3 file :");
        target_file_path += ".mp3";
    }

    else if buffer[head_index+4..=head_index+10] == [0x66, 0x74, 0x79, 0x70, 0x4D, 0x34, 0x41]
    ||      buffer[head_index..=head_index+3]    == [0x4D, 0x34, 0x41, 0x21] {
        print!("\x1b[1;32;3;1m[FOUND] m4a file :");
        target_file_path += ".m4a";
    }

    else {
        print!("other file format...  ");
        target_file_path += ".audio";
    }

    let target_path = Path::new(&target_file_path);
    let target_display = target_path.display();
    let mut target_fd = match File::create(&target_path) {
        Err(why) => panic!("could not create {}: {}", target_display, why),
        Ok(fd) => fd,
    };

    target_fd.write_all(&buffer[au_start_addr as usize .. (au_start_addr+au_file_size) as usize]).expect("Write data failed!");
    
    println!("{}", target_display);
}    
