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
    return 0.0;
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

//map divergence value (x) to a set of r/g/b
fn map_color(x : f32) -> [u8; 3]{
    //Constant is max of u24 (3 u8s)
    let num = (x * 4294967296.0) as u32;
    let mut arr : [u8; 3] = [0; 3];
    
    arr[0] = num as u8;
    arr[1] = (num >> 8) as u8;
    arr[2] = (num >> 16) as u8;
    
    return arr;
}

fn render_mandlebrot(
    arr : &[[f32; HEIGHT as usize]; WIDTH as usize],
    frame : & mut [u8]
    ){
    
    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            let col = map_color(arr[x as usize][y as usize]); 
            // r/g/b/a
            frame[((x + (y * WIDTH)) * 4    ) as usize] = col[0];
            frame[((x + (y * WIDTH)) * 4 + 1) as usize] = col[1];
            frame[((x + (y * WIDTH)) * 4 + 2) as usize] = col[2];
            frame[((x + (y * WIDTH)) * 4 + 3) as usize] = 0xff;
        }
    }
    
}


fn main() -> Result<(),Error>{

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

    event_loop.run(move | event, _, _control_flow | {
        
        if let Event::RedrawRequested(_) = event {
            //render_mandlebrot(&arr,pixels.frame_mut());
           //pixels.render();
        }
    });

}

