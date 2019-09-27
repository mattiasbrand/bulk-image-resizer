use image::GenericImageView;

fn main() -> Result<(), String> {
    let img_paths: Vec<std::path::PathBuf> = std::env::args()
        .skip(1)
        .map(|x| std::path::PathBuf::from(x))
        .collect();

    if img_paths.len() == 0 {
        return Err("No file paths were supplied".to_string());
    }

    // Get width and height from first image
    let first_img =
        image::open(img_paths.first().unwrap()).map_err(|x| -> String { x.to_string() })?;
    let current_width = first_img.width();
    let current_height = first_img.height();
    println!(
        "Source image has width {:?} and height {:?}",
        current_width, current_height
    );

    // Get desired width from user
    println!("What new width do you want for these images (will maintain ratio)?");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let new_width: u32 = input.trim().parse().unwrap();

    // Perform conversion
    let new_height = current_height / (current_width / new_width);
    let file_prefix = format!("{}x{}", new_width, new_height);
    for img_path in &img_paths {
        let p = img_path.as_path();
        let new_file_name = format!(
            "{}_{}.{}",
            p.file_stem().unwrap().to_string_lossy(),
            file_prefix,
            p.extension().unwrap().to_string_lossy()
        );
        let mut new_path = std::path::PathBuf::new();
        new_path.push(p.parent().unwrap());
        new_path.push(new_file_name);
        let img = image::open(img_path).map_err(|x| -> String { x.to_string() })?;
        if img.width() != current_width || img.height() != current_height {
            println!(
                "Skipping image {:?} due to not having same width: {:?} and height: {:?}",
                img_path,
                img.width(),
                img.height()
            );
            continue;
        } else {
            println!("Writing new image: {:?}", new_path);
        }

        let new_image = img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        new_image.save(new_path).expect("Failed to save image");
    }

    Ok(())
}
