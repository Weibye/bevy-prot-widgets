//! A counter widget that can be increased or decreased

use bevy::{prelude::{Entity, Component, Query, EventReader, Changed}, text::Text};

use super::button::ButtonEvent;

#[derive(Component)]
pub struct CounterWidget {
    pub count: i32,
    pub increase: Entity,
    pub decrease: Entity
}

pub(crate) fn counter_interact(mut q: Query<&mut CounterWidget>, mut reader: EventReader<ButtonEvent>) {
    for event in reader.iter() {
        for mut counter in &mut q {
            counter.count += 
                if counter.increase == event.0 { 1 } 
                else if counter.decrease == event.0 { -1 }
                else { 0 };
        }
    }
}

/// Update the text whenever the counter changes
pub(crate) fn update_counter(mut q: Query<(&mut Text, &CounterWidget), Changed<CounterWidget>>) {
    for (mut text, counter) in &mut q {
        text.sections[0].value = counter.count.to_string();
    }
}