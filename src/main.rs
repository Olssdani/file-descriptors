use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

fn main() {
    let run = Arc::new(AtomicBool::new(true));
    {
        let run = run.clone();
        ctrlc::set_handler(move || run.store(false, Ordering::SeqCst))
            .expect("Error setting Ctrl-C handler");
    }

    let mut programs: HashMap<i32, i32> = HashMap::new();

    while run.load(Ordering::SeqCst) {
        let mut current_fd_open = 0;
        let proc_dirs = fs::read_dir("/proc/").unwrap();

        for proc_dir_result in proc_dirs {
            if let Ok(proc_dir) = proc_dir_result {
                if let Ok(process_id) = proc_dir.file_name().to_string_lossy().parse::<i32>() {
                    let fd_path = format!("{}/fd/", proc_dir.path().to_string_lossy());
                    if let Ok(fd_dir) = fs::read_dir(fd_path.clone()) {
                        let current_fd_count = fd_dir.count() as i32;
                        current_fd_open += current_fd_count;
                        programs
                            .entry(process_id)
                            .and_modify(|fd_counts| {
                                if current_fd_count > *fd_counts {
                                    println!(
                                        "PID {} has increased FD by: {}",
                                        process_id,
                                        current_fd_count - *fd_counts
                                    );
                                }
                            })
                            .or_insert(current_fd_count);
                    }
                }
            }
        }
        println!("Current All Open: {current_fd_open}");

        println!("");
        std::thread::sleep(Duration::from_secs(2));
    }

    /*let mut file = File::create("test.csv").unwrap();
    for (id, descriptors) in programs.iter() {
        let data = id.to_string()
            + ","
            + &descriptors
                .iter()
                .map(|f| f.to_string() + ",")
                .collect::<String>();

        file.write_all(data.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }*/

    //println!("{programs:?}");
}
