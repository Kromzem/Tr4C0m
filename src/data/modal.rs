use std::collections::{HashMap, HashSet};

use serenity::all::{ActionRow, ActionRowComponent, ModalInteractionData};

pub struct ModalData {
    pub id: String,
    pub inputs: HashMap<String, String>,
}

impl ModalData {
    pub fn parse(data: ModalInteractionData) -> ModalData {
        ModalData {
            id: data.custom_id,
            inputs: parse_modal(data.components),
        }
    }
}

fn parse_modal(components: Vec<ActionRow>) -> HashMap<String, String> {
    components
        .into_iter()
        .filter_map(|row| {
            if let Some(ActionRowComponent::InputText(input_text)) =
                row.components.into_iter().nth(0)
            {
                Some((
                    input_text.custom_id,
                    input_text.value.unwrap_or("".to_string()),
                ))
            } else {
                None
            }
        })
        .collect()
}
