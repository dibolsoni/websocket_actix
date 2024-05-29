#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use actix::Actor;
    use actix_test::start;
    use actix_web::test::{call_service, init_service, TestRequest};
    use actix_web::web::{Bytes, Data};
    use actix_web::test;
    use actix_web_actors::ws;
    use crate::lobby::Lobby;
    use futures_util::{SinkExt, StreamExt};
    use crate::create_app;
    use crate::messages::WsMessage;

    #[actix_web::test]
    async fn test_index() {
        let mut app = init_service(
            create_app()
        ).await;
        let req = TestRequest::get().to_request();
        let resp = call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_hello_path_params() {
        let mut app = init_service(
            create_app()
        ).await;
        let req = TestRequest::get().uri("/hello/actix").to_request();
        let resp = call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello actix!"));
    }

    #[actix_web::test]
    async fn test_another_ws() {
        let counter = Data::new(Mutex::new(0i32)).clone();
        let mut server = start(move || {
            create_app()
                .app_data(counter.clone())
        });
        let mut framed = server.ws_at("/another_ws/20").await.unwrap();
        framed.send(ws::Message::Text("hello ws".into())).await.unwrap();
        framed.send(ws::Message::Text("hello ws!".into())).await.unwrap();
        let mut another_framed = server.ws_at("/another_ws/21").await.unwrap();
        another_framed.send(ws::Message::Text("hello ws#".into())).await.unwrap();

        assert_eq!(
            framed.next().await.unwrap().unwrap(),
            ws::Frame::Text(Bytes::from_static(b"AnotherWsConn started: 20"))
        );
        assert_eq!(
            framed.next().await.unwrap().unwrap(),
            ws::Frame::Text(Bytes::from_static(b"counter: 1 - msg: hello ws"))
        );
        assert_eq!(
            framed.next().await.unwrap().unwrap(),
            ws::Frame::Text(Bytes::from_static(b"counter: 2 - msg: hello ws!"))
        );


        assert_eq!(
            another_framed.next().await.unwrap().unwrap(),
            ws::Frame::Text(Bytes::from_static(b"AnotherWsConn started: 21"))
        );
        assert_eq!(
            another_framed.next().await.unwrap().unwrap(),
            ws::Frame::Text(Bytes::from_static(b"counter: 3 - msg: hello ws#"))
        );
    }

    fn assert_ws_message(msg: ws::Frame, expected: &str) {
        let m = match msg {
            ws::Frame::Text(text) => text,
            _ => panic!("Expected text frame"),
        };
        let msg_bin = m.to_vec();
        let msg_str = String::from_utf8_lossy(&msg_bin);
        let message: WsMessage = serde_json::from_str(&msg_str).unwrap();
        assert_eq!(message.message, expected);
    }

    #[actix_web::test]
    async fn test_ws()
    {
        let lobby = Data::new(Lobby::default().start()).clone();
        let mut server = start(move || {
            create_app()
                .app_data(lobby.clone())
        });
        let mut framed = server.ws_at("/ws/12321412-4212-3212-1312-321212132132").await.unwrap();
        framed.send(ws::Message::Text("hello ws".into())).await.unwrap();

        assert_ws_message(
            framed.next().await.unwrap().unwrap(),
            "Connected Lobby: 12321412-4212-3212-1312-321212132132"
        );

        assert_ws_message(
            framed.next().await.unwrap().unwrap(),
            "hello ws"
        );
    }
}
