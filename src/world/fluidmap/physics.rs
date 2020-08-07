use crate::prelude::*;

impl FluidMap {
	pub(in super) fn move_by_velocity(&mut self, t: &TileMap) {
		for grid_entry in self.grid.iter_mut() {
			for fluid in grid_entry.iter_mut() {
				move_fluid_by_velocity(fluid, t);
			}
		}
	}
}

fn move_fluid_by_velocity(f: &mut Fluid, t: &TileMap) {
	let mut remaining_vel = f.velocity;

	while remaining_vel != 0.into() {
		let xroute = route(remaining_vel.x, f.position.x);
		let yroute = route(remaining_vel.y, f.position.y);

		let xroute_ex = xroute + remaining_vel.x.signum();
		let yroute_ex = yroute + remaining_vel.y.signum();

		if xroute.abs() >= remaining_vel.x.abs() && yroute.abs() >= remaining_vel.y.abs() { // if no more collisions can happen!
			f.position += remaining_vel;
			break;
		} else if (remaining_vel.y == 0 && yroute_ex == 0) /* edge case */ || (xroute_ex * remaining_vel.y).abs() < (yroute_ex * remaining_vel.x).abs() { //    <->    xroute / self.velocity.x < yroute / self.velocity.y    <->    xtime < ytime
			assert!(remaining_vel.x != 0);

			let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
			let change = GameVec::new(xroute, ychange);

			let change_ex = change + (remaining_vel.x.signum(), 0);
			if is_colliding(f.position + change_ex, t) {
				assert!(!is_colliding(f.position + change, t));

				remaining_vel -= change;
				f.position += change;

				remaining_vel.x = 0;
				f.velocity.x = 0;
			} else {
				remaining_vel -= change_ex;
				f.position += change_ex;
			}
		} else {
			assert!(remaining_vel.y != 0);

			let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
			let change = GameVec::new(xchange, yroute);

			let change_ex = change + (0, remaining_vel.y.signum());
			if is_colliding(f.position + change_ex, t) {
				assert!(!is_colliding(f.position + change, t));

				remaining_vel -= change;
				f.position += change;

				remaining_vel.y = 0;
				f.velocity.y = 0;
			} else {
				remaining_vel -= change_ex;
				f.position += change_ex;
			}
		}
	}
}


fn is_colliding(point: GameVec, t: &TileMap) -> bool {
	let point = point.to_tile();
	let point = Vec2u::new(point.x as u32, point.y as u32);
	t.get(point).is_solid()
}

// returns the change required to touch, but not collide the next tile
fn route(velocity: i32, pos: i32) -> i32 {
	if velocity < 0 {
		-(pos % TILESIZE)
	} else {
		(TILESIZE-1) - (pos % TILESIZE)
	}
}
