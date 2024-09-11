use crate::chain::*;
use iced::{
    widget::canvas::{Frame, Path, Stroke},
    Color, Point, Vector,
};
use std::f32::consts::PI;

pub struct Snake {
    pub chain: Chain,
    pub destination: Vector,
    pub vision_angle: f32,
    pub speed: f32,
    min_speed: f32,
    max_speed: f32,
    pub color: Color,
    action: FsmAction,
    tail_size: FsmTailSize,
    tail_shake: FsmTailShake,
}

// Enum for the actions of the move automaton
#[derive(Debug)]
enum FsmAction {
    GoStraight,
    TurnLeft,
    TurnRight,
    Target,
    Look,
    LookLeft,
    LookRight,
    Orient,
    Reach,
}

// Enum for pulsating the tail, the integer represents how many frames we've been in this action
enum FsmTailSize {
    Normal(i32),
    Shrink(i32),
    Grow(i32),
}

// Enum for moving the tail, the integer represents how many frames we've been in this action
enum FsmTailShake {
    Left(i32),
    Right(i32),
}

// If the action is temporary we move to another action in the same frame
// Tipically, temporary actions have no move action
impl FsmAction {
    pub fn is_temporary(&self) -> bool {
        match self {
            Self::GoStraight => false,
            Self::TurnLeft => false,
            Self::TurnRight => false,
            Self::Target => true,
            Self::Look => true,
            Self::LookLeft => true,
            Self::LookRight => true,
            Self::Orient => true,
            Self::Reach => true,
        }
    }
}

impl Snake {
    pub fn new() -> Self {
        let mut chain = Chain::new()
            .circles_radii(vec![30.0, 48.0, 70.0, 60.5, 40.0, 30.5, 20.0, 20.0, 25.5])
            .circles_offsets(vec![0.0, 0.0, 13.0, -20.2, -10.0, -10.0, 15.0, 30.0, 0.0])
            .circles_positions(|i: usize, r: f32| {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                (
                    Some(i as f32 * r * 3.0 + 100.0),
                    Some(rng.gen_range(-300.0..300.0)),
                )
            })
            .build();
        chain.update_positions(0);
        let destination = chain.circles[0].position;
        Snake {
            chain,
            destination,
            vision_angle: PI / 6.0,
            speed: 3.0,
            min_speed: 3.0,
            max_speed: 7.0,
            color: Color::from_rgb8(168, 58, 50),
            action: FsmAction::Reach,
            tail_size: FsmTailSize::Normal(0),
            tail_shake: FsmTailShake::Left(5),
        }
    }

    // Function to transition between FSM actions
    pub fn transition(&mut self) {
        self.action = match self.action {
            FsmAction::Target => FsmAction::Look,
            FsmAction::Look => {
                if Chain::angle_2_vectors(
                    self.chain.circles[0].direction,
                    self.destination - self.chain.circles[0].position,
                ) < self.vision_angle
                {
                    FsmAction::GoStraight
                } else {
                    FsmAction::Orient
                }
            }
            FsmAction::Orient => {
                match Chain::orientation_test(
                    self.chain.circles[0].position,
                    self.chain.circles[0].position + self.chain.circles[0].direction * 100.0,
                    self.destination,
                ) {
                    Orientation::LEFT => FsmAction::TurnLeft,
                    Orientation::RIGHT | Orientation::CENTER => FsmAction::TurnRight,
                }
            }
            FsmAction::LookLeft => {
                if Chain::angle_2_vectors(
                    self.chain.circles[0].direction,
                    self.destination - self.chain.circles[0].position,
                ) > self.vision_angle / 4.0
                {
                    FsmAction::TurnLeft
                } else {
                    FsmAction::GoStraight
                }
            }
            FsmAction::LookRight => {
                if Chain::angle_2_vectors(
                    self.chain.circles[0].direction,
                    self.destination - self.chain.circles[0].position,
                ) > self.vision_angle / 4.0
                {
                    FsmAction::TurnRight
                } else {
                    FsmAction::GoStraight
                }
            }
            FsmAction::TurnLeft => FsmAction::LookLeft,
            FsmAction::TurnRight => FsmAction::LookRight,
            FsmAction::GoStraight => FsmAction::Reach,
            FsmAction::Reach => {
                if Chain::vector_length(self.chain.circles[0].position - self.destination)
                    < self.max_speed + 10.0
                {
                    FsmAction::Target
                } else {
                    FsmAction::GoStraight
                }
            }
        }
    }

    // Function to increase/decrease speed, makes sure we stay inside [min_speed:max_speed]
    pub fn modify_speed(&mut self, acceleration: f32) {
        self.speed += acceleration;
        self.speed = self.speed.max(self.min_speed);
        self.speed = self.speed.min(self.max_speed);
    }

    // Function to move the snake depending on the FSM action
    pub fn move_action(&mut self) {
        match self.action {
            FsmAction::GoStraight => {
                // Head straight for the point once the target is in the field of vision
                self.chain.circles[0].set_target(self.destination);
                self.modify_speed(0.05);
            }
            FsmAction::TurnLeft => {
                self.chain.circles[0].direction =
                    Chain::rotate_vector(self.chain.circles[0].direction, -PI / 100.0);
                self.modify_speed(-0.05);
            }
            FsmAction::TurnRight => {
                self.chain.circles[0].direction =
                    Chain::rotate_vector(self.chain.circles[0].direction, PI / 100.0);
                self.modify_speed(-0.05);
            }
            _ => {}
        }
    }

    // Function to perform an extra action depending on the FSM action
    pub fn extra_action(&mut self) {
        match self.action {
            FsmAction::Target => {
                // Set the snake's destination to a random point
                use rand::Rng;
                let mut rng = rand::thread_rng();
                self.destination =
                    Vector::new(rng.gen_range(-400.0..400.0), rng.gen_range(-300.0..300.0));
            }
            _ => {}
        }
    }

    // FSM Transition function for tail size enum
    pub fn tail_size_transition(&mut self) {
        self.tail_size = match self.tail_size {
            FsmTailSize::Normal(i) => {
                if i == 30 {
                    FsmTailSize::Grow(0)
                } else {
                    FsmTailSize::Normal(i + 1)
                }
            }
            FsmTailSize::Grow(i) => {
                if i == 15 {
                    FsmTailSize::Shrink(0)
                } else {
                    FsmTailSize::Grow(i + 1)
                }
            }
            FsmTailSize::Shrink(i) => {
                if i == 15 {
                    FsmTailSize::Normal(0)
                } else {
                    FsmTailSize::Shrink(i + 1)
                }
            }
        }
    }

    // Increasing/Decreasing the tail size
    pub fn tail_size_move(&mut self) {
        let size = self.chain.circles.len() - 1;
        match self.tail_size {
            FsmTailSize::Grow(_) => {
                self.chain.circles[size].radius += 0.4;
                self.chain.circles[size - 1].radius += 0.2;
            }
            FsmTailSize::Shrink(_) => {
                self.chain.circles[size].radius -= 0.4;
                self.chain.circles[size - 1].radius -= 0.2;
            }
            FsmTailSize::Normal(_) => {}
        }
    }

    // FSM Transition function for tail shake enum
    pub fn tail_shake_transition(&mut self) {
        self.tail_shake = match self.tail_shake {
            FsmTailShake::Left(i) => {
                if i >= 10 {
                    FsmTailShake::Right(0)
                } else {
                    FsmTailShake::Left(i + 1)
                }
            }
            FsmTailShake::Right(i) => {
                if i >= 10 {
                    FsmTailShake::Left(0)
                } else {
                    FsmTailShake::Right(i + 1)
                }
            }
        }
    }

    // Shaking the tail
    pub fn tail_shake_move(&mut self) {
        let size = self.chain.circles.len() - 1;
        match self.tail_shake {
            FsmTailShake::Right(_) => {
                self.chain.circles[size].position = self.chain.circles[size].position
                    + Chain::rotate_vector(self.chain.circles[size].direction, PI / 2.0)
                        * self.chain.circles[size].radius
                        * 0.04;
            }
            FsmTailShake::Left(_) => {
                self.chain.circles[size].position = self.chain.circles[size].position
                    + Chain::rotate_vector(self.chain.circles[size].direction, -PI / 2.0)
                        * self.chain.circles[size].radius
                        * 0.04;
            }
        }
    }

    pub fn update(&mut self) {
        loop {
            self.transition();
            self.extra_action();
            self.move_action();
            if !self.action.is_temporary() {
                break;
            }
        }
        // Move the chain in the direction it's pointing
        self.chain.circles[0].normalize_direction();
        self.chain.circles[0].position =
            self.chain.circles[0].position + self.chain.circles[0].direction * self.speed;

        self.tail_size_transition();
        self.tail_size_move();
        self.tail_shake_transition();
        self.tail_shake_move();

        self.chain.update_positions(0);
    }

    pub fn draw(&self, frame: &mut Frame) {
        // Draw the target
        frame.fill(
            &Path::circle(frame.center() + self.destination, 5.0),
            self.color,
        );

        // Draw the chain outline
        frame.stroke(
            &self.chain.path(frame.center()),
            Stroke {
                style: Color::WHITE.into(),
                width: 4.0,
                ..Default::default()
            },
        );
        // Color the chain
        frame.fill(&self.chain.path(frame.center()), self.color);

        frame.fill(&self.eyes_path(frame.center()), Color::WHITE)
    }

    // Function for drawing the snake's eyes
    pub fn eyes_path(&self, frame_center: Point) -> Path {
        Path::new(|builder| {
            builder.circle(
                frame_center
                    + self.chain.circles[0].position
                    + Chain::rotate_vector(self.chain.circles[0].direction, -PI / 4.0)
                        * self.chain.circles[0].radius
                        * 0.75,
                5.0,
            );
            builder.circle(
                frame_center
                    + self.chain.circles[0].position
                    + Chain::rotate_vector(self.chain.circles[0].direction, PI / 4.0)
                        * self.chain.circles[0].radius
                        * 0.75,
                5.0,
            );
            builder.close();
        })
    }
}
