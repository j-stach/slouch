
// NOTE: "All fixed-width alpha fields are left-justified and padded on the right with spaces"

/// 
pub enum Capacity {
    Agency,
    Principal,
    Riskless,
    Other
}

impl Capacity {

    pub(crate) fn encode(&self) -> char {

        use Self::*;
        match self {
            Agency => 'A',
            Principal => 'P',
            Riskless => 'R',
            Other => 'O',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, OuchError> {

        use Self::*;
        match data as char {
            'A' => Agency,
            'P' => Principal,
            'R' => Riskless,
            'O' => Other,
        }
    }
}

