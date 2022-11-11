//PRECODE

use macroquad::prelude::*;
const SPAWN: (f32, f32) = (500.0, 500.0);
const RIGHTRAD: f32 = 1.57079632679;

//MAIN

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::start().await;

    loop { 
        game.tick().await; 
        next_frame().await;
    }
}

//GAME

struct Game {
    state: State,
    player: Player,
    map: Vec<Object>,
    particles: Vec<Particle>,
    mouse: Option<(u64, bool)>,
    //initiate
}

impl Game {
    async fn start() -> Self { 
        Self {
            state: State::Play,
            player: Player::spawn(),
            map: vec![Object::new(500.0, 500.0)],
            particles: vec![],
            mouse: None,
        } 
    }

    async fn tick(&mut self) {
        match self.state {
            State::Play => {self.play().await;},
        }
    }

    async fn play(&mut self) {
        //touch code: 
        /*let touches = touches();
        for touch in touches {
            match touch.phase {
                TouchPhase::Started => {
                    if touch.position.x > screen_width()/2.0+220.0 
                        && touch.position.x < screen_width()/2.0+380.0 
                        && touch.position.y > screen_height()/2.0+220.0 
                        && touch.position.y < screen_height()/2.0+380.0 {
                        
                    }
                },
                _ => {}
            }
        }*/
        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            match self.mouse {
                None => {
                    if is_mouse_button_down(MouseButton::Left) {
                        if Vec2::new(mx, my).distance(Vec2::new(screen_width()-300.0, screen_height()-300.0)) < 80.0 {
                            self.mouse = Some((0, true));
                        }

                        if Vec2::new(mx, my).distance(Vec2::new(300.0, screen_height()-300.0)) < 80.0 {
                            self.mouse = Some((0, false));
                        }
                    }   
                }

                Some(_) => {
                    /*let mouse = self.mouse.unwrap();
                    
                    let c = if mouse.1 {
                        Vec2::new(screen_width()-300.0, screen_height()-300.0)
                    } else {
                        Vec2::new(300.0, screen_height()-300.0)
                    };

                    let (js, mp, d): (f32, Vec2, f32) =
                        ((((my - c.y)/(mx - c.x)).tan()-RIGHTRAD).atan(), 
                        Vec2::new((mx + c.x)/2.0, (my + c.y)/2.0), 
                        Vec2::new(mx, my).distance(c));

                    //let x = (d*d/4.0 - (y-mp.y).pow(2)).sqrt() - mp.x;


                    //println!("js: {}, ", js);

                    draw_triangle(Vec2::new(100.0, 100.0), Vec2::new(0.0, 0.0), Vec2::new(mx, my),Color::new(1.0, 1.0, 1.0, 0.8));

                    if !is_mouse_button_down(MouseButton::Left) {
                        self.mouse = None;
                        
                    }*/
                }
            }
        }

        self.draw()
    }

    fn draw(&self) {

        //backdrop


        //frontdrop

        draw_poly(self.player.cam.target.x-screen_width()/2.0+300.0, self.player.cam.target.y+screen_height()/2.0-300.0, 254, 80.0, 0.0, WHITE);
        draw_poly(self.player.cam.target.x-screen_width()/2.0+300.0, self.player.cam.target.y+screen_height()/2.0-300.0, 254, 60.0, 0.0, BLACK);
        draw_poly(self.player.cam.target.x+screen_width()/2.0-300.0, self.player.cam.target.y+screen_height()/2.0-300.0, 254, 80.0, 0.0, WHITE);
        draw_poly(self.player.cam.target.x+screen_width()/2.0-300.0, self.player.cam.target.y+screen_height()/2.0-300.0, 254, 60.0, 0.0, BLACK);
        
    }
}

//BASIC FUNCTIONS

fn solve() {}

//STRUCTURE

struct Particle {

}

struct Player {
    cam: Camera2D,
    object: usize,
}

struct Object {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

//BASIC IMPLEMENTATIONS

impl Object {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x, y, vx: 0.0, vy: 0.0
        }
    }
}

impl Player {
    fn spawn() -> Self {
        Self {
            cam: Camera2D::from_display_rect(
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: screen_width(),
                    h: screen_height()
                }
            ),
            object: 0
        }
    }
}

//ENUMS

enum State {
    Play
}

//CONFIG

fn window_conf() -> Conf {
    Conf {
        window_width: 2532,
        window_height: 1170,
        window_resizable: false,
        window_title: String::from("Macroquad Hacking"),
        ..Default::default()
    }
}