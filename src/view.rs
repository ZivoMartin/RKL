use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color; 
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::Sdl;  
use sdl2::EventPump;
use std::time::Duration;
use std::path::Path;
use sdl2::ttf::Font;
use crate::interpreteur::Interpreteur;


#[allow(dead_code)]
pub struct View{
    context: Sdl,
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    interpreteur: Interpreteur,
    cursor_pos: Xy,
    case_size: Xy
}

struct Xy{
    x: u32,
    y: u32
}

impl Xy {

    fn new(x: u32, y: u32) -> Xy{
        Xy{x: x, y: y}
    }
    
    fn change(&mut self, x: u32, y: u32){
        self.x = x;
        self.y = y;
    }
}


impl View{

    pub fn new() -> Result<View, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
    
        let window = video_subsystem.window("Rusteroids", 800, 600)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        
        let canvas = window.into_canvas().build().expect("could not make a canvas");
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump()?;

        let mut interpreteur = Interpreteur::new();

        

        interpreteur.sqlrequest(String::from("DROP TABLE Humain;"))?;
        interpreteur.sqlrequest(String::from("CREATE TABLE Humain(id INT PRIMARY KEY, name VARCHAR(50), age INT, vivant BOOL DEFAULT true);"))?;
        interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (1, 'Joah', 20);"))?;
        interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (2, 'Martin', 19);"))?;
        interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (3, 'Raghid', 17);"))?;
        interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (4, 'Dabi', 18);"))?;
        interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (5, 'Vico', 18);"))?;
        let _res = interpreteur.sqlrequest(String::from("SELECT age, name FROM Humain WHERE age>18;"))?;
        interpreteur.sqlrequest(String::from("DELETE FROM Humain WHERE (2!=5 AND 8>7) AND age<19;"))?;

        Ok(View{
            context: sdl_context,
            canvas: canvas,
            event_pump: event_pump,
            texture_creator: texture_creator,
            interpreteur: interpreteur,
            cursor_pos: Xy::new(30, 30),
            case_size: Xy::new(10, 20)
        })
    }

    fn action(&mut self, font: &Font) -> Result<(), String> {
        let color = Color::RGB(0, 0, 0);
        self.canvas.set_draw_color(color);
        self.canvas.clear();        
        self.draw_cursor();
        self.canvas.present();
        Ok(())
    }


    fn draw_cursor(&mut self){
        self.canvas.fill_rect(Rect::new(self.cursor_pos.x as i32, self.cursor_pos.y as i32, self.case_size.x, self.case_size.y)).expect("Failed to draw rectangle");
    }

    fn draw_text(&mut self, pos: Xy, txt: &str, font: &Font) -> Result<(), String>{
        let surface = font.render(txt).blended(Color::RGBA(255, 0, 0, 0)).map_err(|e| e.to_string())?;
        let texture = self.texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        let target = Rect::new(pos.x as i32, pos.y as i32, self.case_size.x, self.case_size.y);
        self.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
        let font = ttf_context.load_font(Path::new(&"fonts/OpenSans-Bold.ttf"), 128)?;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => {} 
                }
            }
    
            self.action(&font)?;
    
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }
}
