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
#[wasm_bindgen]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
#[wasm_bindgen]
pub struct AttractorObj {
    width: u32,
    height: u32,
    data: Vec<Rgba>,
    iters: u32,
}   
#[wasm_bindgen]
impl AttractorObj {
    // ...

    pub fn new(w: u32, h: u32) -> AttractorObj {
        let width = w;
        let height = h;
        let iters: u32 = 0;
        console_log!("Creating  AttractorObj {} x {} Life Universe ", w, h);

        let data: Vec<Rgba> = (0..width * height)
            .map(|_i| {
               Rgba{r: 0,  g: 200, b: 100, a:255}
            })
            .collect();    

            AttractorObj {
            width,
            height,
            data,
            iters,
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

    pub fn data(&self) -> *const Rgba {
        // console_log!("Reference to {} x {} Life Universe ", self.width, self.height);
        self.data.as_ptr()
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

