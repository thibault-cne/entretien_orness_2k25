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

fn main() {
    //    let commands: Vec<Command> = SOUND
    //        .split(";")
    //        .map(|c| {
    //            let (r#type, value) = c.split_once(" ").unwrap();
    //
    //            match r#type {
    //                "interval" => Command::Interval(value.parse::<f32>().unwrap()),
    //                "sleep" => Command::Sleep(value.parse::<f32>().unwrap()),
    //                "note" => Command::Note(value.to_string()),
    //                _ => unreachable!(),
    //            }
    //        })
    //        .collect();

    // let host = cpal::default_host();
    // let mut devices = host.devices().unwrap();
    // // devices.for_each(|d| println!("{:?}", d.name()));
    // let first_device = devices.skip(4).next().unwrap();
    // println!("{:?}", first_device.name());

    // let (_, stream_handle) = OutputStream::try_from_device(&first_device).unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.5));
    // sink.append(source);

    // let source = SineWave::new(440.0)
    //     .take_duration(Duration::from_secs_f32(1.0))
    //     .amplify(0.0);
    // sink.append(source);

    // sink.sleep_until_end();

    let n = Command::Note("E5".to_string());

    println!("{:?}", n.calc_freq());
}

// command value; command value; ...
const SOUND: &str = "";
