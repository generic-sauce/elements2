use crate::prelude::*;

const VIEW_SIZE: GameVec = TileVec::new(128, 72).to_game();

/* from (-1, -1) to (1, 1)
 * corresponds to the window surface area
 */
pub struct SurfaceParam;
pub type SurfaceVec = Vec2t<f32, SurfaceParam>;

pub trait IntoSurfaceVec {
	fn to_surface(self, window_size: Vec2u) -> SurfaceVec;
}

impl IntoSurfaceVec for GameVec {
	fn to_surface(self, _window_size: Vec2u) -> SurfaceVec {
		let mut v = self.to_f() / VIEW_SIZE.to_f();
		v = v * 2.0 - 1.0;

		SurfaceVec::new(v.x, v.y)
	}
}

impl IntoSurfaceVec for CanvasVec {
	fn to_surface(self, _window_size: Vec2u) -> SurfaceVec {
		let view_size = VIEW_SIZE.to_f();
		let aspect = view_size.x / view_size.y;
		let mut v = self.to_f();
		v.x /= aspect;
		v = v * 2.0 - 1.0;

		SurfaceVec::new(v.x, v.y)
	}
}
