use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    //在real_main函数中处理真正的程序
    //为了干净的结束和退出，在主函数中使用它
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return 1;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "源文件大小: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("目标文件大小: {:?}", output.metadata().unwrap().len());
    println!("压缩时间: {:?}", start.elapsed());
    0
}
