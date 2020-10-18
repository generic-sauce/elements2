use crate::prelude::*;

pub(in super) fn draw(pl: &Player, target: &impl RenderTarget, context: &GameDrawContext) {
	let mut size = GameVec::new(PLAYER_SIZE.x, TILESIZE / 3);
	let offset = GameVec::new(-PLAYER_SIZE.x, PLAYER_SIZE.y + TILESIZE);
	let left_bot = pl.left_bot + offset;
	context.draw_rect(target, left_bot, size, Color::BLACK, Origin::LeftBottom);
	size.x = (size.x as f32 * (pl.health as f32 / MAX_HEALTH as f32)) as i32;
	context.draw_rect(target, left_bot, size, Color::GREEN, Origin::LeftBottom);
}
