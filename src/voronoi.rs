
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    vx: f64,
    vy: f64,
    pub rgb: [u8; 3],
}

impl Point {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, rgb: [u8; 3]) -> Self {
        Self { x, y, vx, vy, rgb }
    }

    pub fn update(&mut self, time: f64, width: usize, height: usize) {
        self.x = (self.x + self.vx * time).rem_euclid(width as f64);
        self.y = (self.y + self.vy * time).rem_euclid(height as f64);
    }
}

#[derive(Debug, Default)]
pub struct Cell {
    pub points: Vec<Point>,
}

impl Cell {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug)]
pub struct DebugInformation {
    pub grid_cell_checked_number: u64,
    pub get_closest_neighbour_number: u64,
}

impl DebugInformation {
    pub fn new() -> Self {
        Self {
            grid_cell_checked_number: 0,
            get_closest_neighbour_number: 0,
        }
    }
}


#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub matrix_width: usize,
    pub matrix_height: usize,
    pub cell_size: f64,
    pub matrix: Vec<Cell>,
}

impl Grid {
    pub fn new(
        width: usize,
        height: usize,
        number_of_points: usize,
        points_per_cell: f64,
        speed_range: f64,
        seed: u64,
    ) -> Self {
        assert!(width > 0 && height > 0, "World dimensions must be positive");
        assert!(
            points_per_cell.is_finite() && points_per_cell > 0.0,
            "points_per_cell must be finite and positive"
        );
        assert!(
            speed_range.is_finite() && speed_range >= 0.0,
            "speed_range must be finite and non-negative"
        );

        let area = (width as f64) * (height as f64);
        let cell_size = if number_of_points == 0 {
            width.max(height) as f64
        } else {
            (area * points_per_cell / number_of_points as f64).sqrt()
        };

        let matrix_width = ((width as f64) / cell_size).ceil() as usize;
        let matrix_height = ((height as f64) / cell_size).ceil() as usize;

        let mut grid = Self {
            width,
            height,
            matrix_width,
            matrix_height,
            cell_size,
            matrix: (0..matrix_width * matrix_height)
                .map(|_| Cell::new())
                .collect(),
        };

        let mut rng = StdRng::seed_from_u64(seed);

        for _ in 0..number_of_points {
            let point = Point::new(
                rng.gen_range(0.0..width as f64),
                rng.gen_range(0.0..height as f64),
                rng.gen_range(-speed_range..speed_range),
                rng.gen_range(-speed_range..speed_range),
                [rng.r#gen(), rng.r#gen(), rng.r#gen()],
            );

            let index = grid.cell_index(point.x, point.y);
            grid.matrix[index].points.push(point);
        }

        grid
    }

    #[inline]
    fn matrix_index(&self, x: usize, y: usize) -> usize {
        y * self.matrix_width + x
    }

    #[inline]
    fn cell_index(&self, x: f64, y: f64) -> usize {
        let x = x.rem_euclid(self.width as f64);
        let y = y.rem_euclid(self.height as f64);

        let xi = ((x / self.cell_size) as usize).min(self.matrix_width - 1);
        let yi = ((y / self.cell_size) as usize).min(self.matrix_height - 1);

        self.matrix_index(xi, yi)
    }

    pub fn update(&mut self, time: f64) {
        assert!(time.is_finite(), "time must be finite");

        let old_cells = std::mem::take(&mut self.matrix);
        let mut new_cells: Vec<Cell> = (0..self.matrix_width * self.matrix_height)
            .map(|_| Cell::new())
            .collect();

        for cell in old_cells {
            for mut point in cell.points {
                point.update(time, self.width, self.height);

                let index = self.cell_index(point.x, point.y);
                new_cells[index].points.push(point);
            }
        }

        self.matrix = new_cells;
    }

    #[inline]
    fn toroidal_distance_squared(
        &self,
        x: f64,
        y: f64,
        point: &Point,
    ) -> f64 {
        let dx = (x - point.x)
            .abs()
            .rem_euclid(self.width as f64);
        let dy = (y - point.y)
            .abs()
            .rem_euclid(self.height as f64);

        let dx = dx.min(self.width as f64 - dx);
        let dy = dy.min(self.height as f64 - dy);

        dx * dx + dy * dy
    }

    pub fn closest_neighbour(&self, x: f64, y: f64, debug_info: &mut Option<&mut DebugInformation>) -> Option<&Point> {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }

        let x = x.rem_euclid(self.width as f64);
        let y = y.rem_euclid(self.height as f64);

        if let Some(info) = debug_info.as_deref_mut() {
            info.get_closest_neighbour_number += 1;
        }

        self.matrix
            .iter()
            .inspect(|_|
                if let Some(info) = debug_info.as_deref_mut() {
                    info.grid_cell_checked_number += 1;
                }
            )
            .flat_map(|cell| cell.points.iter())
            .min_by(|a, b| {
                self.toroidal_distance_squared(x, y, a)
                    .total_cmp(&self.toroidal_distance_squared(x, y, b))
            })
    }

    pub fn get_color(&self, x: f64, y: f64, debug_info: &mut Option<&mut DebugInformation>) -> [u8; 3] {
        self.closest_neighbour(x, y, debug_info)
            .map_or([0, 0, 0], |point| point.rgb)
    }
}
