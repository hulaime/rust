use std::fs;
use std::io;
use std::path::Path;
use zip::ZipArchive;
use std::time::Instant;
fn main() {
    //在real_main函数中处理真正的程序
    //为了干净的结束和退出，在主函数中使用它
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    //创建一个名为 args 的向量来收集用户在 CLI 中的输入
    let args: Vec<_> = std::env::args().collect();
    //如果 args 小于 2，就会出现问题，因为你需要发送
    // zip 文件，它会告诉你如何使用
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }
    //args在第2个位置，第一个索引表示的是文件名
    let start = Instant::now();
    let fname = Path::new(&*args[1]);
    //使用标准fs打开文件
    let file = fs::File::open(&fname).unwrap();

    //使用存档阅读器功能
    let mut archive = ZipArchive::new(file).unwrap();

    //从0开始，覆盖整个archive的长度
    // zip 存档中会有多个文件，我们需要提取所有文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        //设置提取文件的路径
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        // zip 也可以包含其他文件夹
        if (*file.name()).ends_with('/') {
            println!("设置文件夹路径 {} 的目录 \"{}\"", i, outpath.display());
            //递归创建一个新目录
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "文件 {} 提取到 \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            //如果这些文件没有父目录，则创建一个新目录
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Linux下获取和设置提取文件的权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    println!("解压时间: {:?}", start.elapsed());
    0
}
