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
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");

        set_panic_hook();

        console_log!("Creating  AttractorObj {} x {} Attractor Object ", w, h);
        console_log!("the current time (in ms) is {}", performance.now());
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
        let mut px: u32 = (((x - self.xmin) / self.x_range) * f64::from(self.width)) as u32;
        // if ((px < 0) || (px > this.width)) console.log(" bad x " + px + " " + x);
        px = if px > self.width - 1 {
            self.width - 1
        } else {
            px
        };
        return px;
    }

    fn pixely(&self, y: f64) -> u32 {
        let mut py: u32 = (((y - self.ymin) / self.y_range) * f64::from(self.height)) as u32;
        // if ((px < 0) || (px > this.width)) console.log(" bad x " + px + " " + x);
        py = if py > self.height - 1 {
            self.height - 1
        } else {
            py
        };
        return py;
    }
    fn dec_pixel(&mut self, x: u32, y: u32) {
        let i: usize = (y * self.width + x) as usize;

        let temp = self.pixels[i];
        if temp.r == 255 {
            self.n_touched += 1;
        } else if temp.r == 1 {
            self.n_maxed += 1;
        } else if temp.r == 0 {
            return;
        }
        self.pixels[i] = Rgba {
            r: temp.r - 1,
            g: temp.g - 1,
            b: temp.b - 1,
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

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello Rust , {} ", name));
}

#[wasm_bindgen]
pub fn double(num: i32) -> i32 {
    return num + num;
}
#[wasm_bindgen]
pub fn triple(num: i32) -> i32 {
    console_log!("Rust says triple function returns {}", num + num + num);
    return num + num + num;
}

#[test]
fn test1() {
    let js_points: [Point; 10] = [
        Point(0.1, 0.1),
        Point(-0.17843260803156397, 0.7448124141417501),
        Point(-0.9793456534060968, 0.19872181239176576),
        Point(-0.3632328562224842, 1.6109032262161058),
        Point(-0.2247127104379469, 0.00605893484249198),
        Point(-0.015265172289474743, 1.5244218723046492),
        Point(-0.3681787462683442, -0.8437688656557629),
        Point(0.9925018189930358, 0.3301774005806771),
        Point(-0.5531847253681578, 0.020639799299899786),
        Point(-0.0465233064417923, 1.9804268218281909),
    ];
    let seq = Generator {
        p: Point(0.1, 0.1),
        a: -2.3983540752995394,
        b: -1.8137134453341095,
        c: 0.010788338377923257,
        d: 1.0113015602664608,
    };
    let mut index = 0;

    for i in seq.take(10) {
        println!("> {:?} {:?}", i, js_points[index]);
        //   sassert_eq!(i,js_points[index]);
        assert!((i.0 - js_points[index].0).abs() < 1.0E-10);
        assert!((i.1 - js_points[index].1).abs() < 1.0E-10);
        index += 1;
    }
    for i in 0..js_points.len() {
        println!("{:?}", js_points[i]);
    }

    println!(" Hello World ");
    println!(" {:?} ", seq);
    assert_eq!(0.0_f64.sin(), 0.0)
}
#[test]
fn test_bounds() {
    let seq = Generator {
        p: Point(0.1, 0.1),
        a: -2.3983540752995394,
        b: -1.8137134453341095,
        c: 0.010788338377923257,
        d: 1.0113015602664608,
    };

    let mut xmin = 1.0E10;
    let mut xmax = -1.0E10;
    let mut ymin = 1.0E10;
    let mut ymax = -1.0E10;

    for xy in seq.take(10000 as usize) {
        let Point(x, y) = xy;
        if x < xmin {
            xmin = x
        };
        if x > xmax {
            xmax = x
        };
        if y < ymin {
            ymin = y
        };
        if y > ymax {
            ymax = y
        };
    }
    assert!((xmin - -1.010769171380747).abs() < 1.0E-3);
    assert!((xmax - 1.010723115987805).abs() < 1.0E-3);
    assert!((ymin - -2.0112973084247994).abs() < 1.0E-3);
    assert!((ymax - 2.0110038969879342).abs() < 1.0E-3);
}
