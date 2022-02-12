#[derive(PartialEq, Eq, Hash)]
pub enum Slot {
    Flower,
    Plume,
    Sands,
    Goblet,
    Circlet,
}

use super::stat::Stat::{self, *};
pub use Slot::*;

impl Slot {
    /// 生成包含所有部位的Vec
    pub fn list() -> Vec<Slot> {
        vec![
            Flower,  // 花
            Plume,   // 羽毛
            Sands,   // 沙漏
            Goblet,  // 杯子
            Circlet, // 头
        ]
    }
    /// 获取相应部位的主词条的概率
    /// - self: 部位
    /// - stat: 查询的主词条
    pub fn get_main_stat_probability(&self, stat: &Stat) -> f64 {
        match self {
            Flower => match stat {
                Hp => 1.0,
                _ => 0.0,
            },
            Plume => match stat {
                Atk => 1.0,
                _ => 0.0,
            },
            Sands => match stat {
                HpRatio => 0.2668,
                AtkRatio => 0.2666,
                DefRatio => 0.2666,
                EnergyRecharge => 0.1,
                ElementalMastery => 0.1,
                _ => 0.0,
            },
            Goblet => match stat {
                HpRatio => 0.2125,
                AtkRatio => 0.2125,
                DefRatio => 0.2000,
                ElementalMastery => 0.0250,
                PhysicalBonus => 0.0500,
                AnemoBonus => 0.0500,
                GeoBonus => 0.0500,
                ElectroBonus => 0.0500,
                HydroBonus => 0.0500,
                PyroBonus => 0.0500,
                CryoBonus => 0.0500,
                _ => 0.0,
            },
            Circlet => match stat {
                HpRatio => 0.2200,
                AtkRatio => 0.2200,
                DefRatio => 0.2200,
                ElementalMastery => 0.0400,
                HealingBonus => 0.1000,
                CritRate => 0.1000,
                CritDamage => 0.1000,
                _ => 0.0,
            },
        }
    }
}
