pub mod artifact_enhancer;

use artifact::*;
use artifact_enhancer::ArtifactEnhancer;
use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ArtifactZip(pub u64);

impl Artifact for ArtifactZip {
    /// 获取星级
    fn get_star_level(&self) -> Option<u8> {
        let star_level: u8 = (self.0 >> 53 & 0b111) as u8;
        match star_level {
            0 => Some(5),
            x @ 1..=5 => Some(x),
            _ => None,
        }
    }
    /// 获取部位
    fn get_slot(&self) -> Option<slot::Slot> {
        ArtifactZip::decode_slot((self.0 >> 50 & 0b111) as u8)
    }
    /// 获取等级
    fn get_level(&self) -> Option<u8> {
        let level: u8 = (self.0 >> 45 & 0b11111) as u8;
        match level {
            x @ 0..=20 => Some(x),
            _ => None,
        }
    }
    /// 获取主词条
    fn get_main_stat(&self) -> Option<stat::Stat> {
        ArtifactZip::decode_stat((self.0 >> 40 & 0b11111) as u8)
    }
    /// 获取副词条
    fn get_sub_stat(&self, x: usize) -> Option<stat::Stat> {
        if x >= 4 {
            None
        } else {
            ArtifactZip::decode_stat((self.0 >> (x * 10 + 6) & 0b1111) as u8)
        }
    }
    /// 获取副词条强化等级
    fn get_sub_stat_level(&self, x: usize) -> u8 {
        if x >= 4 || ArtifactZip::get_sub_stat(self, x).is_none() {
            0
        } else {
            (self.0 >> (x * 10) & 0b111111) as u8
        }
    }
}

impl ArtifactZip {
    pub fn new(slot: &slot::Slot, main_stat: &stat::Stat) -> ArtifactZip {
        let mut ret = ArtifactZip(0);
        ret.0 |= (ArtifactZip::encode_slot(slot) as u64) << 50;
        ret.0 |= (ArtifactZip::encode_stat(main_stat) as u64) << 40;
        ret
    }
    pub fn decode_slot(x: u8) -> Option<slot::Slot> {
        match x {
            x @ 1..=5 => Some(match x {
                1 => slot::Flower,
                2 => slot::Plume,
                3 => slot::Sands,
                4 => slot::Goblet,
                _ => slot::Circlet,
            }),
            _ => None,
        }
    }
    pub fn encode_slot(x: &slot::Slot) -> u8 {
        match x {
            slot::Slot::Flower => 1,
            slot::Slot::Plume => 2,
            slot::Slot::Sands => 3,
            slot::Slot::Goblet => 4,
            slot::Slot::Circlet => 5,
        }
    }
    pub fn decode_stat(x: u8) -> Option<stat::Stat> {
        match x {
            x @ 1..=18 => Some(match x {
                1 => stat::Hp,
                2 => stat::HpRatio,
                3 => stat::Atk,
                4 => stat::AtkRatio,
                5 => stat::Def,
                6 => stat::DefRatio,
                7 => stat::ElementalMastery,
                8 => stat::EnergyRecharge,
                9 => stat::CritRate,
                10 => stat::CritDamage,
                11 => stat::HealingBonus,
                12 => stat::PhysicalBonus,
                13 => stat::AnemoBonus,
                14 => stat::GeoBonus,
                15 => stat::ElectroBonus,
                16 => stat::HydroBonus,
                17 => stat::CryoBonus,
                _ => stat::PyroBonus,
            }),
            _ => None,
        }
    }
    pub fn encode_stat(x: &stat::Stat) -> u8 {
        match x {
            stat::Stat::Hp => 1,
            stat::Stat::HpRatio => 2,
            stat::Stat::Atk => 3,
            stat::Stat::AtkRatio => 4,
            stat::Stat::Def => 5,
            stat::Stat::DefRatio => 6,
            stat::Stat::ElementalMastery => 7,
            stat::Stat::EnergyRecharge => 8,
            stat::Stat::CritRate => 9,
            stat::Stat::CritDamage => 10,
            stat::Stat::HealingBonus => 11,
            stat::Stat::PhysicalBonus => 12,
            stat::Stat::AnemoBonus => 13,
            stat::Stat::GeoBonus => 14,
            stat::Stat::ElectroBonus => 15,
            stat::Stat::HydroBonus => 16,
            stat::Stat::CryoBonus => 17,
            stat::Stat::PyroBonus => 18,
        }
    }
    pub fn standard(&mut self) {
        if self.get_suit().is_none() {
            self.0 ^= (self.0 >> 56) << 56;
        }
        if self.get_star_level().is_none() {
            self.0 ^= (self.0 >> 53 & 0b111) << 53;
        }
        if self.get_slot().is_none() {
            self.0 ^= (self.0 >> 50 & 0b111) << 50;
        }
        if self.get_level().is_none() {
            self.0 ^= (self.0 >> 45 & 0b11111) << 45;
        }
        if self.get_main_stat().is_none() {
            self.0 ^= (self.0 >> 40 & 0b11111) << 40;
        }
        for i in 0..4 {
            if self.get_sub_stat(i).is_none() {
                self.0 ^= (self.0 >> (i * 10) & 0b111111) << (i * 10);
            }
        }
    }
    pub fn sort_sub_stat(&mut self) {
        self.standard();
        let mut vec = Vec::new();
        for i in 0..4 {
            vec.push(self.0 >> (10 * i) & 0b1111111111);
            self.0 ^= (self.0 >> (i * 10) & 0b111111) << (i * 10);
        }
        vec.sort_unstable();
        let mut sub_stat = 0;
        for i in vec {
            sub_stat <<= 10;
            sub_stat |= i;
        }
        self.0 |= sub_stat;
    }

    fn set_sub_stat(&mut self, x: usize, num: u64) -> Result<(), ()> {
        if num & 0b1111111111 != num {
            return Err(());
        }
        if x >= 4 {
            Err(())
        } else {
            self.0 ^= (self.0 >> (x * 10) & 0b1111111111) << (x * 10);
            self.0 |= num << (x * 10);
            Ok(())
        }
    }
    fn have_stat(&self, stat: &stat::Stat) -> bool {
        if let Some(main_stat) = self.get_main_stat() {
            if *stat == main_stat {
                return true;
            }
        }
        for j in 0..4 {
            if let Some(sub_stat) = self.get_sub_stat(j) {
                if *stat == sub_stat {
                    return true;
                }
            }
        }
        false
    }
    fn get_sub_stat_probability(&self, stat: &stat::Stat) -> f64 {
        if self.have_stat(stat) {
            return 0.0;
        }
        let mut sum = 0.0;
        for i in stat::Stat::list() {
            if self.have_stat(&i) {
                continue;
            }
            sum += i.get_sub_stat_weight();
        }
        stat.get_sub_stat_weight() / sum
    }
}

#[derive(Clone)]
pub struct ArtifactP(pub ArtifactZip, pub f64);

impl ArtifactEnhancer for ArtifactP {
    fn gain(&self) -> Vec<ArtifactP> {
        let mut arti = self.0.clone();
        arti.sort_sub_stat();
        let mut ret = Vec::new();
        if arti.get_sub_stat(3).is_some() {
            // 四个词条已经满了
            // 枚举强化词条 i
            for i in 0..4 {
                for j in 7..=10 {
                    let mut new_arti = arti.clone();
                    new_arti
                        .set_sub_stat(i, ((arti.0 >> (i * 10)) & 0b1111111111) + j)
                        .unwrap();
                    ret.push(ArtifactP(new_arti, self.1 * 0.25 * 0.25));
                }
            }
        } else {
            // 四个词条未满
            // 先找到第一个空词条
            let mut empty_index = 0;
            while arti.get_sub_stat(empty_index).is_some() {
                empty_index += 1;
            }
            // 枚举新词条
            for stat in stat::Stat::list() {
                // 获取当前圣遗物强化出本词条的概率
                // 已考虑词条重复情况
                let p = arti.get_sub_stat_probability(&stat);
                if p == 0.0 {
                    continue;
                }
                for j in 7..=10 {
                    let mut new_arti = arti.clone();
                    new_arti
                        .set_sub_stat(
                            empty_index,
                            (ArtifactZip::encode_stat(&stat) << 6 | j).into(),
                        )
                        .unwrap();
                    ret.push(ArtifactP(new_arti, self.1 * p * 0.25));
                }
            }
        }
        ret
    }

    fn gain_many(artis: Vec<ArtifactP>) -> Vec<ArtifactP> {
        let mut ret = Vec::new();
        for arti in artis {
            ret.append(&mut arti.gain());
        }
        merged(&ret)
    }
}

fn merged(list: &[ArtifactP]) -> Vec<ArtifactP> {
    let mut map = HashMap::new();
    for i in list {
        let mut arti = i.0.clone();
        arti.sort_sub_stat();
        let x = map.entry(arti).or_insert(0.0);
        *x += i.1;
    }
    let mut res: Vec<ArtifactP> = Vec::new();
    for i in map {
        res.push(ArtifactP(i.0, i.1));
    }
    res
}
