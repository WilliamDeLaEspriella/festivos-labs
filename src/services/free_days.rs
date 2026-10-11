#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tipo {
    /// Fecha fija, no se traslada.
    Fijo { mes: u32, dia: u32 },
    /// Fecha fija que se traslada al lunes siguiente si no cae en lunes (Ley 51 de 1983).
    Lunes { mes: u32, dia: u32 },
    /// Días respecto al Domingo de Pascua. Si `trasladable`, se mueve al lunes siguiente.
    Pascua { dias: i64, trasladable: bool },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Regla {
    pub nombre: &'static str,
    pub tipo: Tipo,
    /// Primer año en que rige este festivo.
    pub desde: i32,
}

pub const ANIO_LEY_EMILIANI: i32 = 1984;

pub const REGLAS: &[Regla] = &[
    // Fijos
    Regla {
        nombre: "Año Nuevo",
        tipo: Tipo::Fijo { mes: 1, dia: 1 },
        desde: 1984,
    },
    Regla {
        nombre: "Día del Trabajo",
        tipo: Tipo::Fijo { mes: 5, dia: 1 },
        desde: 1984,
    },
    Regla {
        nombre: "Día de la Independencia",
        tipo: Tipo::Fijo { mes: 7, dia: 20 },
        desde: 1984,
    },
    Regla {
        nombre: "Batalla de Boyacá",
        tipo: Tipo::Fijo { mes: 8, dia: 7 },
        desde: 1984,
    },
    Regla {
        nombre: "Inmaculada Concepción",
        tipo: Tipo::Fijo { mes: 12, dia: 8 },
        desde: 1984,
    },
    Regla {
        nombre: "Navidad",
        tipo: Tipo::Fijo { mes: 12, dia: 25 },
        desde: 1984,
    },
    // Trasladables al lunes (Ley Emiliani)
    Regla {
        nombre: "Reyes Magos",
        tipo: Tipo::Lunes { mes: 1, dia: 6 },
        desde: 1984,
    },
    Regla {
        nombre: "San José",
        tipo: Tipo::Lunes { mes: 3, dia: 19 },
        desde: 1984,
    },
    Regla {
        nombre: "San Pedro y San Pablo",
        tipo: Tipo::Lunes { mes: 6, dia: 29 },
        desde: 1984,
    },
    Regla {
        nombre: "Virgen del Rosario de Chiquinquirá",
        tipo: Tipo::Lunes { mes: 7, dia: 9 },
        desde: 2026, // Ley 2578 de 2026
    },
    Regla {
        nombre: "Asunción de la Virgen",
        tipo: Tipo::Lunes { mes: 8, dia: 15 },
        desde: 1984,
    },
    Regla {
        nombre: "Día de la Raza",
        tipo: Tipo::Lunes { mes: 10, dia: 12 },
        desde: 1984,
    },
    Regla {
        nombre: "Todos los Santos",
        tipo: Tipo::Lunes { mes: 11, dia: 1 },
        desde: 1984,
    },
    Regla {
        nombre: "Independencia de Cartagena",
        tipo: Tipo::Lunes { mes: 11, dia: 11 },
        desde: 1984,
    },
    // Basados en la Pascua (días originales respecto al Domingo de Pascua)
    Regla {
        nombre: "Jueves Santo",
        tipo: Tipo::Pascua {
            dias: -3,
            trasladable: false,
        },
        desde: 1984,
    },
    Regla {
        nombre: "Viernes Santo",
        tipo: Tipo::Pascua {
            dias: -2,
            trasladable: false,
        },
        desde: 1984,
    },
    Regla {
        nombre: "Ascensión del Señor",
        tipo: Tipo::Pascua {
            dias: 39,
            trasladable: true,
        },
        desde: 1984,
    },
    Regla {
        nombre: "Corpus Christi",
        tipo: Tipo::Pascua {
            dias: 60,
            trasladable: true,
        },
        desde: 1984,
    },
    Regla {
        nombre: "Sagrado Corazón",
        tipo: Tipo::Pascua {
            dias: 68,
            trasladable: true,
        },
        desde: 1984,
    },
];

use chrono::{Datelike, Duration, NaiveDate};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Festivo {
    pub nombre: &'static str,
    /// Día en que se descansa.
    pub fecha: NaiveDate,
    /// Día en que cae la celebración antes de aplicar la Ley Emiliani.
    pub fecha_original: NaiveDate,
    pub trasladado: bool,
}

/// Domingo de Pascua (algoritmo anónimo gregoriano / Meeus-Jones-Butcher).
pub fn domingo_de_pascua(anio: i32) -> NaiveDate {
    let a = anio % 19;
    let b = anio / 100;
    let c = anio % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let mes = (h + l - 7 * m + 114) / 31;
    let dia = (h + l - 7 * m + 114) % 31 + 1;
    NaiveDate::from_ymd_opt(anio, mes as u32, dia as u32).expect("fecha de Pascua válida")
}

/// Lunes siguiente (o el mismo día si ya es lunes).
pub fn al_lunes(fecha: NaiveDate) -> NaiveDate {
    let dias = (7 - fecha.weekday().num_days_from_monday()) % 7;
    fecha + Duration::days(dias as i64)
}

impl Regla {
    /// Fechas (original, observada) de esta regla en `anio`, o `None` si aún no regía.
    pub fn fechas(&self, anio: i32) -> Option<(NaiveDate, NaiveDate)> {
        if anio < self.desde {
            return None;
        }
        let emiliani = anio >= ANIO_LEY_EMILIANI;
        let (original, trasladable) = match self.tipo {
            Tipo::Fijo { mes, dia } => (NaiveDate::from_ymd_opt(anio, mes, dia)?, false),
            Tipo::Lunes { mes, dia } => (NaiveDate::from_ymd_opt(anio, mes, dia)?, true),
            Tipo::Pascua { dias, trasladable } => {
                (domingo_de_pascua(anio) + Duration::days(dias), trasladable)
            }
        };
        let observada = if trasladable && emiliani {
            al_lunes(original)
        } else {
            original
        };
        Some((original, observada))
    }
}

/// Festivos de Colombia en `anio`, ordenados por fecha observada.
pub fn festivos(anio: i32) -> Vec<Festivo> {
    let mut lista: Vec<Festivo> = REGLAS
        .iter()
        .filter_map(|regla| {
            let (fecha_original, fecha) = regla.fechas(anio)?;
            Some(Festivo {
                nombre: regla.nombre,
                fecha,
                fecha_original,
                trasladado: fecha != fecha_original,
            })
        })
        .collect();
    lista.sort_by_key(|f| f.fecha);
    lista
}

/// Primer festivo en `desde` o después.
pub fn siguiente_festivo(desde: NaiveDate) -> Festivo {
    (desde.year()..)
        .flat_map(festivos)
        .find(|f| f.fecha >= desde)
        .expect("siempre hay un festivo en el futuro")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(anio: i32, mes: u32, dia: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(anio, mes, dia).unwrap()
    }

    #[test]
    fn pascua_conocida() {
        assert_eq!(domingo_de_pascua(2026), d(2026, 4, 5));
        assert_eq!(domingo_de_pascua(2027), d(2027, 3, 28));
        assert_eq!(domingo_de_pascua(2030), d(2030, 4, 21));
    }

    #[test]
    fn al_lunes_respeta_lunes() {
        assert_eq!(al_lunes(d(2026, 10, 12)), d(2026, 10, 12)); // lunes
        assert_eq!(al_lunes(d(2026, 1, 6)), d(2026, 1, 12)); // martes
        assert_eq!(al_lunes(d(2026, 11, 1)), d(2026, 11, 2)); // domingo
    }

    #[test]
    fn festivos_2026() {
        let fechas: Vec<NaiveDate> = festivos(2026).iter().map(|f| f.fecha).collect();
        assert_eq!(
            fechas,
            vec![
                d(2026, 1, 1),
                d(2026, 1, 12),
                d(2026, 3, 23),
                d(2026, 4, 2),
                d(2026, 4, 3),
                d(2026, 5, 1),
                d(2026, 5, 18),
                d(2026, 6, 8),
                d(2026, 6, 15),
                d(2026, 6, 29),
                d(2026, 7, 13),
                d(2026, 7, 20),
                d(2026, 8, 7),
                d(2026, 8, 17),
                d(2026, 10, 12),
                d(2026, 11, 2),
                d(2026, 11, 16),
                d(2026, 12, 8),
                d(2026, 12, 25),
            ]
        );
    }

    #[test]
    fn marca_trasladados() {
        let f = festivos(2026);
        let reyes = f.iter().find(|f| f.nombre == "Reyes Magos").unwrap();
        assert!(reyes.trasladado);
        assert_eq!(reyes.fecha_original, d(2026, 1, 6));
        let raza = f.iter().find(|f| f.nombre == "Día de la Raza").unwrap();
        assert!(!raza.trasladado);
    }

    #[test]
    fn regla_desde_filtra() {
        assert!(festivos(2025).iter().all(|f| f.nombre != "Virgen del Rosario de Chiquinquirá"));
    }

    #[test]
    fn siguiente() {
        assert_eq!(siguiente_festivo(d(2026, 10, 11)).fecha, d(2026, 10, 12));
        assert_eq!(siguiente_festivo(d(2026, 10, 12)).nombre, "Día de la Raza");
        assert_eq!(siguiente_festivo(d(2026, 12, 26)).fecha, d(2027, 1, 1));
    }
}
