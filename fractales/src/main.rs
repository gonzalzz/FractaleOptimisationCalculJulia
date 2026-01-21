use minifb::{Window, WindowOptions};
use num::complex::Complex;
use std::time::Duration;
use v2_interpretation::{CommandsArgs, JuliaArgs, julia};

fn main() {
    let width: u64 = 700;
    let height: u64 = 700;
    let n0s: [u64; 4] = [151, 201, 201, 251];
    let zoom_out: [f64; 2] = [1.01, 1.05];
    let speed: [f64; 2] = [0.01, 0.05];
    let steps: [u64; 4] = [4, 3, 2, 1];

    // Creation de la matrice
    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

    // Creation des arguments
    let mut julia_params = JuliaArgs {
        height,
        width,
        z0_center: Complex::new(0., 0.),
        c: Complex::new(-0.75, 0.1),
        x_span: 2.,
        y_span: 2.,
        n0: n0s[0],
        n0_max: n0s[n0s.len() - 1],
        step: steps[0],
        lower: true,
    };

    // Creation de la strucutre de controle
    let mut pilot = CommandsArgs::new();

    // Configuration fenetre minifb
    let mut window = match Window::new(
        "Julia Set Renderer - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(e) => {
            println!("Unable to create window: {}", e);
            return;
        }
    };

    // Fps : ici 60 fps
    window.limit_update_rate(Some(Duration::from_millis(16)));

    // Exploration bord de Mandelbrot
    let mut a: f64 = 0.;
    let r = 0.04;
    let center_x = -0.745;
    let center_y = 0.11;

    // Boucle d'affichage
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Cas de modification
        let space = window.is_key_down(minifb::Key::Space);
        let left = window.is_key_down(minifb::Key::Left);
        let right = window.is_key_down(minifb::Key::Right);
        let up = window.is_key_down(minifb::Key::Up);
        let down = window.is_key_down(minifb::Key::Down);
        let shift = window.is_key_down(minifb::Key::LeftShift);
        let w = window.is_key_down(minifb::Key::W);
        let s = window.is_key_down(minifb::Key::S);
        if space || left || right || up || down || w || s {
            let scale_zoom_in = if shift {
                1. / zoom_out[1]
            } else {
                1. / zoom_out[0]
            };
            let scale_zoom_out = if shift { zoom_out[1] } else { zoom_out[0] };
            let moving_speed = if shift { speed[1] } else { speed[0] };
            if w ^ s {
                if w {
                    julia_params.x_span *= scale_zoom_in;
                    julia_params.y_span *= scale_zoom_in;
                } else {
                    julia_params.x_span *= scale_zoom_out;
                    julia_params.y_span *= scale_zoom_out;
                }
                pilot.action();
            }
            if space {
                let x = center_x + r * a.cos();
                let y = center_y + r * a.sin();
                julia_params.c = Complex::new(x, y);
                a += 0.05;
                pilot.action();
            }
            if right ^ left {
                let dz = if right {
                    Complex::new(moving_speed * julia_params.x_span, 0.)
                } else {
                    Complex::new(-moving_speed * julia_params.x_span, 0.)
                };
                julia_params.z0_center += dz;
                pilot.action();
            }
            if up ^ down {
                let dz = if down {
                    Complex::new(0., moving_speed * julia_params.y_span)
                } else {
                    Complex::new(0., -moving_speed * julia_params.y_span)
                };
                julia_params.z0_center += dz;
                pilot.action();
            }
        } else {
            pilot.nothing();
        }

        pilot.update_time();
        if pilot.time < 3 {
            pilot.change = true;
            julia_params.n0 = n0s[0];
            julia_params.step = steps[0];
            julia_params.lower = true;
        }
        if pilot.time < 9 && pilot.time >= 3 {
            pilot.change = true;
            julia_params.n0 = n0s[1];
            julia_params.step = steps[1];
            julia_params.lower = true;
        }
        if pilot.time >= 9 && pilot.time < 18 {
            pilot.change = true;
            julia_params.n0 = n0s[2];
            julia_params.step = steps[2];
            julia_params.lower = false;
        }
        if pilot.time >= 18 {
            pilot.change = true;
            julia_params.n0 = n0s[3];
            julia_params.step = steps[3];
            julia_params.lower = false;
        }

        // Actualisation
        if pilot.change {
            julia(&julia_params, &mut buffer);
            pilot.change = false;
        }

        // Gestion des erreurs d'affichage
        if let Err(e) = window.update_with_buffer(&buffer, width as usize, height as usize) {
            println!("Failed to update window: {}", e);
            break;
        }
    }
}
