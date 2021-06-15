use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;
use std::time::Duration;

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handle_ids = vec![];
    handle_ids.push(asset_server.load_untyped("textures/player.png").id);
    handle_ids.push(asset_server.load_untyped("textures/tree.png").id);
    handle_ids.push(asset_server.load_untyped("audio/background.ogg").id);

    commands.insert_resource(LoadingHandles {
        handles: handle_ids,
    });
}

fn check_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
    loading_handles: Res<LoadingHandles>,
) {
    if asset_server.get_group_load_state(loading_handles.handles.clone()) == LoadState::Loaded {
        commands.insert_resource(AudioAssets {
            background: asset_server.get_handle("audio/background.ogg"),
        });
        commands.insert_resource(TextureAssets {
            player: asset_server.get_handle("textures/player.png"),
            tree: asset_server.get_handle("textures/tree.png"),
        });
        commands.remove_resource::<LoadingHandles>();
        state.set(AppState::Game).expect("Failed to set next State");
    }
}

fn spawn(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.player.clone().into()),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.tree.clone().into()),
        transform: Transform::from_translation(Vec3::new(100., 10., 0.)),
        ..Default::default()
    });
}

fn play_audio(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut timer: Local<AudioTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    timer.timer.tick(time.delta());
    if asset_server.get_load_state(audio_assets.background.clone()) != LoadState::Loaded {
        println!(
            "audio handle not loaded! Has state: {:?}",
            asset_server.get_load_state(audio_assets.background.clone())
        );
    } else {
        println!("Audio is loaded");
    }
    if timer.timer.just_finished() {
        audio.play(audio_assets.background.clone());
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Load,
    Game,
}

struct LoadingHandles {
    handles: Vec<HandleId>,
}

struct TextureAssets {
    tree: Handle<Texture>,
    player: Handle<Texture>,
}

struct AudioAssets {
    background: Handle<AudioSource>,
}

struct AudioTimer {
    timer: Timer,
}

impl Default for AudioTimer {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::from_secs(31), true);
        timer.tick(Duration::from_millis(30999));
        Self { timer }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Load)
        .add_system_set(SystemSet::on_enter(AppState::Load).with_system(start_loading.system()))
        .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_loading.system()))
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn.system()))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(play_audio.system()))
        .run();
}
