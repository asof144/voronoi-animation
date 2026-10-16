# Real-Time Voronoi Animation

A real-time Voronoi visualization tool written in **Rust**. It simulates moving points and generates an animated Voronoi-style visualization, exported as an MP4 video.

## Features

- Animated points with toroidal boundary conditions.
- Spatial grid for organizing points and checking nearby cells.
- Video rendering with H.264 encoding using `h264_nvenc`.
- Configurable resolution, frame rate, and animation duration.
- Optional debugging counters for measuring cell accesses and nearest-neighbour query performance.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- A compatible NVIDIA GPU and drivers for `h264_nvenc`, if using hardware encoding.

### Run

Clone the repository and run the project:

```bash
git clone https://github.com/your-username/your-repository.git
cd your-repository
cargo run --release
```

The generated animation is saved as `output.mp4`.

## Performance

The current nearest-neighbour search checks points to determine the closest one. Its worst-case time complexity is **O(N) per query**, where \(N\) is the number of points. Rendering a frame of \(W \times H\) pixels therefore has a worst-case complexity of **O(WHN)** with this search approach.

Optional instrumentation tracks cell accesses to help evaluate and improve performance.

## Technologies

- **Rust**
- **FFmpeg / H.264 NVENC**
- Spatial partitioning and nearest-neighbour search

## License

Add your preferred license here.
