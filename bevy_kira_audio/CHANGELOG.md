# Changelog

## v0.8.0
- Update to Bevy version 0.6

## v0.7.0
- The playback position of audio can be requested from the `Audio` resource
- Update to Rust edition 2021
- Removed direct dependencies on bevy sub crates

## v0.6.0
- Relicense under dual MIT or Apache-2.0
- Clean up stopped instances
- "ogg" is now a default feature 
- No longer panic when no Audio device can be found (analogue to bevy/audio)
- Files can be loaded with a semantic duration (see [the example](examples/semantic_duration.rs))
- The plugin will no longer compile if none of the features "mp3", "ogg", "wav", or "flac" are set
- Allow playing looped sounds with an intro
