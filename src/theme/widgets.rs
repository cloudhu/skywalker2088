//! Helper traits for creating common widgets.

use crate::theme::{interaction::InteractionPalette, language::LocalizeText, palette::*};
use bevy::prelude::Val::{Percent, Px};
use bevy::prelude::{
    default, AlignItems, BackgroundColor, BuildChildren, Bundle, Button, ChildBuild, ChildBuilder,
    Commands, EntityCommands, FlexDirection, Font, Handle, JustifyContent, Name, Node,
    PositionType, Text, TextColor, TextFont,
};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, key: impl Into<String>, font: Handle<Font>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, key: impl Into<String>, font: Handle<Font>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, key: impl Into<String>, font: Handle<Font>) -> EntityCommands;
    fn content(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, keyword: impl Into<String>, font: Handle<Font>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            Button,
            Node {
                width: Px(200.0),
                height: Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NODE_BACKGROUND),
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_child((
            Name::new("Button Text"),
            Text::new(""),
            TextFont {
                font,
                font_size: 40.0,
                ..default()
            },
            TextColor(BUTTON_TEXT),
            LocalizeText::from_section(keyword),
        ));

        entity
    }

    fn header(&mut self, keyword: impl Into<String>, font: Handle<Font>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Header"),
            Node {
                width: Px(500.0),
                height: Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NODE_BACKGROUND),
        ));
        entity.with_child((
            Name::new("Header Text"),
            Text::new(""),
            TextFont {
                font,
                font_size: 40.0,
                ..default()
            },
            TextColor(HEADER_TEXT),
            LocalizeText::from_section(keyword),
        ));
        entity
    }

    fn label(&mut self, keyword: impl Into<String>, font: Handle<Font>) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Label"),
            Text::new(""),
            Node {
                width: Px(500.0),
                ..default()
            },
            TextFont {
                font,
                font_size: 24.0,
                ..default()
            },
            TextColor(LABEL_TEXT),
            LocalizeText::from_section(keyword),
        ));
        entity
    }

    fn content(&mut self, text: impl Into<String>) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Label"),
            Text::new(text),
            Node {
                width: Px(500.0),
                ..default()
            },
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(LABEL_TEXT),
        ));
        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}
