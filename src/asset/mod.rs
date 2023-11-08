//! This module defines all assets for 2D Animations.
//!

use bevy::{
    prelude::{App, Asset, AssetApp, Handle, Image, Plugin},
    reflect::TypePath,
    sprite::TextureAtlas,
    utils::HashMap,
};

use self::asset_loader::Animation2DLoader;

pub mod asset_loader;

/// Adds support for spritesheet animation manifest files loading to the app.
pub struct Animation2DLoaderPlugin;

impl Plugin for Animation2DLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationClip2D>()
            .init_asset::<AnimationClipSet2D>()
            .init_asset_loader::<Animation2DLoader>();
    }
}

/// Keyframes for a 2D animation.
#[derive(Debug)]
pub enum Keyframes2D {
    /// For SpriteSheet animations this contains the [`TextureAtlas`](bevy::sprite::TextureAtlas) [`Handle`](bevy::asset::Handle) and an ordered list of indices.
    SpriteSheet(Handle<TextureAtlas>, Vec<usize>),
    /// For Sprite animations this contains an ordered list of [`Image`](bevy::render::texture::Image) [`Handle`](bevy::asset::Handle)s.
    Sprite(Vec<Handle<Image>>),
}

impl Default for Keyframes2D {
    fn default() -> Self {
        Self::Sprite(vec![])
    }
}

/// AnimationClip for a 2D animation.
#[derive(Asset, TypePath, Debug)]
pub struct AnimationClip2D {
    /// Timestamps for each keyframe in seconds.
    keyframe_timestamps: Vec<f32>,
    /// An ordered list of incides of the TextureAtlas or Images that represent the frames of this animation.
    keyframes: Keyframes2D,
    /// Total duration of this animation clip in seconds.
    duration: f32,
}

impl AnimationClip2D {
    /// Timestamps for each keyframe in seconds.
    #[inline]
    pub fn keyframe_timestamps(&self) -> &[f32] {
        &self.keyframe_timestamps
    }

    /// Ordered list of [`Keyframes2D`] elements for this animation.
    #[inline]
    pub fn keyframes(&self) -> &Keyframes2D {
        &self.keyframes
    }

    /// Total duration of this animation clip in seconds.
    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }
}

/// AnimationClipSet for 2D animations.
#[derive(Asset, TypePath, Debug)]
pub struct AnimationClipSet2D {
    /// Optional name of this animation set.
    name: Option<String>,
    /// A map of all animations in this set, identified by their names.
    animations: HashMap<String, Handle<AnimationClip2D>>,
}

impl AnimationClipSet2D {
    /// Gets the name of this animation set.
    ///
    /// Returns `None` if no name was set.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// HashMap of list of [`AnimationClip2D`] [`Handle`](bevy::asset::Handle)s. Indexed by the animation clip name.
    #[inline]
    pub fn animations(&self) -> &HashMap<String, Handle<AnimationClip2D>> {
        &self.animations
    }
}
