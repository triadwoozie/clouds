//! Optional block-behavior registries used by feature placement: physical
//! properties, face support, collision and survival rules. Loaded from the
//! bundle's `blocks/` directory (Java-exported, see crates/horizon-prepare).
use super::generator::GeneratorError;
use crate::{
    block_collision::BlockCollisionRegistry,
    block_properties::BlockPropertiesRegistry,
    block_support::BlockSupportRegistry,
    block_survival::{BlockSurvivalRegistry, SurvivalDefinitions, SurvivalRule},
    collision::CachedCollisionRegistry,
    registry::{BlockRegistry, BlockTableFile},
};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub(super) struct BlockBehavior {
    pub(super) properties: BlockPropertiesRegistry<'static>,
    pub(super) support: Arc<BlockSupportRegistry<'static>>,
    pub(super) collision: Arc<BlockCollisionRegistry<'static>>,
    pub(super) survival: BlockSurvivalRegistry<'static>,
    pub(super) double_plants: BTreeSet<String>,
    pub(super) mossy_carpets: BTreeSet<String>,
}
fn resource(message: impl Into<String>) -> GeneratorError {
    GeneratorError::Resource(message.into())
}
fn rule(name: &str) -> Option<SurvivalRule> {
    use SurvivalRule::*;
    if let Some(tag) = name.strip_prefix("TagBelow:") {
        return Some(TagBelow(tag.to_owned()));
    }
    Some(match name {
        "CactusFlower" => CactusFlower,
        "Seagrass" => Seagrass,
        "BubbleColumn" => BubbleColumn,
        "RedstoneWire" => RedstoneWire,
        "DoublePlant" => DoublePlant,
        "Door" => Door,
        "BigDripleaf" => BigDripleaf,
        "BigDripleafStem" => BigDripleafStem,
        "Farmland" => Farmland,
        "Cocoa" => Cocoa,
        "Crop" => Crop,
        "GrowingPlant" => GrowingPlant,
        "MangrovePropagule" => MangrovePropagule,
        "Speleothem" => Speleothem,
        "Scaffolding" => Scaffolding,
        "PistonHead" => PistonHead,
        "ChorusPlant" => ChorusPlant,
        "ChorusFlower" => ChorusFlower,
        "PitcherCrop" => PitcherCrop,
        "Multiface" => Multiface,
        "Vine" => Vine,
        "HangingMoss" => HangingMoss,
        "SnowLayer" => SnowLayer,
        "SeaPickle" => SeaPickle,
        "Fire" => Fire,
        "Cactus" => Cactus,
        "Path" => Path,
        "Mushroom" => Mushroom,
        "Bell" => Bell,
        "SugarCane" => SugarCane,
        "LilyPad" => LilyPad,
        "Frogspawn" => Frogspawn,
        "SporeBlossom" => SporeBlossom,
        "SmallDripleaf" => SmallDripleaf,
        "TallSeagrass" => TallSeagrass,
        "Always" => Always,
        "SolidBelow" => SolidBelow,
        "SolidBehind" => SolidBehind,
        "NonAirBelow" => NonAirBelow,
        "MossCarpet" => MossCarpet,
        "CenterBelow" => CenterBelow,
        "Lantern" => Lantern,
        "FullAbove" => FullAbove,
        "FullBehind" => FullBehind,
        "RigidOrCenterBelow" => RigidOrCenterBelow,
        "RigidBelow" => RigidBelow,
        "FullBelow" => FullBelow,
        "CenterAbove" => CenterAbove,
        "FaceAttached" => FaceAttached,
        "SupportsBamboo" => SupportsBamboo,
        _ => return None,
    })
}
impl BlockBehavior {
    /// Files: `block-properties.nbt`, `cached-collision.nbt`,
    /// `cached-support.nbt`, `survival-rules.json`,
    /// `survival-extra-definitions.json` and `placement-definitions.json`.
    pub(super) fn load(
        dir: &Path,
        registry: &'static BlockRegistry,
        tags: &BTreeMap<String, Vec<String>>,
        fluid_tags: &BTreeMap<String, Vec<String>>,
    ) -> Result<Option<Self>, GeneratorError> {
        let prop_path = dir.join("block-properties.nbt");
        let exists = horizon_core::prepared_files::vfs_exists(prop_path.to_str().unwrap_or(""))
            || prop_path.exists();
        if !exists {
            return Ok(None);
        }
        let read = |name: &str| -> Result<BlockTableFile<'static>, GeneratorError> {
            BlockTableFile::open(&dir.join(name), registry)
                .map_err(|e| resource(format!("{name}: {e}")))
        };
        let json = |name: &str| -> Result<Value, GeneratorError> {
            let path = dir.join(name);
            let text = horizon_core::prepared_files::read_to_string(&path)
                .or_else(|_| {
                    let clean = path.to_str().unwrap_or("").trim_start_matches('.').trim_start_matches('/');
                    horizon_core::prepared_files::read_to_string(Path::new(clean))
                })
                .or_else(|_| fs::read_to_string(&path))
                .map_err(|e| resource(format!("{name}: {e}")))?;
            Ok(serde_json::from_str(&text)?)
        };
        // The support table reads on its own thread while the collision and
        // property tables (which need the collisions) read on this one.
        let (collisions, properties, support) = std::thread::scope(|scope| {
            let support = scope.spawn(|| -> Result<_, GeneratorError> {
                Ok(BlockSupportRegistry::from_table(
                    read("cached-support.nbt")?.table("support")?,
                )?)
            });
            let collisions = CachedCollisionRegistry::from_table(
                read("cached-collision.nbt")?.table("collision")?,
            )?;
            let properties = BlockPropertiesRegistry::from_table(
                read("block-properties.nbt")?.table("block property")?,
                &collisions,
            )?;
            let support = support
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic))?;
            Ok::<_, GeneratorError>((collisions, properties, Arc::new(support)))
        })?;
        let collision = Arc::new(BlockCollisionRegistry::new(Arc::new(collisions))?);
        let rules = json("survival-rules.json")?
            .as_object()
            .ok_or_else(|| resource("survival rules"))?
            .iter()
            .map(|(name, value)| {
                let text = value.as_str().unwrap_or_default();
                rule(text)
                    .map(|rule| (name.clone(), rule))
                    .ok_or_else(|| resource(format!("unknown survival rule {text}")))
            })
            .collect::<Result<_, _>>()?;
        let extra = json("survival-extra-definitions.json")?;
        let definitions = SurvivalDefinitions {
            rules,
            ignition_odds: serde_json::from_value(extra["ignition_odds"].clone())?,
            fence_gates: serde_json::from_value(extra["fence_gates"].clone())?,
        };
        let survival = BlockSurvivalRegistry::new(
            registry,
            &properties,
            support.clone(),
            collision.clone(),
            &definitions,
            tags,
            fluid_tags,
        )?;
        let placement = json("placement-definitions.json")?;
        Ok(Some(Self {
            double_plants: serde_json::from_value(placement["double_plants"].clone())?,
            mossy_carpets: serde_json::from_value(placement["mossy_carpets"].clone())?,
            properties,
            support,
            collision,
            survival,
        }))
    }
}
