
pub enum AudioEventType {
  CharacterAttack,
	CharacterHit,
	WallHit,
	WallBreak,
}
pub struct AudioEvent(AudioEventType);


