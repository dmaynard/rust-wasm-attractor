mod utils;

// use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Public methods, exported to JavaScript.
extern crate js_sys;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(f64, f64);

#[wasm_bindgen]
pub struct AttractorObj {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
    iters: u32,
    n_touched: u32,
    n_maxed: u32,
    p: Point,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    x_range: f64,
    y_range: f64,
}

#[wasm_bindgen]
impl AttractorObj {
    // ...

    pub fn new(
        w: u32,
        h: u32,
        px: f64,
        py: f64,
        pa: f64,
        pb: f64,
        pc: f64,
        pd: f64,
    ) -> AttractorObj {
        let width = w;
        let height = h;
        let iters: u32 = 0;
        set_panic_hook();
        // move this to an unsafe static block to avoid allocation memory evey new
        let pixels: Vec<Rgba> = (0..width * height)
            .map(|_i| Rgba {
                r: 255,
                g: 255,
                b: 255,
                alpha: 255,
            })
            .collect();
        // console_log!(
        //     " data vector length = {}, first element = {:?}",
        //     pixels.len(),
        //     pixels[0]
        // );

        AttractorObj {
            width,
            height,
            pixels, // reference to rgbs Vec
            iters,
            n_maxed: 0,
            n_touched: 0,
            xmin: 10.0,
            xmax: -10.0,
            ymin: 10.0,
            ymax: -10.0,
            p: Point(px, py),
            a: pa,
            b: pb,
            c: pc,
            d: pd,
            x_range: 0.0,
            y_range: 0.0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn getn_maxed(&self) -> u32 {
        self.n_maxed
    }

    pub fn getn_touched(&self) -> u32 {
        self.n_touched
    }

    pub fn iters(&self) -> u32 {
        self.iters
    }

    pub fn set_iters(&mut self, n: u32) {
        self.iters = n;
    }

    pub fn pixels(&self) -> *const Rgba {
        // console_log!("Reference to {} x {} Life Universe ", self.width, self.height);
        self.pixels.as_ptr()
    }

    pub fn calculate_frame(
        &mut self,
        ms_budget: i32,
        first_frame: bool,
        n_first_frame: i32,
    ) -> i32 {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let start_time = performance.now();
        let mut ms_elapsed = 0;

        if first_frame {
            let mut x;
            let mut y;
            // calculate bounds if the attractor but don't plot anything
            for _i in 0..n_first_frame {
                x = self.p.0;
                y = self.p.1;
                if x < self.xmin {
                    self.xmin = x
                };
                if x > self.xmax {
                    self.xmax = x
                };
                if y < self.ymin {
                    self.ymin = y
                };
                if y > self.ymax {
                    self.ymax = y
                };
                self.p = self.iterate_point(x, y);
            }
            self.x_range = self.xmax - self.xmin;
            self.y_range = self.ymax - self.ymin;
            n_first_frame
        } else {
            // all successive frames()
            let mut loop_count: i32 = 0;
            let mut x;
            let mut y;
            while ms_elapsed < ms_budget {
                {
                    loop_count += 1;

                    x = self.p.0;
                    y = self.p.1;
                    self.dec_pixel(self.pixelx(x), self.pixely(y));

                    self.p = self.iterate_point(x, y);
                    if (loop_count % 1024) == 0 {
                        ms_elapsed = (performance.now() - start_time) as i32;
                    }
                }
            }

            loop_count
        }
    }

    fn pixelx(&self, x: f64) -> u32 {
        let  px: u32 = (((x - self.xmin) / self.x_range) * f64::from(self.width)) as u32;
        if px > self.width - 1 {
            self.width - 1
        } else 
            {px}
        
    }

    fn pixely(&self, y: f64) -> u32 {
        let py: u32 = (((y - self.ymin) / self.y_range) * f64::from(self.height)) as u32;
      
        if py > self.height - 1 {
            self.height - 1
        } else {
            py
        }
    }
    fn dec_pixel(&mut self, x: u32, y: u32) {
        let i: usize = (y * self.width + x) as usize;

        let prv: u8 = self.pixels[i].r;
        match prv {
            255 => self.n_touched += 1,
            1 => self.n_maxed += 1,
            0 => return,
            _ => (),
        }

        self.pixels[i] = Rgba {
            r: prv - 1,
            g: prv - 1,
            b: prv - 1,
            alpha: 255,
        };
    }
    fn iterate_point(&self, x: f64, y: f64) -> Point {
        Point(
            (y * self.b).sin() - self.c * (x * self.b).sin(),
            (x * self.a).sin() + self.d * (y * self.a).cos(),
        )
    }

    pub fn free_pixels(&mut self) -> bool {
        drop(&self.pixels);
        true
    }
}