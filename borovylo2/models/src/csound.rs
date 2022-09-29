pub struct Enc {
   pub min: f64,
   pub max: f64,
   pub step: f64,
}

pub const ENCODERS: [Enc; 21] = [
   Enc {
      min: 0.0,
      max: 2.0,
      step: 0.005,
   }, // 0 - "level"
   Enc {
      min: 0.02,
      max: 500.0,
      step: 0.001,
   }, // 1 - "density"
   Enc {
      min: 0.01,
      max: 2.0,
      step: 0.005,
   }, // 2 - "size"
   Enc {
      min: 0.0,
      max: 8.0,
      step: 0.01,
   }, // 3 - "oct_div"
   Enc {
      min: 0.0,
      max: 1.0,
      step: 0.001,
   }, // 4 - "freeze"
   Enc {
      min: -2.0,
      max: 2.0,
      step: 0.001,
   }, // 5 - "pitch"
   Enc {
      min: 0.0,
      max: 30.0,
      step: 0.01,
   }, // 6 - "port"
   Enc {
      min: 0.0,
      max: 0.5,
      step: 0.0001,
   }, // 7 - "rnd_pos"
   Enc {
      min: 0.0,
      max: 0.5,
      step: 0.001,
   }, // 8 - "rnd_pitch"
   Enc {
      min: 0.0,
      max: 100.0,
      step: 0.02,
   }, // 9 - "band"
   Enc {
      min: 0.0,
      max: 8.0,
      step: 0.02,
   }, // 10 - "lfo_rate"
   Enc {
      min: -4.0,
      max: 4.0,
      step: 0.01,
   }, // 11 - "lfo_dens"
   Enc {
      min: -2.0,
      max: 2.0,
      step: 0.01,
   }, // 12 - "lfo_size"
   Enc {
      min: -2.0,
      max: 2.0,
      step: 0.002,
   }, // 13 - "speed"
   Enc {
      min: 0.0,
      max: 1.0,
      step: 0.001,
   }, // 14 - "start"
   Enc {
      min: 0.0,
      max: 1.0,
      step: 0.001,
   }, // 15 - "range"
   Enc {
      min: 0.01,
      max: 2.0,
      step: 0.002,
   }, // 16 - "release"
   Enc {
      min: 0.002,
      max: 0.2,
      step: 0.002,
   }, // 17 - "decay"
   Enc {
      min: 0.002,
      max: 0.2,
      step: 0.002,
   }, // 18 - "rise"
   Enc {
      min: 0.0,
      max: 1.0,
      step: 0.001,
   }, // 19 - "attack"
   Enc {
      min: 0.0,
      max: 0.5,
      step: 0.001,
   }, // 20 - "rnd_dens"
];
