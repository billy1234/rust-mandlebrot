use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::EventLoop,
    window::WindowBuilder,
};


const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

fn calc_mandle_divergence(mut a : f32, mut b : f32, max_iter : u32) -> f32{
    let z0_a : f32 = a;
    let z0_b : f32 = b;
    for i in 0..max_iter{
        if a.abs() + b.abs() > 2.0 {
            return i as f32 / max_iter as f32;
        }
        //square Z[I]
        let a_new : f32 = a * a - b * b;
        let b_new : f32 = 2.0 * a * b;

        a = a_new;
        b = b_new;

        //Z[I] + Z[0]
        a = a + z0_a;
        b = b + z0_b;

    }
    return 1.0;
}

fn calc_mandlebrot_set(a : f32, b: f32, zoom_level: f32, max_iter: u32) 
        -> [[f32; HEIGHT as usize]; WIDTH as usize]{
    //A is the real part of the complex number
    //B is the coefficent to I

    let mut arr: [[f32; HEIGHT as usize]; WIDTH as usize] 
        = [[0.0; HEIGHT as usize]; WIDTH as usize];

    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            arr[x as usize][y as usize] = calc_mandle_divergence(
                a + (x as f32 - WIDTH as f32 /2.0 ) * zoom_level,
                b + (y as f32 - HEIGHT as f32 /2.0) * zoom_level,
                max_iter
            );
        }
    }

    return arr;
}

fn render_mandlebrot(
    arr : &[[f32; HEIGHT as usize]; WIDTH as usize],
    frame : & mut [u8]
    ){

    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            frame[((x + (y * WIDTH)) * 4) as usize] 
                = (arr[x as usize][y as usize] 
                    * (u8::MAX as f32)) as u8;
            

            frame[((x + (y * WIDTH)) * 4 + 3) as usize] = 0xff;
        }
    }
    
}


fn main() -> Result<(),Error>{
    println!("Hello world.");
    
    let arr = calc_mandlebrot_set(0.0,0.0,0.01,255);
    
    let event_loop = EventLoop::new();

    let window = { 
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0); 
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
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    
    render_mandlebrot(&arr,pixels.frame_mut());
    pixels.render()?;

    event_loop.run(move | event, _, control_flow | {
        
        if let Event::RedrawRequested(_) = event {
            println!("event missing")
        }
    });

}


