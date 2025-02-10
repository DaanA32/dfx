use dfx_base::field_map::FieldMapError;
use dfx_base::message::Message;
use dfx_base::session_id::SessionId;

#[derive(Debug, Clone)]
pub enum ApplicationError {
    // DoNotAccept,
    // LogonReject,
    DoNotSend,
    FieldMapError(FieldMapError),
}

#[derive(Debug, Clone)]
pub enum FromAppError {
    UnknownMessageType { message: Message, msg_type: String },
    FieldMapError(FieldMapError),
}

#[derive(Debug, Clone)]
pub struct LogonReject {
    pub reason: Option<String>,
}
#[derive(Debug, Clone)]
pub struct DoNotAccept;

impl From<FieldMapError> for ApplicationError {
    fn from(e: FieldMapError) -> Self {
        ApplicationError::FieldMapError(e)
    }
}

impl From<FieldMapError> for FromAppError {
    fn from(e: FieldMapError) -> Self {
        FromAppError::FieldMapError(e)
    }
}

pub trait Application: Send {
    fn on_create(&mut self, session_id: &SessionId) -> Result<(), DoNotAccept>;
    fn on_logon(&mut self, session_id: &SessionId) -> Result<(), LogonReject>;
    fn on_logout(&mut self, session_id: &SessionId) -> Result<(), ApplicationError>;
    fn to_admin(
        &mut self,
        message: Message,
        session_id: &SessionId,
    ) -> Result<Message, FieldMapError>;
    fn from_admin(
        &mut self,
        message: &Message,
        session_id: &SessionId,
    ) -> Result<(), FieldMapError>;
    fn to_app(
        &mut self,
        message: &mut Message,
        session_id: &SessionId,
    ) -> Result<(), ApplicationError>;
    fn from_app(&mut self, message: &Message, session_id: &SessionId) -> Result<(), FromAppError>;
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
    use dfx_base::session_id::SessionId;

    use super::{Application, ApplicationExt, DoNotAccept, LogonReject};
    use dfx_base::message::Message;

    pub struct TestApplication;

    impl Application for TestApplication {
        fn on_create(&mut self, _session_id: &SessionId) -> Result<(), DoNotAccept> {
            Ok(())
        }

        fn on_logon(&mut self, _session_id: &SessionId) -> Result<(), LogonReject> {
            Ok(())
        }

        fn on_logout(&mut self, _session_id: &SessionId) -> Result<(), super::ApplicationError> {
            Ok(())
        }

        fn to_admin(
            &mut self,
            message: Message,
            _session_id: &SessionId,
        ) -> Result<Message, dfx_base::field_map::FieldMapError> {
            Ok(message)
        }

        fn from_admin(
            &mut self,
            _message: &Message,
            _session_id: &SessionId,
        ) -> Result<(), dfx_base::field_map::FieldMapError> {
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
        ) -> Result<(), super::FromAppError> {
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
            message: Message,
            _session_id: &SessionId,
        ) -> Result<Message, super::ApplicationError> {
            Ok(message)
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
