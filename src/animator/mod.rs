use bevy::{
    asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Animation>()
            .init_asset_loader::<AnimationLoader>()
            .add_systems(Update, animate);
    }
}

// Create the animation asset
#[derive(TypeUuid, TypePath, Deref)]
#[uuid = "ae6a74db-f6fa-43c4-ac16-01d13b50e4c6"]
struct Animation(HashMap<String, benimator::Animation>);

// Create the animation state component
#[derive(Default, Component)]
struct AnimationPlayer {
    pub is_playing: bool,
    pub animate_name: String,
    pub(crate) state: benimator::State,
}

impl AnimationPlayer {
    pub fn play(&mut self, animate_name: &str) {
        self.animate_name = animate_name.to_owned()
    }
}

// Create (and implement) the asset loader
#[derive(Default)]
struct AnimationLoader;

impl AssetLoader for AnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            // `Animation` implements `serde::Deserialize`,
            // so you may use any serde-compatible deserialization library
            let animation: Animation = Animation(serde_json::from_slice(bytes)?);
            load_context.set_default_asset(LoadedAsset::new(animation));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["animation.json"]
    }
}

fn animate(
    time: Res<Time>,
    animations: Res<Assets<Animation>>,
    mut query: Query<(
        &mut AnimationPlayer,
        &mut TextureAtlasSprite,
        &Handle<Animation>,
    )>,
) {
    for (mut player, mut texture, handle) in query.iter_mut() {
        // Get the animation from handle (or skip this entity if not yet loaded)
        let animation = match animations
            .get(handle)
            .and_then(|animation| animation.get(&player.animate_name))
        {
            Some(anim) => anim,
            None => continue,
        };

        // Update the state
        player.state.update(animation, time.delta());

        player.is_playing = !player.state.is_ended();

        // Update the texture atlas
        texture.index = player.state.frame_index();
    }
}
