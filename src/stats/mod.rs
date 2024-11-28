use bevy::app::App;
pub mod historical_metrics;
pub mod metrics;
mod scanner;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((metrics::plugin,scanner::plugin,));
}
