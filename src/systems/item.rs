use crate::prelude::*;

#[system(for_each)]
#[read_component(Item)]
#[read_component(ItemReceived)]
#[read_component(AmuletOfYala)]
pub fn item(
    message_entity: &Entity,
    item_received: &ItemReceived,
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    println!("Receiving item!");

    if ecs.entry_ref(item_received.item)
        .unwrap()
        .get_component::<AmuletOfYala>()
        .is_ok()
    {
        *turn_state = TurnState::Victory
    }

    // maybe get power-ups and other stuff here!!!

    //if maybe some items remain.....
    commands.remove(item_received.item);
    //

    commands.remove(*message_entity);
}
