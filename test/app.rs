
use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;
use std::default::Default;
use nui::{
    Button,
    Slider,
    Point,
    Widget,
    Specific,
    Color,
};
use piston::{
    Game,
    RenderArgs,
    UpdateArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseMoveArgs,
};
use graphics::{
    Context,
    AddColor,
    Draw,
};

pub struct App {
    button: Button,
    slider: Slider<int>,
    gl: Gl,       // OpenGL drawing backend.
}

impl App {

    pub fn new() -> App {
        let mut button: Button = Default::default();
        button.set_abs_pos(Point::new(50i, 50, 0));
        let mut slider = Slider::new(Specific(Point::new(50i, 150, 0)), 40u, 200u,
                                     Color::new(0.5f32, 0.6f32, 0.8f32, 1f32),
                                     3u, 0, 128, 80);
        slider.set_abs_pos(Point::new(50i, 150, 0));
        App {
            button: button,
            slider: slider,
            gl: Gl::new(),
        }
    }

    fn draw_background(&mut self, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Draw the background.
        context.rgba(0.075,0.05,0.1,1.0).draw(&mut self.gl);
    }

}

impl Game<GameWindowSDL2> for App {

    fn render(&mut self, _w: &mut GameWindowSDL2, args: &RenderArgs) {
        self.draw_background(args);
        self.button.draw(args, &mut self.gl);
        self.slider.draw(args, &mut self.gl);
    }

    fn mouse_press(&mut self, _w: &mut GameWindowSDL2, args: &MousePressArgs) {
        self.button.mouse_press(args);
        self.slider.mouse_press(args);
    }

    fn mouse_release(&mut self, _w: &mut GameWindowSDL2, args: &MouseReleaseArgs) {
        self.button.mouse_release(args);
        self.slider.mouse_release(args);
    }

    fn mouse_move(&mut self, _w: &mut GameWindowSDL2, args: &MouseMoveArgs) {
        self.button.mouse_move(args);
        self.slider.mouse_move(args);
    }

    fn update(&mut self, _w: &mut GameWindowSDL2, _args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }

}

