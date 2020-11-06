use crate::prelude::*;

mod player;
use player::*;

mod world;
use world::*;

mod texture;
pub use texture::*;

mod vec;
pub use vec::*;

#[derive(Copy, Clone)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

#[allow(unused)]
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

impl Mul for Color {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Color {
			r: self.r * rhs.r,
			g: self.g * rhs.g,
			b: self.b * rhs.b,
			a: self.a * rhs.a,
		}
	}
}

impl Mul<f32> for Color {
	type Output = Color;

	fn mul(self, rhs: f32) -> Self::Output {
		Color {
			r: self.r * rhs,
			g: self.g * rhs,
			b: self.b * rhs,
			a: self.a * rhs,
		}
	}
}

#[derive(PartialEq, Eq)]
pub enum Flip {
	Normal,
	Horizontal,
}

#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: ViewVec,
	pub uv: TextureVec,
	pub color: Color,
}

pub type Triangle = [Vertex; 3];
pub type Triangles = Vec<Triangle>;
pub type TextureTriangles = Vec<Triangles>;

pub struct Text {
	pub left_bot: ViewVec,
	pub scale: f32,
	pub color: Color,
	pub string: String,
}

pub trait IntoTextureIndex {
	fn into_texture_index(self) -> usize;
}

pub struct Draw {
	pub clear_color: Option<Color>,
	pub triangles: TextureTriangles,
	pub texts: Vec<Text>,
	pub world: Option<GraphicsWorld>,
}

impl Draw {
	pub fn new() -> Draw {
		let clear_color = None;
		let mut triangles = TextureTriangles::new();
		triangles.resize_with(TextureId::texture_count(), Default::default);
		let texts = Vec::new();
		Draw {
			clear_color,
			triangles,
			texts,
			world: None,
		}
	}

	pub fn set_clear_color(&mut self, clear_color: Color) {
		if let Some(_) = self.clear_color {
			panic!("clear color was set already");
		}
		self.clear_color = Some(clear_color);
	}

	#[allow(unused)]
	pub fn texture(
		&mut self,
		left_bot: impl IntoViewVec,
		right_top: impl IntoViewVec,
		texture_index: impl IntoTextureIndex,
		flip: Flip,
		color: Option<Color>,
	) {
		let texture_index = texture_index.into_texture_index();
		let triangles = &mut self.triangles[texture_index];
		let left_bot = left_bot.to_view();
		let right_top = right_top.to_view();
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
		left_bot: impl IntoViewVec,
		right_top: impl IntoViewVec,
		color: Color,
	) {
		let triangles = &mut self.triangles[TextureId::White as usize];
		let left_bot = left_bot.to_view();
		let right_top = right_top.to_view();

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

	#[allow(unused)]
	pub fn text(
		&mut self,
		left_bot: impl IntoViewVec,
		scale: f32,
		color: Color,
		string: &str,
	) {
		let left_bot = left_bot.to_view();
		let string = string.to_string();

		let text = Text {
			left_bot,
			scale,
			color,
			string,
		};

		self.texts.push(text);
	}
}
