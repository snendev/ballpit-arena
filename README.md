# BallPit Arena

The ballpit is a war zone.
Fight for survival as the walls collapse around you, and rack up the score while you do!

### Controls

Ballpit Arena is a rudimentary platform fighter, with breakable blocks surrounding the map.
When a character leaves the stage, it dies.
Each character has a Hype (good) and a Combo (bad) counter, which change how fast attacks launch opponents.
These values can be inferred by the levels of blue and red in the player's outline color:
- A blue border implies that the character has gained Hype.
- A red border => Combo
- A purple border => both

Control with either a gamepad (xbox controls used below) or keyboard.
Transform into different shapes to attack your foes.

Move: analog / wasd
Jump: (A) / space - Jump (you get two, any wall contact resets)
Triangle: (X) / q - Turn into a triangle and hit stuff near your tip
Square: (B) / e - Turn into a square and hit stuff
Octogon: (Y) / r - Counter (no damage on hit, cancellable)

### About This Game

This project is a Ludum Dare 50 Compo submission attempt that fell short in gameplay engineering time.

It was built using [Bevy](https://bevyengine.org/), an ECS-based game development engine written in Rust.
I had not built a project with Bevy before, and aimed to build a couple of unique features such as the audio
generation, so these presented challenges in the development process.
Still, I am impressed by the engine's capabilities, and I thoroughly enjoyed using the engine.

Much time was spent on the audio engine, which dynamically generates waveforms.
This was inspired by the Pentatonic Sounds feature on lichess.org.
Unfortunately, performance and time limitations made this difficult to do well.
This work was heavily supported by repositories made by the RustAudio group, such as pitch_calc and kira, as well as the bevy_kira_audio plugin, which has been forked and checked into the repository with some small edits.

All rendering is performed using bevy_prototype_lyon, a simple shape-rendering library plugin for Bevy.

A simple physics system handles most of the action, which makes it liable to certain bugs ("avoiding the inevitable").
Replacing home-brew with Rapier would be a positive future direction.

I do not intend to work on this game further beyond bugfixes when possible, but if you're curious to play with it, feel free.
Next steps include building a proper AssetHandle / proxy for managing the audio buffer via the Resource Manager, reorganizing the physics via Rapier.
