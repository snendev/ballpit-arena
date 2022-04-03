use bevy::{prelude::*, utils::Uuid};
use bevy_kira_audio::{
    AudioPlugin as KiraAudioPlugin,
    AudioStreamPlugin,
    StreamedAudio,
    AudioSource,
    Audio,
    AudioStream,
    Frame,
    AudioChannel,
};
use dasp::{Sample, signal, Signal};
use pitch_calc::{
    Letter,
    LetterOctave,
    Step,
    hz_from_step,
    step_from_letter_octave,
};

#[derive(Debug)]
pub enum Trigger {
    CharacterJump,
    CharacterAttack,
	CharacterHit,
	WallBreak,
}

#[derive(Debug)]
pub struct Offset(pub f32);

#[derive(Debug)]
pub struct Event(pub Entity, pub Trigger, pub Offset);

const SAMPLE_RATE: usize = 44_100;

#[derive(Debug, Default)]
pub struct OutputStream(Vec<Frame>);

// models a synth after https://github.com/RustAudio/dasp/blob/master/examples/synth.rs
fn build_synth_stream(pitch: f64, amplitude: f64) -> impl Iterator<Item = Frame> {
    let one_sec = SAMPLE_RATE;
    let fundamental = signal::rate(SAMPLE_RATE as f64).const_hz(pitch);
    let harmonic_l1 = signal::rate(SAMPLE_RATE as f64).const_hz(pitch * 2.);
    let harmonic_h1 = signal::rate(SAMPLE_RATE as f64).const_hz(pitch / 2.);
    let harmonic_h2 = signal::rate(SAMPLE_RATE as f64).const_hz(pitch / 4.);

    fundamental.clone().sine()
        .add_amp(harmonic_l1.sine().scale_amp(0.3))
        .add_amp(harmonic_h1.sine().scale_amp(0.2))
        .add_amp(harmonic_h2.sine().scale_amp(0.08))
        .scale_amp(amplitude)
        .take(one_sec / 2)
        .zip(0..(one_sec / 2))
        .map(|(s, index)| {
            let damp: f32 = (index as f32) / (2. * SAMPLE_RATE as f32);
            let sample = s.to_sample::<f32>() * 0.2;
            let value = sample * damp;
            Frame {
                left: value,
                right: value,
            }
        })
}

impl AudioStream for OutputStream {
    fn next(&mut self, dt: f64) -> Frame {
        self.0.pop().unwrap_or_else(|| Frame { left: 0., right: 0.})
    }
}

pub fn handle_audio_cleanup(
    mut channels: ResMut<AudioChannelsBuffer>,
    audio: Res<StreamedAudio<OutputStream>>,
    time: Res<Time>,
) {
    let drain = channels.0.drain_filter(|mut item| {
        item.1 -= time.delta().as_secs_f64();
        item.1 <= 0.
    });
    for removed_item in drain {
        audio.stop_channel(
            &AudioChannel::new(removed_item.0.to_string().to_owned()),
        )
    }
}

pub fn handle_audio_event(
    mut events: EventReader<Event>,
    mut channels: ResMut<AudioChannelsBuffer>,
    audio: Res<StreamedAudio<OutputStream>>,
) {
    let tag = Uuid::new_v4();
    if events.iter().count() + channels.0.len() > 40 {
        info!("{}, {}", events.iter().count(), channels.0.len());
        return
    }
    let streams: Vec<Vec<Frame>> = events.iter()
        .map(|event| {
            let step = step_from_letter_octave(Letter::C, 4);
            // 10 steps per fifth
            let pentatonic_step = event.2.0 * 10.;
            let hz = hz_from_step(step + event.2.0);
            match event.1 {
                Trigger::CharacterJump => {
                    build_synth_stream(hz.into(), 0.25).collect()
                }
                Trigger::CharacterAttack => {
                    build_synth_stream(hz.into(), 0.8).collect()
                }
                Trigger::CharacterHit => {
                    build_synth_stream(hz.into(), 0.6).collect()
                }
                Trigger::WallBreak => {
                    build_synth_stream(hz.into(), 0.4).collect()
                }
            }
        })
        .collect();
    for synth in streams {
        let output = OutputStream(synth);
        channels.0.push((tag, 2.));
        audio.stream_in_channel(
            output,
            &AudioChannel::new(tag.to_string().to_owned()),
        );
    }
}

#[derive(Default)]
pub struct AudioChannelsBuffer(Vec<(Uuid, f64)>);

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioChannelsBuffer>()
            .add_event::<Event>()
            .add_plugin(KiraAudioPlugin)
            .add_plugin(AudioStreamPlugin::<OutputStream>::default())
            .add_system(handle_audio_event)
            .add_system(handle_audio_cleanup);
    }
}
