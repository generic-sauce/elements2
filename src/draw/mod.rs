use crate::prelude::*;

mod player;
use player::*;

mod world;
use world::*;

mod texture;
pub use texture::*;

mod canvas_vec;
pub use canvas_vec::*;

#[derive(Copy, Clone)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Color {
	pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
		Color {
			r, g, b, a: 1.0,
		}
	}

	pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color {
			r, g, b, a,
		}
	}

	pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
	pub const GRAY: Color = Color::rgb(0.2, 0.2, 0.2);
	pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
	pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
	pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
	pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
}

#[derive(PartialEq, Eq)]
pub enum Flip {
	Normal,
	Horizontal,
}

#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: CanvasVec,
	pub uv: TextureVec,
	pub color: Color,
}

pub type Triangle = [Vertex; 3];
pub type Triangles = Vec<Triangle>;
pub type TextureTriangles = Vec<Triangles>;

pub struct Draw {
	pub triangles: TextureTriangles,
	pub world: Option<GraphicsWorld>,
	pub elapsed_time: Duration,
}

const UNUSED_TILEVEC: TileVec = TileVec::new(0, 0);

impl Draw {
	pub fn new(elapsed_time: Duration) -> Draw {
		let mut triangles = TextureTriangles::new();
		triangles.resize_with(TextureId::texture_count(), Default::default);
		Draw {
			triangles,
			world: None,
			elapsed_time,
		}
	}

	#[allow(unused)]
	pub fn texture(
		&mut self,
		left_bot: impl IntoCanvasVec,
		right_top: impl IntoCanvasVec,
		texture_index: impl IntoTextureIndex,
		flip: Flip,
		color: Option<Color>,
	) {
		let texture_index = texture_index.into_texture_index();
		let triangles = &mut self.triangles[texture_index];
		let left_bot = left_bot.to_canvas(UNUSED_TILEVEC);
		let right_top = right_top.to_canvas(UNUSED_TILEVEC);
		let color = color.unwrap_or(Color::WHITE);
		let (left_uv, right_uv) = match flip {
			Flip::Normal => (0.0, 1.0),
			Flip::Horizontal => (1.0, 0.0),
		};

		triangles.push([
			Vertex { position: left_bot,                   uv: TextureVec::new(left_uv, 0.0),  color },
			Vertex { position: v(right_top.x, left_bot.y), uv: TextureVec::new(right_uv, 0.0), color },
			Vertex { position: right_top,                  uv: TextureVec::new(right_uv, 1.0), color },
		]);

		triangles.push([
			Vertex { position: left_bot,                   uv: TextureVec::new(left_uv, 0.0),  color },
			Vertex { position: right_top,                  uv: TextureVec::new(right_uv, 1.0), color },
			Vertex { position: v(left_bot.x, right_top.y), uv: TextureVec::new(left_uv, 1.0),  color },
		]);
	}

	#[allow(unused)]
	pub fn rectangle(
		&mut self,
		left_bot: impl IntoCanvasVec,
		right_top: impl IntoCanvasVec,
		color: Color,
	) {
		let triangles = &mut self.triangles[TextureId::White as usize];
		let left_bot = left_bot.to_canvas(UNUSED_TILEVEC);
		let right_top = right_top.to_canvas(UNUSED_TILEVEC);

		triangles.push([
			Vertex { position: left_bot,                   uv: TextureVec::new(0.0, 0.0), color },
			Vertex { position: v(right_top.x, left_bot.y), uv: TextureVec::new(1.0, 0.0), color },
			Vertex { position: right_top,                  uv: TextureVec::new(1.0, 1.0), color },
		]);

		triangles.push([
			Vertex { position: left_bot,                   uv: TextureVec::new(0.0, 0.0), color },
			Vertex { position: right_top,                  uv: TextureVec::new(1.0, 1.0), color },
			Vertex { position: v(left_bot.x, right_top.y), uv: TextureVec::new(0.0, 1.0), color },
		]);
	}

	pub fn world(&mut self, tilemap: &TileMap, fluidmap: &FluidMap) {
		self.world = Some(GraphicsWorld::new(tilemap, fluidmap));
	}
}
