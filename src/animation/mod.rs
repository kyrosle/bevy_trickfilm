//! This module handles playing animations from an ['AnimationClip2D'](crate::asset::AnimationClip2D) asset using the ['AnimationPlayer2D'](crate::animation::AnimationPlayer2D) component.
//!

mod animation_spritesheet;

use crate::prelude::AnimationClip2D;
use bevy::{
    animation::RepeatAnimation,
    prelude::{App, Component, Handle, Plugin, ReflectComponent, Update},
    reflect::Reflect,
};

use self::animation_spritesheet::animation_player_spritesheet;

/// Adds support for spritesheet animation playing.
pub struct AnimationPlayer2DPlugin;

impl Plugin for AnimationPlayer2DPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimationPlayer2D>()
            .add_systems(Update, animation_player_spritesheet);
    }
}

#[derive(Reflect)]
struct PlayingAnimation2D {
    repeat: RepeatAnimation,
    reverse: bool,
    clip_finished: bool,
    speed: f32,
    elapsed: f32,
    seek_time: f32,
    animation_clip: Handle<AnimationClip2D>,
    completions: u32,
}

impl Default for PlayingAnimation2D {
    fn default() -> Self {
        Self {
            repeat: Default::default(),
            reverse: false,
            clip_finished: false,
            speed: 1.0,
            elapsed: 0.0,
            seek_time: 0.0,
            animation_clip: Default::default(),
            completions: 0,
        }
    }
}

impl PlayingAnimation2D {
    /// Check if the animation has finished, based on its repetition behavior and the number of times it has repeated.
    ///
    /// Note: An animation with `RepeatAnimation::Forever` will never finish.
    #[inline]
    pub fn is_finished(&self) -> bool {
        match self.repeat {
            RepeatAnimation::Forever => false,
            RepeatAnimation::Never => self.completions >= 1,
            RepeatAnimation::Count(n) => self.completions >= n,
        }
    }

    /// Update the animation given the delta time and the duration of the clip being played.
    #[inline]
    fn update(&mut self, delta: f32, clip_duration: f32) {
        if self.is_finished() {
            return;
        }

        let direction_multiplier = if self.reverse { -1.0 } else { 1.0 };
        self.elapsed += delta;
        self.seek_time += delta * self.speed * direction_multiplier;

        if self.seek_time >= clip_duration {
            self.seek_time %= clip_duration;
        } else if self.seek_time < 0.0 {
            self.seek_time += clip_duration;
        }

        if (self.reverse && self.seek_time <= 0.0) || (!self.reverse && self.seek_time >= clip_duration) {
            self.completions += 1;
            self.seek_time = if self.reverse { clip_duration } else { 0.0 };
        }
    }

    /// Reset back to the initial state as if no time has elapsed.
    fn replay(&mut self) {
        self.completions = 0;
        self.elapsed = 0.0;
        self.seek_time = 0.0;
    }
}

/// Animation controls
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct AnimationPlayer2D {
    paused: bool,
    animation: PlayingAnimation2D,
}

impl AnimationPlayer2D {
    /// Start playing an animation, resetting state of the player.
    pub fn start(&mut self, handle: Handle<AnimationClip2D>) -> &mut Self {
        self.animation = PlayingAnimation2D {
            animation_clip: handle,
            ..Default::default()
        };
        self
    }

    /// Start playing an animation, resetting state of the player, unless the requested animation is already playing.
    pub fn play(&mut self, handle: Handle<AnimationClip2D>) -> &mut Self {
        if self.animation.animation_clip != handle || self.is_paused() {
            self.start(handle);
        }
        self
    }

    /// Handle to the animation clip being played.
    pub fn animation_clip(&self) -> &Handle<AnimationClip2D> {
        &self.animation.animation_clip
    }

    /// Check if the given animation clip is being played.
    pub fn is_playing_clip(&self, handle: &Handle<AnimationClip2D>) -> bool {
        self.animation_clip() == handle
    }

    /// Check if the playing animation has finished, according to the repetition behavior.
    pub fn is_finished(&self) -> bool {
        self.animation.is_finished()
    }

    /// Check if the clip animation has finished, regardless of its clip behavior.
    pub fn is_clip_finished(&self) -> bool {
        self.animation.clip_finished
    }

    /// Play animation in reverse.
    pub fn reverse(&mut self) -> &mut Self {
        self.animation.reverse = true;
        self
    }

    /// Stop playing animation in reverse.
    pub fn stop_reverse(&mut self) -> &mut Self {
        self.animation.reverse = false;
        self
    }

    /// Is the animation playing in reverse
    pub fn is_reverse(&self) -> bool {
        self.animation.reverse
    }

    /// Sets repeat to [`RepeatAnimation::Forever`].
    ///
    /// See also [`Self::set_repeat`].
    pub fn repeat(&mut self) -> &mut Self {
        self.animation.repeat = RepeatAnimation::Forever;
        self
    }

    /// Set the repetition behaviour of the animation.
    pub fn set_repeat(&mut self, repeat: RepeatAnimation) -> &mut Self {
        self.animation.repeat = repeat;
        self
    }

    /// Repetition behavior of the animation.
    pub fn repeat_mode(&self) -> RepeatAnimation {
        self.animation.repeat
    }

    /// Number of times the animation has completed.
    pub fn completions(&self) -> u32 {
        self.animation.completions
    }

    /// Check if the animation is playing in reverse.
    pub fn is_playback_reversed(&self) -> bool {
        self.animation.speed < 0.0
    }

    /// Pause the animation.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Unpause the animation
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Is the animation paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Speed of the animation playback
    pub fn speed(&self) -> f32 {
        self.animation.speed
    }

    /// Set the speed of the animation playback
    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.animation.speed = speed;
        self
    }

    /// Time elapsed playing the animation
    pub fn elapsed(&self) -> f32 {
        self.animation.elapsed
    }

    /// Seek time inside of the animation. Always within the range [0.0, clip duration].
    pub fn seek_time(&self) -> f32 {
        self.animation.seek_time
    }

    /// Seek to a specific time in the animation.
    pub fn seek_to(&mut self, seek_time: f32) -> &mut Self {
        self.animation.seek_time = seek_time;
        self
    }

    /// Reset the animation to its initial state, as if no time has elapsed.
    pub fn replay(&mut self) {
        self.animation.replay();
    }
}
