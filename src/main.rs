use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;

fn check_asset_handles(asset_server: Res<AssetServer>, textures: Res<TextureAssets>) {
    println!(
        "AssetStates: player {:?}, tree {:?}",
        asset_server.get_load_state(textures.player.id),
        asset_server.get_load_state(textures.tree.id)
    )
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handle_ids = vec![];
    handle_ids.push(asset_server.load_untyped("textures/player.png").id);
    handle_ids.push(asset_server.load_untyped("textures/tree.png").id);

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

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Load)
        .add_system_set(SystemSet::on_enter(AppState::Load).with_system(start_loading.system()))
        .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_loading.system()))
        .add_system_set(
            SystemSet::on_update(AppState::Game).with_system(check_asset_handles.system()),
        )
        .run();
}
