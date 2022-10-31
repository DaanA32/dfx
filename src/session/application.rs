use crate::session::SessionId;
use crate::message::Message;

#[derive(Debug, Clone)]
pub enum ApplicationError {
    DoNotAccept,
    LogonReject,
    DoNotSend,
}

pub trait Application {
    fn on_create(&mut self, session_id: &SessionId) -> Result<(), ApplicationError>;
    fn on_logon(&mut self, session_id: &SessionId) -> Result<(), ApplicationError>;
    fn on_logout(&mut self, session_id: &SessionId) -> Result<(), ApplicationError>;
    fn to_admin(
        &mut self,
        message: Message,
        session_id: &SessionId,
    ) -> Result<Message, ApplicationError>;
    fn from_admin(
        &mut self,
        message: Message,
        session_id: &SessionId,
    ) -> Result<(), ApplicationError>;
    fn to_app(
        &mut self,
        message: Message,
        session_id: &SessionId,
    ) -> Result<Message, ApplicationError>;
    fn from_app(
        &mut self,
        message: Message,
        session_id: &SessionId,
    ) -> Result<(), ApplicationError>;
}

#[cfg(test)]
mod tests {
    use session_id::SessionId;

    use super::Application;
    use crate::session::session_id;
    use crate::message::Message;

    struct TestApplication;

    impl Application for TestApplication {

        fn on_create(
            &mut self,
            _session_id: &crate::session::SessionId,
        ) -> Result<(), super::ApplicationError> {
            todo!()
        }

        fn on_logon(
            &mut self,
            _session_id: &crate::session::SessionId,
        ) -> Result<(), super::ApplicationError> {
            todo!()
        }

        fn on_logout(
            &mut self,
            _session_id: &crate::session::SessionId,
        ) -> Result<(), super::ApplicationError> {
            todo!()
        }

        fn to_admin(
            &mut self,
            message: Message,
            _session_id: &crate::session::SessionId,
        ) -> Result<Message, super::ApplicationError> {
            Ok(message)
        }

        fn from_admin(
            &mut self,
            _message: Message,
            _session_id: &crate::session::SessionId,
        ) -> Result<(), super::ApplicationError> {
            todo!()
        }

        fn to_app(
            &mut self,
            _message: Message,
            _session_id: &crate::session::SessionId,
        ) -> Result<Message, super::ApplicationError> {
            todo!()
        }

        fn from_app(
            &mut self,
            _message: Message,
            _session_id: &crate::session::SessionId,
        ) -> Result<(), super::ApplicationError> {
            todo!()
        }
    }

    #[test]
    fn test_inject() {
        let session_id = SessionId::new(
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
        );
        let mut app = TestApplication {};
        let msg = Message::default();
        let res = app.to_admin(msg, &session_id);
        assert!(res.is_ok());
        let _res = res.unwrap();
        //assert!(res.a == 0);
        let msg = Message::default();
        //msg.c = true;
        let res = app.to_admin(msg, &session_id);
        assert!(res.is_ok());
        let _res = res.unwrap();
        //assert!(res.a == 1234);
    }
}
