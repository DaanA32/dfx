use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use dfx::{
    message::Message,
    session::{
        Application, ApplicationError, ApplicationExt, DoNotAccept, FromAppError, LogonReject,
        Session,
    },
    tags,
};

#[derive(Clone)]
pub struct TestApplication {
    ids_seen: Arc<Mutex<HashSet<String>>>,
    stop_me_event: Option<u32>,
}
impl TestApplication {
    pub fn new() -> Self {
        TestApplication {
            ids_seen: Arc::new(Mutex::new(HashSet::new())),
            stop_me_event: None,
        }
    }
    fn seen(&self, cl_ord_id: &String) -> bool {
        match self.ids_seen.lock() {
            Ok(seen) => {
                if seen.contains(cl_ord_id) {
                    println!("Contained {}", cl_ord_id);
                    true
                } else {
                    println!("Did not contain {}", cl_ord_id);
                    false
                }
            }
            Err(e) => panic!("Poisened {e:?}"),
        }
    }
    fn insert(&self, cl_ord_id: String) {
        match self.ids_seen.lock() {
            Ok(mut seen) => {
                seen.insert(cl_ord_id);
            }
            Err(e) => panic!("Poisened {e:?}"),
        }
    }
    pub fn clear(&self) {
        match self.ids_seen.lock() {
            Ok(mut seen) => seen.clear(),
            Err(e) => panic!("Poisened {e:?}"),
        }
    }
    fn echo(&self, message: &Message, session_id: &dfx::session_id::SessionId) {
        Session::send_to_session(session_id, message.clone()).unwrap();
    }
    fn handle_nos(
        &self,
        message: &Message,
        session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::field_map::FieldMapError> {
        println!("handle_nos");
        let poss_resend = message.header().get_field(tags::PossResend).is_some()
            && message.header().get_bool(tags::PossResend);

        let cl_ord_id = message.get_string(tags::ClOrdID)?;
        if poss_resend && self.seen(&cl_ord_id) {
        } else {
            self.insert(cl_ord_id);
            Session::send_to_session(session_id, message.clone()).unwrap();
        }
        Ok(())
    }
    fn handle_news(
        &self,
        message: &Message,
        session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::field_map::FieldMapError> {
        println!("handle_news");
        if message.is_field_set(tags::Headline) && message.get_string(tags::Headline)? == "STOPME" {
            if let Some(event) = self.stop_me_event {
                todo!("STOPME: {:?}", event);
            }
        } else {
            Session::send_to_session(session_id, message.clone()).unwrap();
        }
        Ok(())
    }
}
impl Application for TestApplication {
    fn on_create(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), DoNotAccept> {
        println!("TestApplication: {}", _session_id);
        Ok(())
    }

    fn on_logon(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), LogonReject> {
        println!("TestApplication Logon: {}", _session_id);
        Ok(())
    }

    fn on_logout(
        &mut self,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication Logout: {}", _session_id);
        Ok(())
    }

    fn to_admin(
        &mut self,
        message: Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<Message, dfx::field_map::FieldMapError> {
        println!("TestApplication To Admin: {}", _session_id);
        Ok(message)
    }

    fn from_admin(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::field_map::FieldMapError> {
        println!("TestApplication From Admin: {}", _session_id);
        Ok(())
    }

    fn to_app(
        &mut self,
        _message: &mut Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication To App: {}", _session_id);
        Ok(())
    }

    fn from_app(
        &mut self,
        message: &Message,
        session_id: &dfx::session_id::SessionId,
    ) -> Result<(), FromAppError> {
        println!("TestApplication From App: {}", session_id);

        let msg_type = message.header().get_string(tags::MsgType)?;
        println!("{}", msg_type);
        match msg_type.as_str() {
            "D" => self.handle_nos(message, session_id)?,
            "d" => self.echo(message, session_id),
            "B" => self.handle_news(message, session_id)?,
            "AE" => {}
            "AD" => {}
            "R" => self.echo(message, session_id),
            _ => {
                return Err(FromAppError::UnknownMessageType {
                    message: message.clone(),
                    msg_type,
                })
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct SendTestApplication;
impl SendTestApplication {
    pub fn new() -> Self {
        SendTestApplication
    }
}
impl ApplicationExt for SendTestApplication {
    fn early_intercept(
        &mut self,
        message: Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<Message, ApplicationError> {
        panic!("{}", message);
        //Ok(message)
    }
}
impl Application for SendTestApplication {
    fn on_create(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), DoNotAccept> {
        println!("TestApplication: {}", _session_id);
        Ok(())
    }

    fn on_logon(&mut self, session_id: &dfx::session_id::SessionId) -> Result<(), LogonReject> {
        println!("TestApplication Logon: {}", session_id);
        let mut message = Message::default();
        message.header_mut().set_tag_value(tags::MsgType, "V");

        Session::send_to_session(session_id, message.clone()).unwrap_or(());
        Session::send_to_session(session_id, message).unwrap_or(());
        Ok(())
    }

    fn on_logout(
        &mut self,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication Logout: {}", _session_id);
        Ok(())
    }

    fn to_admin(
        &mut self,
        message: Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<Message, dfx::field_map::FieldMapError> {
        println!("TestApplication To Admin: {}", _session_id);
        Ok(message)
    }

    fn from_admin(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::field_map::FieldMapError> {
        println!("TestApplication From Admin: {}", _session_id);
        Ok(())
    }

    fn to_app(
        &mut self,
        _message: &mut Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication To App: {}", _session_id);
        Ok(())
    }

    fn from_app(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), FromAppError> {
        println!("TestApplication From App: {}", _session_id);
        Ok(())
    }
}
