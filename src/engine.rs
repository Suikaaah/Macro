use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{TextureCreator, TextureQuery, WindowCanvas},
    ttf::{Font, Sdl2TtfContext},
    video::WindowContext,
    Sdl,
};

pub struct Engine<'ttf> {
    pub sdl: Sdl,
    canvas: WindowCanvas,
    font: Font<'ttf, 'static>,
    tex_creator: TextureCreator<WindowContext>,
}

impl<'ttf> Engine<'ttf> {
    pub fn new(size: (u32, u32), ttf: &'ttf Sdl2TtfContext) -> Result<Self, String> {
        let sdl = sdl2::init()?;

        let canvas = {
            let video_subsys = sdl.video()?;

            let window = video_subsys
                .window("Window", size.0, size.1)
                .position_centered()
                .build()
                .map_err(|e| e.to_string())?;

            window
                .into_canvas()
                .accelerated()
                .build()
                .map_err(|e| e.to_string())?
        };

        let font = ttf
            .load_font("clacon2.ttf", 16)
            .map_err(|e| e.to_string())?;

        let tex_creator = canvas.texture_creator();

        Ok(Self {
            sdl,
            canvas,
            font,
            tex_creator,
        })
    }

    pub fn text<S, P>(&mut self, text: S, pos: P) -> Result<(), String>
    where
        S: AsRef<str>,
        P: Into<Point>,
    {
        let surface = self
            .font
            .render(text.as_ref())
            .solid(Color::RGB(0xFF, 0xFF, 0xFF))
            .map_err(|e| e.to_string())?;

        let texture = self
            .tex_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = {
            let TextureQuery { width, height, .. } = texture.query();
            let pos = pos.into();
            Rect::new(pos.x, pos.y, width, height)
        };

        self.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }
}
