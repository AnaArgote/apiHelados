use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mysql::{prelude::Queryable};
use mysql::Pool;
use serde::{Deserialize, Serialize};
use mysql::params;

// ... Conexión con MySql ...
fn establecer_conexion_mysql() -> mysql::Pool {
    mysql::Pool::new("mysql://ana_argote:AnaArgote@192.168.1.77/TiendaDeHelados")
        .expect("Error al conectar a MySQL")
}
//.........................................................................................
//.........................................................................................
// ... GET ...
#[derive(Debug, Serialize, Deserialize)]
struct Helado {
    id: i32,
    sabor: String,
    stock: f64
}
async fn obtener_helados(pool: web::Data<Pool>) -> impl Responder {
    let pool: &mysql::Pool = &pool.into_inner();
    let mut conn = pool.get_conn().expect("Error al obtener la conexión MySQL");

    let query = "SELECT * FROM helados";
    let helados: Vec<Helado> = conn.query_map(query, |(id, sabor, stock)| {
        Helado {
            id,
            sabor,
            stock,
        }
    }).expect("Error al ejecutar la consulta");

    HttpResponse::Ok().json(helados)
}

//.........................................................................................
//.........................................................................................
// ... DELETE ...
async fn eliminar_helado(
    web::Path(id): web::Path<i32>,
    pool: web::Data<mysql::Pool>,
) -> HttpResponse {
    let pool: &mysql::Pool = &pool.into_inner();
    let mut conn = pool.get_conn().expect("Error al obtener la conexión MySQL");

    let delete_query = "DELETE FROM helados WHERE id = ?";
    let result = conn.exec_drop(delete_query, (id,));

    match result {
        Ok(_) => HttpResponse::Ok().body("Helado eliminado exitosamente"),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar el helado"),
    }
}

//.........................................................................................
//.........................................................................................
// ... INSERT ...
#[derive(Debug, Serialize, Deserialize)]
struct HeladoInsertRequest {
    sabor: String,
    stock: f64
}

async fn agregar_helado(
    helado: web::Json<HeladoInsertRequest>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let pool: &mysql::Pool = &pool.into_inner();
    let mut conn = pool.get_conn().expect("Error al obtener la conexión MySQL");

    let insert_query = "INSERT INTO helados (sabor, stock) VALUES (?, ?)";
    let helado_data = helado.into_inner();
    let result = conn.exec_drop(
        insert_query,
        (helado_data.sabor, helado_data.stock,)
    );

    match result {
        Ok(_) => HttpResponse::Created().json("Helado agregado exitosamente"),
        Err(_) => HttpResponse::InternalServerError().json("Error al agregar el helado"),
    }
}

//.........................................................................................
//.........................................................................................
// ... UPDATE ...
#[derive(Debug, Serialize, Deserialize)]
struct ModificarHelado {
    sabor: String,
    stock: f64
}

async fn modificar_helado(
    web::Path(id): web::Path<i32>,
    nuevo_helado: web::Json<ModificarHelado>,
    pool: web::Data<mysql::Pool>,
) -> HttpResponse {
    let pool: &mysql::Pool = &pool.into_inner();
    let mut conn = pool.get_conn().expect("Error al obtener la conexión MySQL");

    let update_query = "UPDATE helados SET sabor = ?, stock = ? WHERE id = ?";
    let nuevo_helado_data = nuevo_helado.into_inner();
    let result = conn.exec_drop(
        update_query,
        (nuevo_helado_data.sabor, nuevo_helado_data.stock, id),
    );

    match result {
        Ok(_) => HttpResponse::Ok().body("Helado modificado exitosamente"),
        Err(_) => HttpResponse::InternalServerError().body("Error al modificar el helado"),
    }
}

//.........................................................................................
//.........................................................................................
// ... RUTAS DE LAS APIS ...
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establecer_conexion_mysql();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/helados", web::get().to(obtener_helados))
            .route("/helados/{id}", web::put().to(modificar_helado))
            .route("/helados", web::post().to(agregar_helado))
            .route("/helados/{id}", web::delete().to(eliminar_helado))
            
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
