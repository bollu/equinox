use self::State::*;

 #[derive(Eq)]
enum State {
	Attack, //transitioning to sustain
	Sustain, //in stasis
	Decay, //transitioning to comatose
	Comatose, //spits out base value
}

pub struct ASD {
	base_value: f32,
	sustain_value: f32,


	attack_slope: f32,
	decay_slope: f32,

	current_value: f32,
	state: State,

}



impl ASD{
	pub fn new (base_value: f32, sustain_value: f32, attack_time: f32, decay_time: f32) -> ASD {
		assert!(base_value <= sustain_value); //many signs would change if this were not true.
		assert!(attack_time > 0f32);
		assert!(decay_time > 0f32);

		ASD { base_value: base_value, 
			sustain_value: sustain_value, 
		
			attack_slope: (sustain_value - base_value) / attack_time,
			decay_slope: (base_value - sustain_value) / decay_time,
			current_value: base_value,

			state: Comatose 
		}
	}

	//I'm not sure what the best design is here. for now, I'll ignore attack() if you're already attacking.
	pub fn attack(&mut self) {
		//if self.state == Attack || self.state == Sustain { return }
		//self.current_time = 0.;
		self.state = Attack;
	}

	pub fn decay(&mut self) {
		//if self.state == Decay || self.state == Comatose { return }
		//self.current_time = 0.;
		self.state = Decay;
	}


	pub fn attacking(&self) -> bool {
		self.state == Attack
	}

	pub fn sustaining(&self) -> bool {
		self.state == Sustain
	}

	pub fn decaying(&self) -> bool {
		self.state == Decay
	}

	pub fn comatose(&self) -> bool {
		self.state == Comatose
	}

	pub fn Tick(&mut self, dt : f32) { 

		if self.state == Attack {
		
			self.current_value += self.current_value + self.attack_slope * dt;

			if self.current_value >= self.sustain_value {
				self.state = Sustain;
				self.current_value = self.sustain_value;
			}

		}

		else if self.state == Decay {
			self.current_value += self.decay_slope * dt;


			if self.current_value <= self.base_value {
				self.state = Comatose;
				self.current_value = self.base_value;
			}
		}
	}

	pub fn val(&self) -> f32 {	
		self.current_value
	}
}
