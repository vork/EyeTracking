use std::sync::mpsc::{channel, Receiver};
use std::thread;
use streamer::Stream;
use image::{GenericImage, DynamicImage, Rgb, Pixel};
use image::imageops::colorops;
use image_loader;


#[derive(Clone)]
pub struct dummy_stream{
    dim: (u32, u32),
    img: DynamicImage
}

impl Stream for dummy_stream {
    fn setup() -> dummy_stream {
        let img = image_loader::open_image("eye.jpg");
        dummy_stream{ dim: (img.width(), img.height()), img: img }
    }

    fn fetch_images(&self) -> (thread::JoinHandle<()>, Receiver<Vec<u8>>) {
        let (sender, receiver) = channel();
        let self_clone = self.clone();

        let handler = thread::spawn(move || {
            loop {
                let gray = self_clone.img.grayscale();
                if let Err(_) = sender.send(gray.raw_pixels()) {
                    println!("Image sending failed!");
                    break;
                }
            }
            println!("No more images in queue");
        });

        println!("End of fetch images");

        (handler, receiver)
    }

    fn get_resolution(&self) -> (u32, u32) {
        self.dim
    }
}