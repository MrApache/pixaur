#![allow(dead_code)]

use std::time::{Duration, Instant};

pub struct FpsCounter {
    last_frame_time: Instant,
    frame_times: Vec<Duration>,
    max_samples: usize,
}

impl FpsCounter {
    pub fn new(max_samples: usize) -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let delta = now - self.last_frame_time;
        self.last_frame_time = now;

        self.frame_times.push(delta);

        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        let avg_duration: Duration =
            self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
        1.0 / avg_duration.as_secs_f64()
    }
}
