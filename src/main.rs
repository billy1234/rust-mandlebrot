use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use std::clone::Clone;
use std::thread;
use std::sync::RwLock;
use std::sync::Arc;

use fixed::FixedI128;
use fixed::types::extra::U117;

type MReal = FixedI128<U117>;
//This type allows a max of 1024/-1024.
//Width or heigh will be the value that decdes this range

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct MandleParams {
    x : MReal,
    y : MReal,
    zoom : MReal,
    iterations : u32,
}

impl std::fmt::Display for MandleParams{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
        write!(
            fmt,"MandleParams[X: {}, Y: {}, Zoom:{}, Iterations:{}]",
            self.x,
            self.y,
            self.zoom,
            self.iterations
        )
    }
}

struct Grid<T: Clone> {
    rows : usize,
    cols : usize,
    contents: Box<[T]>,
}

impl<T : Clone> Grid<T> {

    fn new(rows: usize, cols: usize, default: T) -> Grid<T> {
        Grid::<T>{
            rows: rows,
            cols: cols,
            contents: vec![default; rows * cols]
                .into_boxed_slice(),     
        }
    }

    fn get(&mut self, x : usize, y : usize) -> &mut T{
        if x >= self.rows || y >= self.cols {
            panic!(
                "x:{} y:{} out of bounds for grid[{},{}]",
                x,
                y,
                self.rows,
                self.cols
            );
        } else {
            return &mut self.contents[y * self.rows + x];
        }
    }

    fn get_val(&self, x : usize, y : usize) -> T {
        if x >= self.rows || y >= self.cols {
            panic!(
                "x:{} y:{} out of bounds for grid[{},{}]",
                x,
                y,
                self.rows,
                self.cols
            );
        } else {
            return (self.contents[y * self.rows + x]).clone();
        }
    }
}


fn calc_mandle_divergence(
    mut a : MReal, 
    mut b : MReal, 
    max_iter : u32
) -> f64 {

    let z0_a : MReal = a;
    let z0_b : MReal = b;
    for i in 0..max_iter{
        if a.abs() + b.abs() > 4.0 {
           return i as f64 / max_iter as f64;
        }
        //square Z[I]
        let a_new : MReal = a * a - b * b;
        let b_new : MReal = MReal::from_num(2.0) * a * b;

        //Z[I] + Z[0]
        a = a_new + z0_a;
        b = b_new + z0_b;


    }
    return 0.0;
}


fn calc_mandlebrot_set(
    grid : &mut Grid<f64>,
    a : MReal, 
    b: MReal, 
    zoom_level: MReal, 
    max_iter: u32
    ){
    //A is the real part of the complex number
    //B is the coefficent to I

    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            *grid.get(x as usize,y as usize) = calc_mandle_divergence(
                a + (MReal::from_num(x) - MReal::from_num(WIDTH as f64 / 2.0)) * zoom_level,
                b + (MReal::from_num(y) - MReal::from_num(HEIGHT as f64 / 2.0)) * zoom_level,
                max_iter
            );
        }
    }
}

//map divergence value (x) to a set of r/g/b
fn map_color(x : f64) -> [u8; 3]{
    //Constant is max of u24 (3 u8s)
    let num = (x * 4294967296.0) as u32;
    let mut arr : [u8; 3] = [0; 3];
    
    arr[0] = num as u8;
    arr[1] = (num >> 8) as u8;
    arr[2] = (num >> 16) as u8;
    
    return arr;
}

fn render_mandlebrot(
    grid : & Grid<f64>,
    frame : & mut [u8]
    ){
    
    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            let col = map_color(grid.get_val(x,y)); 
            // r/g/b/a
            frame[((x + (y * WIDTH)) * 4    ) as usize] = col[0];
            frame[((x + (y * WIDTH)) * 4 + 1) as usize] = col[1];
            frame[((x + (y * WIDTH)) * 4 + 2) as usize] = col[2];
            frame[((x + (y * WIDTH)) * 4 + 3) as usize] = 0xff;
        }
    }
    
}

fn update(
    settings : &RwLock<MandleParams>,
    grid : &mut Grid<f64>,
    pixels : &mut Pixels
){
    loop {
        calc_mandlebrot_set(
            grid,
            settings.read().unwrap().x,
            settings.read().unwrap().y,
            settings.read().unwrap().zoom,
            settings.read().unwrap().iterations
        );
        render_mandlebrot(&grid, pixels.frame_mut()); 
        match pixels.render() {
            Ok(_) => {}
            Err(err) => {println!("Error {}", err); break;}
        }
    }
}

fn main() -> Result<(), Error> {
    let settings = Arc::new(RwLock::new(MandleParams{
        x: MReal::from_num(-0.20710786709396773),
        y: MReal::from_num(1.12275706363259748),
        zoom: MReal::from_num(0.01),
        iterations: 300 
    }));

    
    let mut grid: Grid<f64> 
        = Grid::new(WIDTH, HEIGHT, 0.0);

    calc_mandlebrot_set(
        &mut grid,
        settings.read().unwrap().x,
        settings.read().unwrap().y,
        settings.read().unwrap().zoom,
        settings.read().unwrap().iterations
    );

    let event_loop = EventLoop::new();

    let window = { 
        let scaled_size = LogicalSize::new(
            WIDTH as f64 * 3.0, 
            HEIGHT as f64 * 3.0
        ); 
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Mandlebrot set")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(
            window_size.width, 
            window_size.height, 
            &window
        );
        Pixels::new(
            WIDTH as u32,
            HEIGHT as u32,
            surface_texture
        )?
    };
    
    render_mandlebrot(&grid,pixels.frame_mut());
    pixels.render()?;

    window.set_maximized(true);
    
    let mut input = WinitInputHelper::new(); 
    
    thread::spawn({
        let read_settings = Arc::clone(&settings);

        move || update(
            &read_settings,
            &mut grid,
            &mut pixels
        )
    });

    
    event_loop.run(move | event, _, control_flow | {
        *control_flow = ControlFlow::Wait;


        //settings.write().unwrap().zoom = settings.read().unwrap().zoom * MReal::from_num(0.95f64);

        if let Event::RedrawRequested(_) = event {
        
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Space){
                settings.write().unwrap().zoom *= MReal::from_num(0.95f64);
            }
            if input.key_pressed(VirtualKeyCode::RAlt){
                settings.write().unwrap().zoom *= MReal::from_num(1.05f64);
            }
        }
        

    });
}

