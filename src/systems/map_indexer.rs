use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Entity)]
pub fn map_indexer(
    ecs: &SubWorld,
    #[resource]map: &mut Map,
) {
    map.clear_content_index();
    <(Entity, &Point)>::query().iter(ecs).for_each(|(entity, pos)| {
        let idx = Map::map_idx(pos.x as usize,pos.y as usize);
        map.tile_contents[idx].push(*entity);
    });
}