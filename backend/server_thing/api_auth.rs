// use std::fs;
// use std::path::PathBuf;

// use argon2::{self, Config};
// use async_graphql::*;
// use dirs::data_dir;
// use jwt_simple::prelude::*;
// use sea_orm::entity::prelude::*;
// use sea_orm::entity::Set;

// pub fn get_secret_key() -> HS256Key {
//     let mut path = PathBuf::new();

//     if let Some(dir) = data_dir() {
//         path.push(dir);
//         path.push("Budgeteer");
//     };

//     // Create all necessary folders in path
//     fs::create_dir_all(&path).unwrap();

//     path.push("server_secret");

//     // Output to the terminal
//     println!("Key path: {}", path.to_str().unwrap());

//     if let Ok(fkey) = fs::read(&path) {
//         let key = HS256Key::from_bytes(&fkey);
//         println!("Loaded secret key: {:?}", key);
//         key
//     } else {
//         let key = HS256Key::generate();
//         let bytes = key.to_bytes();
//         fs::write(path, bytes).unwrap();
//         println!("Created secret key: {:?}", key);
//         key
//     }
// }

// #[derive(Serialize, Deserialize)]
// pub struct UserClaim
// {
//     username: String,
//     id: i32,
// }
// pub struct Token( pub String );

// pub fn create_jwt( username: String, id: i32, key: &HS256Key ) -> String
// {
//     let time = Duration::from_hours(2);
//     let user_claim = UserClaim { username, id };
//     let claim = Claims::with_custom_claims( user_claim, time );
    
//     // Create token
//     key.authenticate(claim).expect( "Failed to create token!" )
// }

// pub fn authenticate( ctx: &Context<'_> ) -> Result<UserClaim, Error>
// {
//     // Set up verification options, don't allow expired tokens at all!
//     let mut options = VerificationOptions::default();
//     options.time_tolerance = Some( Duration::from_mins( 0 ) );

//     // Get details from the request
//     let skey = ctx.data_unchecked::<HS256Key>();
//     let token = &ctx.data::<Token>()?.0;

//     // Go through the verification process
//     Ok( skey.verify_token::<UserClaim>(&token, Some( options ))?.custom )
// }

// #[derive(Default)]
// pub struct AuthQuery;
// #[derive(Default)]
// pub struct AuthMutation;

// // Interface structs
// #[derive(InputObject)]
// pub struct UserInput {
//     pub username: String,
//     pub key: String,
// }

// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
// #[graphql(name = "user")]
// #[sea_orm(table_name = "user")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub username: String,
//     pub key: String,
// }
// type User = Model;

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}

// impl ActiveModelBehavior for ActiveModel {}

// #[Object]
// impl AuthQuery {
//     /// Returns the sum of a and b
//     async fn user_validate(
//         &self,
//         ctx: &Context<'_>,
//         user: String,
//         password: String,
//     ) -> Result<bool, Error> {
//         let conn = ctx.data_unchecked::<DatabaseConnection>();
//         let skey = ctx.data_unchecked::<HS256Key>();

//         // Find the requested server entity
//         let user = &Entity::find()
//             .filter(Column::Username.eq(user))
//             .all(conn)
//             .await
//             .expect("Request failed...")[0];

//         // Test given password
//         let verified = argon2::verify_encoded(&user.key, password.as_bytes()).unwrap();

//         // Construct token
//         if verified {
//             let token = create_jwt( user.username.clone(), user.id, skey );
//             println!("Created token: {}", token);

//             // let verified = skey.verify_token(&token, None);
//             ctx.append_http_header("Set-Cookie", format!("token={}", token));
//             Ok(true)
//         }
//         else {
//             Err( Error::new( "Sorry, you suck" ) )
//         }
        
//     }

//     /// Returns a list of all the usernames and keys in the system
//     /// Requires user to be logged in!
//     async fn list_users(&self, ctx: &Context<'_>) -> Result<Vec<User>, Error> {
//         // Make sure user is logged in
//         authenticate(ctx)?;

//         let conn = ctx.data_unchecked::<DatabaseConnection>();
//         Ok( Entity::find().all(conn).await.unwrap() )
//     }
// }

// #[Object]
// impl AuthMutation {
//     async fn user_add(&self, ctx: &Context<'_>, new_guy: UserInput) -> User {
//         let salt = format!("Budgeteer{}", new_guy.username);
//         let config = Config::default();
//         let hash = argon2::hash_encoded(new_guy.key.as_bytes(), salt.as_bytes(), &config).unwrap();

//         let conn = ctx.data_unchecked::<DatabaseConnection>();
//         let active = ActiveModel {
//             username: Set(new_guy.username),
//             key: Set(hash),
//             ..Default::default()
//         };
//         active.insert(conn).await.unwrap()
//     }
// }
