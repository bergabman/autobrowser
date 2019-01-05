extern crate autopilot;
extern crate rand;

extern crate image;
use autopilot::geometry::{Point, Rect, Size};
use std::path::Path;

use rand::Rng;

// move the mouse across the screen 
const TWO_PI: f64 = std::f64::consts::PI * 2.0;
fn main() {
    let screen_size = autopilot::screen::size();
    println!("{}", &screen_size);

    let scoped_height = screen_size.height / 2.0 - 10.0; // Stay in screen bounds.
    println!("{}", &scoped_height);
    
    let mut rng = rand::thread_rng();
    for x in 0..screen_size.width as u64 {
        //println!("x {}", &x);
        let y = (scoped_height * ((TWO_PI * x as f64) / screen_size.width).sin() + 
                 scoped_height).round();
        //println!("y {}", &y);
        let duration: u64 = rng.gen_range(1, 3);
        //println!("duration {}", &duration);
        autopilot::mouse::move_to(autopilot::geometry::Point::new(
            x as f64,
            y as f64
        )).expect("Unable to move mouse");
        std::thread::sleep(std::time::Duration::from_millis(duration));
    }
    //type_hello();
    let bmp = autopilot::bitmap::capture_screen().expect("Unable to capture screen");
    let portion = autopilot::bitmap::capture_screen_portion(Rect::new(
        Point::new(100.0, 100.0),
        Size::new(100.0, 100.0),
    )).expect("Unable to capture screen portion");
    let bmp_path = Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("screenshot.png");
    let portion_path = Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("screenshot_cropped.png");
    let _ = bmp
        .image
        .save(&bmp_path)
        .expect("Unable to save screenshot");
    let _ = portion
        .image
        .save(&portion_path)
        .expect("Unable to save cropped screenshot");
    //needle = autopy.bitmap.Bitmap.open('needle.png')
    //let pos = autopilot::bitmap::Bitmap::
    let ffox_path = image::open("C:\\rust\\autobrowser\\ffox_new.png").expect("Cant open ffox.png");
    let ffox = autopilot::bitmap::Bitmap::new(ffox_path, Some(1 as f64));

    println!("bitmap loaded, image searct startup");
    let pos = bmp.find_bitmap(&ffox, None, None, None).unwrap_or(autopilot::geometry::Point::new(
            0 as f64,
            0 as f64
        ));

    autopilot::mouse::move_to(pos).expect("Unable to move mouse");
    autopilot::mouse::click(autopilot::mouse::Button::Left, None);

    //autopilot::mouse::move_to(autopilot::geometry::Point::new(x as f64, y as f64)).expect("Unable to move mouse");

    println!("{:?}", pos);  //printing position of the found bitmap sample
    println!("Scale factor {}", autopilot::screen::scale());
    println!("Screen size {}", autopilot::screen::size());
    println!("Saved screenshot at {}", bmp_path.to_str().unwrap_or(""));
    println!("Saved cropped screenshot at {}", portion_path.to_str().unwrap_or(""));
}

fn type_hello() {
    autopilot::key::type_string("Hello, world!", &[], 200., 0.);
    let _ = autopilot::alert::alert("Hello, world!", None, None, None);
}