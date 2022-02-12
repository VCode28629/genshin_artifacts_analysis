#[derive(PartialEq, Eq, Hash)]
pub enum Stat {
    /// 生命值
    Hp,
    /// 百分比生命值
    HpRatio,
    /// 攻击力
    Atk,
    /// 百分比攻击力
    AtkRatio,
    /// 防御力
    Def,
    /// 百分比防御力
    DefRatio,
    /// 元素精通
    ElementalMastery,
    /// 元素充能效率
    EnergyRecharge,
    /// 暴击率
    CritRate,
    /// 暴击伤害
    CritDamage,
    /// 治疗加成
    HealingBonus,
    /// 物理伤害加成
    PhysicalBonus,
    /// 风元素伤害加成
    AnemoBonus,
    /// 岩元素伤害加成
    GeoBonus,
    /// 雷元素伤害加成
    ElectroBonus,
    /// 水元素伤害加成
    HydroBonus,
    /// 冰元素伤害加成
    CryoBonus,
    /// 火元素伤害加成
    PyroBonus,
}

pub use Stat::*;

impl Stat {
    /// 生成包含所有词条的Vec
    pub fn list() -> Vec<Stat> {
        vec![
            Hp,
            HpRatio,
            Atk,
            AtkRatio,
            Def,
            DefRatio,
            ElementalMastery,
            EnergyRecharge,
            CritRate,
            CritDamage,
            HealingBonus,
            PhysicalBonus,
            AnemoBonus,
            GeoBonus,
            ElectroBonus,
            HydroBonus,
            CryoBonus,
            PyroBonus,
        ]
    }
    /// 查询一个属性作为副词条的生成权重
    pub fn get_sub_stat_weight(&self) -> f64 {
        match self {
            Hp => 150.0,
            HpRatio => 100.0,
            Atk => 150.0,
            AtkRatio => 100.0,
            Def => 150.0,
            DefRatio => 100.0,
            ElementalMastery => 100.0,
            EnergyRecharge => 100.0,
            CritRate => 75.0,
            CritDamage => 75.0,
            _ => 0.0,
        }
    }
}
