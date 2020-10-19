use crate::prelude::*;

pub struct GraphicsWorld {
	pub tilemap_size: TileVec,
	pub fluidmap_size: FluidVec,
	pub tilemap_data: Vec<u8>,
	pub fluidmap_data: Vec<u8>,
	pub players: [Player; 2],
	pub elapsed_time: Duration,
}

impl GraphicsWorld {
	pub fn new(tilemap_size: TileVec, fluidmap_size: FluidVec, tilemap: &TileMap, fluidmap: &FluidMap, players: [Player; 2]) -> GraphicsWorld {
		let tilemap_data: Vec<u8> = tilemap.iter()
			.map(|p| tilemap.get(p))
			.map(|t| match t {
				Tile::Void => 0,
				Tile::Ground => 1,
				Tile::Wall { owner, .. } => 2 + owner as u8,
			})
			.collect();

		let fluidmap_data: Vec<u8> = fluidmap.grid.iter()
			.map(|i| i.iter().nth(0))
			.map(|i| {
				match i {
					Some(fluid) => fluid.owner as u8,
					None => 255
				}
			})
			.collect();

		GraphicsWorld {
			tilemap_size,
			fluidmap_size,
			tilemap_data,
			fluidmap_data,
			players,
			elapsed_time: Duration::from_millis(0),
		}
	}
}

impl Graphics {
	pub fn draw_players(&mut self, context: &DrawContext2, world: &GraphicsWorld) {
		for p in &world.players {
			self.triangles.draw_sprite(context, p.left_bot, p.left_bot + PLAYER_SIZE, Some(wgpu::Color::RED));
		}
	}
}
