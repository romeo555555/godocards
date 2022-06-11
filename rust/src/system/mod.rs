use bevy_ecs::prelude::World;
use common::{Event, Message};

use crate::{
    component::{
        CharacterRect, DeckRect, EquipmentRect, FactoriesRect, HandRect, PlayerRect, TabelRect,
    },
    input::Sense,
    network::{self, Network},
};

pub fn input_system(world: &mut World, sense: Sense) {
    let mut q1 = world.query::<(
        &PlayerRect,
        &TabelRect,
        &HandRect,
        &EquipmentRect,
        &CharacterRect,
        &DeckRect,
        &FactoriesRect,
    )>();

    for (
        player_rect,
        tabel_rect,
        hand_rect,
        equipment_rect,
        character_rect,
        deck_rect,
        factories_rect,
    ) in q1.iter(&mut world)
    {
        // if sense.contains_rect(player_rect) {
        //     if sense.mouse_x > player_rect.rect.center_x {
        //         if sense.contains_rect(&self.items.rect) {
        //             return ResponseType::Items;
        //         } else if sense.contains_rect(&self.character.rect) {
        //             return ResponseType::Character;
        //         }
        //     } else if sense.contains_rect(&self.deck.rect) {
        //         return ResponseType::Deck;
        //     } else if sense.contains_rect(&self.builds.rect) {
        //         return ResponseType::Builds;
        //     }
        //     if self.hand.contains(&sense) {
        //         if let Some(exclude_card) = exclude_card {
        //             if let Some(card_id) =
        //                 self.hand
        //                     .input_handler_witch_exclude(sense, card_size, exclude_card)
        //             {
        //                 return ResponseType::HandCard(card_id);
        //             }
        //         }
        //         return if let Some(card_id) = self.hand.input_handler(sense, card_size) {
        //             ResponseType::HandCard(card_id)
        //         } else {
        //             ResponseType::Hand
        //         };
        //     } else if self.tabel.contains(&sense) {
        //         return if let Some(card_id) = self.tabel.input_handler(sense, card_size) {
        //             ResponseType::TabelCard(card_id)
        //         } else {
        //             ResponseType::Tabel
        //         };
        //     }
        //     ResponseType::None
        // }
    }
}

pub fn network_event_system(world: &mut World, network: &mut Network) {
    // if let Some(msg) = network.receive_event() {
    //     let Message { player_id, event } = msg.clone();
    //     // godot_print!("recive event : {:?}", event);
    //     match event {
    //         Event::TakeCard(card_id) => {
    //             // self.player_client
    //             //     .add_card_on_hand(res.create_card(owner));
    //             // let card_id = res.create_card(card_name.clone());
    //             // let side_player = self.get_side_player(commmand.player);
    //             // match commmand.line {
    //             //     LineType::Hand => side_player.hand.add_card(card_id),
    //             //     LineType::Tabel => side_player.tabel.add_card(card_id),
    //             //     _ => {}
    //             // }

    //             self.gui
    //                 .get_player(&player_id)
    //                 .add_card_on_hand(ctx.card_new(owner, card_id));
    //         }
    //         Event::CastCardOnTabel(card_id) => {
    //             // if dragged { drop without pos} else{ drop without target?}
    //             self.gui.drop(ctx);
    //             self.gui.get_player(&player_id).cast_on_tabel(card_id);
    //         }
    //         Event::BackCardOnHand(card_id) => {
    //             // self.side_client.back_on_hand(card_id)
    //         }
    //         Event::ManaUpdate(count, color) => {
    //             self.gui.get_player(&player_id).mana_update(count, color);
    //         }
    //         Event::FlipCard(card_id, hash_card) => {
    //             ctx.flip_card(owner, card_id, hash_card);
    //         }
    //         _ => {}
    //     }
    //     self.history.push(msg);
    // }
}
