use macroquad::prelude::*;

const SCR_H: f32 = 360f32;
const SCR_W: f32 = 640f32;
const PADDLE_WIDTH: f32 = 10f32;
const PADDLE_HEIGHT: f32 = 60f32;
const PADDLE_SPEED: f32 = 5f32;
const BALL_SIZE: f32 = 10f32;
const BALL_SPEED: f32 = 5f32;

#[derive(PartialEq)]
enum GameState {
    Start,
    Playing,
    End,
    Quit,
}

struct Context {
    state: GameState,
    player: Paddle,
    enemy: Paddle,
    ball: Ball,
    score: (u32, u32),
}
impl Context {
    pub fn new() -> Self {
        Self {
            state: GameState::Start,
            player: Paddle::new(
                Vec2::new(10.0, SCR_H / 2.0 - PADDLE_HEIGHT),
                Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            ),
            enemy: Paddle::new(
                Vec2::new(SCR_W - 10.0 - PADDLE_WIDTH, SCR_H / 2.0 - PADDLE_HEIGHT / 2.0),
                Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            ),
            ball: Ball::new(
                Vec2::new(SCR_W / 2.0 - BALL_SIZE / 2.0, SCR_H / 2.0 - BALL_SIZE / 2.0),
                Vec2::new(BALL_SIZE, BALL_SIZE),
            ),
            score: (0, 0),
        }
    }

    pub fn reset(&mut self) {
        *self = Context::new();
    }

    pub fn get_ai_move(&self) -> Vec2 {
        let ball_pos = &self.ball.pos;
        let enemy = &self.enemy;
        if ball_pos.y > enemy.pos.y + enemy.size.y {
            vec2(0.0, 1.0)
        } else if ball_pos.y + BALL_SIZE < enemy.pos.y {
            vec2(0.0, -1.0)
        } else {
            vec2(0.0, 0.0)
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Start => {
                if is_key_pressed(KeyCode::Enter) {
                    self.state = GameState::Playing
                }
            }
            GameState::Playing => {
                if is_key_down(KeyCode::W) {
                    self.player.move_by(Vec2::new(0., -PADDLE_SPEED))
                }
                if is_key_down(KeyCode::S) {
                    self.player.move_by(Vec2::new(0., PADDLE_SPEED))
                }
                if is_key_pressed(KeyCode::Q) {
                    self.state = GameState::End
                }

                self.enemy.move_by(self.get_ai_move() * PADDLE_SPEED);

                let ball_pos = self.ball.move_by(self.ball.velocity * BALL_SPEED);
                if ball_pos.x > SCR_W {
                    self.score.0 += 1;
                    self.ball.reset();
                } else if ball_pos.x < 0.0 {
                    self.score.1 += 1;
                    self.ball.reset();
                } else if self.player.is_colliding(ball_pos) || self.enemy.is_colliding(ball_pos) {
                    self.ball.velocity.x *= -1.0;
                    self.ball.pos.x = clamp(
                        self.ball.pos.x,
                        self.player.pos.x + PADDLE_WIDTH,
                        self.enemy.pos.x,
                    );
                    let bounced_center = if (self.ball.pos.x - self.player.pos.x).abs()
                        < (self.ball.pos.x - self.enemy.pos.x).abs()
                    {
                        self.player.pos.y + PADDLE_HEIGHT / 2.0
                    } else {
                        self.enemy.pos.y + PADDLE_HEIGHT / 2.0
                    };
                    if self.ball.pos.y + BALL_SIZE / 2.0 < bounced_center {
                        self.ball.velocity.y = -1.0;
                    } else {
                        self.ball.velocity.y = 1.0;
                    }
                }
            }
            GameState::End => {
                if is_key_pressed(KeyCode::Enter) {
                    self.reset();
                } else if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Quit
                }
            }
            GameState::Quit => {}
        }
    }
    pub fn draw(&self) {
        match self.state {
            GameState::Start => {
                let pong_label = "PONG";
                let press_start = "Press Enter to play";
                let pong_size = measure_text(pong_label, None, 16u16, 1.0);
                let start_size = measure_text(press_start, None, 16u16, 1.0);
                draw_text(
                    pong_label,
                    SCR_W / 2.0 - pong_size.width / 2.0,
                    SCR_H / 2.0 - pong_size.height / 2.0,
                    16.0,
                    WHITE,
                );
                draw_text(
                    press_start,
                    SCR_W / 2.0 - start_size.width / 2.0,
                    (SCR_H / 2.0 - start_size.height / 2.0) + 18.0,
                    16.0,
                    WHITE,
                );
            }
            GameState::Playing => {
                let score = format!("{} - {}", self.score.0, self.score.1);
                let score_size = measure_text(score.as_str(), None, 16, 1.0);
                draw_text(
                    score.as_str(),
                    SCR_W / 2.0 - score_size.width / 2.0,
                    16.0,
                    16.0,
                    WHITE,
                );
                draw_rectangle(
                    self.player.pos.x,
                    self.player.pos.y,
                    self.player.size.x,
                    self.player.size.y,
                    WHITE,
                );
                draw_rectangle(
                    self.enemy.pos.x,
                    self.enemy.pos.y,
                    self.enemy.size.x,
                    self.enemy.size.y,
                    WHITE,
                );
                draw_rectangle(
                    self.ball.pos.x,
                    self.ball.pos.y,
                    self.ball.size.x,
                    self.ball.size.y,
                    WHITE,
                );
            }
            GameState::End => {
                let score = format!("Final score: {} - {}", self.score.0, self.score.1);
                let score_size = measure_text(score.as_str(), None, 16, 1.0);
                let press_message = "Press Enter to restart the game or Esc to exit";
                let press_message_size = measure_text(press_message, None, 12, 1.0);
                let main_text_y = SCR_H / 2.0 - score_size.height / 2.0;
                draw_text(
                    score.as_str(),
                    SCR_W / 2.0 - score_size.width / 2.0,
                    main_text_y,
                    16.0,
                    WHITE,
                );
                draw_text(
                    press_message,
                    SCR_W / 2.0 - press_message_size.width / 2.0,
                    main_text_y + 16.0,
                    12.0,
                    WHITE,
                );
            }
            GameState::Quit => {}
        }
    }
}
struct Paddle {
    pos: Vec2,
    size: Vec2,
}

impl Paddle {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }
    pub fn move_by(&mut self, mov: Vec2) {
        self.pos += mov;
        self.pos.y = clamp(self.pos.y, 0.0, SCR_H - self.size.y);
    }
    pub fn is_colliding(&self, ball_pos: &Vec2) -> bool {
        self.pos.x < ball_pos.x + BALL_SIZE
            && self.pos.x + self.size.x > ball_pos.x
            && self.pos.y < ball_pos.y + BALL_SIZE
            && self.pos.y + self.size.y > ball_pos.y
    }
}

struct Ball {
    pos: Vec2,
    size: Vec2,
    velocity: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            pos,
            size,
            velocity: Vec2::new(-1.0, 1.0),
        }
    }
    pub fn move_by(&mut self, mov: Vec2) -> &Vec2 {
        self.pos += mov;
        self.pos.y = clamp(self.pos.y, 0.0, SCR_H - self.size.y);

        if (self.pos.y - (SCR_H - self.size.y).abs()).abs() < f32::EPSILON || self.pos.y == 0f32 {
            //screen bounce
            self.velocity.y *= -1.0
        }
        &self.pos
    }
    pub fn reset(&mut self) {
        *self = Ball::new(
            Vec2::new(SCR_W / 2.0 - BALL_SIZE / 2.0, SCR_H / 2.0 - BALL_SIZE / 2.0),
            Vec2::new(BALL_SIZE, BALL_SIZE),
        )
    }
}
#[macroquad::main("PONG")]
async fn main() {
    let mut ctx = Context::new();

    set_camera(&Camera2D {
        zoom: vec2(1. / SCR_W * 2., -1. / SCR_H * 2.),
        target: vec2(SCR_W / 2., SCR_H / 2.),
        ..Default::default()
    });

    loop {
        if ctx.state == GameState::Quit {
            return;
        }
        ctx.update();
        clear_background(BLACK);
        ctx.draw();
        next_frame().await;
    }
}
