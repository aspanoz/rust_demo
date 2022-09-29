use super::{
   Code::{self, *},
   Db,
};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tide::{http, log::*};
use tide_websockets_sink::{Message, WebSocket};

#[derive(Serialize, Deserialize, Debug)]
struct ClientSignal {
   code: Code,
   id: Option<usize>,
}

pub async fn run(state: Db) -> Result<()> {
   let mut app = tide::with_state(state);

   app.at("/").get(|_| async move {
      Ok(tide::Response::builder(200)
         .body(super::web_view::INDEX_HTML)
         .content_type(http::mime::HTML)
         .build())
   });

   app.at("/media/:idx")
      .get(|req: tide::Request<Db>| async move {
         let state = req.state().lock().expect("Unable to lock state");
         let id = req.param("idx")?.parse::<usize>()?;
         async_std::task::block_on(async {
            let mut res = tide::Response::new(tide::StatusCode::Ok);
            res.set_body(tide::Body::from_file(state.get_file_path(id)).await?);
            Ok(res)
         })
      });

   app.at("/ws").get(WebSocket::new(
      |request: tide::Request<Db>, stream| async move {
         info!("new websocket opened");
         let state = request.state();

         let (send_ws_msg_tx, send_ws_msg_rx) = async_std::channel::unbounded::<Message>();
         let mut send_ws_msg_rx = send_ws_msg_rx.fuse();

         state
            .lock()
            .expect("Unable to lock state")
            .add_connection(send_ws_msg_tx);

         let mut stream = stream.fuse();
         loop {
            let ws_msg = futures::select! {
                ws_msg = stream.select_next_some() => {
                     match ws_msg {
                        Ok(Message::Close(_)) => {
                            info!("peer disconnected");
                            break
                        },

                        Ok(Message::Text(data)) => {
                           match serde_json::from_str(&data).expect("Unable to get client signal") {

                              ClientSignal { code: GetRandomMedia, .. } => {
                                  let mut store = state.lock().expect("Unable to lock state");
											 let message = store.get_rand_media_id();
                                  Some(Message::Text(message))
                              },

                              ClientSignal { code: MoveMediaToTrash, id } if id.is_some() => {
                                  let mut store = state.lock().expect("Unable to lock state");

                                  let id = id.unwrap();
                                  debug!("recive trigger for move media with id={} into trash", id);

                                  store.move_to_trash(id);
                                  Some(Message::Text(store.get_rand_media_id()))
                              },

                              ClientSignal { code: SetLike, id } if id.is_some() => {
                                  let mut store = state.lock().expect("Unable to lock state");
                                  let id = id.unwrap();

                                  Some(Message::Text(store.set_like_by_id(id)))
                              },

                              _ => None
                        }},

                        _ => None
                    }
                },

                ws_msg = send_ws_msg_rx.select_next_some() => Some(ws_msg),
                complete => break,
            };

            if let Some(ws_msg) = ws_msg {
               stream.send(ws_msg).await?;
            }
         }

         Ok(())
      },
   ));

   info!("http://127.0.0.1:3001");
   app.listen("0.0.0.0:3001").await?;

   Ok(())
}
