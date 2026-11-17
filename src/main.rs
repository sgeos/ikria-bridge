//! Single-session verification bridge. No flight law lives in this process.
use ikria_records::flight::{self, BridgeFrame, HarnessFrame, Telemetry};
use ikria_records::{GameTime, PartId, Request, Situation, Vector3, VesselCommand, VesselId};
use krpc_client::{
    Client,
    services::{
        krpc::{GameScene, KRPC},
        space_center::*,
    },
};
use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    time::timeout,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const DEADLINE: Duration = Duration::from_millis(700);

struct Game {
    sc: SpaceCenter,
    krpc: KRPC,
    vessel: Vessel,
    control: Control,
    parts: Parts,
    engine: Engine,
    chute: Parachute,
    flight: Flight,
    frame: ReferenceFrame,
    surface: ReferenceFrame,
    world: ReferenceFrame,
    loads: u32,
    name: String,
    armed: bool,
}

async fn identify_save(sc: &SpaceCenter, saves: &Path) -> Result<String> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let name = format!("ikria-bridge-guard-{}-{nonce}", std::process::id());
    sc.save(name.clone()).await?;
    let mut found = Vec::new();
    for entry in std::fs::read_dir(saves)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && entry.path().join(format!("{name}.sfs")).is_file() {
            found.push(entry.path());
        }
    }
    if found.len() != 1 {
        return Err("loaded save could not be established".into());
    }
    let folder = found[0]
        .file_name()
        .and_then(|x| x.to_str())
        .ok_or("invalid save name")?
        .to_owned();
    for extension in ["sfs", "loadmeta"] {
        let path = found[0].join(format!("{name}.{extension}"));
        if path.exists() {
            std::fs::remove_file(path)?;
        }
    }
    Ok(folder)
}

impl Game {
    async fn connect(saves: &Path, name: String, armed: bool) -> Result<Self> {
        let client = Client::new("Ikria verification bridge", "127.0.0.1", 50000, 50001).await?;
        let sc = SpaceCenter::new(client.clone());
        let krpc = KRPC::new(client);
        if identify_save(&sc, saves).await? != flight::VERIFICATION_SAVE {
            return Err("verification save required".into());
        }
        if krpc.get_game_scene().await? != GameScene::Flight {
            return Err("flight scene required".into());
        }
        let vessel = sc.get_active_vessel().await?.ok_or("no active vessel")?;
        if vessel.get_name().await? != name {
            return Err("active vessel name mismatch".into());
        }
        let parts = vessel.get_parts().await?;
        let engines = parts.get_engines().await?;
        let chutes = parts.get_parachutes().await?;
        if engines.len() != 1 || chutes.len() != 1 {
            return Err("exactly one engine and parachute required".into());
        }
        let control = vessel.get_control().await?;
        let frame = vessel.get_reference_frame().await?;
        let surface = vessel.get_surface_reference_frame().await?;
        let world = vessel
            .get_orbit()
            .await?
            .get_body()
            .await?
            .get_reference_frame()
            .await?;
        let flight = vessel.flight(Some(&world)).await?;
        let loads = krpc.get_game_state_loads().await?;
        let game = Self {
            sc,
            krpc,
            vessel,
            control,
            parts,
            engine: engines[0].clone(),
            chute: chutes[0].clone(),
            flight,
            frame,
            surface,
            world,
            loads,
            name,
            armed,
        };
        let t = game.sample().await?;
        if t.crew != 0 || !t.physics_active {
            return Err("empty loaded vessel required".into());
        }
        if armed
            && (t.situation != Situation::PreLaunch
                || t.engine_active
                || t.chute_deployed
                || t.parts != 8
                || (t.solid_fuel - 35.0).abs() > 0.001
                || game.name != "IKR-TS-001B"
                || (game.engine.get_thrust_limit().await? - 0.2).abs() > 0.0001)
        {
            return Err("armed profile requires untouched IKR-TS-001B on the pad".into());
        }
        if armed {
            let mut names = Vec::new();
            for part in game.parts.get_all().await? {
                names.push(part.get_name().await?);
            }
            names.sort();
            let mut expected = [
                "mk1pod.v2",
                "parachuteSingle",
                "GooExperiment",
                "GooExperiment",
                "solidBooster.sm.v2",
                "basicFin",
                "basicFin",
                "basicFin",
            ];
            expected.sort();
            if names != expected {
                return Err("part inventory does not match revision B".into());
            }
        }
        if game.control.get_sas().await? {
            return Err("built-in attitude assistance must be off".into());
        }
        Ok(game)
    }

    async fn sample(&self) -> Result<Telemetry> {
        if self.krpc.get_game_state_loads().await? != self.loads
            || self.krpc.get_game_scene().await? != GameScene::Flight
            || self.sc.get_active_vessel().await? != Some(self.vessel.clone())
            || self.sc.get_rails_warp_factor().await? != 0
            || self.sc.get_physics_warp_factor().await? != 0
        {
            return Err("save, scene, active vessel or warp changed".into());
        }
        let situation = match self.vessel.get_situation().await? {
            VesselSituation::PreLaunch => Situation::PreLaunch,
            VesselSituation::Landed => Situation::Landed,
            VesselSituation::Splashed => Situation::Splashed,
            VesselSituation::Flying => Situation::Flying,
            VesselSituation::SubOrbital => Situation::SubOrbital,
            VesselSituation::Orbiting => Situation::Orbiting,
            VesselSituation::Escaping => Situation::Escaping,
            VesselSituation::Docked => Situation::Docked,
        };
        let up = self
            .sc
            .transform_direction((1.0, 0.0, 0.0), &self.surface, &self.frame)
            .await?;
        let world_rates = self.vessel.angular_velocity(&self.world).await?;
        let rates = self
            .sc
            .transform_direction(world_rates, &self.world, &self.frame)
            .await?;
        let t = Telemetry {
            id: VesselId(1),
            situation,
            altitude_m: self.flight.get_surface_altitude().await?,
            vertical_speed_m_s: self.flight.get_vertical_speed().await?,
            solid_fuel: f64::from(
                self.vessel
                    .get_resources()
                    .await?
                    .amount("SolidFuel".into())
                    .await?,
            ),
            mass_kg: f64::from(self.vessel.get_mass().await?),
            crew: self.vessel.get_crew_count().await?.try_into()?,
            controllable: self.control.get_state().await? == ControlState::Full,
            has_link: self.vessel.get_comms().await?.get_can_communicate().await?,
            physics_active: self.vessel.get_loaded().await? && !self.vessel.get_packed().await?,
            parts: self.parts.get_all().await?.len().try_into()?,
            engine_active: self.engine.get_active().await?,
            chute_deployed: self.chute.get_deployed().await?,
            chute_safe: self.chute.get_safe_state().await? == ParachuteSafeState::Safe,
            up_body: Vector3::new(up.0, up.1, up.2)?,
            rates_body: Vector3::new(rates.0, rates.1, rates.2)?,
        };
        t.validate()?;
        Ok(t)
    }

    async fn apply(&self, t: &Telemetry, c: &VesselCommand) -> Result<()> {
        flight::authorize(t, c, PartId(1), PartId(2), self.armed)?;
        if !self.armed {
            return Ok(());
        }
        if t.controllable {
            self.control.set_throttle(c.throttle.value() as f32).await?;
            self.control.set_pitch(c.pitch.value() as f32).await?;
            self.control.set_yaw(c.yaw.value() as f32).await?;
            self.control.set_roll(c.roll.value() as f32).await?;
        }
        for r in &c.requests {
            match r {
                Request::ActivateEngine { .. } => self.engine.set_active(true).await?,
                Request::DeployParachute { .. } => self.chute.deploy().await?,
                _ => return Err("unsupported request".into()),
            }
        }
        Ok(())
    }
    async fn neutral(&self) -> Result<()> {
        // Never use stale vessel handles across a load. Neutralization is best effort.
        if self.armed && self.krpc.get_game_state_loads().await? == self.loads {
            self.control.set_throttle(0.0).await?;
            self.control.set_pitch(0.0).await?;
            self.control.set_yaw(0.0).await?;
            self.control.set_roll(0.0).await?;
        }
        Ok(())
    }
}

async fn read<R: tokio::io::AsyncRead + Unpin>(reader: &mut BufReader<R>) -> Result<HarnessFrame> {
    let mut bytes = Vec::new();
    reader
        .take(flight::MAX_FRAME as u64)
        .read_until(b'\n', &mut bytes)
        .await?;
    if bytes.last() != Some(&b'\n') || bytes.len() >= flight::MAX_FRAME {
        return Err("truncated or oversized frame".into());
    }
    Ok(serde_json::from_slice(&bytes)?)
}
async fn write(reader: &mut BufReader<TcpStream>, frame: &BridgeFrame) -> Result<()> {
    let mut bytes = serde_json::to_vec(frame)?;
    if bytes.len() + 1 >= flight::MAX_FRAME {
        return Err("oversized frame".into());
    }
    bytes.push(b'\n');
    reader.get_mut().write_all(&bytes).await?;
    Ok(())
}

async fn session(game: &Game, peer: &mut BufReader<TcpStream>, max_ticks: u64) -> Result<()> {
    write(
        peer,
        &BridgeFrame::Hello {
            version: flight::VERSION,
            save: flight::VERIFICATION_SAVE.into(),
            vessel: game.name.clone(),
            armed: game.armed,
            engine: PartId(1),
            chute: PartId(2),
        },
    )
    .await?;
    if timeout(Duration::from_secs(2), read(peer)).await??
        != (HarnessFrame::Hello {
            version: flight::VERSION,
        })
    {
        return Err("version handshake refused".into());
    }
    let mut previous = None;
    for seq in 0..max_ticks {
        let more = timeout(DEADLINE, async {
            game.krpc.next_tick().await?;
            let ut = GameTime::new(game.sc.get_ut().await?)?;
            if previous.is_some_and(|p: GameTime| {
                ut.seconds() - p.seconds() < 0.001 || ut.seconds() - p.seconds() > 0.03
            }) {
                return Err::<bool, Box<dyn std::error::Error>>(
                    "nonconsecutive physics tick".into(),
                );
            }
            previous = Some(ut);
            let t = game.sample().await?;
            write(
                peer,
                &BridgeFrame::Sample {
                    seq,
                    ut,
                    telemetry: t.clone(),
                },
            )
            .await?;
            match read(peer).await? {
                HarnessFrame::Close {} => Ok(false),
                HarnessFrame::Command {
                    seq: s,
                    ut: u,
                    command,
                } if s == seq && u == ut => {
                    // Expired hold is detectable before any mutation. One-second server
                    // timeout is assumed and must exceed this cycle's 700 ms deadline.
                    if game.sc.get_ut().await? != ut.seconds() {
                        return Err("tick hold expired".into());
                    }
                    game.apply(&t, &command).await?;
                    write(peer, &BridgeFrame::Applied { seq }).await?;
                    Ok(true)
                }
                _ => Err("unexpected sequence, time or message".into()),
            }
        })
        .await??;
        if !more {
            return Ok(());
        }
    }
    Err("tick limit reached".into())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if !(args.len() == 4 || args.len() == 5) || (args.len() == 5 && args[4] != "--arm-verification")
    {
        return Err("usage: ikria-bridge SAVES_DIRECTORY VESSEL LOOPBACK_PORT MAX_TICKS [--arm-verification]".into());
    }
    let port = args[2].parse::<u16>()?;
    let max_ticks = args[3].parse::<u64>()?;
    if max_ticks == 0 || max_ticks > 30000 {
        return Err("tick limit must be 1..30000".into());
    }
    let game = timeout(
        Duration::from_secs(15),
        Game::connect(&PathBuf::from(&args[0]), args[1].clone(), args.len() == 5),
    )
    .await??;
    eprintln!("profile guarded, game state checked");
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).await?;
    eprintln!("ready on {}", listener.local_addr()?);
    let (socket, _) = timeout(Duration::from_secs(30), listener.accept()).await??;
    socket.set_nodelay(true)?;
    let mut peer = BufReader::new(socket);
    let outcome = timeout(
        Duration::from_secs(900),
        session(&game, &mut peer, max_ticks),
    )
    .await;
    // Cancellation may have interrupted a kRPC response. Never reuse that connection
    // after an outer timeout. Process exit disconnects and releases the server hold.
    match outcome {
        Ok(result) => {
            // A cancelled read can leave a partial kRPC response. Never issue another
            // request on that connection. Exiting closes it and releases the hold.
            if result.as_ref().err().is_some_and(|e| {
                e.is::<tokio::time::error::Elapsed>() || e.is::<krpc_client::error::RpcError>()
            }) {
                return result;
            }
            let neutral = timeout(Duration::from_millis(150), game.neutral()).await;
            eprintln!("neutralization: {neutral:?}");
            if !matches!(neutral, Ok(Ok(()))) {
                return Err(
                    "neutralization failed; disconnecting without reusing the RPC stream".into(),
                );
            }
            let released = timeout(Duration::from_millis(150), game.krpc.release_tick()).await;
            eprintln!("release: {released:?}");
            if let Err(ref e) = result {
                let _ = timeout(
                    Duration::from_millis(100),
                    write(
                        &mut peer,
                        &BridgeFrame::Failed {
                            reason: e.to_string(),
                        },
                    ),
                )
                .await;
            }
            result
        }
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn truncated_unknown_and_oversized_frames_fail() {
        for bytes in [
            b"{\"type\":\"close\"}".to_vec(),
            b"{\"type\":\"close\",\"unknown\":1}\n".to_vec(),
            vec![b' '; flight::MAX_FRAME],
        ] {
            let mut peer = BufReader::new(bytes.as_slice());
            assert!(read(&mut peer).await.is_err());
        }
    }
    #[tokio::test]
    async fn frames_are_read_one_at_a_time() {
        let mut peer = BufReader::new(
            b"{\"type\":\"close\"}\n{\"type\":\"hello\",\"version\":2}\n".as_slice(),
        );
        assert_eq!(read(&mut peer).await.unwrap(), HarnessFrame::Close {});
        assert_eq!(
            read(&mut peer).await.unwrap(),
            HarnessFrame::Hello {
                version: flight::VERSION
            }
        );
    }
    #[tokio::test]
    async fn silent_peer_is_bounded_by_deadline() {
        let (_writer, reader) = tokio::io::duplex(32);
        let mut reader = BufReader::new(reader);
        assert!(
            timeout(Duration::from_millis(10), read(&mut reader))
                .await
                .is_err()
        );
    }
}
