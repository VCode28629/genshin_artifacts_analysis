pub trait ArtifactEnhancer {
    fn gain(&self) -> Vec<Self>
    where
        Self: std::marker::Sized;

    fn gain_many(artis: Vec<Self>) -> Vec<Self>
    where
        Self: std::marker::Sized,
    {
        let mut ret = Vec::new();
        for arti in artis {
            ret.append(&mut arti.gain());
        }
        ret
    }
}
