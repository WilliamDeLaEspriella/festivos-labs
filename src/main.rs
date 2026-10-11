use askama::Template;
use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use chrono::{Datelike, FixedOffset, NaiveDate, Utc};
use festivos_labs::services::{
    calendar::{Mes, calendario, fecha_larga, nombre_dia, MESES},
    free_days::{Festivo, REGLAS, festivos, siguiente_festivo},
};
use serde::Deserialize;
use tower_http::services::ServeDir;

/// La app solo muestra años desde 2026 en adelante.
const ANIO_MIN: i32 = 2026;
const ANIO_MAX: i32 = 9999;

/// Fecha actual en Colombia (UTC-5, sin horario de verano).
fn hoy_colombia() -> NaiveDate {
    let bogota = FixedOffset::west_opt(5 * 3600).unwrap();
    Utc::now().with_timezone(&bogota).date_naive()
}

struct FestivoVista {
    nombre: &'static str,
    fecha: String,
    original: String,
    trasladado: bool,
    dia_semana: &'static str,
    dia: u32,
    mes: &'static str,
}

impl From<&Festivo> for FestivoVista {
    fn from(f: &Festivo) -> Self {
        Self {
            nombre: f.nombre,
            fecha: fecha_larga(f.fecha),
            original: fecha_larga(f.fecha_original),
            trasladado: f.trasladado,
            dia_semana: nombre_dia(f.fecha),
            dia: f.fecha.day(),
            mes: MESES[f.fecha.month0() as usize],
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    anio: i32,
    anterior: Option<i32>,
    siguiente_anio: Option<i32>,
    proximo: FestivoVista,
    dias_faltan: i64,
    meses: Vec<Mes>,
    lista: Vec<FestivoVista>,
    trasladados: usize,
}

#[derive(Deserialize)]
struct Params {
    anio: Option<String>,
}

async fn index(Query(params): Query<Params>) -> Response {
    let hoy = hoy_colombia();
    let anio = params
        .anio
        .and_then(|a| a.trim().parse::<i32>().ok())
        .unwrap_or(hoy.year())
        .clamp(ANIO_MIN, ANIO_MAX);

    let proximo = siguiente_festivo(hoy);
    let lista = festivos(anio);
    let pagina = Index {
        anio,
        anterior: (anio > ANIO_MIN).then(|| anio - 1),
        siguiente_anio: (anio < ANIO_MAX).then(|| anio + 1),
        dias_faltan: (proximo.fecha - hoy).num_days(),
        proximo: (&proximo).into(),
        meses: calendario(anio, hoy),
        trasladados: lista.iter().filter(|f| f.trasladado).count(),
        lista: lista.iter().map(FestivoVista::from).collect(),
    };

    match pagina.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn api_festivos(Path(anio): Path<i32>) -> Response {
    if !(ANIO_MIN..=ANIO_MAX).contains(&anio) {
        return (
            StatusCode::BAD_REQUEST,
            format!("el año debe estar entre {ANIO_MIN} y {ANIO_MAX}"),
        )
            .into_response();
    }
    Json(festivos(anio)).into_response()
}

async fn api_siguiente() -> Json<Festivo> {
    Json(siguiente_festivo(hoy_colombia()))
}

async fn health() -> &'static str {
    "OK"
}

async fn reglas() -> String {
    format!("{:#?}", REGLAS)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/festivos/{anio}", get(api_festivos))
        .route("/api/siguiente", get(api_siguiente))
        .route("/health", get(health))
        .route("/reglas", get(reglas))
        .nest_service("/static", ServeDir::new("static"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("escuchando en {addr}");

    axum::serve(listener, app).await.unwrap();
}
