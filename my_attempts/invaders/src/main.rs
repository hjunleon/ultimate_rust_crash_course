use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use invaders::frame::{new_frame, Drawable};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::{frame, render};
use rusty_audio::Audio;
use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> { // need Box to allocate to heap, because Error's size cant be predicted before compile time
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; // question mark on Result<ok,err> will just crash on error
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    // Render loop in thread

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game loop

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        //cur frame init
        let mut curr_frame = new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    _ => {}
                }
            }
        }

        player.update(delta);
        if invaders.update(delta){
            audio.play("move")
        }
        if player.detect_hits(&mut invaders){
            audio.play("explode")
        }

        // Draw & Render
        // player.draw(&mut curr_frame);
        // invaders.draw(&mut curr_frame);

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _  = render_tx.send(curr_frame); // ignore whatever errors may come if thread starts later and cant push to channel
        thread::sleep(Duration::from_millis(1));

        // Win or lose

        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop
        } 
        if invaders.reached_bottom(){
            audio.play("lose");
            break 'gameloop
        }
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}