use rom_res_rs::*;
use std::io::{Cursor, Read, Seek, SeekFrom};

use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use regex::{Regex};

const HUMAN_HEADER_SIZE: i64 = 0x014F;
const ITEM_HEADER_SIZE: i64 = 0xAD;
const MAGIC_ITEM_HEADER_SIZE: i64 = 0x23;
const PARAMETER_HEADER_SIZE: i64 = 0x123;
const SHAPE_HEADER_SIZE: i64 = 0x66;
const SPELL_HEADER_SIZE: i64 = 0x14E;
const STRUCTURE_HEADER_SIZE: i64 = 0x56;
const UNIT_HEADER_SIZE: i64 = 0x026B;
const WORLD_RES: &[u8] = include_bytes!("WORLD.RES");

#[derive(Default, Clone, PartialEq, Debug)]
struct CP866String(String);

impl Reflectable for CP866String {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_cp866_string(&mut self.0)
    }
}

#[derive(Default, Clone, PartialEq)]
struct U32Wrapper(u32);

impl Reflectable for U32Wrapper {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.0)
    }
}

#[derive(Default, Clone, PartialEq)]
struct U8Wrapper(u8);

impl Reflectable for U8Wrapper {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_u8(&mut self.0)
    }
}

fn look_ahead<Stream: Seek + Read>(stream: &mut Stream) -> u8 {
    let mut buf = [0u8; 1];
    stream.read(&mut buf).unwrap();
    stream.seek(SeekFrom::Current(-1)).unwrap();
    buf[0]
}

fn read_entry_count<Stream: Seek + Read>(stream: &mut Stream) -> u32 {
    let cnt = U32Wrapper::deserialize(stream, Endianness::LittleEndian).unwrap();
    cnt.0
}

fn read_corrected_entry_count<Stream: Seek + Read>(stream: &mut Stream) -> u32 {
    read_entry_count(stream) - 1
}

#[derive(PartialEq, Copy, Clone)]
enum SectionKind {
    Human,
    Item,
    MagicItem,
    Parameter,
    Shapes,
    Spell,
    Structure,
    Unit,
    Unknown,
}

#[derive(PartialEq, Default, Clone, Debug)]
struct HumanInfo {
    name: CP866String,
    details: HumanRecord,
    items_wearing: Vec<CP866String>,
}
impl HumanInfo {
    fn read_from_stream<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let human_unit_name_regexp =
            Regex::new(r"^(?:PC|NPC|NPC\d{1,3}|.|M\d{1,3}|Man.*)_.*")
                .unwrap();

        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        let name_string = CP866String::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        stream.seek(SeekFrom::Current(2)).unwrap();

        let human_rec = HumanRecord::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        let mut items_wearing = Vec::with_capacity(10);
        'item_loop: for _ in 0..10 {
            let look_ahead_v = look_ahead(stream);
            if look_ahead_v >= 128 || look_ahead_v == 0 {
                stream.seek(SeekFrom::Current(1)).unwrap();
                continue;
            }
            let textual_info = CP866String::deserialize(
                stream,
                Endianness::LittleEndian,
            ).unwrap();

            if human_unit_name_regexp.is_match(&textual_info.0) {
                stream.seek(SeekFrom::Current(-(textual_info.0.len() as i64 + 1))).unwrap();
                break;
            }
            items_wearing.push(textual_info);

            for __ in 0..3 {
                if look_ahead(stream) != 0 {
                    continue 'item_loop;
                }
                stream.seek(SeekFrom::Current(1)).unwrap();
            }

            break;
        }

        Self {
            name: name_string,
            details: human_rec,
            items_wearing,
        }
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct HumanRecord {
    body: i32,
    reaction: i32,
    mind: i32,
    spirit: i32,
    health_max: i32,
    mana_max: i32,
    speed: i32,
    rotation_speed: i32,
    scan_range: i32,
    defence: i32,
    skill_general: i32,
    skill_blade_fire: i32,
    skill_axe_water: i32,
    skill_bludgeon_air: i32,
    skill_pike_earth: i32,
    skill_shooting_astral: i32,
    type_id: i32,
    face: i32,
    gender: i32,
    attack_charge_time: i32,
    attack_relax_time: i32,
    token_size: i32,
    movement_type: i32,
    dying_time: i32,
    server_id: i32,
    known_spells: i32,
}
impl Reflectable for HumanRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.body)?;
        reflector.reflect_i32(&mut self.reaction)?;
        reflector.reflect_i32(&mut self.mind)?;
        reflector.reflect_i32(&mut self.spirit)?;
        reflector.reflect_i32(&mut self.health_max)?;
        reflector.reflect_i32(&mut self.mana_max)?;
        reflector.reflect_i32(&mut self.speed)?;
        reflector.reflect_i32(&mut self.rotation_speed)?;
        reflector.reflect_i32(&mut self.scan_range)?;
        reflector.reflect_i32(&mut self.defence)?;
        reflector.reflect_i32(&mut self.skill_general)?;
        reflector.reflect_i32(&mut self.skill_blade_fire)?;
        reflector.reflect_i32(&mut self.skill_axe_water)?;
        reflector.reflect_i32(&mut self.skill_bludgeon_air)?;
        reflector.reflect_i32(&mut self.skill_pike_earth)?;
        reflector.reflect_i32(&mut self.skill_shooting_astral)?;
        reflector.reflect_i32(&mut self.type_id)?;
        reflector.reflect_i32(&mut self.face)?;
        reflector.reflect_i32(&mut self.gender)?;
        reflector.reflect_i32(&mut self.attack_charge_time)?;
        reflector.reflect_i32(&mut self.attack_relax_time)?;
        reflector.reflect_i32(&mut self.token_size)?;
        reflector.reflect_i32(&mut self.movement_type)?;
        reflector.reflect_i32(&mut self.dying_time)?;
        reflector.reflect_i32(&mut self.server_id)?;
        reflector.reflect_i32(&mut self.known_spells)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct ItemInfo {
    name: CP866String,
    nop: u16,
    details: ItemRecord,
}
impl Reflectable for ItemInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector:
        &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop)?;
        reflector.reflect_composite(&mut self.details)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct ItemRecord {
    shape: i32,
    material: i32,
    price: i32,
    weight: i32,
    slot: i32,
    attack_type: i32,
    physical_min: i32,
    physical_max: i32,
    to_hit: i32,
    defence: i32,
    absorption: i32,
    range: i32,
    charge: i32,
    relax: i32,
    two_handed: i32,
    suitable_for: i32,
    other_parameter: i32,
    mysterious_field0: i32,
    mysterious_field1: i32,
    mysterious_field2: i32,
}
impl Reflectable for ItemRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.shape)?;
        reflector.reflect_i32(&mut self.material)?;
        reflector.reflect_i32(&mut self.price)?;
        reflector.reflect_i32(&mut self.weight)?;
        reflector.reflect_i32(&mut self.slot)?;
        reflector.reflect_i32(&mut self.attack_type)?;
        reflector.reflect_i32(&mut self.physical_min)?;
        reflector.reflect_i32(&mut self.physical_max)?;
        reflector.reflect_i32(&mut self.to_hit)?;
        reflector.reflect_i32(&mut self.defence)?;
        reflector.reflect_i32(&mut self.absorption)?;
        reflector.reflect_i32(&mut self.range)?;
        reflector.reflect_i32(&mut self.charge)?;
        reflector.reflect_i32(&mut self.relax)?;
        reflector.reflect_i32(&mut self.two_handed)?;
        reflector.reflect_i32(&mut self.suitable_for)?;
        reflector.reflect_i32(&mut self.other_parameter)?;
        reflector.reflect_i32(&mut self.mysterious_field0)?;
        reflector.reflect_i32(&mut self.mysterious_field1)?;
        reflector.reflect_i32(&mut self.mysterious_field2)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct MagicItemInfo {
    nop0: u16,
    name: CP866String,
    details: MagicItemRecord,
    textual_info: CP866String,
    nop1: u8,
}
impl Reflectable for MagicItemInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop0)?;
        reflector.reflect_composite(&mut self.details)?;
        reflector.reflect_u8(&mut self.nop1)?;
        reflector.reflect_composite(&mut self.textual_info)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct MagicItemRecord {
    price: i32,
    weight: i32,
}
impl Reflectable for MagicItemRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector:
        &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.price)?;
        reflector.reflect_i32(&mut self.weight)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct ParameterInfo {
    name: CP866String,
    nop: u16,
    details: ParameterRecord,
}
impl Reflectable for ParameterInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop)?;
        reflector.reflect_composite(&mut self.details)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct ParameterRecord {
    cost_mp: i32,
    affect_min: i32,
    affect_max: i32,
    usable_by: i32,
    in_weapon: i32,
    in_shield: i32,
    nop1: i32,
    in_ring: i32,
    in_amulet: i32,
    in_helm: i32,
    in_mail: i32,
    in_cuirass: i32,
    in_bracers: i32,
    in_gauntlets: i32,
    nop2: i32,
    in_boots: i32,
    in_weapon2: i32,
    nop3: i32,
    nop4: i32,
    in_ring2: i32,
    in_amulet2: i32,
    in_hat: i32,
    in_robe: i32,
    in_cloak: i32,
    nop5: i32,
    in_gloves: i32,
    nop6: i32,
    in_shoes: i32,
}
impl Reflectable for ParameterRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.cost_mp)?;
        reflector.reflect_i32(&mut self.affect_min)?;
        reflector.reflect_i32(&mut self.affect_max)?;
        reflector.reflect_i32(&mut self.usable_by)?;
        reflector.reflect_i32(&mut self.in_weapon)?;
        reflector.reflect_i32(&mut self.in_shield)?;
        reflector.reflect_i32(&mut self.nop1)?;
        reflector.reflect_i32(&mut self.in_ring)?;
        reflector.reflect_i32(&mut self.in_amulet)?;
        reflector.reflect_i32(&mut self.in_helm)?;
        reflector.reflect_i32(&mut self.in_mail)?;
        reflector.reflect_i32(&mut self.in_cuirass)?;
        reflector.reflect_i32(&mut self.in_bracers)?;
        reflector.reflect_i32(&mut self.in_gauntlets)?;
        reflector.reflect_i32(&mut self.nop2)?;
        reflector.reflect_i32(&mut self.in_boots)?;
        reflector.reflect_i32(&mut self.in_weapon2)?;
        reflector.reflect_i32(&mut self.nop3)?;
        reflector.reflect_i32(&mut self.nop4)?;
        reflector.reflect_i32(&mut self.in_ring2)?;
        reflector.reflect_i32(&mut self.in_amulet2)?;
        reflector.reflect_i32(&mut self.in_hat)?;
        reflector.reflect_i32(&mut self.in_robe)?;
        reflector.reflect_i32(&mut self.in_cloak)?;
        reflector.reflect_i32(&mut self.nop5)?;
        reflector.reflect_i32(&mut self.in_gloves)?;
        reflector.reflect_i32(&mut self.nop6)?;
        reflector.reflect_i32(&mut self.in_shoes)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct ShapeInfo {
    name: CP866String,
    nop0: u64,
    nop1: u64,
    details: ShapeRecord,
}
impl Reflectable for ShapeInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u64(&mut self.nop0)?;
        reflector.reflect_u64(&mut self.nop1)?;
        reflector.reflect_composite(&mut self.details)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct ShapeRecord {
    price: f64,
    weight: f64,
    damage: f64,
    to_hit: f64,
    defence: f64,
    absorption: f64,
    mag_cap_level: f64,
}
impl Reflectable for ShapeRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_f64(&mut self.price)?;
        reflector.reflect_f64(&mut self.weight)?;
        reflector.reflect_f64(&mut self.damage)?;
        reflector.reflect_f64(&mut self.to_hit)?;
        reflector.reflect_f64(&mut self.defence)?;
        reflector.reflect_f64(&mut self.absorption)?;
        reflector.reflect_f64(&mut self.mag_cap_level)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct SpellInfo {
    name: CP866String,
    nop: u16,
    details: SpellRecord,
    textual_info: CP866String,
}
impl Reflectable for SpellInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop)?;
        reflector.reflect_composite(&mut self.details)?;
        reflector.reflect_composite(&mut self.textual_info)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct SpellRecord {
    complication_level: i32,
    mana_cost: i32,
    sphere: i32,
    item: i32,
    spell_target: i32,
    delivery_system: i32,
    max_range: i32,
    spell_effect_speed: i32,
    distribution_system: i32,
    radius: i32,
    area_effect_affect: i32,
    area_effect_duration: i32,
    area_effect_frequency: i32,
    apply_on_unit_method: i32,
    spell_duratuion: i32,
    spell_frequency: i32,
    damage_min: i32,
    damage_max: i32,
    defensive: i32,
    skill_offset: i32,
    scroll_cost: i32,
    book_cost: i32,
}
impl Reflectable for SpellRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.complication_level)?;
        reflector.reflect_i32(&mut self.mana_cost)?;
        reflector.reflect_i32(&mut self.sphere)?;
        reflector.reflect_i32(&mut self.item)?;
        reflector.reflect_i32(&mut self.spell_target)?;
        reflector.reflect_i32(&mut self.delivery_system)?;
        reflector.reflect_i32(&mut self.max_range)?;
        reflector.reflect_i32(&mut self.spell_effect_speed)?;
        reflector.reflect_i32(&mut self.distribution_system)?;
        reflector.reflect_i32(&mut self.radius)?;
        reflector.reflect_i32(&mut self.area_effect_affect)?;
        reflector.reflect_i32(&mut self.area_effect_duration)?;
        reflector.reflect_i32(&mut self.area_effect_frequency)?;
        reflector.reflect_i32(&mut self.apply_on_unit_method)?;
        reflector.reflect_i32(&mut self.spell_duratuion)?;
        reflector.reflect_i32(&mut self.spell_frequency)?;
        reflector.reflect_i32(&mut self.damage_min)?;
        reflector.reflect_i32(&mut self.damage_max)?;
        reflector.reflect_i32(&mut self.defensive)?;
        reflector.reflect_i32(&mut self.skill_offset)?;
        reflector.reflect_i32(&mut self.scroll_cost)?;
        reflector.reflect_i32(&mut self.book_cost)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct StructureInfo {
    name: CP866String,
    nop: u16,
    details: StructureRecord,
}
impl Reflectable for StructureInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop)?;
        reflector.reflect_composite(&mut self.details)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct StructureRecord {
    size_x: i32,
    size_y: i32,
    scan_range: i32,
    health_max: i16,
    passability: i8,
    building_present: i8,
    start_id: i32,
    tiles: i16,
    nop: i16
}
impl Reflectable for StructureRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.size_x)?;
        reflector.reflect_i32(&mut self.size_y)?;
        reflector.reflect_i32(&mut self.scan_range)?;
        reflector.reflect_i16(&mut self.health_max)?;
        reflector.reflect_i8(&mut self.passability)?;
        reflector.reflect_i8(&mut self.building_present)?;
        reflector.reflect_i32(&mut self.start_id)?;
        reflector.reflect_i16(&mut self.tiles)?;
        reflector.reflect_i16(&mut self.nop)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct UnitInfo {
    name: CP866String,
    details: UnitRecord,
    textual_info: CP866String
}
impl UnitInfo {
    fn read_from_stream<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        let name = CP866String::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        stream.seek(SeekFrom::Current(2)).unwrap();

        let details = UnitRecord::deserialize(
            stream,
            Endianness::LittleEndian
        ).unwrap();

        let textual_info = CP866String::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        Self {
            name,
            details,
            textual_info
        }
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct UnitRecord {
    body: i32,
    reaction: i32,
    mind: i32,
    spirit: i32,
    health_max: i32,
    hp_regeneration: i32,
    mana_max: i32,
    mp_regeneration: i32,
    speed: i32,
    rotation_speed: i32,
    scan_range: i32,
    physical_min: i32,
    physical_max: i32,
    attack_kind: i32,
    to_hit: i32,
    defence: i32,
    absorption: i32,
    attack_charge_time: i32,
    attack_relax_time: i32,
    protect_fire: i32,
    protect_water: i32,
    protect_air: i32,
    protect_earth: i32,
    protect_astral: i32,
    resist_blade: i32,
    resist_axe: i32,
    resist_bludgeon: i32,
    resist_pike: i32,
    resist_shooting: i32,
    type_id: i32,
    face: i32,
    token_size: i32,
    movement_type: i32,
    dying_time: i32,
    withdraw: i32,
    wimpy: i32,
    see_invisible: i32,
    xp_value: i32,
    treasure1_gold: i32,
    treasure_min1: i32,
    treasure_max1: i32,
    treasure2_item: i32,
    treasure_min2: i32,
    treasure_max2: i32,
    treasure3_magic: i32,
    treasure_min3: i32,
    treasure_max3: i32,
    power: i32,
    spell1: i32,
    probability1: i32,
    spell2: i32,
    probability2: i32,
    spell3: i32,
    probability3: i32,
    spell_power: i32
}
impl Reflectable for UnitRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.body)?;
        reflector.reflect_i32(&mut self.reaction)?;
        reflector.reflect_i32(&mut self.mind)?;
        reflector.reflect_i32(&mut self.spirit)?;
        reflector.reflect_i32(&mut self.health_max)?;
        reflector.reflect_i32(&mut self.hp_regeneration)?;
        reflector.reflect_i32(&mut self.mana_max)?;
        reflector.reflect_i32(&mut self.mp_regeneration)?;
        reflector.reflect_i32(&mut self.speed)?;
        reflector.reflect_i32(&mut self.rotation_speed)?;
        reflector.reflect_i32(&mut self.scan_range)?;
        reflector.reflect_i32(&mut self.physical_min)?;
        reflector.reflect_i32(&mut self.physical_max)?;
        reflector.reflect_i32(&mut self.attack_kind)?;
        reflector.reflect_i32(&mut self.to_hit)?;
        reflector.reflect_i32(&mut self.defence)?;
        reflector.reflect_i32(&mut self.absorption)?;
        reflector.reflect_i32(&mut self.attack_charge_time)?;
        reflector.reflect_i32(&mut self.attack_relax_time)?;
        reflector.reflect_i32(&mut self.protect_fire)?;
        reflector.reflect_i32(&mut self.protect_water)?;
        reflector.reflect_i32(&mut self.protect_air)?;
        reflector.reflect_i32(&mut self.protect_earth)?;
        reflector.reflect_i32(&mut self.protect_astral)?;
        reflector.reflect_i32(&mut self.resist_blade)?;
        reflector.reflect_i32(&mut self.resist_axe)?;
        reflector.reflect_i32(&mut self.resist_bludgeon)?;
        reflector.reflect_i32(&mut self.resist_pike)?;
        reflector.reflect_i32(&mut self.resist_shooting)?;
        reflector.reflect_i32(&mut self.type_id)?;
        reflector.reflect_i32(&mut self.face)?;
        reflector.reflect_i32(&mut self.token_size)?;
        reflector.reflect_i32(&mut self.movement_type)?;
        reflector.reflect_i32(&mut self.dying_time)?;
        reflector.reflect_i32(&mut self.withdraw)?;
        reflector.reflect_i32(&mut self.wimpy)?;
        reflector.reflect_i32(&mut self.see_invisible)?;
        reflector.reflect_i32(&mut self.xp_value)?;
        reflector.reflect_i32(&mut self.treasure1_gold)?;
        reflector.reflect_i32(&mut self.treasure_min1)?;
        reflector.reflect_i32(&mut self.treasure_max1)?;
        reflector.reflect_i32(&mut self.treasure2_item)?;
        reflector.reflect_i32(&mut self.treasure_min2)?;
        reflector.reflect_i32(&mut self.treasure_max2)?;
        reflector.reflect_i32(&mut self.treasure3_magic)?;
        reflector.reflect_i32(&mut self.treasure_min3)?;
        reflector.reflect_i32(&mut self.treasure_max3)?;
        reflector.reflect_i32(&mut self.power)?;
        reflector.reflect_i32(&mut self.spell1)?;
        reflector.reflect_i32(&mut self.probability1)?;
        reflector.reflect_i32(&mut self.spell2)?;
        reflector.reflect_i32(&mut self.probability2)?;
        reflector.reflect_i32(&mut self.spell3)?;
        reflector.reflect_i32(&mut self.probability3)?;
        reflector.reflect_i32(&mut self.spell_power)
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Section {
    Human(Vec<HumanInfo>),
    Item {
        wieldables: Vec<ItemInfo>,
        shields: Vec<ItemInfo>,
        weapons: Vec<ItemInfo>,
    },
    MagicItem(Vec<MagicItemInfo>),
    Parameter(Vec<ParameterInfo>),
    Shapes { rarities: Vec<ShapeInfo>, materials: Vec<ShapeInfo> },
    Spell(Vec<SpellInfo>),
    Structure(Vec<StructureInfo>),
    Unit(Vec<UnitInfo>),
}

fn main() {
    let cursor = Cursor::new(WORLD_RES);
    if let Ok(resource_file) = ResourceFile::new(cursor) {
        let mut resource_file = resource_file;
        if let Ok(data_bin) = resource_file.get_resource_bytes("data/data.bin") {
            let mut header_buffer = [0u8; 20];
            let mut cursor = Cursor::new(data_bin);
            let mut section_set = vec![
                SectionKind::Human,
                SectionKind::Item,
                SectionKind::MagicItem,
                SectionKind::Parameter,
                SectionKind::Shapes,
                SectionKind::Spell,
                SectionKind::Structure,
                SectionKind::Unit
            ];

            let mut sections = Vec::new();

            while section_set.len() > 0 {
                cursor.read(&mut header_buffer).unwrap();
                cursor.seek(SeekFrom::Current(-20)).unwrap();
                let current_section = match &header_buffer[3..8] {
                    [b'S', b'h', b'a', b'p', b'e'] => {
                        SectionKind::Shapes
                    }
                    [b'P', b'a', b'r', b'a', b'm'] => {
                        SectionKind::Parameter
                    }
                    [b'I', b't', b'e', b'm', _   ] => {
                        SectionKind::Item
                    }
                    [b'M', b'a', b'g', b'i', b'c'] => {
                        SectionKind::MagicItem
                    }
                    [b'U', b'n', b'i', b't', _   ] => {
                        SectionKind::Unit
                    }
                    [b'H', b'u', b'm', b'a', b'n'] => {
                        SectionKind::Human
                    }
                    [b'B', b'u', b'i', b'l', b'd'] => {
                        SectionKind::Structure
                    }
                    [b'S', b'p', b'e', b'l', b'l'] => {
                        SectionKind::Spell
                    }
                    _ => SectionKind::Unknown
                };

                match current_section {
                    SectionKind::Human => {
                        cursor.seek(SeekFrom::Current(HUMAN_HEADER_SIZE)).unwrap();
                        let entry_count = {
                            cursor.seek(SeekFrom::Current(4)).unwrap();
                            0xD2
                        };
                        let mut human_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            human_vec.push(HumanInfo::read_from_stream(&mut cursor));
                        }
                        while look_ahead(&mut cursor) == 0 {
                            cursor.seek(SeekFrom::Current(1)).unwrap();
                        }
                        sections.push(Section::Human(human_vec));
                    }
                    SectionKind::Item => {
                        cursor.seek(SeekFrom::Current(ITEM_HEADER_SIZE)).unwrap();
                        let entry_count = read_corrected_entry_count(&mut cursor) as usize;
                        let mut wieldables_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            wieldables_vec.push(
                                ItemInfo::deserialize(
                                    &mut cursor,
                                    Endianness::LittleEndian,
                                ).unwrap()
                            );
                        }
                        let entry_count = read_corrected_entry_count(&mut cursor) as usize;
                        let mut shields_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            shields_vec.push(
                                ItemInfo::deserialize(
                                    &mut cursor,
                                    Endianness::LittleEndian,
                                ).unwrap()
                            );
                        }
                        let entry_count = read_corrected_entry_count(&mut cursor) as usize;
                        let mut weapons_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            weapons_vec.push(
                                ItemInfo::deserialize(
                                    &mut cursor,
                                    Endianness::LittleEndian,
                                ).unwrap()
                            );
                        }
                        sections.push(
                            Section::Item {
                                wieldables: wieldables_vec,
                                shields: shields_vec,
                                weapons: weapons_vec,
                            }
                        );
                    }
                    SectionKind::MagicItem => {
                        cursor.seek(SeekFrom::Current(MAGIC_ITEM_HEADER_SIZE)).unwrap();
                        let entry_count = read_corrected_entry_count(&mut cursor) as usize;
                        let mut magic_item_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            magic_item_vec.push(
                                MagicItemInfo::deserialize(&mut cursor, Endianness::LittleEndian).unwrap()
                            );
                        }
                        sections.push(Section::MagicItem(magic_item_vec));
                    }
                    SectionKind::Parameter => {
                        cursor.seek(SeekFrom::Current(PARAMETER_HEADER_SIZE)).unwrap();
                        let entry_count = read_entry_count(&mut cursor) as usize;
                        let mut parameter_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            parameter_vec.push(
                                ParameterInfo::deserialize(&mut cursor, Endianness::LittleEndian).unwrap()
                            );
                        }
                        sections.push(Section::Parameter(parameter_vec));
                    }
                    SectionKind::Shapes => {
                        cursor.seek(SeekFrom::Current(SHAPE_HEADER_SIZE)).unwrap();
                        let entry_count = read_entry_count(&mut cursor) as usize;
                        let mut rarity_def_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            rarity_def_vec.push(
                                ShapeInfo::deserialize(
                                    &mut cursor,
                                    Endianness::LittleEndian,
                                ).unwrap()
                            )
                        }
                        let entry_count = read_entry_count(&mut cursor) as usize;
                        let mut material_def_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            material_def_vec.push(
                                ShapeInfo::deserialize(
                                    &mut cursor,
                                    Endianness::LittleEndian,
                                ).unwrap()
                            )
                        }
                        sections.push(
                            Section::Shapes {
                                rarities: rarity_def_vec,
                                materials: material_def_vec,
                            }
                        );
                    }
                    SectionKind::Spell => {
                        cursor.seek(SeekFrom::Current(SPELL_HEADER_SIZE)).unwrap();
                        let entry_count = read_corrected_entry_count(&mut cursor) as usize;
                        let mut spell_item_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            spell_item_vec.push(
                                SpellInfo::deserialize(&mut cursor, Endianness::LittleEndian).unwrap()
                            );
                        }
                        sections.push(Section::Spell(spell_item_vec));
                    }
                    SectionKind::Structure => {
                        cursor.seek(SeekFrom::Current(STRUCTURE_HEADER_SIZE)).unwrap();
                        let entry_count = read_corrected_entry_count(&mut cursor) as usize;
                        let mut structure_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            structure_vec.push(
                                StructureInfo::deserialize(&mut cursor, Endianness::LittleEndian).unwrap()
                            );
                        }
                        sections.push(Section::Structure(structure_vec));
                    }
                    SectionKind::Unit => {
                        cursor.seek(SeekFrom::Current(UNIT_HEADER_SIZE)).unwrap();
                        let entry_count = {
                            cursor.seek(SeekFrom::Current(4)).unwrap();
                            0x38
                        };
                        let mut unit_vec = Vec::with_capacity(entry_count);
                        for _ in 0..entry_count {
                            unit_vec.push(UnitInfo::read_from_stream(&mut cursor));
                        }
                        while look_ahead(&mut cursor) == 0 {
                            cursor.seek(SeekFrom::Current(1)).unwrap();
                        }
                        sections.push(Section::Unit(unit_vec));
                    }
                    SectionKind::Unknown => unreachable!()
                }

                // relatively cheap deletion from a list:
                for i in (0..section_set.len()).rev() {
                    if section_set[i] == current_section {
                        let last_id = section_set.len() - 1;
                        section_set[i] = section_set[last_id];
                        section_set.remove(last_id);
                    }
                }
            }

            println!("data content:\n {:?}", sections);
        }
    }
}