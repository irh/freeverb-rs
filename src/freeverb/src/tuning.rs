pub const FIXED_GAIN: f32 = 0.015;

pub const SCALE_WET: f32 = 3.0;
pub const SCALE_DAMPENING: f32 = 0.4;

pub const SCALE_ROOM: f32 = 0.28;
pub const OFFSET_ROOM: f32 = 0.7;

pub const STEREO_SPREAD: usize = 23;

pub const COMB_TUNING_L1: usize = 1116;
pub const COMB_TUNING_R1: usize = COMB_TUNING_L1 + STEREO_SPREAD;
pub const COMB_TUNING_L2: usize = 1188;
pub const COMB_TUNING_R2: usize = COMB_TUNING_L2 + STEREO_SPREAD;
pub const COMB_TUNING_L3: usize = 1277;
pub const COMB_TUNING_R3: usize = COMB_TUNING_L3 + STEREO_SPREAD;
pub const COMB_TUNING_L4: usize = 1356;
pub const COMB_TUNING_R4: usize = COMB_TUNING_L4 + STEREO_SPREAD;
pub const COMB_TUNING_L5: usize = 1422;
pub const COMB_TUNING_R5: usize = COMB_TUNING_L5 + STEREO_SPREAD;
pub const COMB_TUNING_L6: usize = 1491;
pub const COMB_TUNING_R6: usize = COMB_TUNING_L6 + STEREO_SPREAD;
pub const COMB_TUNING_L7: usize = 1557;
pub const COMB_TUNING_R7: usize = COMB_TUNING_L7 + STEREO_SPREAD;
pub const COMB_TUNING_L8: usize = 1617;
pub const COMB_TUNING_R8: usize = COMB_TUNING_L8 + STEREO_SPREAD;

pub const ALLPASS_TUNING_L1: usize = 556;
pub const ALLPASS_TUNING_R1: usize = ALLPASS_TUNING_L1 + STEREO_SPREAD;
pub const ALLPASS_TUNING_L2: usize = 441;
pub const ALLPASS_TUNING_R2: usize = ALLPASS_TUNING_L2 + STEREO_SPREAD;
pub const ALLPASS_TUNING_L3: usize = 341;
pub const ALLPASS_TUNING_R3: usize = ALLPASS_TUNING_L3 + STEREO_SPREAD;
pub const ALLPASS_TUNING_L4: usize = 225;
pub const ALLPASS_TUNING_R4: usize = ALLPASS_TUNING_L4 + STEREO_SPREAD;
