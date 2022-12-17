use dfx_core::field_map::FieldMapError;
use dfx_core::message::Message;
use dfx_core::session_id::SessionId;

#[derive(Debug, Clone)]
pub enum ApplicationError {
    DoNotAccept,
    LogonReject,
    DoNotSend(Box<Message>),
    FieldMapError(FieldMapError),
}

impl From<FieldMapError> for ApplicationError {
    fn from(e: FieldMapError) -> Self {
        ApplicationError::FieldMapError(e)
    }
}

pub trait Application: Send {
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
        message: &Message,
        session_id: &SessionId,
    ) -> Result<(), ApplicationError>;
    fn to_app(
        &mut self,
        message: &mut Message,
        session_id: &SessionId,
    ) -> Result<(), ApplicationError>;
    fn from_app(
        &mut self,
        message: &Message,
        session_id: &SessionId,
    ) -> Result<(), ApplicationError>;
}

pub trait ApplicationExt: Application {
    fn early_intercept(
        &mut self,
        message: Message,
        session_id: &SessionId,
    ) -> Result<Message, ApplicationError>;
}

#[cfg(test)]
pub mod tests {
    use dfx_core::session_id::SessionId;

    use super::{Application, ApplicationExt};
    use dfx_core::message::Message;
    use crate::session;

    pub struct TestApplication;

    impl Application for TestApplication {
        fn on_create(
            &mut self,
            _session_id: &SessionId,
        ) -> Result<(), super::ApplicationError> {
            Ok(())
        }

        fn on_logon(
            &mut self,
            _session_id: &SessionId,
        ) -> Result<(), super::ApplicationError> {
            Ok(())
        }

        fn on_logout(
            &mut self,
            _session_id: &SessionId,
        ) -> Result<(), super::ApplicationError> {
            Ok(())
        }

        fn to_admin(
            &mut self,
            message: Message,
            _session_id: &SessionId,
        ) -> Result<Message, super::ApplicationError> {
            Ok(message)
        }

        fn from_admin(
            &mut self,
            _message: &Message,
            _session_id: &SessionId,
        ) -> Result<(), super::ApplicationError> {
            Ok(())
        }

        fn to_app(
            &mut self,
            _message: &mut Message,
            _session_id: &SessionId,
        ) -> Result<(), super::ApplicationError> {
            Ok(())
        }

        fn from_app(
            &mut self,
            _message: &Message,
            _session_id: &SessionId,
        ) -> Result<(), super::ApplicationError> {
            Ok(())
        }
    }

    #[test]
    fn test_inject() {
        let session_id = SessionId::new("", "", "", "", "", "", "");
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

    impl ApplicationExt for TestApplication {
        fn early_intercept(
            &mut self,
            _message: Message,
            _session_id: &SessionId,
        ) -> Result<Message, super::ApplicationError> {
            todo!()
        }
    }
    #[test]
    fn test_ext() {
        // let app = TestApplication {};
        // let boxed: Box<dyn Application> = Box::new(app);
        // let early = super::get_early_intercept(boxed.as_ref());
        // let session_id = SessionId::default();
        // let msg = Message::default();
        // early.unwrap()(boxed.as_mut(), msg, &session_id);
    }
}
