pub struct SustainedToggle {
	on: bool	
}

/*
pub impl SustainedToggle {
	pub fn new(on: bool) {
		SustainedToggle { on: on}
	}

	pub fn switch_on(&mut self) {
		self.on = true;
	}

	pub fn switch_off(&mut self) {
		self.on = false;
	}

	pub fn is_on(&self) {
		self.on
	}
}*/

/*
enum ImpulseToggleState {
	Idle, 
	On,
	Off
}


//goes from ...idle -> off -> *on* -> idle -> idle -> .... -> off
pub struct ImpulseToggle {
	state: ImpulseToggleState
}

pub impl ImpulseToggle {
	pub fn new(on: bool) {
		let state = if on { On } else { Off};
		ImpulseToggle {state: state}
	}

	pub fn switch_on(&mut self) {
		let new_state = match self.state {
			Off => On,
			On => Idle,
			Idle => Idle,
		}

		self.state = new_state;
	}

	pub fn switch_off(&mut self) {
		self.state = Off;
	}
}*/