
extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;







fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;


    let window = video_subsystem.window("Commie Pong: Rust Edition", 1010, 622)
        .position_centered().build().map_err(|e| e.to_string())?;


    let mut canvas = window.into_canvas()
        .accelerated().build().map_err(|e| e.to_string())?;
    
    
    let texture_creator = canvas.texture_creator();


    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));




    let mut event_pump = sdl_context.event_pump()?;



    let background_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/background.bmp"))?;
    let background_texture = texture_creator.create_texture_from_surface(&background_surface)
        .map_err(|e| e.to_string())?;




    let noah_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/noah.bmp"))?;
    let noah_texture = texture_creator.create_texture_from_surface(&noah_surface)
        .map_err(|e| e.to_string())?;

    let orion_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/orion.bmp"))?;
    let orion_texture = texture_creator.create_texture_from_surface(&orion_surface)
        .map_err(|e| e.to_string())?;


    let ball_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/ball.bmp"))?;
    let ball_texture = texture_creator.create_texture_from_surface(&ball_surface)
        .map_err(|e| e.to_string())?;








    let paddle_image_size = (100,100);


    let source_background_image = Rect::new(0, 0, 1010, 622);
    let mut dest_background_image = Rect::new(0, 0, 1010, 622);
    dest_background_image.center_on(Point::new(505, 311));




    let source_noah_paddle = Rect::new(0, 0, paddle_image_size.0, paddle_image_size.0);
    let mut dest_noah_paddle = Rect::new(0, 0, paddle_image_size.0, paddle_image_size.0);
    dest_noah_paddle.center_on(Point::new(100, 311));


    let source_orion_paddle = Rect::new(0, 0, paddle_image_size.0, paddle_image_size.0);
    let mut dest_orion_paddle = Rect::new(0, 0, paddle_image_size.0, paddle_image_size.0);
    dest_orion_paddle.center_on(Point::new(900, 311));


    let source_ball_image = Rect::new(0, 0, 40, 40);
    let mut dest_ball_image = Rect::new(0, 0, 40, 40);
    dest_ball_image.center_on(Point::new(395, 290));



    let mut w_key = false;
    let mut s_key = false;
    let mut i_key = false;
    let mut k_key = false;

    let mut left = 0;
    let mut right = 0;

    let mut paddle_a = 311;
    let mut paddle_b = 311;

    let mut ball_x = 395;
    let mut ball_y = 290;

    let mut move_x = 2;
    let mut move_y = 2;








    let mut running = true;

    while running {



        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },

                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    w_key = true;
                },

                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    s_key = true;
                },
                
                Event::KeyDown {keycode: Some(Keycode::I), ..} => {
                    i_key = true;
                },

                Event::KeyDown {keycode: Some(Keycode::K), ..} => {
                    k_key = true;
                },

                Event::KeyUp {keycode: Some(Keycode::W), ..} => {
                    w_key = false;
                },

                Event::KeyUp {keycode: Some(Keycode::S), ..} => {
                    s_key = false;
                },
                
                Event::KeyUp {keycode: Some(Keycode::I), ..} => {
                    i_key = false;
                },

                Event::KeyUp {keycode: Some(Keycode::K), ..} => {
                    k_key = false;
                },

                _ => {}
            }
        }


        let paddlespeed = 5;

        if w_key == true{
            if paddle_a - 4 >= 50 {
            paddle_a = paddle_a - paddlespeed;
            }
        }

        if s_key == true{

            if paddle_a + 4 <= 572 {
            paddle_a = paddle_a + paddlespeed;
            }

        }

        if i_key == true{
            if paddle_b - 4 >= 50 {

            paddle_b = paddle_b - paddlespeed;
            }
        }

        if k_key == true{
            if paddle_b + 4 <= 572 {

            paddle_b = paddle_b + paddlespeed;
            }
        }


        //Move Ball

        ball_x = ball_x + move_x;
        ball_y = ball_y + move_y;


        //Ball Upper Wall Collision

        if ball_y <= 20 {
            move_y = move_y * -1;

        }
        
        if ball_y >= 602 {
            move_y = move_y*-1;

        }


        //Ball Paddle Collision


        if ball_x >= 50 && ball_x <= 150 && ball_y >= paddle_a-50 && ball_y <= paddle_a+50 {
            move_x = move_x*-1;


        }
        

        if ball_x >= 850 && ball_x <= 950 && ball_y >= paddle_b-50 && ball_y <= paddle_b+50 {
            move_x = move_x*-1;


        }


        //Ball Scoring

        if ball_x > 1010 {

            ball_x = 505;
            ball_y = 300;


            right = right + 1;


        }
        
        if ball_x < 0 {

            ball_x = 505;
            ball_y = 300;

            left = left+1;


        }


        //Update positions of paddles

        dest_orion_paddle.center_on(Point::new(900, paddle_b));
        dest_noah_paddle.center_on(Point::new(100, paddle_a));
        dest_ball_image.center_on(Point::new(ball_x, ball_y));


        //Clear

        canvas.clear();

        
        // Copy the frames to the canvas

        canvas.copy_ex(&background_texture, Some(source_background_image), Some(dest_background_image), 0.0, None, false, false)?;

        canvas.copy_ex(&noah_texture, Some(source_noah_paddle), Some(dest_noah_paddle), 0.0, None, false, false)?;

        canvas.copy_ex(&orion_texture, Some(source_orion_paddle), Some(dest_orion_paddle), 0.0, None, false, false)?;

        canvas.copy_ex(&ball_texture, Some(source_ball_image), Some(dest_ball_image), 0.0, None, false, false)?;



        canvas.present();

        std::thread::sleep(Duration::from_millis(5));
    }



    Ok(())
}




