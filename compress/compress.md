
## 1. 使用compress压缩文件
### 1.1 使用到的库`flate2 `,配置文件Cargo.toml中添加(或通过cargo add flate2 添加)
```toml
[dependencies]
flate2 = "1.0.25"
```
### 1.2 使用库`flate2 `压缩代码
```rust
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
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
}
```
### 1.3 使用方式（以book.pdf为例）
```bash
cargo run book.pdf book
```
### 1.4 运行输出结果
```bash
源文件大小: 2307697
目标文件大小: 1885921
压缩时间: 1.703851963s
```
## 2. 使用decompress解压文件(解压`zip`格式文件)

### 2.1 使用到的库`zip `,配置文件`Cargo.toml`中添加(或通过`cargo add zip`添加)
```toml
[dependencies]
zip = "0.6.4"
```
### 2.2 使用库解压代码
```rust
use std::fs;
use std::io;
use std::path::Path;
use zip::ZipArchive;

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
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    //args在第2个位置，第一个索引表示的是文件名
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

    0
}

```
### 2.3 使用方式（以book.pdf为例）
```bash
cargo run images.zip 
```
### 2.4 运行结果
- 运行结果如下：
```bash
设置文件夹路径 0 的目录 "images/"
设置文件夹路径 1 的目录 "images/axif/"
文件 2 提取到 "images/axif/img3.avif" (34794 bytes)
文件 3 提取到 "images/img1.jpg" (721480 bytes)
文件 4 提取到 "images/img2.jpg" (715163 bytes)
解压时间: 134.253736ms
```
- 解压文件路径如下：
```bash
images
├── axif
│   └── img3.avif
├── img1.jpg
└── img2.jpg
```