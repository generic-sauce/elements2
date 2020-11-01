use crate::prelude::*;

pub struct Local {
	gamepad_states: [RawGamepadState; 2],
	client_world: ClientWorld,
}

impl Local {
	pub fn new(best_of_n: u32) -> Local {
		Local {
			gamepad_states: [RawGamepadState::new(), RawGamepadState::new()],
			client_world: ClientWorld::new(best_of_n),
		}
	}
}

impl Runnable for Local {
	fn tick(&mut self, app: &mut App) {
		for (i, gamepad_state) in self.gamepad_states.iter_mut().enumerate() {
			self.client_world.world.players[i].input.update_gamepad(&gamepad_state);
		}
		self.client_world.world.players.last_mut().unwrap().input.update_peripherals(&app.peripherals_state);
		self.client_world.tick(app);
	}

	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		// self.client_world.draw(app, timed_loop_info);

		let world = &self.client_world.world;
		let graphics_world = GraphicsWorld::new(
			&world.tilemap,
			&world.fluidmap,
			world.players.clone(),
			timed_loop_info.elapsed_time,
		);
		app.graphics_sender.send(graphics_world).unwrap();
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		RunnableChange::from_world(&self.client_world.world)
	}
}
