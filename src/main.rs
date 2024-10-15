mod model;
mod schema;
use self::model::*;
use self::schema::posts::dsl::*;
use actix_files::{Files,NamedFile};
use actix_web::{web,App,HttpServer,HttpResponse,Result,error,Error};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self,ConnectionManager};
use dotenvy::dotenv;
use std::env;
use std::collections::HashMap;
use awmp::Parts;
type DdPool=r2d2::Pool<ConnectionManager<PgConnection>>;

//this help to setup database
fn setup_database() -> DdPool{
    dotenv().ok();
    let database_url=env::var("DATABASE_URL").expect("Database must be given");
    let manager= ConnectionManager::<PgConnection>::new(&database_url);
    let pool =r2d2::Pool::builder()
                        .build(manager)
                        .expect("Fialled to create DB Pool");
    pool
                        }

//get_data()  receive data form database and sent it to client
async fn get_data(pool:web::Data<DdPool>) -> Result<HttpResponse,Error>
{
    let mut connection = pool.get().expect("Fail to have connection from pool");
    let post_data= web::block( move ||{
        posts.limit(100).load::<Post>(& mut connection)
    }
        ).await
         .map_err(error::ErrorInternalServerError)?;
    let  data =post_data.unwrap();
    
    Ok(HttpResponse::Ok().json(data))
}

// post_data() get data_request, parse them to data and store it to database
async fn post_data(pool:web::Data<DdPool>,
                   mut parts:awmp::Parts,) -> Result<HttpResponse,Error>
{
    let file_parts=parts
                  .files
                  .take("file")
                  .pop()
                  .and_then(|f| f.persist_in("./files").ok())
                  .unwrap_or_default();
   let text_fields:HashMap<_,_> =
       parts.texts.as_pairs().into_iter().collect();

   let mut connection = pool.get().expect("Can't get db connection from pool");

   let new_post= NewPost {
       firstname:text_fields.get("firstname").unwrap().to_string(),
       lastname:text_fields.get("lastname").unwrap().to_string(),
       email:text_fields.get("email").unwrap().to_string(),
       phone:text_fields.get("phone").unwrap().to_string(),
       filepath:file_parts.to_string_lossy().to_string(),
   };

  let _ = web::block(move || 
                       diesel::insert_into(posts)
                              .values(&new_post)
                              .execute(&mut connection)).await
                                                              .map_err(error::ErrorInternalServerError)?;
                                 Ok(HttpResponse::Created().finish())
}
// configuration of App under scope api
 fn api_config(cfg: &mut web::ServiceConfig) -> (){
     cfg.service(
         web::scope("/api")
         .route("/data",web::get().to(get_data))
         .route("/add_data",web::post().to(post_data))
);
 }

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    println!("Listerning on 8080 port");
    let  pool = setup_database();
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(awmp::PartsConfig::default().with_temp_dir("./tmp"))
            .service(
                Files::new("/static","static")
                       .show_files_listing())
            .service(
                Files::new("/file","file")
                        .show_files_listing())
            .configure(api_config)
            })
              .bind("127.0.0.1:8080")?
              .run()
              .await
}

// here is integration test
#[cfg(test)]
mod tests  {   //begining of module test
    use super::*;
    use actix_web ::{test,App};
    async fn test_get_data(){  //main test function    
        let pool = setup_database();
        let mut app = test::init_service( // similar
                                                                                          // to
                                                                                          // httpserver
                                                                                          
                             App::new().app_data(
                                 web::Data::new(pool.clone()))
                                .configure(api_config),
                            ).await ;
            
        let req=test::TestRequest::get() //building request test simular to browser or
                                                  //curl
            .uri("/api/data")
            .to_request();
        let response= test::call_service(&mut app,req).await;//get response
        assert!(response.status().is_success());//compare response
    }
}

                             
