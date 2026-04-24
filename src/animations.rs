use std::default;
use std::io::{Write, stdout};
use std::os::linux::raw::stat;
use std::sync::{
    Arc, Mutex, mpsc,
    mpsc::{Receiver, Sender},
};
use std::thread::sleep;
use std::time::Duration;
use std::{marker::PhantomData, sync::atomic::AtomicBool};

use crate::{Colors, cprint};

pub trait Animation: Sized + std::default::Default + 'static + Send + Sync {
    fn new(args: NewAnimationOptions<Self>) -> Self;
    fn start(&mut self);
    fn stop(&mut self);
    fn next_frame(&mut self);

    fn animate(args: NewAnimationOptions<Self>) -> AnimationController {
        let stop = Arc::new(AtomicBool::new(false));
        let s1 = stop.clone();
        let progress = args.progress.clone();
        let animation_thread = Some(std::thread::spawn(move || {
            let frame_period_ms = args.frame_period_ms;
            let minimum_time = args.minimum_time;
            let mut animation = Self::new(args);
            animation.start();
            let s_time = std::time::Instant::now();
            loop {
                std::thread::sleep(Duration::from_millis(frame_period_ms));
                animation.next_frame();
                if let Some(t) = minimum_time {
                    if s_time.elapsed() < t {
                        continue;
                    }
                }

                if s1.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
            }
            animation.stop();
        }));

        AnimationController {
            animation: animation_thread,
            progress,
            stop,
        }
    }
}

pub struct AnimationController {
    animation: Option<std::thread::JoinHandle<()>>,
    stop: Arc<AtomicBool>,
    progress: Option<Arc<Mutex<Progress>>>,
}

impl AnimationController {
    fn _stop(&mut self) {
        if let Some(t) = self.animation.take() {
            self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
            let _ = t.join();
        }
    }

    pub fn progress_delta_n(&mut self, dn: usize) {
        if let Some(status) = self.progress.as_mut() {
            let mut tmp = status.lock().unwrap();
            tmp.n += dn;
            if tmp.n > tmp.max {
                tmp.n = tmp.max;
            }
        }
    }

    pub fn progress_set_n(&mut self, n: usize) {
        if let Some(status) = self.progress.as_mut() {
            let mut tmp = status.lock().unwrap();
            tmp.n = n;
            if tmp.n > tmp.max {
                tmp.n = tmp.max;
            }
        }
    }

    pub fn progress_delta_p(&mut self, dp: f64) {
        if let Some(status) = self.progress.as_mut() {
            let mut tmp = status.lock().unwrap();
            tmp.percentage += dp;
            if tmp.percentage > 1.0 {
                tmp.percentage = 1.0;
            }
        }
    }

    pub fn progress_set_percentage(&mut self, p: f64) {
        if let Some(status) = self.progress.as_mut() {
            let mut tmp = status.lock().unwrap();
            tmp.percentage = p;
            if tmp.percentage > 1.0 {
                tmp.percentage = 1.0;
            }
        }
    }

    pub fn stop(mut self) {
        self._stop();
    }
}

impl Drop for AnimationController {
    fn drop(&mut self) {
        self._stop();
    }
}

#[derive(Debug, Default, Clone)]
pub struct NewAnimationOptions<T: Animation> {
    frame_period_ms: u64,
    minimum_time: Option<Duration>,
    color: Option<Colors>,
    text: Option<String>,
    progress_abs: Option<bool>,
    progress: Option<Arc<Mutex<Progress>>>,
    progress_bytes: Option<bool>,
    bar_width: Option<usize>,
    _phantom_data: PhantomData<T>,
}

pub fn new<T: Animation>(frame_period_ms: u64) -> NewAnimationOptions<T> {
    NewAnimationOptions {
        frame_period_ms,
        ..Default::default()
    }
}

impl<T: Animation> NewAnimationOptions<T> {
    pub fn minimum_time(self, duration: Duration) -> Self {
        Self {
            minimum_time: Some(duration),
            ..self
        }
    }

    pub fn animate(self) -> AnimationController {
        T::animate(self)
    }
}

/******************************/
/****** BASIC ANIMATIONS ******/
/******************************/
pub trait BasicAnimation {}

impl<T: BasicAnimation + Animation> NewAnimationOptions<T> {
    pub fn text<S: AsRef<str>>(self, text: S) -> Self {
        Self {
            text: Some(String::from(text.as_ref())),
            ..self
        }
    }

    pub fn color(self, color: Colors) -> Self {
        Self {
            color: Some(color),
            ..self
        }
    }
}

pub mod basic_animations {
    use std::io::{Write, stdout};

    use super::*;

    // SPINNER
    #[derive(Debug, Default)]
    pub struct Spinner {
        spin_state: u8,
        color: Option<Colors>,
        clear_str: String,
        prepend_text: String,
    }
    impl BasicAnimation for Spinner {}
    impl Animation for Spinner {
        fn start(&mut self) {
            self.spin_state = 0;
        }

        fn stop(&mut self) {
            print!("{}\r", self.clear_str);
        }

        fn new(args: NewAnimationOptions<Self>) -> Self {
            let prepend_text = if let Some(t) = args.text {
                String::from(t)
            } else {
                String::new()
            };
            let clear_str = String::from_utf8(vec![b' '; prepend_text.len() + 1]).unwrap();

            Self {
                clear_str,
                prepend_text,
                spin_state: 0,
                color: args.color,
            }
        }

        fn next_frame(&mut self) {
            self.spin_state += 1;
            let c = match self.spin_state {
                1 => "|",
                2 => "/",
                3 => "-",
                4 => "\\",
                5 => "|",
                6 => "/",
                7 => "-",
                _ => {
                    self.spin_state = 0;
                    "\\"
                }
            };
            cprint!([self.color]; "{}{}\r", self.prepend_text, c);
            let _ = stdout().flush();
        }
    }

    // ELIPSE
    #[derive(Debug, Default)]
    pub struct Elipse {
        elipse_state: u8,
        color: Option<Colors>,
        clear_str: String,
        prepend_text: String,
    }

    impl BasicAnimation for Elipse {}
    impl Animation for Elipse {
        fn start(&mut self) {
            self.elipse_state = 0;
        }

        fn stop(&mut self) {
            print!("{}\r", self.clear_str);
        }

        fn new(args: NewAnimationOptions<Self>) -> Self {
            let prepend_text = if let Some(t) = args.text {
                String::from(t)
            } else {
                String::new()
            };
            let clear_str = String::from_utf8(vec![b' '; prepend_text.len() + 3]).unwrap();

            Self {
                clear_str,
                prepend_text,
                color: args.color,
                ..Default::default()
            }
        }

        fn next_frame(&mut self) {
            self.elipse_state += 1;
            let c = match self.elipse_state {
                1 => "   ",
                2 => ".  ",
                3 => ".. ",
                _ => {
                    self.elipse_state = 0;
                    "..."
                }
            };
            cprint!([self.color]; "{}{}\r", self.prepend_text, c);
            let _ = stdout().flush();
        }
    }
}

/******************************/
/****** TEXT ANIMATIONS  ******/
/******************************/

impl NewAnimationOptions<Pulse> {
    pub fn text<S: AsRef<str>>(self, text: S) -> Self {
        Self {
            text: Some(String::from(text.as_ref())),
            ..self
        }
    }
}

impl NewAnimationOptions<Wave> {
    pub fn text<S: AsRef<str>>(self, text: S) -> Self {
        Self {
            text: Some(String::from(text.as_ref())),
            ..self
        }
    }
}

pub mod text_animations {
    use std::io::{Write, stdout};

    use crate::{
        animations::{self, Animation},
        print_reset,
    };

    #[derive(Debug, Default)]
    pub struct Pulse {
        text: String,
        clear_str: String,
        i: u8,
        di: bool,
    }
    impl Animation for Pulse {
        fn new(args: super::NewAnimationOptions<Self>) -> Self {
            let text = if let Some(t) = args.text {
                String::from(t)
            } else {
                String::new()
            };
            let clear_str = String::from_utf8(vec![b' '; text.len() + 3]).unwrap();

            Self {
                text,
                clear_str,
                ..Default::default()
            }
        }

        fn start(&mut self) {
            self.i = 234;
        }

        fn stop(&mut self) {
            print!("{}\r", self.clear_str);
            print_reset();
        }

        fn next_frame(&mut self) {
            let idx = self.i;
            if !self.di {
                self.i += 1;
            } else {
                self.i -= 1;
            }
            if self.i == 253 {
                self.di = !self.di;
            } else if self.i == 234 {
                self.di = !self.di;
            }

            print!("\x1b[38;5;{}m", idx);
            print!("{}\r", self.text);
            print_reset();
            let _ = stdout().flush();
        }
    }

    #[derive(Debug, Default)]
    pub struct Wave {
        text: String,
        clear_str: String,
        i: usize,
        i_max: usize,
        di: bool,
    }

    impl Animation for Wave {
        fn new(args: animations::NewAnimationOptions<Self>) -> Self {
            let text = if let Some(t) = args.text {
                String::from(t)
            } else {
                String::new()
            };
            let clear_str = String::from_utf8(vec![b' '; text.len() + 3]).unwrap();

            let i_max = text.len();
            Self {
                text,
                clear_str,
                i: 0,
                i_max,
                di: false,
            }
        }

        fn start(&mut self) {}

        fn stop(&mut self) {
            print!("{}\r", self.clear_str);
            print_reset();
        }

        fn next_frame(&mut self) {
            let idx = self.i;
            if !self.di {
                self.i += 1;
            } else {
                self.i -= 1;
            }

            if self.i >= self.i_max - 1 || self.i == 0 {
                self.di = !self.di;
            }

            let mut ic: i32 = 0;
            for ch in self.text.chars() {
                let mut c: i32 = 255;
                if ic > idx as i32 {
                    c = c - 2 * (ic - idx as i32);
                } else if idx as i32 > ic {
                    c = c - 2 * (idx as i32 - ic);
                }
                if c < 242 {
                    c = 242;
                }

                print!("\x1b[38;5;{}m", c);
                print!("{}", ch);
                ic += 1;
            }
            print!("\r");

            print_reset();
            let _ = stdout().flush();
        }
    }
}

const PARTIALS: [&'static str; 8] = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];

#[derive(Debug, Clone, Copy, Default)]
pub struct Progress {
    pub percentage: f64,
    pub n: usize,
    pub max: usize,
}

#[derive(Debug, Default)]
pub struct StatusBar {
    bar_width: usize,
    progress: Arc<Mutex<Progress>>,
    abs: bool,
    bytes: bool,
}

fn print_byte_unit(i: usize) {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = i as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        print!("{}{}", i, units[unit]);
    } else {
        print!("{:.1}{}", size, units[unit]);
    }
}

fn print_progress_bytes(n: usize, max_n: usize) {
    print!("(");
    print_byte_unit(n);
    print!(" / ");
    print_byte_unit(max_n);
    print!(")");
}

impl Animation for StatusBar {
    fn new(args: NewAnimationOptions<Self>) -> Self {
        Self {
            bar_width: args.bar_width.unwrap_or(50),
            progress: args.progress.expect("Invalid status setup"),
            abs: args.progress_abs.unwrap_or_default(),
            bytes: args.progress_bytes.unwrap_or_default(),
        }
    }

    fn start(&mut self) {}

    fn stop(&mut self) {
        print!("\x1b[2K\r");
        let _ = stdout().flush();
    }

    fn next_frame(&mut self) {
        print!("\x1b[2K\r");
        let status = self.progress.lock().unwrap();
        let mut progress = if self.abs {
            (*status).n as f64 / (*status).max as f64
        } else {
            (*status).percentage
        };
        if progress > 1.0 {
            progress = 1.0;
        }

        let frac = progress * self.bar_width as f64;
        let full = frac.floor() as usize;
        let remainder = frac - full as f64;
        let remainder_idx = (remainder * 8.0).floor() as usize;
        let mut left: i32 = self.bar_width as i32 - (full + 1) as i32;
        if left < 0 {
            left = 0;
        }
        cprint!(_; " ");
        for _ in 0..full {
            cprint!(base2 on base2; "{}", PARTIALS[7]);
        }
        if remainder_idx == 0 {
            cprint!(_ on base7; " ");
        } else {
            cprint!(base2 on base7; "{}", PARTIALS[remainder_idx]);
        }
        for _ in 0..left {
            cprint!(_ on base7; " ");
        }

        if self.bytes {
            print!(" ");
            print_progress_bytes(status.n, status.max);
        }
        cprint!(_; "\t\r");
        let _ = stdout().flush();
    }
}

impl NewAnimationOptions<StatusBar> {
    pub fn absolute(self, max_n: usize) -> Self {
        Self {
            progress_abs: Some(true),
            progress: Some(Arc::new(Mutex::new(Progress {
                percentage: 0.0,
                n: 0,
                max: max_n,
            }))),
            ..self
        }
    }

    pub fn percentage(self) -> Self {
        Self {
            progress_abs: Some(false),
            progress: Some(Arc::new(Mutex::new(Progress {
                percentage: 0.0,
                n: 0,
                max: 0,
            }))),
            ..self
        }
    }

    pub fn bar_width(self, width: usize) -> Self {
        Self {
            bar_width: Some(width),
            ..self
        }
    }

    pub fn bytes(self, b: bool) -> Self {
        Self {
            progress_bytes: Some(b),
            ..self
        }
    }

}

use text_animations::*;
#[test]
fn animation_test() {
    let max_n: usize = 1024 * 1024 * 8; // 1GB
    let mut n = 0;


    let mut animation = new::<StatusBar>(50).absolute(max_n).bytes(true).animate();

    while n < max_n {
        sleep(Duration::from_nanos(1));
        n += 500;
        animation.progress_set_n(n);
    }

    animation.stop();
}
