use chrono::{Datelike, Duration, NaiveDate};

use super::free_days::{Festivo, festivos};

pub const MESES: [&str; 12] = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
];

pub const DIAS_SEMANA: [&str; 7] = ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado", "Domingo"];

#[derive(Debug, Clone)]
pub struct Dia {
    pub numero: u32,
    pub domingo: bool,
    pub hoy: bool,
    pub festivos: Vec<Festivo>,
}

#[derive(Debug, Clone)]
pub struct Mes {
    pub nombre: &'static str,
    /// Semanas de lunes a domingo; `None` = celda vacía fuera del mes.
    pub semanas: Vec<[Option<Dia>; 7]>,
}

/// Los 12 meses de `anio`, con sus festivos marcados.
pub fn calendario(anio: i32, hoy: NaiveDate) -> Vec<Mes> {
    let lista = festivos(anio);
    (1..=12)
        .map(|mes| {
            let primero = NaiveDate::from_ymd_opt(anio, mes, 1).unwrap();
            let mut semanas = Vec::new();
            let mut semana: [Option<Dia>; 7] = Default::default();
            let mut fecha = primero;
            while fecha.month() == mes {
                let col = fecha.weekday().num_days_from_monday() as usize;
                semana[col] = Some(Dia {
                    numero: fecha.day(),
                    domingo: col == 6,
                    hoy: fecha == hoy,
                    festivos: lista.iter().filter(|f| f.fecha == fecha).cloned().collect(),
                });
                if col == 6 {
                    semanas.push(std::mem::take(&mut semana));
                }
                fecha += Duration::days(1);
            }
            if semana.iter().any(Option::is_some) {
                semanas.push(semana);
            }
            Mes { nombre: MESES[mes as usize - 1], semanas }
        })
        .collect()
}

pub fn nombre_dia(fecha: NaiveDate) -> &'static str {
    DIAS_SEMANA[fecha.weekday().num_days_from_monday() as usize]
}

/// "Lunes 12 de octubre de 2026"
pub fn fecha_larga(fecha: NaiveDate) -> String {
    format!(
        "{} {} de {} de {}",
        nombre_dia(fecha),
        fecha.day(),
        MESES[fecha.month0() as usize].to_lowercase(),
        fecha.year()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn octubre_2026() {
        let hoy = NaiveDate::from_ymd_opt(2026, 10, 11).unwrap();
        let meses = calendario(2026, hoy);
        let oct = &meses[9];
        assert_eq!(oct.nombre, "Octubre");
        // 1 de octubre de 2026 es jueves.
        assert!(oct.semanas[0][2].is_none());
        assert_eq!(oct.semanas[0][3].as_ref().unwrap().numero, 1);
        let doce = oct.semanas[2][0].as_ref().unwrap();
        assert_eq!(doce.numero, 12);
        assert_eq!(doce.festivos[0].nombre, "Día de la Raza");
        assert!(oct.semanas[1][6].as_ref().unwrap().hoy);
    }

    #[test]
    fn fecha_en_espanol() {
        let f = NaiveDate::from_ymd_opt(2026, 10, 12).unwrap();
        assert_eq!(fecha_larga(f), "Lunes 12 de octubre de 2026");
    }
}
