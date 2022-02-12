mod artifact;
mod artifact_enhancer;
mod artifactzip;

use std::fs::File;
use std::io::Write;

use artifact::slot::Slot;
use artifact::stat::Stat;
use artifact_enhancer::ArtifactEnhancer;
use artifactzip::*;

fn main() {
    for slot in Slot::list() {
        let mut file = File::create(format!("{}.data.bin", ArtifactZip::encode_slot(&slot))).unwrap();
        for stat in Stat::list() {
            let p = slot.get_main_stat_probability(&stat);
            if p == 0.0 {
                continue;
            }
            const FOUR_SUB_STAT_P: f64 = 0.2;
            const THREE_SUB_STAT_P: f64 = 1.0 - FOUR_SUB_STAT_P;
            let arti = ArtifactP(ArtifactZip::new(&slot, &stat), p);
            let arti = arti.gain();
            let arti = ArtifactP::gain_many(arti);
            let arti = ArtifactP::gain_many(arti);
            let mut res = arti.clone();
            for i in res.iter_mut() {
                i.1 *= THREE_SUB_STAT_P;
            }
            let mut arti = ArtifactP::gain_many(arti);
            for i in arti.iter_mut() {
                i.1 *= FOUR_SUB_STAT_P;
            }
            res.append(&mut arti);
            let res = ArtifactP::gain_many(res);
            let res = ArtifactP::gain_many(res);
            let res = ArtifactP::gain_many(res);
            let res = ArtifactP::gain_many(res);
            let res = ArtifactP::gain_many(res);
            for arti in res {
                file.write(&arti.0 .0.to_le_bytes());
                file.write(&arti.1.to_le_bytes());
            }
        }
    }
}
