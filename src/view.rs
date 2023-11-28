use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color; 
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::Sdl;  
use std::time::Duration;
use std::path::Path;
use sdl2::ttf::Font;

use crate::interpreteur::Interpreteur;


#[allow(dead_code)]
pub struct View{
    context: Sdl,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    interpreteur: Interpreteur,
    cursor_pos: Xy,
    case_size: Xy,
    size_window: Xy,
    background_color: Color,
    iter: u32,
    char_tab: Vec<String>,
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
        let size_window = Xy::new(1400, 800);
        let case_size = Xy::new(10, 20);
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("Rusteroids", size_window.x, size_window.y)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        
        let canvas = window.into_canvas().build().expect("could not make a canvas");
        let texture_creator = canvas.texture_creator();
        let interpreteur = Interpreteur::new();

        let mut char_vec: Vec<String> = Vec::new();
        let height = size_window.y/case_size.y;
        char_vec.push(String::from(">"));
        for _ in 1..height{
            char_vec.push(String::from(" "));
        }
        Ok(View{
            context: sdl_context,
            canvas: canvas,
            texture_creator: texture_creator,
            interpreteur: interpreteur,
            cursor_pos: Xy::new(1, 0),
            case_size: case_size,
            size_window: size_window,
            background_color: Color::RGB(0, 0, 0),
            iter: 0,
            char_tab: char_vec
        })
    }

    fn action(&mut self, font: &Font) -> Result<(), String> {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..self.char_tab.len(){
            let txt = self.char_tab[i].clone();
            self.draw_text(i as i32, &txt, &font)?;
        }
        if !(self.iter % 60 <= 10){    
            self.draw_cursor();
        }
        self.canvas.present();
        Ok(())
    }


    fn draw_cursor(&mut self){
        self.canvas.fill_rect(Rect::new((self.cursor_pos.x*self.case_size.x) as i32, (self.cursor_pos.y*self.case_size.y) as i32, self.case_size.x, self.case_size.y)).expect("Failed to draw rectangle");
    }

    fn draw_text(&mut self, y: i32, txt: &str, font: &Font) -> Result<(), String>{
        let surface = font.render(txt).blended(Color::RGBA(255, 255, 255, 0)).map_err(|e| e.to_string())?;
        let texture = self.texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        let target = Rect::new(0, y*(self.case_size.y as i32), self.case_size.x*(txt.len() as u32), self.case_size.y);
        self.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
        let font = ttf_context.load_font(Path::new(&"fonts/OpenSans-Bold.ttf"), 128)?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                        self.entry_key();
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        if self.cursor_pos.x < self.char_tab[self.cursor_pos.y as usize].len() as u32{
                            self.cursor_pos.x += 1;
                        }
                    },
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        if self.cursor_pos.x > 1{
                            self.cursor_pos.x -= 1;
                        }
                    },
                    Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                        self.delete_char();
                    },
                    Event::TextInput { text, .. } => {
                        if !text.is_empty() {
                            self.new_entry(&text);
                        }
                    }
                    _ => {} 
                }
            }
    
            self.action(&font)?;
            self.iter += 1;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    fn delete_char(&mut self){
        if self.cursor_pos.x > 1 && self.char_tab[self.cursor_pos.y as usize].len()>1{
            self.cursor_pos.x -= 1;
            self.char_tab[self.cursor_pos.y as usize].remove(self.cursor_pos.x as usize);
        }
    }

    fn new_entry(&mut self, text: &str){
        self.char_tab[self.cursor_pos.y as usize].insert_str(self.cursor_pos.x as usize, text);
        self.cursor_pos.x += 1;
    }

    fn entry_key(&mut self){
        self.char_tab[self.cursor_pos.y as usize].remove(0);
        let text = &self.char_tab[self.cursor_pos.y as usize];
        if text == "clear"{
            for i in 0..(self.cursor_pos.y+1){
                self.char_tab[i as usize] = String::from(" ");
            }
            self.char_tab[0] = String::from(">");
            self.cursor_pos.change(1, 0);
        }else{
            self.cursor_pos.change(1, self.cursor_pos.y + 1);
            self.char_tab[(self.cursor_pos.y) as usize] = String::from(">");
            if self.char_tab[(self.cursor_pos.y-1) as usize].len() == 0{
                self.char_tab[(self.cursor_pos.y-1) as usize] = String::from(" ");
            }
        }
        
    }
}
