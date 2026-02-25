// tariff.rs

// Data structures for tariff calculations

/// A struct representing a charging tariff.
pub struct Tariff {
    pub name: String,
    pub price_per_kwh: f64,
    pub peak_hours: (u32, u32), // Start and end hours for peak
    pub is_active: bool,
}

impl Tariff {
    /// Creates a new Tariff
    pub fn new(name: &str, price_per_kwh: f64, peak_hours: (u32, u32), is_active: bool) -> Self {
        Tariff {
            name: name.to_string(),
            price_per_kwh,
            peak_hours,
            is_active,
        }
    }

    /// Calculate cost for given kWh
    pub fn calculate_cost(&self, kwh: f64) -> f64 {
        if self.is_active {
            kwh * self.price_per_kwh
        } else {
            0.0
        }
    }
}

/// A struct representing a user with a charging tariff.
pub struct UserTariff {
    pub user_id: String,
    pub tariff: Tariff,
}

impl UserTariff {
    /// Creates a new UserTariff
    pub fn new(user_id: &str, tariff: Tariff) -> Self {
        UserTariff {
            user_id: user_id.to_string(),
            tariff,
        }
    }
}