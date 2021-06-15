use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;
use std::time::Duration;

fn check_asset_handles(
    asset_server: Res<AssetServer>,
    audio: Res<AudioAssets>,
    textures: Res<TextureAssets>,
) {
    println!(
        "AssetStates: audio {:?}, player {:?}, tree {:?}",
        asset_server.get_load_state(audio.background.id),
        asset_server.get_load_state(textures.player.id),
        asset_server.get_load_state(textures.tree.id)
    )
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
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        audio.play(audio_assets.background.clone());
    }
}

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
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(play_audio.system())
                .with_system(check_asset_handles.system()),
        )
        .run();
}
