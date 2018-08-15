
use std::time::Instant;
use std::time::Duration;

pub struct Timer {

    running: bool,
    start: Instant,
    stop: Instant,

}

impl Timer {

    pub fn new() -> Self {
        Timer {
            running: false,
            start: Instant::now(),
            stop: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
        self.stop = Instant::now();
        self.running = true;
    }

    pub fn stop(&mut self) {
        if self.running {
            self.stop = Instant::now();
            self.running = false;
        }
    }

    pub fn get_time_nano(&mut self) -> f64 {
        if self.running {
            self.stop = Instant::now();
            f64::from(self.stop.duration_since(self.start).subsec_nanos())
        }
        else {
            f64::from(self.stop.duration_since(self.start).subsec_nanos())
        }
    }

    pub fn get_time_mili(&mut self) -> f64 {
        if self.running {
            self.stop = Instant::now();
            let milis = self.stop.duration_since(self.start).as_secs() as f64 * 1_000_f64;
            let nanos = f64::from(self.stop.duration_since(self.start).subsec_nanos());
            milis + (nanos * 1e-9)
        }
        else {
            let milis = self.stop.duration_since(self.start).as_secs() as f64 * 1_000_f64;
            let nanos = f64::from(self.stop.duration_since(self.start).subsec_nanos());
            milis + (nanos * 1e-9)
        }
    }

    pub fn get_time_sec(&mut self) -> f64 {
        if self.running {
            self.stop = Instant::now();
            let seconds = self.stop.duration_since(self.start).as_secs() as f64;
            let nanos = f64::from(self.stop.duration_since(self.start).subsec_nanos());
            seconds + (nanos * 1e-9)
        }
        else {
            let seconds = self.stop.duration_since(self.start).as_secs() as f64;
            let nanos = f64::from(self.stop.duration_since(self.start).subsec_nanos());
            seconds + (nanos * 1e-9)
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

}

pub struct Time {
    init_timer: Timer,
    runtime_timer: Timer,
    on_second: bool,
    delta: f32,
    avg_fps: f32,
    pub frames_in_second: u32,
    fpsis: f32,
    ftis: f32,
    avg_delta: f32,
    pub on_second_timer: f32,
}

impl Time {
    pub fn new() -> Self {
        let mut return_val = Time {
            init_timer: Timer::new(),
            runtime_timer: Timer::new(),
            on_second: false,
            delta: 0f32,
            avg_fps: 0f32,
            frames_in_second: 0u32,
            fpsis: 0f32,
            ftis: 0f32,
            avg_delta: 0f32,
            on_second_timer: 0f32,
        };

        return_val.init_timer.start();
        return_val.runtime_timer.start();

        return_val
    }

    pub fn update(&mut self) {

        self.runtime_timer.stop();
        self.delta = self.runtime_timer.get_time_sec() as f32;
        self.runtime_timer.start();

        self.on_second = false;

        self.on_second_timer += self.delta;
        self.fpsis += 1_f32 / self.delta;
        self.ftis += self.delta;
        self.frames_in_second += 1_u32;

        if self.on_second_timer >= 1_f32 {
            self.avg_fps = self.fpsis / (self.frames_in_second as f32);
            self.avg_delta = self.ftis / (self.frames_in_second as f32);
            self.on_second_timer = 0_f32;
            self.frames_in_second = 0;
            self.fpsis = 0_f32;
            self.ftis = 0_f32;
            self.on_second = true;
        }

    }

    pub fn delta(&self) -> f32 {
        self.delta
    }

    pub fn elapsed(&mut self) -> f32 {
        self.init_timer.get_time_sec() as f32
    }

    pub fn on_second(&self) -> bool {
        self.on_second
    }

    pub fn avg_fps(&self) -> f32 {
        self.avg_fps
    }

    pub fn fps(&self) -> f32 {
        1_f32 / self.delta
    }

    pub fn avg_delta(&mut self) -> f32 {
        self.avg_delta
    }
}


pub struct NewTimer {

    running: bool,
    start: Instant,
    stop: Instant,

}

impl NewTimer {

    pub fn new() -> Self {
        NewTimer {
            running: false,
            start: Instant::now(),
            stop: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
        //self.stop = Instant::now();
        self.running = true;
    }

    pub fn stop(&mut self) {
        if self.running {
            self.stop = Instant::now();
            self.running = false;
        }
    }

    pub fn elapsed (&mut self) -> Duration {
        if self.running {
            self.stop = Instant::now();
            self.stop.duration_since(self.start)
        }
        else {
            self.stop.duration_since(self.start)
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

}

pub fn duration_to_f64(d: Duration) -> f64 {
    let seconds = d.as_secs() as f64;
    let nanos = f64::from(d.subsec_nanos());
    seconds + (nanos * 1e-9)
}

pub struct NewTime {
    //timer: NewTimer,
    delta: f64,
    on_second: bool,
    on_second_timer: f64,
    elapsed_delta: Duration,
    last_instant: Instant,
}

impl NewTime {
    pub fn new() -> Self {
        NewTime {
            //timer: NewTimer::new(),
            delta: 0_f64,
            on_second: false,
            on_second_timer: 0_f64,
            elapsed_delta: Duration::new(0, 0),
            last_instant: Instant::now()
        }
    }

    pub fn update(&mut self) {

        /*self.timer.stop();
        self.elapsed_delta = self.timer.elapsed();
        self.delta = duration_to_f64(self.elapsed_delta);
        self.timer.start();*/

        let now = Instant::now();
        let time_since_last = now - self.last_instant;
        self.elapsed_delta = time_since_last;
        self.delta = duration_to_f64(self.elapsed_delta);
        self.last_instant = Instant::now();


        self.on_second = false;

        self.on_second_timer += self.delta;

        if self.on_second_timer >= 1_f64 {

            self.on_second = true;
            self.on_second_timer = 0_f64;

        }


    }

    pub fn delta(&self) -> f32 {
        self.delta as f32
    }

    pub fn elapsed(&mut self) -> f32 {
        0_f32
    }

    pub fn on_second(&self) -> bool {
        self.on_second
    }

    pub fn avg_fps(&self) -> f32 {
        0_f32
    }

    pub fn fps(&self) -> f32 {
        1_f32 / (self.delta as f32)
    }

    pub fn avg_delta(&mut self) -> f32 {
        0_f32
    }

    pub fn debug_delta(&self) -> Duration {
        self.elapsed_delta
    }
}