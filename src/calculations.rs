// src/calculations.rs

use crate::ev_models::EVModel;

/// Tariff definition
#[derive(Clone, Debug)]
pub struct ChargingTariff {
    pub name: String,
    pub price_per_kwh: f64,
    pub monthly_fee: f64,
}

impl ChargingTariff {
    pub fn new(name: &str, price_per_kwh: f64, monthly_fee: f64) -> Self {
        ChargingTariff {
            name: name.to_string(),
            price_per_kwh,
            monthly_fee,
        }
    }

    /// Calculate monthly cost for given km distance
    pub fn calculate_monthly_cost(&self, monthly_km: f64, consumption_kwh_per_100km: f64) -> f64 {
        let kwh_needed = (monthly_km / 100.0) * consumption_kwh_per_100km;
        let energy_cost = kwh_needed * self.price_per_kwh;
        energy_cost + self.monthly_fee
    }
}

/// Find breakeven point between two tariffs
pub fn find_breakeven(tariff1: &ChargingTariff, tariff2: &ChargingTariff, consumption: f64) -> f64 {
    if tariff1.price_per_kwh == tariff2.price_per_kwh {
        if tariff1.monthly_fee < tariff2.monthly_fee {
            return f64::INFINITY;
        } else {
            return 0.0;
        }
    }

    let fee_diff = tariff2.monthly_fee - tariff1.monthly_fee;
    let price_diff = tariff1.price_per_kwh - tariff2.price_per_kwh;
    
    if price_diff == 0.0 {
        return f64::INFINITY;
    }

    let kwh = fee_diff / price_diff;
    (kwh * 100.0) / consumption
}

/// Generate data points for a cost curve
pub fn generate_cost_curve(
    tariff: &ChargingTariff,
    consumption: f64,
    max_km: f64,
    step: f64,
) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut km = 0.0;
    
    while km <= max_km {
        let cost = tariff.calculate_monthly_cost(km, consumption);
        points.push((km, cost));
        km += step;
    }
    
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost() {
        let tariff = ChargingTariff::new("Test", 0.30, 10.0);
        let cost = tariff.calculate_monthly_cost(1000.0, 16.0);
        assert!((cost - 58.0).abs() < 0.01);
    }
}