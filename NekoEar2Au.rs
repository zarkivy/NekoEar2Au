use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::mem::transmute;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage:");
        return;
    }

    for next_file_path in &args[1..] {
        conversion(&next_file_path);
    }

    println!("Done!");
}

fn conversion(origin_file_path: &String) {
    let origin_path = Path::new(origin_file_path);
    let origin_display = origin_path.display();
    let mut origin_fd = match File::open(origin_file_path) {
        Err(why) => panic!("could not open {}: {}", origin_display, why),
        Ok(fd) => fd,
    };

    let mut buffer = Vec::new();
    match origin_fd.read_to_end(& mut buffer) {
        Err(why) => panic!("could not read {}: {}", origin_display, why), 
        Ok(_) => (),
    };

    let (au_start_addr, au_file_size) = unsafe {
        ( transmute::<[u8; 4], u32>([buffer[20], buffer[21], buffer[22], buffer[23]]),
        transmute::<[u8; 4], u32>([buffer[40], buffer[41], buffer[42], buffer[43]]) )
    };
    
    println!("{}, {}", au_start_addr, au_file_size);

    let head_index = au_start_addr as usize;
    let mut target_file_path = String::from(origin_file_path);

    if (buffer[head_index] == 0x49 
    && buffer[head_index+1] == 0x44
    && buffer[head_index+2] == 0x33)
    || (buffer[head_index] == 0xff
    && buffer[head_index+1] == 0xfb){
        println!("mp3 file!");
        target_file_path += ".mp3";
    }

    else if (buffer[head_index+4] == 0x66
    && buffer[head_index+5] == 0x74
    && buffer[head_index+6] == 0x79
    && buffer[head_index+7] == 0x70
    && buffer[head_index+8] == 0x4D
    && buffer[head_index+9] == 0x34
    && buffer[head_index+10] == 0x41)
    || (buffer[head_index] == 0x4D
    && buffer[head_index+1] == 0x34
    && buffer[head_index+2] == 0x41
    && buffer[head_index+3] == 0x21) {
        println!("m4a file!");
        target_file_path += ".m4a";
    }

    else {
        println!("other file format...");
        target_file_path += ".audio";
    }

    let target_path = Path::new(&target_file_path);
    let target_display = target_path.display();
    let mut target_fd = match File::create(&target_path) {
        Err(why) => panic!("could not create {}: {}", target_display, why),
        Ok(fd) => fd,
    };

    target_fd.write_all(&buffer[au_start_addr as usize .. (au_start_addr+au_file_size) as usize]).expect("write failed!");
}    
