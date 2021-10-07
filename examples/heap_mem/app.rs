use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::*; // provide GUI functionality and draws shapes to it
use rand::prelude::*; // provides random number

// std::alloc provides facilities for controlling memory allocation
use std::alloc::{GlobalAlloc, Layout, System};

use std::time::Instant; // access to the system’s clock.

#[global_allocator] // marks the following value (ALLOCATOR) as satisfying the GlobalAlloc trait.
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

// Prints the time taken for each allocation to STDOUT as the
// program runs. This provides a fairly accurate indication of
// the time taken for dynamic memory allocation.
struct ReportingAllocator;

// implements proxy/middleware for heap memory allocation
// Plotting heap allocation times against allocation size shows that there is no clear
// relationship between the two. The time taken to allocate memory is essentially unpredictable,
// even when requesting the same amount of memory multiple times.
unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout); // Defers the actual memory allocation to the system’s default memory allocator
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

struct World {
    // Contains the data that is useful for the lifetime of the program
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

struct Particle {
    // Defines an object in 2D space
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width); // <11>
        let y = world.height; // <11>
        let x_velocity = 0.0; // <12>
        let y_velocity = rng.gen_range(-2.0..0.0); // <12>
        let x_acceleration = 0.0; // <13>
        let y_acceleration = rng.gen_range(0.0..0.15); // <13>

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),                               // <14>
            velocity: [x_velocity, y_velocity].into(),             // <14>
            acceleration: [x_acceleration, y_acceleration].into(), // <14>
            color: [1.0, 1.0, 1.0, 0.99],                          // <15>
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration); // <16>
        self.position = add(self.position, self.velocity); // <16>
        self.acceleration = mul_scalar(
            // <17>
            self.acceleration, // <17>
            0.7,               // <17>
        ); // <17>
        self.color[3] *= 0.995; // <18>
    }
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::<Box<Particle>>::new(), // <19>
            height: height,
            width: width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(&self); // <20>
            let boxed_particle = Box::new(particle); // <21>
            self.particles.push(boxed_particle); // <22>
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;

            let particle_iter = self
                .particles // <23>
                .iter() // <23>
                .enumerate(); // <23>

            for (i, particle) in particle_iter {
                // <24>
                if particle.color[3] < 0.02 {
                    // <24>
                    to_delete = Some(i); // <24>
                } // <24>
                break; // <24>
            } // <24>
              // <24>
            if let Some(i) = to_delete {
                // <24>
                self.particles.remove(i); // <24>
            } else {
                // <24>
                self.particles.remove(0); // <24>
            }; // <24>
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3); // <25>

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }
}

// cargo run -q 2> alloc.tsv

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
