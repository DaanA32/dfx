```rust
use dfx::{
    connection::SocketAcceptor,
    session::{Session, SessionSettings},
    message_store::{DefaultStoreFactory, MemoryStoreFactory},
    data_dictionary_provider::DefaultDataDictionaryProvider,
    logging::PrintlnLogFactory,
    message::DefaultMessageFactory,
    session::FromAppError,
    session::ApplicationError,
    session::LogonReject,
    session::DoNotAccept,
    message::Message,
};

#[derive(Clone, Default)]
struct Application;

impl dfx::session::Application for Application {

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
        message: &Message,
        session_id: &dfx::session_id::SessionId,
    ) -> Result<(), FromAppError> {
        println!("TestApplication From App: {}", session_id);
        // TODO Do something here
        Ok(())
    }
}

let config = r#"
[DEFAULT]
FileLogPath=log
ConnectionType=acceptor
SocketAcceptPort=0
NonStopSession=Y
SenderCompID=ISLD
TargetCompID=TW
ResetOnLogon=Y
FileStorePath=store
UseDataDictionary=Y
HeartBtInt=10
LogonTimeout=1
LogoutTimeout=1
MaxLatency=2

[SESSION]
BeginString=FIX.4.0
DataDictionary=../../spec/FIX40.xml
"#;

let app = Application::default();
// Use the following to read it from file:
// let session_settings = SessionSettings::from_file("fix.cfg").unwrap();
let session_settings = SessionSettings::from_string(config).unwrap();
let mut acceptor = SocketAcceptor::new(
    &session_settings,
    app,
    DefaultStoreFactory::new(&session_settings),
    DefaultDataDictionaryProvider::new(),
    PrintlnLogFactory::new(),
    DefaultMessageFactory::new(),
);

acceptor.start();
```
