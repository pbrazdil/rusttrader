use crate::instruments::Instrument;
use core::fmt;

pub struct Quantity {
    pub instrument: Instrument,
    pub size: f64,
    pub path_id: String,
}

#[derive(Debug)]
pub enum Error {
    QuantityOpPathMismatch,
    IncompatibleInstrumentOperation,
}
type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "instrument: {} size: {}, path_id: {}",
            self.instrument, self.size, self.path_id
        )
    }
}

impl std::clone::Clone for Quantity {
    fn clone(&self) -> Self {
        Quantity {
            path_id: self.path_id.clone(),
            size: self.size,
            instrument: self.instrument.clone(),
        }
    }
}

impl Quantity {
    pub fn is_locked(self) -> bool {
        self.path_id.is_empty()
    }

    pub fn lock_for(self, path_id: String) -> Quantity {
        Quantity {
            instrument: self.instrument,
            size: self.size,
            path_id,
        }
    }

    pub fn quantize(self) -> Quantity {
        //TODO: impl
        self
    }

    pub fn free(self) -> Quantity {
        //TODO: impl
        self
    }

    //TODO: impl
    // pub fn contain()

    pub fn validate(left: &Quantity, right: &Quantity) -> Result<(Quantity, Quantity)> {
        let mut left_res: Quantity = left.clone();
        let mut right_res: Quantity = right.clone();

        if left.instrument != right.instrument {
            return Err(Error::IncompatibleInstrumentOperation);
        }

        if (!left.path_id.is_empty() && !right.path_id.is_empty())
            && (left.path_id != right.path_id)
        {
            return Err(Error::QuantityOpPathMismatch);
        } else if !left.path_id.is_empty() && right.path_id.is_empty() {
            right_res.path_id = left.path_id.clone();
        } else if left.path_id.is_empty() && !right.path_id.is_empty() {
            left_res.path_id = right.path_id.clone();
        }

        Ok((left_res, right_res))
    }
}
