use num::{complex::Complex, integer::Roots};
use rayon::prelude::*;

pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl Point {
    pub fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }
}

pub struct JuliaArgs {
    pub height: u64,
    pub width: u64,
    pub z0_center: Complex<f64>,
    pub c: Complex<f64>,
    pub x_span: f64,
    pub y_span: f64,
    pub n0: u64,
    pub n0_max: u64,
    pub step: u64,
    pub lower: bool,
}

pub struct CommandsArgs {
    pub change: bool,
    pub time: u64,
    pub n0_id: u8,
}

impl CommandsArgs {
    pub fn new() -> CommandsArgs {
        CommandsArgs {
            change: true,
            time: 0,
            n0_id: 0,
        }
    }
    pub fn nothing(&mut self) {
        self.change = false;
    }
    pub fn update_time(&mut self) {
        if self.change {
            self.time = 0;
        } else {
            self.time += 1;
        }
    }
    pub fn action(&mut self) {
        self.change = true;
    }
}

fn f(z: Complex<f64>, c: Complex<f64>, step: u64) -> Complex<f64> {
    let mut res = z;
    for _i in 0..step {
        res = res * res + c;
    }
    res
}

pub fn is_in(z0: Complex<f64>, c: Complex<f64>, n0: u64, step: u64) -> Option<u64> {
    let mut z = z0;
    let mut i = 0;
    while i < n0 {
        if z.norm_sqr() >= 4.0 {
            return Some(i);
        }
        z = f(z, c, step);
        i += step;
    }
    None
}

pub fn get_component(color: u32, component_index: u8) -> u8 {
    match component_index {
        0 => ((color >> 16) & 0xFF) as u8,
        1 => ((color >> 8) & 0xFF) as u8,
        2 => (color & 0xFF) as u8,
        _ => 0,
    }
}

pub fn combine_color(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn full_buffer(buffer: &mut Vec<u32>, args: &JuliaArgs) {
    // Parrallelisation
    let log_2: f64 = (2. as f64).log2();
    let x_inter = args.x_span / (args.width as f64);
    let y_inter = args.y_span / (args.height as f64);

    buffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(pixel_index, pixel_color_ref)| {
            let i = (pixel_index as u64) % args.width;
            let j = (pixel_index as u64) / args.width;

            let x_coord = (i as f64 - args.width as f64 / 2.0) * x_inter;
            let y_coord = (j as f64 - args.height as f64 / 2.0) * y_inter;

            let z = args.z0_center + Complex::new(x_coord, y_coord);

            let n_iter = is_in(z, args.c, args.n0, args.step);
            let color: u32 = if let Some(n) = n_iter {
                let c = (n as f64) / (args.n0_max as f64) * 255.;
                combine_color(c as u8, (c * 0.4) as u8, (c * 0.8) as u8)
            } else {
                let c = (args.n0 as f64) / (args.n0_max as f64) * 255.;
                combine_color(c as u8, (c * 0.4) as u8, (c * 0.8) as u8)
            };
            *pixel_color_ref = color;
        });
}

pub fn julia(args: &JuliaArgs, buffer: &mut Vec<u32>) {
    if args.lower {
        let low_width = args.width / 2;
        let low_height = args.height / 2;
        let low_args = JuliaArgs {
            z0_center: args.z0_center,
            n0_max: args.n0_max,
            width: low_width,
            height: low_height,
            c: args.c,
            x_span: args.x_span,
            y_span: args.y_span,
            n0: args.n0,
            step: args.step,
            lower: false,
        };
        let mut low_buffer: Vec<u32> = vec![0; (low_width * low_height) as usize];

        full_buffer(&mut low_buffer, &low_args);

        buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(pixel_index, pixel_color_ref)| {
                let i = (pixel_index as u64) % args.width;
                let j = (pixel_index as u64) / args.width;

                let i0 = i / 2;
                let j0 = j / 2;

                let low_index = (j0 * low_width + i0) as usize;

                *pixel_color_ref = low_buffer[low_index];
            });
    } else {
        full_buffer(buffer, args);
    }
}
