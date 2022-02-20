use ggez::audio::SoundSource;
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics::{self, Color};
use ggez::mint::Point2;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use glam::Vec2;
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::Duration;

pub mod sounds;
use crate::sounds::Sounds;

pub mod images;
use crate::images::Images;

struct Questions {
    question: String,
    answer_a: String,
    answer_b: String,
    answer_c: String,
    answer_d: String,
    correct_answer: String,
}

//choosing random index from vector of lines with questions, removing it from the vector and returning it
fn choose_question_number(vec: &mut Vec<usize>) -> usize {
    let (index, &result) = vec.iter().enumerate().choose(&mut thread_rng()).unwrap();
    vec.remove(index);

    result
}

struct MainState {
    screen_width: f32,
    screen_height: f32,
    time_x: f32,
    time_y: f32,
    mouse_down: bool,
    mouse_clicks: Vec<Vec2>,
    questions: Vec<Questions>,
    question_number: usize, // the index of the current question. Starting from 0
    is_bonus_time_used: bool,
    is_skip_question_used: bool,
    is_freeze_time_used: bool,
    is_answer_correct: bool,
    freeze: bool,
    correct_answers: usize,
    incorrect_answers: usize,
    is_game_over: bool,
    sounds: Sounds,
    images: Images,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sounds = Sounds::new(ctx)?;
        let images = Images::new(ctx)?;

        let s = MainState {
            screen_width: 1200.0,
            screen_height: 500.0,
            time_x: 0.0,
            time_y: 0.0,
            mouse_down: false,
            mouse_clicks: Vec::new(),
            questions: Vec::new(),
            question_number: 0,
            is_bonus_time_used: false,
            is_skip_question_used: false,
            is_freeze_time_used: false,
            is_answer_correct: false,
            freeze: false,
            correct_answers: 0,
            incorrect_answers: 0,
            is_game_over: false,
            sounds: sounds,
            images: images,
        };
        Ok(s)
    }

    fn click_on_answer_a(&self) -> bool {
        return self.mouse_clicks.len() > 0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x >= 300.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x <= 575.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y >= 200.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y <= 260.0;
    }

    fn click_on_answer_b(&self) -> bool {
        return self.mouse_clicks.len() > 0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x >= 625.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x <= 900.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y >= 200.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y <= 260.0;
    }

    fn click_on_answer_c(&self) -> bool {
        return self.mouse_clicks.len() > 0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x >= 300.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x <= 575.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y >= 300.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y <= 360.0;
    }

    fn click_on_answer_d(&self) -> bool {
        return self.mouse_clicks.len() > 0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x >= 625.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x <= 900.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y >= 300.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y <= 360.0;
    }

    fn click_on_lifeline_bonus_time(&self) -> bool {
        return self.mouse_clicks.len() > 0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x >= 250.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x <= 450.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y >= 400.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y <= 440.0;
    }

    fn click_on_lifeline_skip_question(&self) -> bool {
        return self.mouse_clicks.len() > 0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x >= 500.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x <= 700.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y >= 400.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y <= 440.0;
    }

    fn click_on_lifeline_freeze_time(&self) -> bool {
        return self.mouse_clicks.len() > 0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x >= 750.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].x <= 950.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y >= 400.0
            && self.mouse_clicks[self.mouse_clicks.len() - 1].y <= 440.0;
    }

    fn read_questions(&mut self) -> io::Result<()> {
        let questions_file = File::open("Resources/questions.txt")?;
        let questions_file = BufReader::new(questions_file);

        let mut questions_lines: Vec<String> = Vec::new();

        // reading the questions and answers line by line
        for line in questions_file.lines() {
            questions_lines.push(line.unwrap());
        }

        let mut only_questions = Vec::new();
        let mut current = 0;
        while current < questions_lines.len() {
            only_questions.push(current);
            current += 7;
        }

        while only_questions.is_empty() != true {
            let index = choose_question_number(&mut only_questions); // choosing random question

            self.questions.push(Questions {
                question: String::from(questions_lines[index].clone()),
                answer_a: String::from(questions_lines[index + 1].clone()),
                answer_b: String::from(questions_lines[index + 2].clone()),
                answer_c: String::from(questions_lines[index + 3].clone()),
                answer_d: String::from(questions_lines[index + 4].clone()),
                correct_answer: String::from(questions_lines[index + 5].clone()),
            });
        }
        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.freeze == false {
            self.time_x = self.time_x % self.screen_width + 5.0;
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.mouse_down = true;
        self.mouse_clicks.push(Vec2::new(x, y));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32
    ) {
        self.mouse_down = false;

        self.mouse_clicks.push(Vec2::new(x, y));
        if self.click_on_answer_a() == true
            || self.click_on_answer_b() == true
            || self.click_on_answer_c() == true
            || self.click_on_answer_d() == true
        {
            if self.freeze == true {
                self.freeze = false;
            }
            timer::sleep(Duration::new(0, 500000000)); // give time to see if the answer was correct or not
            self.question_number += 1; // go to the next question
            if self.is_answer_correct == true {
                self.correct_answers += 1;
                self.time_x -= 20.0; // if your answer is correct => bonus time
            } else {
                self.incorrect_answers += 1;
                self.time_x += 20.0; // if your answer is incorrect => you loose time
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.question_number < self.questions.len() {   // because the game stops when the questions are over

            graphics::clear(ctx, [0.71, 0.65, 0.5, 1.0].into());

            let logo = graphics::DrawParam::default().dest(Point2 { x: 950.0, y: 35.0 });
            graphics::draw(ctx, &self.images.logo, logo)?;

            let rectangle_time = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    self.screen_width * (-1.0),
                    self.screen_height - 30.0,
                    self.screen_width,
                    30.0,
                ),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &rectangle_time, (Vec2::new(self.time_x, self.time_y),))?;

            let rectangle_question = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                graphics::Rect::new(350.0, 100.0, 500.0, 60.0),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &rectangle_question, (Vec2::new(0.0, 0.0),))?;

            let rectangle_a = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                graphics::Rect::new(300.0, 200.0, 275.0, 60.0),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &rectangle_a, (Vec2::new(0.0, 0.0),))?;

            let rectangle_b = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                graphics::Rect::new(625.0, 200.0, 275.0, 60.0),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &rectangle_b, (Vec2::new(0.0, 0.0),))?;

            let rectangle_c = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                graphics::Rect::new(300.0, 300.0, 275.0, 60.0),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &rectangle_c, (Vec2::new(0.0, 0.0),))?;

            let rectangle_d = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                graphics::Rect::new(625.0, 300.0, 275.0, 60.0),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &rectangle_d, (Vec2::new(0.0, 0.0),))?;

            //lifeline: bonus time
            if self.is_bonus_time_used == false {
                let lifeline_bonus_time = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(3.0),
                    graphics::Rect::new(250.0, 400.0, 200.0, 40.0),
                    Color::WHITE,
                )?;
                graphics::draw(ctx, &lifeline_bonus_time, (Vec2::new(0.0, 0.0),))?;
            } else {
                let lifeline_bonus_time = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(3.0),
                    graphics::Rect::new(250.0, 400.0, 200.0, 40.0),
                    Color::RED,
                )?;
                graphics::draw(ctx, &lifeline_bonus_time, (Vec2::new(0.0, 0.0),))?;
            }

            //lifeline: skip question
            if self.is_skip_question_used == false {
                let lifeline_skip_question = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(3.0),
                    graphics::Rect::new(500.0, 400.0, 200.0, 40.0),
                    Color::WHITE,
                )?;
                graphics::draw(ctx, &lifeline_skip_question, (Vec2::new(0.0, 0.0),))?;
            } else {
                let lifeline_skip_question = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(3.0),
                    graphics::Rect::new(500.0, 400.0, 200.0, 40.0),
                    Color::RED,
                )?;
                graphics::draw(ctx, &lifeline_skip_question, (Vec2::new(0.0, 0.0),))?;
            }

            //lifeline: freeze time
            if self.is_freeze_time_used == false {
                let lifeline_freeze_time = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(3.0),
                    graphics::Rect::new(750.0, 400.0, 200.0, 40.0),
                    Color::WHITE,
                )?;
                graphics::draw(ctx, &lifeline_freeze_time, (Vec2::new(0.0, 0.0),))?;
            } else {
                let lifeline_freeze_time = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(3.0),
                    graphics::Rect::new(750.0, 400.0, 200.0, 40.0),
                    Color::RED,
                )?;
                graphics::draw(ctx, &lifeline_freeze_time, (Vec2::new(0.0, 0.0),))?;
            }

            // displaying the current question, answers and lifelines
            let text_pos: f32 =
                // (question rectangle length - questions length * 9) / 2
                (self.screen_width - self.questions[self.question_number].question.len() as f32 * 9.0) / 2.0; // finding where to put the text in question rectangle
            let text_question =
                graphics::Text::new(self.questions[self.question_number].question.clone());
            graphics::draw(ctx, &text_question, (Vec2::new(text_pos, 125.0),))?;

            let text_answer_a =
                graphics::Text::new(self.questions[self.question_number].answer_a.clone());
            graphics::draw(ctx, &text_answer_a, (Vec2::new(310.0, 225.0),))?;

            let text_answer_b =
                graphics::Text::new(self.questions[self.question_number].answer_b.clone());
            graphics::draw(ctx, &text_answer_b, (Vec2::new(635.0, 225.0),))?;

            let text_answer_c =
                graphics::Text::new(self.questions[self.question_number].answer_c.clone());
            graphics::draw(ctx, &text_answer_c, (Vec2::new(310.0, 325.0),))?;

            let text_answer_d =
                graphics::Text::new(self.questions[self.question_number].answer_d.clone());
            graphics::draw(ctx, &text_answer_d, (Vec2::new(635.0, 325.0),))?;

            if self.is_bonus_time_used == false {
                let text_lifeline_bonus_time = graphics::Text::new("Bonus Time");
                graphics::draw(ctx, &text_lifeline_bonus_time, (Vec2::new(305.0, 415.0),))?;
            } else {
                let text_lifeline_bonus_time = graphics::Text::new("Not active");
                graphics::draw(ctx, &text_lifeline_bonus_time, (Vec2::new(305.0, 415.0),))?;
            }

            if self.is_skip_question_used == false {
                let text_lifeline_skip_question = graphics::Text::new("Skip Question");
                graphics::draw(
                    ctx,
                    &text_lifeline_skip_question,
                    (Vec2::new(545.0, 415.0),),
                )?;
            } else {
                let text_lifeline_skip_question = graphics::Text::new("Not active");
                graphics::draw(
                    ctx,
                    &text_lifeline_skip_question,
                    (Vec2::new(555.0, 415.0),),
                )?;
            }

            if self.is_freeze_time_used == false {
                let text_lifeline_freeze_time = graphics::Text::new("Freeze Time");
                graphics::draw(ctx, &text_lifeline_freeze_time, (Vec2::new(805.0, 415.0),))?;
            } else {
                let text_lifeline_freeze_time = graphics::Text::new("Not active");
                graphics::draw(ctx, &text_lifeline_freeze_time, (Vec2::new(805.0, 415.0),))?;
            }

            // if we click on answer A
            if self.mouse_down == true && self.click_on_answer_a() == true {
                if self.questions[self.question_number].correct_answer == "A" {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::GREEN, // if the answer is correct
                    )?;
                    self.sounds.correct.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = true;
                } else {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::RED, // if the answer is incorrect
                    )?;
                    self.sounds.incorrect.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = false;
                }
            }

            // if we click on answer B
            if self.mouse_down == true && self.click_on_answer_b() == true {
                if self.questions[self.question_number].correct_answer == "B" {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::GREEN,
                    )?;
                    self.sounds.correct.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = true;
                } else {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::RED,
                    )?;
                    self.sounds.incorrect.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = false;
                }
            }

            // if we click on answer C
            if self.mouse_down == true && self.click_on_answer_c() == true {
                if self.questions[self.question_number].correct_answer == "C" {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::GREEN,
                    )?;
                    self.sounds.correct.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = true;
                } else {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::RED,
                    )?;
                    self.sounds.incorrect.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = false;
                }
            }

            // if we click on answer D
            if self.mouse_down == true && self.click_on_answer_d() == true {
                if self.questions[self.question_number].correct_answer == "D" {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::GREEN,
                    )?;
                    self.sounds.correct.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = true;
                } else {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                        Color::RED,
                    )?;
                    self.sounds.incorrect.play(ctx);
                    graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                    self.is_answer_correct = false;
                }
            }

            //if we click on lifeline bonus time
            if self.mouse_down == true
                && self.is_bonus_time_used == false
                && self.click_on_lifeline_bonus_time() == true
                && self.freeze == false
            {
                self.time_x -= 100.0;
                self.is_bonus_time_used = true;
            }

            //if we click on lifeline skip question
            if self.mouse_down == true
                && self.is_skip_question_used == false
                && self.click_on_lifeline_skip_question() == true
                && self.freeze == false
            {
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                    Color::YELLOW,
                )?;
                graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;
                self.is_skip_question_used = true;
                self.question_number += 1; // go to the next question
            }
        }

        //if we click on lifeline freeze time
        if self.mouse_down == true
            && self.is_freeze_time_used == false
            && self.click_on_lifeline_freeze_time() == true
        {
            self.is_freeze_time_used = true;
            self.freeze = true;
        }

        // time`s over
        if self.time_x >= self.screen_width
            || (self.time_x < self.screen_width && self.question_number == self.questions.len())
        {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height),
                Color::BLUE,
            )?;
            graphics::draw(ctx, &rect, (Vec2::new(0.0, 0.0),))?;

            self.is_game_over = true;
            let text_game_over = graphics::Text::new("GAME OVER!");
            let text_pos: f32 = (self.screen_width - "GAME OVER!".len() as f32 * 9.0) / 2.0;
            graphics::draw(ctx, &text_game_over, (Vec2::new(text_pos, 200.0),))?;

            let mut correct_string: String = self.correct_answers.to_string().to_owned();
            let slash : String = "/".to_owned();
            let all_string : String = (self.correct_answers + self.incorrect_answers).to_string();
            correct_string.push_str(&slash);
            correct_string.push_str(&all_string);
            let text_pos_result: f32 = (self.screen_width - correct_string.len() as f32 * 9.0) / 2.0;
            let text_correct = graphics::Text::new(String::from(correct_string));
            graphics::draw(ctx, &text_correct, (Vec2::new(text_pos_result, 270.0),))?;


            if self.incorrect_answers == 0
                && self.correct_answers != 0
                && self.correct_answers != self.questions.len()
            {
                let text_no_mistakes = graphics::Text::new(
                    "Perfect! Now try answering more questions for the same time!",);
                let text_pos: f32 = (self.screen_width - "Perfect! Now try answering more questions for the same time!".len() as f32 * 9.0) / 2.0;
                graphics::draw(ctx, &text_pos, (Vec2::new(text_pos_no_mistakes, 330.0),))?;
            } else {
                if self.correct_answers == self.questions.len() {
                    let text_all_questions_answered = graphics::Text::new(
                        "Amazing! You answered all questions correctly! Come back later and we might have new questions for you!");
                    let text_pos: f32 = (self.screen_width - "Amazing! You answered all questions correctly! Come back later and we might have new questions for you!".len() as f32 * 9.0) / 2.0;
                    graphics::draw(ctx,&text_all_questions_answered,(Vec2::new(text_pos, 330.0),), )?;
                } else {
                    if self.correct_answers > self.incorrect_answers {
                        let text_more_correct = graphics::Text::new(
                            "Good! Now try answering more questions correct!");
                        let text_pos: f32 = (self.screen_width - "Good! Now try answering more questions correct!".len() as f32 * 9.0) / 2.0;
                        graphics::draw(ctx, &text_more_correct, (Vec2::new(text_pos, 330.0),))?;
                    } else {
                        if self.correct_answers == self.incorrect_answers
                            && self.correct_answers != 0
                        {
                            let text_equal = graphics::Text::new("Half way there!");
                            let text_pos: f32 = (self.screen_width - "Half way there!".len() as f32 * 9.0) / 2.0;
                            graphics::draw(ctx, &text_equal, (Vec2::new(text_pos, 330.0),))?;
                        } else {
                            if self.correct_answers < self.incorrect_answers
                                || self.incorrect_answers + self.correct_answers == 0
                            {
                                let text_more_incorrect = graphics::Text::new("Try again! You can do better!");
                                let text_pos: f32 = (self.screen_width - "Try again! You can do better!".len() as f32 * 9.0) / 2.0;
                                graphics::draw(ctx, &text_more_incorrect, (Vec2::new(text_pos, 330.0),),)?;
                            }
                        }
                    }
                }
            }
        }

        graphics::present(ctx)?;

        if self.is_game_over == true {
            timer::sleep(Duration::new(3, 0));
            event::quit(ctx);
        }

        if self.mouse_down == true && self.click_on_lifeline_skip_question() == true {
            timer::sleep(Duration::new(0, 500000000)); // 0.5 sec
        }

        Ok(())
    }

}

fn tests(ctx: &mut Context) {
    let mut test_state = MainState::new(ctx).unwrap();

    // click_on_answer_a() returns true if we click inside rectangle_a and false otherwise
    test_state.mouse_clicks.push(Vec2::new(349.0, 257.0));
    assert!(matches!(test_state.click_on_answer_a(), true));
    test_state.mouse_clicks.push(Vec2::new(299.0, 200.0)); // x outside, y inside
    assert!(matches!(test_state.click_on_answer_a(), false));

    // click_on_answer_b() returns true if we click inside rectangle_b and false otherwise
    test_state.mouse_clicks.push(Vec2::new(626.0, 200.0));
    assert!(matches!(test_state.click_on_answer_b(), true));
    test_state.mouse_clicks.push(Vec2::new(700.0, 199.0)); // x inside, y outside
    assert!(matches!(test_state.click_on_answer_b(), false));

    // click_on_answer_c() returns true if we click inside rectangle_c and false otherwise
    test_state.mouse_clicks.push(Vec2::new(570.0, 350.0));
    assert!(matches!(test_state.click_on_answer_c(), true));
    test_state.mouse_clicks.push(Vec2::new(200.0, 100.0)); // x outside, y outside
    assert!(matches!(test_state.click_on_answer_c(), false));

    // click_on_answer_d() returns true if we click inside rectangle_d and false otherwise
    test_state.mouse_clicks.push(Vec2::new(650.0, 350.0));
    assert!(matches!(test_state.click_on_answer_d(), true));
    test_state.mouse_clicks.push(Vec2::new(500.0, 400.0)); // x outside, y outside
    assert!(matches!(test_state.click_on_answer_d(), false));

}

fn main() -> GameResult {
    let conf = Conf::new().window_mode(WindowMode {
        width: 1200.0,
        height: 500.0,
        ..Default::default()
    });

    let (mut ctx, event_loop) = ContextBuilder::new("quiz game", "Tsvetelina")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    tests(&mut ctx);

    let mut state = MainState::new(&mut ctx).unwrap();
    state.read_questions();
    event::run(ctx, event_loop, state);
}
