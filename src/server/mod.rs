use crate::prelude::*;

// update_desire is within 0..=1000
const UPDATE_DESIRE_PER_FRAME: u32 = 100;

pub struct Server {
	world: World,
	socket: UdpSocket,
	peers: [SocketAddr; 2],
	update_desire: [u32; 2],
}

impl Server {
	pub fn new() -> Server {
		let mut socket = UdpSocket::bind(("0.0.0.0", PORT)).expect("Could not create server socket");
		socket.set_nonblocking(true).unwrap();

		let peers = wait_for_players(&mut socket);

		Server {
			world: World::new(),
			socket,
			peers,
			update_desire: [0, 0],
		}
	}

	pub fn run(&mut self) {
		let timed_loop = TimedLoop::with_fps(60);
		let interval = timed_loop.interval;
		for (_elapsed_time, delta_time, _fps, _load) in timed_loop {
			if delta_time > interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), interval.as_millis());
			}

			// receive packets
			while let Some((input_state, recv_addr)) = recv_packet(&mut self.socket) {
				let mut index: i32 = -1;
				for i in 0i32..2i32 {
					if recv_addr == self.peers[i as usize] {
						index = i;
					}
				}
				if index == -1 {
					eprintln!("got packet from {}, which is not a known peer", recv_addr);
				} else {
					let i = index as usize;
					let diff = self.world.players[i].input.diff(&input_state);
					self.update_desire[0] += diff;
					self.update_desire[1] += diff;
					self.world.players[i].input = input_state;
				}
			}

			self.tick();

			// send game update
			for i in 0..2 {
				self.update_desire[i] += UPDATE_DESIRE_PER_FRAME;
				if self.update_desire[i] >= 1000 {
					self.update_desire[i] = 0;
					let update = self.world.update();
					send_packet_to(&mut self.socket, &update, self.peers[i]);
				}
			}

			self.check_restart();
		}
	}

	fn check_restart(&mut self) {
		if let Some(p) = self.world.player_dead() {
			self.world.kills[1-p] += 1;
			let _ = self.world.reset();
		}
	}

	fn tick(&mut self) {
		let _ = self.world.tick();
	}
}

fn wait_for_players(socket: &mut UdpSocket) -> [SocketAddr; 2] {
	let mut peers = vec!();

	for _ in TimedLoop::with_fps(10) {
		if let Some((Init::Init, recv_addr)) = recv_packet(socket) {
			peers.push(recv_addr);
			println!("new player joined {}", recv_addr);
			if peers.len() == 2 {
				break;
			}
		}
	}

	for (i, peer) in peers.iter().enumerate() {
		let go = Go { your_player_id: i };
		send_packet_to(socket, &go, *peer);
	}

	return [peers[0], peers[1]];
}
