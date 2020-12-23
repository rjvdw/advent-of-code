pub type CupNumber = u64;
pub type Labeling = u64;
pub type PickedUpCups = (CupNumber, CupNumber, CupNumber);

pub enum Part {
    One,
    Two,
}

pub trait WithCupsCount {
    const LEN: usize;
}

pub trait WithContainsCup {
    fn contains_cup(&self, el: CupNumber) -> bool;
}

impl WithCupsCount for PickedUpCups {
    const LEN: usize = 3;
}

impl WithContainsCup for PickedUpCups {
    fn contains_cup(&self, el: CupNumber) -> bool {
        self.0 == el || self.1 == el || self.2 == el
    }
}
