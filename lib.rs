/**
 * Program Name:    Solana Insurance
 * Description:     Se crean las reglas, lógica y funciones para implementar
                    un modelo de seguro de vida como lo establece la
                    Comisión Nacional de Seguros y Fianzas, para conectarlo
                    a la red de Solana a partir de una cartera digital llamad Phantom
 * Author:          Berenice Domínguez Sánchez
 *  * Date Started:    May 27, 2023
 */
 struct InsurancePolicy {
    coverage_period: u32,
    premium: f64,
    coverage_amount: f64,
    beneficiaries: Vec<String>,
    payment_status: bool,
}

impl InsurancePolicy {
    fn new(period: u32, premium: f64, coverage: f64, beneficiaries: Vec<String>) -> Self {
        InsurancePolicy {
            coverage_period: period,
            premium,
            coverage_amount: coverage,
            beneficiaries,
            payment_status: false,
        }
    }

    fn make_payment(&mut self) {
        // Lógica para procesar el pago de la prima
        self.payment_status = true;
    }

    fn file_claim(&self, death_verification: bool) -> Option<Vec<String>> {
        if self.payment_status && death_verification {
            Some(self.beneficiaries.clone())
        } else {
            None
        }
    }
}

fn main() {
    let beneficiaries = vec!["Juan Perez".to_string(), "Maria Lopez".to_string()];
    let mut policy = InsurancePolicy::new(20, 100.0, 100000.0, beneficiaries);

    policy.make_payment();

    // Verificar la autenticidad de la muerte del titular (en este caso se asume que es verdadera)
    let death_verification = true;

    if let Some(beneficiaries) = policy.file_claim(death_verification) {
        // Pago del siniestro a los beneficiarios
        println!("Beneficiaries: {:?}", beneficiaries);
        // Implementar la lógica de pago a los beneficiarios aquí
    } else {
        println!("No eligible claim.");
    }
}
