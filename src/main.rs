
use ff_encode::{VideoCodec, VideoEncoder};
use ff_format::{PixelFormat, PooledBuffer, Rational, Timestamp, VideoFrame};

mod voronoi;

use voronoi::Grid;
use voronoi::DebugInformation;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: u32 = 30;
const DURATION_SECONDS: f64 = 5.0;

const NUMBER_OF_POINTS: usize = 10;
const POINTS_PER_CELL: f64 = 3.0;
const SPEED_RANGE: f64 = 50.0;
const SEED: u64 = 0;

const OUTPUT_PATH: &str = "output.mp4";
const BLUR_ENABLED: bool = true;

const DEBUG_FLAG: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame_count = (DURATION_SECONDS * FPS as f64).round() as u64;
    let frame_duration = 1.0 / FPS as f64;
    let stride = WIDTH * 3;

    let mut debug_info: Option<&mut DebugInformation> = if DEBUG_FLAG { 
        Some(&mut DebugInformation::new()) 
    } 
    else { 
        None
    };

    let mut encoder = VideoEncoder::create(OUTPUT_PATH)
        .video(WIDTH as u32, HEIGHT as u32, FPS as f64)
        .video_codec(VideoCodec::H264)
        .build()?;

    println!("Encoder: {}", encoder.actual_video_codec());

    let mut grid = Grid::new(
        WIDTH,
        HEIGHT,
        NUMBER_OF_POINTS,
        POINTS_PER_CELL,
        SPEED_RANGE,
        SEED,
    );

    let buffer_len = WIDTH * HEIGHT * 3;
    let mut pixels = vec![0u8; buffer_len];
    let mut blurred_pixels = vec![0u8; buffer_len];

    for frame_index in 0..frame_count {
        render_frame(&mut pixels, &grid, &mut debug_info);

        let frame_pixels = if BLUR_ENABLED {
            blur_rgb(
                &pixels,
                &mut blurred_pixels,
                WIDTH,
                HEIGHT,
            );
            &blurred_pixels
        } else {
            &pixels
        };

        let timestamp = Timestamp::new(
            frame_index as i64,
            Rational::new(1, FPS as i32),
        );

        let video_frame = VideoFrame::new(
            vec![PooledBuffer::standalone(frame_pixels.to_vec())],
            vec![stride],
            WIDTH as u32,
            HEIGHT as u32,
            PixelFormat::Rgb24,
            timestamp,
            frame_index == 0,
        )?;

        encoder.push_video(&video_frame)?;

        grid.update(frame_duration);

        if (frame_index + 1) % FPS as u64 == 0 {
            println!(
                "Rendered {}/{} frames",
                frame_index + 1,
                frame_count,
            );
        }
    }

    encoder.finish()?;

    println!(
        "Created {} ({} frames)",
        OUTPUT_PATH,
        frame_count,
    );

    if let Some(info) = debug_info {
        println!(
            "{:.2} cell accesses per get_color",
            info.grid_cell_checked_number as f64
                / info.get_closest_neighbour_number as f64
        );
    } 

    Ok(())
}

fn render_frame(frame: &mut [u8], grid: &Grid, debug_info: &mut Option<&mut DebugInformation>) {
    assert_eq!(frame.len(), grid.width * grid.height * 3);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let index = (y * grid.width + x) * 3;
            let rgb = grid.get_color(x as f64, y as f64, debug_info);

            frame[index..index + 3].copy_from_slice(&rgb);
        }
    }
}

/// Applies a 3x3 box blur with wrapping image boundaries.
/// Source and destination must be separate buffers.
fn blur_rgb(
    source: &[u8],
    destination: &mut [u8],
    width: usize,
    height: usize,
) {
    assert!(width > 0 && height > 0);
    assert_eq!(source.len(), width * height * 3);
    assert_eq!(destination.len(), source.len());

    for y in 0..height {
        for x in 0..width {
            let mut sums = [0u32; 3];

            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    let nx =
                        (x as isize + dx).rem_euclid(width as isize)
                            as usize;
                    let ny =
                        (y as isize + dy).rem_euclid(height as isize)
                            as usize;

                    let index = (ny * width + nx) * 3;

                    for channel in 0..3 {
                        sums[channel] += source[index + channel] as u32;
                    }
                }
            }

            let output_index = (y * width + x) * 3;

            for channel in 0..3 {
                destination[output_index + channel] =
                    (sums[channel] / 9) as u8;
            }
        }
    }
}
