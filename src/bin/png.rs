use crate::source::{Fps, Source};
use crate::BinResult;
use gifski::Collector;
use std::path::PathBuf;

pub struct Lodecoder {
    frames: Vec<PathBuf>,
    fps: f64,
    custom_delays: Option<Vec<Option<u32>>>,
}

impl Lodecoder {
    pub fn new(frames: Vec<PathBuf>, params: Fps) -> Self {
        Self {
            frames,
            fps: f64::from(params.fps) * f64::from(params.speed),
            custom_delays: None,
        }
    }

    pub fn new_with_delays(frames: Vec<PathBuf>, params: Fps, delays: Vec<Option<u32>>) -> Self {
        Self {
            frames,
            fps: f64::from(params.fps) * f64::from(params.speed),
            custom_delays: Some(delays),
        }
    }
}

impl Source for Lodecoder {
    fn total_frames(&self) -> Option<u64> {
        Some(self.frames.len() as u64)
    }

    #[inline(never)]
    fn collect(&mut self, dest: &mut Collector) -> BinResult<()> {
        let dest = &*dest;
        let f = std::mem::take(&mut self.frames);
        let delays = self.custom_delays.take();
        
        let mut accumulated_time = 0.0;
        let default_frame_duration = 1.0 / self.fps;
        
        for (i, frame) in f.into_iter().enumerate() {
            let presentation_timestamp = accumulated_time;
            
            // Calculate duration for this frame
            let frame_duration = if let Some(ref delays) = delays {
                if let Some(Some(custom_delay_ms)) = delays.get(i) {
                    // Use custom delay: convert milliseconds to seconds
                    (*custom_delay_ms as f64) / 1000.0
                } else {
                    // Use default FPS timing
                    default_frame_duration
                }
            } else {
                // No custom delays, use default FPS timing
                default_frame_duration
            };
            
            dest.add_frame_png_file(i, frame, presentation_timestamp)?;
            accumulated_time += frame_duration;
        }
        Ok(())
    }
}