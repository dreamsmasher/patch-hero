mod note;

use std::collections::VecDeque;
use std::future::Future;

use macroquad;
use macroquad::prelude::*;
use note::Note;

#[derive(Default)]
struct GameState {
    notes: VecDeque<note::Note>,
    track_start_time: f64,
}

impl GameState {
    fn new() -> Self {
        let track_start_time = get_time();
        return Self {
            notes: Default::default(),
            track_start_time,
        }
    }
}
struct PatchHero {
    screen_width: f32,
    track_width: f32,
    track_height: f32,
    screen_height: f32,
    background_color: Color,
    track_color: Color,
    scroll_velocity: f32, /// how long a note is on screen for
    state: GameState,
}

const TRACK_VERTICAL_PADDING: f32 = 30.;
const TRACK_SPACING: f32 = 10.;
const TRACK_HORIZONTAL_PADDING: f32 = 30.;

impl PatchHero {
    pub fn new() -> Self {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let unpadded_track_width = (screen_width - (2. * TRACK_HORIZONTAL_PADDING)) / 4.;
        let track_width = unpadded_track_width.min(70.) + TRACK_HORIZONTAL_PADDING;
        let track_height = screen_height - 2. * TRACK_VERTICAL_PADDING;
        Self {
            screen_height,
            screen_width,
            track_width,
            background_color: BLACK,
            track_color: WHITE,
            track_height,
            scroll_velocity: 10.,
            state: GameState::default(),
        }
    }
    fn draw_tracks(&self) {
        for i in 0..4 {
            draw_rectangle(
                TRACK_HORIZONTAL_PADDING + (i as f32 * (self.track_width + TRACK_SPACING)),
                TRACK_VERTICAL_PADDING,
                self.track_width,
                self.track_height,
                self.track_color,
            )
        }
    }

    fn paint_notes(&mut self) {
        let rel_time = get_time() - self.state.track_start_time;
        for note in self.state.notes.iter() {
            let pos = self.scroll_velocity / (rel_time - (note.timestamp)) as f32;
            let pos = self.track_height / pos;

            for i in 0..4 {
                if note.position & (1 << i) == 0 {
                    continue;
                }

                draw_rectangle(
                    TRACK_HORIZONTAL_PADDING + (i as f32 * (self.track_width + TRACK_SPACING)),
                    TRACK_VERTICAL_PADDING + pos,
                    self.track_width,
                    20.,
                    BLUE,
                );
            }

        }
    }

    fn get_current_time(&self) -> f64 {
        get_time() - self.state.track_start_time
    }

    fn on_keypress(&mut self) {
        let rel_time = self.get_current_time();

        while let Some(note) = self.state.notes.front() {
            self.state.notes.pop_front();
            // todo check if note has passed a threshold, assign a score 
            // according to how close the keypress time was to the note's
            // expected press time
        }
    }

    pub fn add_note(&mut self, note: note::Note) {
        self.state.notes.push_back(note)
    }
    pub async fn update(&mut self) {
        clear_background(self.background_color);
        self.draw_tracks();
        self.paint_notes();
        if rand::rand() & 0b11111 == 0 {
            let shift = rand::rand() as u8 % 4;
            self.add_note(
                Note {position: 1 << shift, timestamp: get_time()}
            )
        }

        next_frame().await;
    }
}
#[macroquad::main("patch_hero")]
async fn main() {
    let mut game = PatchHero::new();

    loop {
        game.update().await;
    }
}
