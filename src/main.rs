extern crate autopilot;
extern crate image;
use autopilot::geometry::Point;
use autopilot::mouse;

use std::{thread, time};

fn main() {
    let ffox_path =
        image::open("C:\\rust\\autobrowser\\assets\\ffox_new.png").expect("Cant open ffox_new.png");
    let home_button_path =
        image::open("C:\\rust\\autobrowser\\assets\\home_button.png").expect("Cant open home_button.png");
    let goto_path =
        image::open("C:\\rust\\autobrowser\\assets\\goto.png").expect("Cant open goto.png");
    let newtab_path =
        image::open("C:\\rust\\autobrowser\\assets\\newtab.png").expect("Cant open newtab.png");
    let login_path =
        image::open("C:\\rust\\autobrowser\\assets\\login.png").expect("Cant open login.png");
    let loginbutton_path =
        image::open("C:\\rust\\autobrowser\\assets\\loginbutton.png").expect("Cant open loginbutton.png");
    let upvotebutton_path =
        image::open("C:\\rust\\autobrowser\\assets\\upvote.png").expect("Cant open upvotebutton.png");
    let newposts_path =
        image::open("C:\\rust\\autobrowser\\assets\\newposts.png").expect("Cant open newposts.png");

    let ffox = autopilot::bitmap::Bitmap::new(ffox_path, Some(1 as f64));
    let home_button = autopilot::bitmap::Bitmap::new(home_button_path, Some(1 as f64));
    let goto = autopilot::bitmap::Bitmap::new(goto_path, Some(1 as f64));
    let newtab = autopilot::bitmap::Bitmap::new(newtab_path, Some(1 as f64));
    let login = autopilot::bitmap::Bitmap::new(login_path, Some(1 as f64));
    let loginbutton = autopilot::bitmap::Bitmap::new(loginbutton_path, Some(1 as f64));
    let upvotebutton = autopilot::bitmap::Bitmap::new(upvotebutton_path, Some(1 as f64));
    let newposts = autopilot::bitmap::Bitmap::new(newposts_path, Some(1 as f64));

    println!("Bitmaps loaded, image search startup");
    let screenshot = autopilot::bitmap::capture_screen().expect("Unable to capture screen");

    let pos_ffox = screenshot
        .find_bitmap(&ffox, None, None, None)
        .unwrap_or(Point::new(0 as f64, 0 as f64));
    println!("{:?}", pos_ffox);
    if pos_ffox == Point::new(0 as f64, 0 as f64) {
        println!("firefox not found");
    } else {
        smooth_mmove(pos_ffox);
        mouse::click(mouse::Button::Left, None);
        thread::sleep(time::Duration::from_millis(30));
    }

    let screenshot_1 = autopilot::bitmap::capture_screen().expect("Unable to capture screen");
    let pos_newtab = screenshot_1
        .find_bitmap(&newtab, None, None, None)
        .unwrap_or(Point::new(0 as f64, 0 as f64));
    mouse::smooth_move(pos_newtab, Some(0.01 as f64)).expect("Unable to move mouse");
    mouse::click(mouse::Button::Left, None); //click on the position with 100ms

    let pos_home = screenshot_1
        .find_bitmap(&home_button, None, None, None)
        .unwrap_or(Point::new(0 as f64, 0 as f64));
    let newpt = Point::new(pos_home.x + 500 as f64, pos_home.y + 10 as f64);
    mouse::smooth_move(newpt, Some(0.01 as f64)).expect("Unable to move mouse");
    mouse::click(mouse::Button::Left, None); //click on the position with 100ms
    autopilot::key::type_string("steemit.com", &[], 200., 0.);

    let screenshot_2 = autopilot::bitmap::capture_screen().expect("Unable to capture screen");
    let pos_goto = screenshot_2
        .find_bitmap(&goto, None, None, None)
        .unwrap_or(Point::new(0 as f64, 0 as f64));
    mouse::smooth_move(pos_goto, Some(0.0001 as f64)).expect("Unable to move mouse");
    mouse::click(mouse::Button::Left, None); //click on the position with 100ms
    thread::sleep(time::Duration::from_millis(3000));

    let screenshot_3 = autopilot::bitmap::capture_screen().expect("Unable to capture screen");
    let pos_login = screenshot_3
        .find_bitmap(&login, None, None, None)
        .unwrap_or(Point::new(0 as f64, 0 as f64));
    let pos_login = Point::new(pos_login.x + 10 as f64, pos_login.y + 10 as f64);
    mouse::smooth_move(pos_login, Some(0.001 as f64)).expect("Unable to move mouse");
    mouse::click(mouse::Button::Left, None); //click on the position with 100ms
    thread::sleep(time::Duration::from_millis(3000));

    let screenshot_4 = autopilot::bitmap::capture_screen().expect("Unable to capture screen");
    let pos_loginbutton = screenshot_4
        .find_bitmap(&loginbutton, None, None, None)
        .unwrap_or(Point::new(0 as f64, 0 as f64));
    mouse::smooth_move(pos_loginbutton, Some(0.01 as f64)).expect("Unable to move mouse");
    mouse::click(mouse::Button::Left, None); //click on the position with 100ms
    thread::sleep(time::Duration::from_millis(4000));
    autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Down, 3);

    let screenshot_5 = autopilot::bitmap::capture_screen().expect("Unable to capture screen");
    let pos_newposts = screenshot_5
        .find_bitmap(&newposts, None, None, None)
        .unwrap_or(Point::new(0 as f64, 500 as f64));
    let pos_newposts_new = Point::new(pos_newposts.x + 5 as f64, pos_newposts.y + 5 as f64);
    mouse::smooth_move(pos_newposts_new, Some(0.01 as f64)).expect("Unable to move mouse");
    mouse::click(mouse::Button::Left, None); //click on the position with 100ms
    thread::sleep(time::Duration::from_millis(4000));
    autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Down, 10);

    let mut counter: usize = 0;

    // loop to take screenshot and search for first upvote button and click it
    loop {
        let screenshot_6 =
            autopilot::bitmap::capture_screen().expect("Unable to capture screen");
        let pos_upvote = screenshot_6
            .find_bitmap(&upvotebutton, Some(0.1), None, None)
            .unwrap_or(Point::new(0 as f64, 500 as f64));
        let pos_upvote_new = Point::new(pos_upvote.x + 3 as f64, pos_upvote.y + 5 as f64);
        mouse::smooth_move(pos_upvote_new, Some(0.01 as f64)).expect("Unable to move mouse");
        mouse::click(mouse::Button::Left, None); //click on the position with 100ms
        thread::sleep(time::Duration::from_millis(3000));

        mouse::smooth_move(Point::new(1200 as f64, 600 as f64), Some(0.1 as f64))
            .expect("Unable to move mouse");
        autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Down, 3);
        thread::sleep(time::Duration::from_millis(500));
        counter += 1;

        //  A bit of random scrolling 
        if counter == 4 {
            autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Down, 10);
            thread::sleep(time::Duration::from_millis(300));
            autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Down, 10);
            thread::sleep(time::Duration::from_millis(500));
            autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Down, 10);
            counter = 0;
        }
    }
}

fn smooth_mmove(pos: Point) {
    let newpt = Point::new(pos.x + 10 as f64, pos.y + 10 as f64);
    println!("{}", newpt);
    mouse::smooth_move(newpt, Some(0.1 as f64)).expect("Unable to move mouse");
}
