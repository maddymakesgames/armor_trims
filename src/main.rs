use quartz_nbt::{
    io::Flavor,
    serde::{deserialize_from, serialize_into},
    NbtCompound, NbtTag,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
};

const MATERIALS: [&'static str; 10] = [
    "minecraft:iron",
    "minecraft:copper",
    "minecraft:gold",
    "minecraft:netherite",
    "minecraft:emerald",
    "minecraft:diamond",
    "minecraft:redstone",
    "minecraft:quartz",
    "minecraft:lapis",
    "minecraft:amethyst",
];

const ARMOR_MATERIALS: [&'static str; 6] = [
    "minecraft:chainmail",
    "minecraft:iron",
    "minecraft:golden",
    "minecraft:diamond",
    "minecraft:netherite",
    "minecraft:turtle",
];

const ARMOR_TYPES: [&'static str; 4] = ["_helmet", "_chestplate", "_leggings", "_boots"];

const TRIMS: [&'static str; 11] = [
    "minecraft:wild",
    "minecraft:ward",
    "minecraft:vex",
    "minecraft:tide",
    "minecraft:spire",
    "minecraft:snout",
    "minecraft:sentry",
    "minecraft:rib",
    "minecraft:eye",
    "minecraft:dune",
    "minecraft:coast",
];

#[derive(Serialize, Deserialize, Clone)]
struct Format {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    blocks: Vec<Block>,
    entities: Vec<Entity>,
    palette: NbtTag,
    size: NbtTag,
}

#[derive(Serialize, Deserialize, Clone)]
struct Block {
    pos: [i32; 3],
    state: i32,
    nbt: Option<BlockNbt>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BlockNbt {
    #[serde(rename = "Items")]
    items: Vec<ChestItem>,
    id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ChestItem {
    #[serde(rename = "Count")]
    count: i8,
    #[serde(rename = "Slot")]
    slot: i8,
    id: String,
    tag: ArmorItemTag,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Entity {
    #[serde(rename = "blockPos")]
    block_pos: [i32; 3],
    pos: [f64; 3],
    nbt: ArmorStandNbt,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ArmorStandNbt {
    #[serde(rename = "AbsorptionAmount")]
    absorption_amount: f32,
    #[serde(rename = "Air")]
    air: i16,
    #[serde(rename = "ArmorItems")]
    armor_items: Vec<ArmorWrapper<ArmorItems>>,
    #[serde(rename = "Attributes")]
    attributes: NbtTag,
    #[serde(rename = "Brain")]
    brain: NbtTag,
    #[serde(rename = "DeathTime")]
    death_time: i16,
    #[serde(rename = "DisabledSlots")]
    disabled_slots: i32,
    #[serde(rename = "FallDistance")]
    fall_distance: f32,
    #[serde(rename = "FallFlying")]
    fall_flying: bool,
    #[serde(rename = "Fire")]
    fire: i16,
    #[serde(rename = "HandItems")]
    hand_items: NbtTag,
    #[serde(rename = "Health")]
    health: f32,
    #[serde(rename = "HurtByTimestamp")]
    hurt_by_timestamp: i32,
    #[serde(rename = "HurtTime")]
    hurt_time: i16,
    #[serde(rename = "Invisible")]
    invisible: bool,
    #[serde(rename = "Invulnerable")]
    invulnerable: bool,
    #[serde(rename = "Motion")]
    motion: [f64; 3],
    #[serde(rename = "NoBasePlate")]
    no_base_plate: bool,
    #[serde(rename = "OnGround")]
    on_ground: bool,
    #[serde(rename = "PortalCooldown")]
    portal_cooldown: i32,
    #[serde(rename = "Pos")]
    pos: [f64; 3],
    #[serde(rename = "Pose")]
    pose: NbtTag,
    #[serde(rename = "Rotation")]
    rotation: [f32; 2],
    #[serde(rename = "ShowArms")]
    show_arms: bool,
    #[serde(rename = "Small")]
    small: bool,
    #[serde(rename = "UUID")]
    uuid: [i32; 4],
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ArmorWrapper<T: Serialize + Debug> {
    Armor(T),
    Empty(NbtCompound),
}
impl<T: Serialize + Debug> ArmorWrapper<T> {
    fn as_ref(&self) -> ArmorWrapper<&T> {
        match self {
            Self::Armor(a) => ArmorWrapper::Armor(a),
            Self::Empty(e) => ArmorWrapper::Empty(e.clone()),
        }
    }

    fn as_mut(&mut self) -> ArmorWrapper<&mut T> {
        match self {
            Self::Armor(a) => ArmorWrapper::Armor(a),
            Self::Empty(e) => ArmorWrapper::Empty(e.clone()),
        }
    }

    fn unwrap(self) -> T {
        match self {
            Self::Armor(a) => a,
            Self::Empty(_e) => panic!("Uh oh"),
        }
    }
}

impl<T: Clone + Serialize + Debug> Clone for ArmorWrapper<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Armor(a) => ArmorWrapper::Armor(a.clone()),
            Self::Empty(e) => ArmorWrapper::Empty(e.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ArmorItems {
    #[serde(rename = "Count")]
    count: i8,
    id: String,
    tag: ArmorItemTag,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ArmorItemTag {
    #[serde(rename = "Damage")]
    damage: i32,
    #[serde(rename = "Trim")]
    trim: ArmorTrim,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ArmorTrim {
    material: String,
    pattern: String,
}

fn main() -> anyhow::Result<()> {
    let mut file = OpenOptions::new().read(true).open("./armor.nbt")?;

    let (nbt, _root_name) = deserialize_from::<File, Format>(&mut file, Flavor::GzCompressed)?;

    for armor_material_idx in 0..ARMOR_MATERIALS.len() {
        let mut structure = nbt.clone();
        let entities = &mut structure.entities;
        let blocks = &mut structure.blocks;
        for trim_idx in 0..TRIMS.len() {
            for material_idx in 0..MATERIALS.len() {
                let entity = entities
                    .iter_mut()
                    .find(|e| {
                        e.block_pos[1] == (material_idx as i32 * 4) + 1
                            && e.block_pos[2] == trim_idx as i32
                    })
                    .unwrap();
                let block = blocks
                    .iter_mut()
                    .find(|e| {
                        e.pos[1] == (material_idx as i32 * 4) + 1
                            && e.pos[2] == trim_idx as i32
                            && e.nbt.is_some()
                    })
                    .unwrap();

                for armor_type_idx in 0..ARMOR_TYPES.len() {
                    let block_nbt = block.nbt.as_mut().unwrap();
                    let chest_armor_nbt = block_nbt
                        .items
                        .iter_mut()
                        .find(|i| i.id.contains(ARMOR_TYPES[armor_type_idx]))
                        .unwrap();
                    chest_armor_nbt.id = format!(
                        "{}{}",
                        ARMOR_MATERIALS[armor_material_idx], ARMOR_TYPES[armor_type_idx]
                    );
                    chest_armor_nbt.tag.trim.material = MATERIALS[material_idx].to_owned();
                    chest_armor_nbt.tag.trim.pattern = TRIMS[trim_idx].to_owned();

                    let armor_item_nbt = entity
                        .nbt
                        .armor_items
                        .iter_mut()
                        .find(|i| i.as_ref().unwrap().id.contains(ARMOR_TYPES[armor_type_idx]))
                        .unwrap()
                        .as_mut()
                        .unwrap();
                    armor_item_nbt.id = format!(
                        "{}{}",
                        ARMOR_MATERIALS[armor_material_idx], ARMOR_TYPES[armor_type_idx]
                    );
                    armor_item_nbt.tag.trim.material = MATERIALS[material_idx].to_owned();
                    armor_item_nbt.tag.trim.pattern = TRIMS[trim_idx].to_owned();

                    if armor_material_idx == 5 {
                        println!("{:?} {block_nbt:?}", entity.nbt.armor_items);
                        entity.nbt.armor_items.iter_mut().for_each(|e| {
                            if e.as_ref().unwrap().id != "minecraft:turtle_helmet" {
                                *e = ArmorWrapper::Empty(NbtCompound::new())
                            }
                        });
                        block_nbt
                            .items
                            .retain(|i| i.id == "minecraft:turtle_helmet");
                        break;
                    }
                }
            }
        }

        let mut file = OpenOptions::new().write(true).create(true).open(format!(
            "./armor_{}.nbt",
            ARMOR_MATERIALS[armor_material_idx]
                .split(':')
                .nth(1)
                .unwrap()
        ))?;

        serialize_into(&mut file, &structure, None, Flavor::GzCompressed)?;
    }

    Ok(())
}
