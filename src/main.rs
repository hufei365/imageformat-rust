extern crate image;
extern crate glob;

use std::env;
use std::path::PathBuf;
use glob::glob;

fn main() {
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: image-converter <output_format> <input_path>");
        return;
    }

    // 获取命令行参数
    let output_format = &args[1];
    let input_path = &args[2];

    // 使用glob模式获取输入路径下的所有匹配文件
    let mut input_files: Vec<PathBuf> = Vec::new();
    for entry in glob(input_path).expect("Failed to read input path") {
        if let Ok(path) = entry {
            input_files.push(path);
        }
    }

    // 遍历每个输入文件并进行格式转换
    for input_file in input_files {
        // 打开输入图片文件
        let input_image = image::open(&input_file).expect("Failed to open input image");

        // 构造输出图片文件路径
        let output_filename = format!("imageshrap_{}.{}", input_file.file_stem().unwrap().to_string_lossy(), output_format);
        let output_path = input_file.with_file_name(output_filename);

        // 将图片保存为指定格式
        input_image.save(&output_path).expect("Failed to save output image");

        println!("Image converted to {} format: {}", output_format, output_path.display());
    }
}