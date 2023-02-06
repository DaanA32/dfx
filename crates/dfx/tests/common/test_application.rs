use std::{collections::HashSet, sync::{Mutex, Arc}};

use dfx::{
    message::Message,
    session::{Application, ApplicationError, ApplicationExt, Session, DoNotAccept, LogonReject},
    tags, fields::MsgType,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref SEEN: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
}

#[derive(Clone)]
pub struct TestApplication;
impl TestApplication {
    fn seen(&self, cl_ord_id: String) -> bool {
        match SEEN.lock() {
            Ok(mut seen) => {
                if seen.contains(&cl_ord_id) {
                    println!("Contained {}", cl_ord_id);
                    true
                } else {
                    println!("Did not contain {}", cl_ord_id);
                    seen.insert(cl_ord_id);
                    false
                }
            },
            Err(e) => panic!("Poisened {e:?}"),
        }
    }
    pub fn clear() {
        match SEEN.lock() {
            Ok(mut seen) => {
                seen.clear()
            },
            Err(e) => panic!("Poisened {e:?}"),
        }
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

    fn on_logout(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), ApplicationError> {
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
    ) -> Result<(), dfx::field_map::FieldMapError> {
        println!("TestApplication From App: {}", _session_id);
        if _message.header().get_field(tags::PossResend).is_none()
        || !_message.header().get_bool(tags::PossResend) {
            let msg_type = _message.header().get_string(tags::MsgType);
            if matches!(&msg_type, Ok(d) if d == "D") && _message.get_field(tags::ClOrdID).is_some() {
                println!("Adding to SEEN");
                self.seen(_message.get_string(tags::ClOrdID)?);
            }
            Session::send_to_session(_session_id, _message.clone()).unwrap();

        } else if _message.header().get_bool(tags::PossResend) {
            //TODO skip if NewOrderSingle and ID has been seen
            println!("PossDup!");
            let msg_type = _message.header().get_string(tags::MsgType);
            if matches!(&msg_type, Ok(d) if d == "D")
                && !self.seen(_message.get_string(tags::ClOrdID)?)
            {
                println!("Was not SEEN");
                Session::send_to_session(_session_id, _message.clone()).unwrap();
            } else {
                println!("Was     SEEN");
                println!("{msg_type:?}");
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct SendTestApplication;
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

    fn on_logout(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), ApplicationError> {
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
    ) -> Result<(), dfx::field_map::FieldMapError> {
        println!("TestApplication From App: {}", _session_id);
        Ok(())
    }
}
