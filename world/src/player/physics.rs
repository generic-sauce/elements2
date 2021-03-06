use crate::prelude::*;

const STAIR_NUM: i32 = 2;

impl Player {
	pub(in super) fn move_by_velocity(&mut self, t: &TileMap) {
		if is_colliding(self.left_bot, t) {
			println!("A player is glitched.");
			return;
		}

		let mut remaining_vel = self.velocity;

		while remaining_vel != 0.into() {
			let xroute = route(remaining_vel.x, self.left_bot.x, PLAYER_SIZE.x);
			let yroute = route(remaining_vel.y, self.left_bot.y, PLAYER_SIZE.y);

			let xroute_ex = xroute + remaining_vel.x.signum();
			let yroute_ex = yroute + remaining_vel.y.signum();

			if xroute.abs() >= remaining_vel.x.abs() && yroute.abs() >= remaining_vel.y.abs() { // if no more collisions can happen!
				self.left_bot += remaining_vel;
				break;
			} else if (remaining_vel.y == 0 && yroute_ex == 0) /* edge case */ || (xroute_ex * remaining_vel.y).abs() < (yroute_ex * remaining_vel.x).abs() { //    <->    xroute / self.velocity.x < yroute / self.velocity.y    <->    xtime < ytime
				assert!(remaining_vel.x != 0);

				let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
				let change = GameVec::new(xroute, ychange);

				let change_ex = change + (remaining_vel.x.signum(), 0);
				let stair_plus_y = |i: i32| {
					let tile = GameVec::new(0, self.left_bot.y + change_ex.y).to_tile() + (0,i);
					tile.to_game().y
				};
				let mut stair_iter = // returns position where to be placed afterwards
					Some(self.left_bot.y + change_ex.y).into_iter()
						.chain((1..(STAIR_NUM+1))
							.map(stair_plus_y)
						).map(|y| GameVec::new(self.left_bot.x + change_ex.x, y));
				if let Some(new_pos) = stair_iter.find(|&v| !is_colliding(v, t)) {
					remaining_vel -= change_ex;
					self.left_bot = new_pos;
				} else { // collision
					remaining_vel -= change;
					self.left_bot += change;

					remaining_vel.x = 0;
					self.velocity.x = 0;
				}
			} else {
				assert!(remaining_vel.y != 0);

				let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
				let change = GameVec::new(xchange, yroute);

				let change_ex = change + (0, remaining_vel.y.signum());
				if is_colliding(self.left_bot + change_ex, t) {
					assert!(!is_colliding(self.left_bot + change, t));

					remaining_vel -= change;
					self.left_bot += change;

					remaining_vel.y = 0;
					self.velocity.y = 0;
				} else {
					remaining_vel -= change_ex;
					self.left_bot += change_ex;
				}
			}
		}
	}
}

fn is_colliding(left_bot: GameVec, t: &TileMap) -> bool {
	let min = GameVec::new(left_bot.x, left_bot.y).to_tile();
	let max = GameVec::new(left_bot.x + PLAYER_SIZE.x - 1, left_bot.y + PLAYER_SIZE.y - 1).to_tile();

	// TODO write this using .any()?
	for x in min.x..=max.x {
		for y in min.y..=max.y {
			let p = TileVec::new(x, y);
			if t.check_solid(p) { return true; }
		}
	}
	false
}

// returns the change required to touch, but not collide the next tile
fn route(velocity: i32, min: i32, size: i32) -> i32 {
	if velocity < 0 {
		-(min % TILESIZE)
	} else {
		let max = min + size - 1;
		(TILESIZE-1) - (max % TILESIZE)
	}
}
