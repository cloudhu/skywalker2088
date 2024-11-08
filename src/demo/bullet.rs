// use bevy::prelude::*;
// use crate::AppSet;
// use crate::demo::animation::PlayerAnimation;
// use crate::demo::player::PlayerAssets;
//
// pub(super) fn plugin(app: &mut App) {
//     // Animate and play sound effects based on controls.
//     app.register_type::<PlayerAnimation>();
//     app.add_systems(
//         Update,
//         (
//             ()
//                 .chain()
//                 .run_if(resource_exists::<PlayerAssets>)
//                 .in_set(AppSet::Update),
//         ),
//     );
// }
//
