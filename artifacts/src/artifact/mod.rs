pub mod slot;
pub mod stat;
pub mod suit;

pub trait Artifact {
    /// 获取套装
    fn get_suit(&self) -> Option<suit::Suit> {
        None
    }
    /// 获取星级
    fn get_star_level(&self) -> Option<u8> {
        None
    }
    /// 获取部位
    fn get_slot(&self) -> Option<slot::Slot> {
        None
    }
    /// 获取等级
    fn get_level(&self) -> Option<u8> {
        None
    }
    /// 获取主词条
    fn get_main_stat(&self) -> Option<stat::Stat> {
        None
    }
    /// 获取副词条
    fn get_sub_stat(&self, _: usize) -> Option<stat::Stat> {
        None
    }
    /// 获取副词条强化等级
    fn get_sub_stat_level(&self, _: usize) -> u8 {
        0
    }
}
