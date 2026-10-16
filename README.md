# Voronoi Animation Engine

A Voronoi animation engine written in **Rust** that simulates moving points with toroidal boundary conditions and renders the resulting animation to MP4 video.

## Features

- **Moving points** with wraparound boundaries.
- **Spatial grid partitioning** to organize points into cells.
- **Voronoi visualization** based on nearest-neighbour searches.
- **MP4 video export** using H.264 encoding with NVIDIA NVENC support.
- **Configurable parameters**, including resolution, frame rate, and animation duration.
- **Performance instrumentation** to track cell accesses and nearest-neighbour query costs.

## Getting Started

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install).
- FFmpeg with `h264_nvenc` support and a compatible NVIDIA GPU and driver if using hardware encoding.

### Installation and Execution

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/asof144/voronoi-animation.git
cd voronoi-animation
```

Run the project in release mode:

```bash
cargo run --release
```

The generated animation is saved as `output.mp4`.

## Performance

The current nearest-neighbour search scans the points to find the closest one, giving a worst-case time complexity of **O(N) per query**, where \(N\) is the number of points.

For an image with width \(W\) and height \(H\), rendering a frame using this search for every pixel has a worst-case time complexity of **O(WHN)**, excluding other processing and encoding costs.

Optional instrumentation helps analyze cell accesses and identify opportunities for optimization.

## Technologies

- **FFmpeg / H.264 NVENC** — video encoding.
- **Spatial partitioning** — grid-based organization of points.

## License

This project is licensed under the MIT License. See [`LICENSE`](LICENSE) for details.
