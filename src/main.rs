extern crate image;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize)]
struct ReglTexture {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = std::path::Path::new(&args[1]);
    assert!(path.exists());
    let img = image::open(&path).expect("open image failed").to_rgba();
    //let img = image::imageops::flip_horizontal(&img);
    let (width, height) = img.dimensions();
    let data: Vec<u8> = img.into_raw();
    let regl_texture = ReglTexture {
        width,
        height,
        data,
    };
    let json = serde_json::to_string(&regl_texture).unwrap();
    print!("{}", json);
}
