
use std::time;
use std::thread;



pub mod gameloop {
    use std::time;
    fn gameloop() {
        const MS_PER_UPDATE: f64 = 1.0/60.0;

        let mut previousTime = time::Instant::now();
        let mut lag: f64 = 0.0;

        loop {
            //calculate the elapsed time between frames to keep the game deterministic in any arquitecture
            let currentTime = time::Instant::now();
            let elapsed = currentTime.duration_since(previousTime).as_secs_f64();
            previousTime = currentTime;
            lag += elapsed;
            
            processInput();

            while lag >= MS_PER_UPDATE {
                update(elapsed);
                lag -= MS_PER_UPDATE;
            }

            //render times a pretty much random, to avoid sttutering the positions of elements while rendering are being infered
            render(lag/MS_PER_UPDATE);
            
        }
    }

    fn render(elapsed: f64) {

    }

    fn processInput() {

    }

    fn update(elapsed: f64) {

    }
}
