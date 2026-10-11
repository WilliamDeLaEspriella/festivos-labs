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
