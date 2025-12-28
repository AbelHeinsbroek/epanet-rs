use rayon::prelude::*;
use std::time::Instant;

struct Pipes {
    flow: Vec<f32>,
    diameter: Vec<f32>,
    length: Vec<f32>,
    roughness: Vec<f32>,
    headloss: Vec<f32>,
    gradient: Vec<f32>,
}

impl Pipes {
    fn fast_pow_09(x: f32) -> f32 {
        // Approximate x^0.9 using exp2/log2
        // Error ~0.5–1%, acceptable for hydraulics
        let x_bits = x.to_bits();
        let exp = ((x_bits >> 23) & 0xFF) as i32 - 127;
        let mant = (x_bits & 0x7FFFFF) | 0x3F800000;
        let m = f32::from_bits(mant);
        let y = m.ln() * 0.9 + (exp as f32) * std::f32::consts::LN_2 * 0.9;
        y.exp()
    }

    fn headloss_parallel(&mut self) {
        // Zip all 6 buffers into one parallel iterator
        self.headloss.par_iter_mut()
            .zip(self.gradient.par_iter_mut())
            .zip(self.flow.par_iter())
            .zip(self.diameter.par_iter())
            .zip(self.length.par_iter())
            .zip(self.roughness.par_iter())
            .for_each(|(((((hl, grad), &q_raw), &d_raw), &l_raw), &r_raw)| {
                let flow = q_raw.abs();
                
                // Constants defined inside for scope/clarity
                const A1: f32 = 0.7853981634; // pi/4
                const A2: f32 = -0.8685889638065037; // -2/ln(10)
                // const RHO: f32 = 1000.0;

                let diameter_m = d_raw / 1000.0;
                let area = A1 * diameter_m * diameter_m;
                const INV_3600: f32 = 1.0 / 3600.0;
                let velocity = flow * INV_3600 / area;
                let re = velocity * diameter_m / 1.0e-6;
                let epsilon_d = (r_raw / 1000.0) / diameter_m;

                // let y1 = 5.74 / Self::fast_pow_09(re);
                let y1 = 5.74 / re.powf(0.9);
                let y2 = epsilon_d / 3.7 + y1;
                let y3 = A2 * y2.ln();
                
                let f = 1.0 / (y3 * y3);
                // Pre-calculate common terms for the gradient
                let df_dq = 1.8 * f * y1 * A2 / y2 / y3 / flow;
                const HL_CONST: f32 = 1000.0 / (2.0 * 9810.0);
                let hl_val = f * l_raw * velocity * velocity * HL_CONST / diameter_m;
                
                *hl = hl_val;
                *grad = (2.0 * hl_val / flow) + (hl_val / f * df_dq);
            });
    }
}
fn main() {

  let iterations = 10_000_000;
  println!("Creating {} pipes", iterations);
  let start = Instant::now();
  let mut pipes = Pipes {
    flow: vec![0.0; iterations],
    diameter: vec![300.0; iterations],
    length: vec![100.0; iterations],
    roughness: vec![0.25; iterations],
    headloss: vec![0.0; iterations],
    gradient: vec![0.0; iterations]
  };
  for i in 0..iterations {
    pipes.flow[i] = i as f32 + 1.0;
  }
  let duration = start.elapsed();
  println!("Time taken for creation: {:?}", duration);
  let start = Instant::now();

  pipes.headloss_parallel();
  let tot = pipes.headloss.iter().sum::<f32>();
  let duration = start.elapsed();
  println!("Time taken for headloss: {:?}", duration);
  println!("Iterations per second: {}", iterations as f32 / duration.as_secs_f32());
  println!("Total headloss: {}", tot);
}
