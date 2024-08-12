use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{TextureCreator, TextureQuery, WindowCanvas},
    ttf::{Font, Sdl2TtfContext},
    video::WindowContext,
    EventPump, Sdl,
};

pub struct SDLHandler<'ttf> {
    sdl: Sdl,
    canvas: WindowCanvas,
    font: Font<'ttf, 'static>,
    tex_creator: TextureCreator<WindowContext>,
}

impl<'ttf> SDLHandler<'ttf> {
    pub fn new(size: (u32, u32), ttf: &'ttf Sdl2TtfContext) -> Self {
        let sdl = sdl2::init().unwrap();

        let canvas = {
            let video_subsys = sdl.video().unwrap();

            let window = video_subsys
                .window("Window", size.0, size.1)
                .position_centered()
                .build()
                .unwrap();

            window.into_canvas().accelerated().build().unwrap()
        };

        let font = ttf.load_font("clacon2.ttf", 16).unwrap();

        let tex_creator = canvas.texture_creator();

        Self {
            sdl,
            canvas,
            font,
            tex_creator,
        }
    }

    pub fn event_pump(&self) -> EventPump {
        self.sdl.event_pump().unwrap()
    }

    pub fn rect<R, C>(&mut self, rect: R, color: C)
    where
        R: Into<Option<Rect>>,
        C: Into<Color>,
    {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn text<S, P, C>(&mut self, text: S, pos: P, color: C)
    where
        S: AsRef<str>,
        P: Into<Point>,
        C: Into<Color>,
    {
        let surface = self.font.render(text.as_ref()).solid(color.into()).unwrap();

        let texture = self
            .tex_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let target = {
            let TextureQuery { width, height, .. } = texture.query();
            let pos = pos.into();
            Rect::new(pos.x, pos.y, width, height)
        };

        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }
}
