use crate::models::ranking::GameRankingList;

use warp::{reject::custom, reply, Rejection, Reply};

use crate::db::sqlite::SQLITEPOOL;

use log::error;

use super::INTERNAL_SERVER_ERROR;

pub async fn game_ranking_list() -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            let game_ranking_list = GameRankingList::rank(&conn);

            match game_ranking_list {
                Ok(data) => Ok(reply::json(&data)),
                Err(e) => {
                    error!("{:#?}", e);
                    Err(custom(INTERNAL_SERVER_ERROR))
                }
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };

    response
}

// pub async fn game_ranking_list() -> Result<impl Reply, Rejection> {
//     let response = match SQLITEPOOL.get() {
//         Ok(conn) => {
//             let mut ctx = Context::new();

//             let game_ranking_list = GameRankingList::rank(&conn);

//             match game_ranking_list {
//                 Ok(data) => {
//                     // ctx.insert("game_ranking_list", &items);
//                     // let payload = render("ranking.tera", &ctx)?;
//                     Ok(reply::json(&data))
//                 }
//                 Err(e) => {
//                     error!("{:#?}", e);
//                     Err(custom(INTERNAL_SERVER_ERROR))
//                 }
//             }
//         },
//         Err(e) => {
//             error!("{:#?}", e);
//             Err(custom(INTERNAL_SERVER_ERROR))
//         }
//     };

//     response
// }
