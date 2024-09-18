use std::io::Read;

// 10mb
const THRESHOLD: usize = 10 * 1024 * 1024;

fn main() {
    let mut msi = msi::open("Setup.msi").unwrap();
    let streams = msi.streams();
    for stream in streams {
        println!("Stream: {stream}");
    }

    let data1 = "Data1.cab";
    let mut sr = msi.read_stream(data1).unwrap();
    let mut total = 0;
    let mut buf = [0u8; 8*4096];
    loop {
        let n = sr.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        total += n;
        let total_mb = total as f64 / 1024.0 / 1024.0;
        println!("read: {total_mb:.2} - chunk: {n}");

        if total > THRESHOLD {
            break;
        }
    }
}
