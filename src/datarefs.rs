use std::fmt::{Debug, Formatter};
use std::path::Path;

use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, ReadOnly, ReadWrite, StringRead};

pub struct BorrowedDataRefs {
    // Path of current livery. Ends in dir separator. WARNING: slow dataref, don't read a lot!
    pub acf_livery_path: DataRef<[u8], ReadOnly>,
    // Fuel Tank 1 Weight (kg)
    pub m_fuel1: DataRef<f32, ReadWrite>,
    // Fuel Tank 2 Weight (kg)
    pub m_fuel2: DataRef<f32, ReadWrite>,
    // Fuel Total Weight (kg)
}

impl Debug for BorrowedDataRefs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BorrowedDataRefs")
            .field(
                "acf_livery_path",
                &self.acf_livery_path.get_as_string().unwrap_or_default(),
            )
            .field("m_fuel1", &self.m_fuel1.get())
            .field("m_fuel2", &self.m_fuel2.get())
            .finish()
    }
}

impl BorrowedDataRefs {
    pub fn initialize() -> Result<Self, FindError> {
        let datarefs = BorrowedDataRefs {
            acf_livery_path: DataRef::find("sim/aircraft/view/acf_livery_path")?,
            m_fuel1: DataRef::find("sim/flightmodel/weight/m_fuel1")?.writeable()?,
            m_fuel2: DataRef::find("sim/flightmodel/weight/m_fuel2")?.writeable()?,
        };

        Ok(datarefs)
    }

    pub fn livery(&self) -> String {
        let livery_path = self.acf_livery_path.get_as_string().unwrap_or_default();
        let livery_os_str = Path::new(&livery_path).file_name().unwrap_or_default();
        let livery = livery_os_str.to_string_lossy().to_string();

        livery
    }
}
