#[derive(Clone, Copy, Default)]
pub struct Upgrade {
    pub purchased: bool,
}

impl From<Upgrade> for bool {
    fn from(value: Upgrade) -> Self {
        value.purchased
    }
}

