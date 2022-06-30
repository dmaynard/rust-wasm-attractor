mod utils;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use wasm_bindgen::prelude::*;

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

const WHITE: Rgba = Rgba {
    r: 0xff,
    g: 0xff,
    b: 0xff,
    alpha: 0xff,
};
const BLACK: Rgba = Rgba {
    r: 0x00,
    g: 0x00,
    b: 0x00,
    alpha: 0xff,
};
const ALMOST_BLACK: Rgba = Rgba {
    r: 0x01,
    g: 0x01,
    b: 0x01,
    alpha: 0xff,
};
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(f64, f64);

#[wasm_bindgen]
pub struct AttractorObj {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
    data: u64,
    iters: u32,
    nTouched: u32,
    nMaxed: u32,
    p: Point,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    xRange: f64,
    yRange: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Generator {
    p: Point,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

#[wasm_bindgen]
impl AttractorObj {
    // ...

    pub fn new(randomize: bool, w: u32, h: u32) -> AttractorObj {
        let width = w;
        let height = h;
        let iters: u32 = 0;
        let seq: Generator;
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");

        console_log!("Creating  AttractorObj {} x {} Attractor Object ", w, h);
        console_log!("the current time (in ms) is {}", performance.now());
        let x: f64;
        let y: f64;
        let a: f64;
        let b: f64;
        let c: f64;
        let d: f64;
        // move this to an unsafe static block to avoid allocation memory evey new
        let pixels: Vec<Rgba> = (0..width * height)
            .map(|_i| Rgba {
                r: 255,
                g: 255,
                b: 255,
                alpha: 255,
            })
            .collect();
        console_log!(
            " data vector length = {}, first element = {:?}",
            pixels.len(),
            pixels[0]
        );
        x = 0.1;
        y = 0.1;
        if randomize {
            a = 3.0 * (js_sys::Math::random() * 2.0 - 1.0);
            b = 3.0 * (js_sys::Math::random() * 2.0 - 1.0);
            c = js_sys::Math::random() * 2.0 - 1.0 + 0.5;
            d = js_sys::Math::random() * 2.0 - 1.0 + 0.5;
        } else {
            a = -2.3983540752995394;
            b = -1.8137134453341095;
            c = 0.010788338377923257;
            d = 1.0113015602664608;
        }

        AttractorObj {
            width,
            height,
            pixels,  // reference to rgbs Vec
            data: 0, // set and used in es6 to point to pixel buffer within the wasm memory
            // new Uint8Array(this.wasmbg.memory.buffer, this.att.pixels(), this.width * this.height*4);
            iters,
            nMaxed: 0,
            nTouched: 0,
            xmin: 10.0,
            xmax: -10.0,
            ymin: 10.0,
            ymax: -10.0,
            p: Point(x, y),
            a: a,
            b: b,
            c: c,
            d: d,
            xRange: 0.0,
            yRange: 0.0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn getnMaxed(&self) -> u32 {
        self.nMaxed
    }

    pub fn getnTouched(&self) -> u32 {
        self.nTouched
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

    pub fn calculateFrame(&mut self, ms_budget: i32, first_frame: bool, n_first_frame: i32) -> i32 {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let start_time = performance.now();
        let mut msElapsed = 0;
        let mut loop_count: i32 = 0;
        if first_frame {
            let mut x = self.p.0;
            let mut y = self.p.1;

            // calculate bounds if the attractor but don't plot anything
            for i in 0..n_first_frame {
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
                if i < 10 {
                    console_log!(" i {:?}, x: {} y: {}", i, x, y);
                }
                self.p = self.iteratePoint(x, y);
            }
            self.xRange = self.xmax - self.xmin;
            self.yRange = self.ymax - self.ymin;

            console_log!(
                "xmin {}, xmax {} ymin {} ymax {}",
                self.xmin,
                self.xmax,
                self.ymin,
                self.ymax
            );
            n_first_frame
        } else {
            // all successive frames()
            let mut x = self.p.0;
            let mut y = self.p.0;
            loop_count = 0;
            while (msElapsed < ms_budget) {
                {
                    loop_count += 1;

                    x = self.p.0;
                    y = self.p.1;
                    self.dec_pixel(self.pixelx(x), self.pixely(y));

                    self.p = self.iteratePoint(x, y);
                    if (loop_count % 1024) == 0 {
                        msElapsed = (performance.now() - start_time) as i32;
                    }
                }
            }
            console_log!(
                " loop_count {} nTouched {}, nMaxed {}",
                loop_count,
                self.nTouched,
                self.nMaxed
            );
            loop_count
        }
    }

    fn pixelx(&self, x: f64) -> u32 {
        let mut px: u32 = (((x - self.xmin) / self.xRange) * f64::from(self.width)) as u32;
        // if ((px < 0) || (px > this.width)) console.log(" bad x " + px + " " + x);
        px = if px > self.width - 1 {
            self.width - 1
        } else {
            px
        };
        return px;
    }

    fn pixely(&self, y: f64) -> u32 {
        let mut py: u32 = (((y - self.ymin) / self.yRange) * f64::from(self.height)) as u32;
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

        let mut temp = self.pixels[i];
        if temp.r == 255 {
            self.nTouched += 1;
        } else if temp.r == 1 {
            self.nMaxed += 1;
        } else if (temp.r == 0) {
            return;
        }
        self.pixels[i] = Rgba {
            r: temp.r - 1,
            g: temp.g - 1,
            b: temp.b - 1,
            alpha: 255,
        };
    }
    fn iteratePoint(&self, x: f64, y: f64) -> Point {
        Point(
            (y * self.b).sin() - self.c * (x * self.b).sin(),
            (x * self.a).sin() + self.d * (y * self.a).cos(),
        )
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
    console_log!("triple function returns {}", num + num + num);
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
