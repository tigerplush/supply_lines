use bevy::app::plugin_group;

plugin_group! {
    pub struct AppPlugins {
        main_state:::MainStatePlugin,
        splashscreen:::SplashscreenPlugin,
        states:::StatesPlugin,
    }
}
