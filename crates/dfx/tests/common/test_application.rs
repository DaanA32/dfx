use dfx::{
    message::Message,
    session::{Application, ApplicationError, ApplicationExt, Session},
    tags,
};

#[derive(Clone)]
pub struct TestApplication;
impl TestApplication {}
impl Application for TestApplication {
    fn on_create(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), ApplicationError> {
        println!("TestApplication: {}", _session_id);
        Ok(())
    }

    fn on_logon(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), ApplicationError> {
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
    ) -> Result<Message, ApplicationError> {
        println!("TestApplication To Admin: {}", _session_id);
        Ok(message)
    }

    fn from_admin(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication From Admin: {}", _session_id);
        Ok(())
    }

    fn to_app(
        &mut self,
        _message: Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<Message, ApplicationError> {
        println!("TestApplication To App: {}", _session_id);
        Ok(_message)
    }

    fn from_app(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication From App: {}", _session_id);
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
    fn on_create(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), ApplicationError> {
        println!("TestApplication: {}", _session_id);
        Ok(())
    }

    fn on_logon(&mut self, session_id: &dfx::session_id::SessionId) -> Result<(), ApplicationError> {
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
    ) -> Result<Message, ApplicationError> {
        println!("TestApplication To Admin: {}", _session_id);
        Ok(message)
    }

    fn from_admin(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication From Admin: {}", _session_id);
        Ok(())
    }

    fn to_app(
        &mut self,
        _message: Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<Message, ApplicationError> {
        println!("TestApplication To App: {}", _session_id);
        Ok(_message)
    }

    fn from_app(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("TestApplication From App: {}", _session_id);
        Ok(())
    }
}
