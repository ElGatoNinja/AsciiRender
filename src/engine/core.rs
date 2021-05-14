
use std::time;
use std::thread;

pub fn gameloop() {
    const TICKS: f64 = 128f64;
    const TIME_PER_TICK: f64 = 1.0/TICKS;


    println!("{}",TIME_PER_TICK);

    let mut previous_time = time::Instant::now();
    let mut lag: f64 = 0f64;

    loop {
        //calculate the elapsed time between frames to keep the game deterministic in any arquitecture
        let current_time = time::Instant::now();
        let frame_time = current_time.duration_since(previous_time).as_secs_f64();
        previous_time = current_time;
        lag += frame_time;

        processInput();

        let mut n = 0;

        while lag >= TIME_PER_TICK {
            update(TIME_PER_TICK);
            lag -= TIME_PER_TICK;
            n += 1;
        }
        println!("{}",n);
        //render times a pretty much random, to avoid sttutering the positions of elements while rendering are being infered
        render(lag as f64 / TIME_PER_TICK);
    }
}

fn render(to_update_ratio: f64) { 
    thread::sleep(time::Duration::from_millis(20));
    println!("Render");
}

fn processInput() {

}

fn update(elapsed: f64) {
    thread::sleep(time::Duration::from_millis(5));
    
}

