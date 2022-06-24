mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
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
#[derive (Debug, Clone)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
}

#[wasm_bindgen]
#[derive (Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub struct AttractorObj {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
    data: u64,
    iters: u32,
    seq: Generator,
}   

#[derive (Debug, Clone, Copy)]
pub struct Generator {
    p: Point,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

// Implement `Iterator` for `Generator`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Generator {
    // We can refer to this type using Self::Item
    type Item = Point;
    
    // Here, we define the sequence of point by iterating the attractor
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let new_point = Point {x: self.p.x,
            y: self.p.y};

        self.p = new_point;
        

        // Since there's no endpoint to a Arrtactor sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
        Some(self.p)
    }
}
#[wasm_bindgen]
impl AttractorObj {
    // ...

    pub fn new(randomize: bool, w: u32, h: u32) -> AttractorObj {
        let width = w;
        let height = h;
        let mut iters: u32 = 0;
        let mut seq: Generator;

        console_log!("Creating  AttractorObj {} x {} Attractor Object ", w, h);
        
        let pixels: Vec<Rgba> = (0..width * height)
            .map(|_i| {
               Rgba{r:0,  g: 200, b: 100, alpha:255}
            })
            .collect();    
            console_log!(" data vector length = {}, first element = {:?}", pixels.len(), pixels[0]);
        if randomize {
            seq = Generator {p: Point {x: 0.1, y: 0.1},
            a:  3.0 * (js_sys::Math::random() * 2.0 - 1.0),
            b:  3.0 * (js_sys::Math::random() * 2.0 - 1.0),
            c:  js_sys::Math::random() * 2.0 - 1.0 + 0.5,
            d: js_sys::Math::random() * 2.0 - 1.0 + 0.5,
            }
        }
        else {
            seq = Generator {
                p: Point {x: 0.1,
                    y: 0.1},
                a: -2.3983540752995394,
                b: -1.8137134453341095,
               c: 0.010788338377923257,
               d: 1.0113015602664608};
        }
        
        AttractorObj {
            width,
            height,
            pixels, // reference to pgbs Ved
            data: 0,    // set and used in es6 to point to pixel buffer within the wasm memory
                    // new Uint8Array(this.wasmbg.memory.buffer, this.att.pixels(), this.width * this.height*4);
            iters,
            seq,

            }
        }
        
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn iters(&self) -> u32 {
        self.iters
    }

    pub fn setIters(&mut self, n :u32)  {
       self.iters = n;
    }

    pub fn pixels(&self) -> *const Rgba {
        // console_log!("Reference to {} x {} Life Universe ", self.width, self.height);
        self.pixels.as_ptr()
    }
     
}
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello Rust , {} ", name));
}

#[wasm_bindgen]
pub fn double (num: i32) -> i32 {
    return num+num;
}
#[wasm_bindgen]
pub fn triple (num: i32) -> i32 {
    console_log!("triple function returns {}", num+num+num);
    return num+num+num;
}

