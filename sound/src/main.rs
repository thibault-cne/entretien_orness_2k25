use std::io::Read;
use std::io::Seek;
use std::time::Duration;

use rodio::Device;
use rodio::DeviceTrait;
use rodio::cpal;
use rodio::cpal::traits::HostTrait;
use rodio::{OutputStream, Sink, Source, source::SineWave};

enum Command {
    Sleep(f32),
    Interval(f32),
    Note(String),
}

impl Command {
    fn execute(&self, sink: &Sink, state: &mut State) {
        match self {
            Self::Interval(val) => state.interval = *val,
            Self::Sleep(val) => {
                let source = SineWave::new(440.0)
                    .take_duration(Duration::from_secs_f32(*val))
                    .amplify(0.0);
                sink.append(source);
            }
            Self::Note(_) => {
                // SAFETY: this is safe to unwrap because a note command always return a frequency
                let frequency = self.calc_freq().unwrap();
                let source =
                    SineWave::new(frequency).take_duration(Duration::from_secs_f32(state.interval));
                sink.append(source);
            }
        }
    }

    fn calc_freq(&self) -> Option<f32> {
        match self {
            Self::Sleep(_) | Self::Interval(_) => None,
            Self::Note(note) => {
                let mut chars = note.chars();
                let letter = chars.next().unwrap();
                let nb = chars.next().unwrap() as u8;
                let nb = nb - 48;

                let dec_lettre = match letter {
                    'A' => 0,
                    'B' => 2,
                    'C' => 3,
                    'D' => 5,
                    'E' => 7,
                    'F' => 8,
                    'G' => 10,
                    _ => unimplemented!(),
                };

                let n = (nb - 1) * 12 + dec_lettre;

                // calculated freq
                let freq = 55.00 * 2_f32.powf(n as f32 / 12_f32);

                Some(freq)
            }
        }
    }
}

/// A structure to hold the state of the sound
/// it holds the current interval set.
struct State {
    interval: f32,
}

fn main() {
    // We retrieve the file path of the executable via the first arguments given to the executable
    let mut args = std::env::args();
    let path = args.next().unwrap();

    // We then open the file to read the content
    // SAFETY: this is safe because the file exists
    let mut file = std::fs::File::open(&path).unwrap();
    file.seek(std::io::SeekFrom::End(-8)).unwrap();

    // Last 8 bytes of the file are reserved to know the size of the sound command to retrieve
    // This is formated like so ...SOUND_COMMAND\nSOUND_COMMAND_SIZE
    // SOUND_COMMAND_SIZE is exactly 8 bytes long make is possible to have a size of 99_999_999 (deemed large enough)
    let mut buf: [u8; 8] = [0; 8];
    file.read(&mut buf).unwrap();
    file.rewind().unwrap();

    let sound_command_size = str::from_utf8(&buf)
        .expect("unexpected end of file the last 8 bits must be a trimed number")
        .trim()
        .parse::<u32>()
        .expect("unexpected end of file the last 8 bits must be a trimed number");

    // Seek the file to the appropriate location
    let location = -(sound_command_size as i64 + 1) - 8;
    file.seek(std::io::SeekFrom::End(location)).unwrap();

    let mut buf = vec![0u8; sound_command_size as usize];
    file.read_exact(&mut buf).unwrap();

    let sound_command = String::from_utf8(buf).unwrap();
    println!("{sound_command}");

    // We initialize the state of the binary
    let mut state = State { interval: 0.0 };

    let host = cpal::default_host();
    let devices = host.devices().unwrap();
    devices.for_each(|d| println!("{:?}", d.name()));

    let devices = host.devices().unwrap();
    let device = devices
        .filter(|d| d.name().unwrap() == "MacBook Air Speakers")
        .next()
        .unwrap();
    println!("{:?}", device.name());

    let (_, stream_handle) = OutputStream::try_from_device(&device).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // We parse the command in order to utilize each command
    sound_command
        .split(";")
        .map(|c| {
            let (r#type, value) = c.split_once(" ").unwrap();

            match r#type {
                "interval" => Command::Interval(value.parse::<f32>().unwrap()),
                "sleep" => Command::Sleep(value.parse::<f32>().unwrap()),
                "note" => Command::Note(value.to_string()),
                _ => unreachable!(),
            }
        })
        .for_each(|c| c.execute(&sink, &mut state));

    let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.5));
    sink.append(source);

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(1.0))
        .amplify(0.0);
    sink.append(source);

    sink.sleep_until_end();
}
