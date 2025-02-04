# DFX

A FIX protocol engine.

## Goals

- [x] Runtime Message verification
- [x] Read config from file
- [x] Pass the test suite available
  - [x] FIX40
  - [x] FIX41
  - [x] FIX42
  - [x] FIX43
  - [x] FIX44
    - [x] FIX: fix44::test_resend_repeating_group
      > Now the tests compares ordered fields, so the response match the expectation but are not exactly the same.
  - [x] FIXT11
    - [x] FIX50
    - [x] FIX50SP1
    - [x] FIX50SP2
  - [ ] FUTURE
    > IGNORED!
    > Currently not supported by Quickfix or Quickfix/N
  - [x] MISC
- [x] SSL / TLS
- [x] `FileStore` for messages
- [x] `FileLogger`
  - [x] Similar to quickfix
  - [x] [`log`](https://docs.rs/log/latest/log/) Logger

## WIP

- Add inline and doc comments
 - Doc External
 - Doc Internal
 - Inline
 
## TODO
- Add message factory from data dictionary.
- Codegen static data dictionary from xml.
- Replace with Traits where possible
- Allow compile time message definitions
- `MessageCracker`
- Cleanup session.rs
  - Simplify message handling
  - Simplify next / `next_msg()`

## Credits
Heavily derived / inspired from [QuickfixN](https://github.com/connamara/quickfixn/)

## Examples
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
        println!("Application: {}", _session_id);
        Ok(())
    }

    fn on_logon(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), LogonReject> {
        println!("Application Logon: {}", _session_id);
        Ok(())
    }

    fn on_logout(&mut self, _session_id: &dfx::session_id::SessionId) -> Result<(), ApplicationError> {
        println!("Application Logout: {}", _session_id);
        Ok(())
    }

    fn to_admin(
        &mut self,
        message: Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<Message, dfx::field_map::FieldMapError> {
        println!("Application To Admin: {}", _session_id);
        Ok(message)
    }

    fn from_admin(
        &mut self,
        _message: &Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::field_map::FieldMapError> {
        println!("Application From Admin: {}", _session_id);
        Ok(())
    }

    fn to_app(
        &mut self,
        _message: &mut Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), ApplicationError> {
        println!("Application To App: {}", _session_id);
        Ok(())
    }

    fn from_app(
        &mut self,
        message: &Message,
        session_id: &dfx::session_id::SessionId,
    ) -> Result<(), FromAppError> {
        println!("Application From App: {}", session_id);
        // Echo back to sender
        Session::send_to_session(session_id, message.clone()).unwrap();
        Ok(())
    }
}

let app = Application::default();
// TODO Use the following to read it from file:
// let session_settings = SessionSettings::from_file("fix.cfg").unwrap();
let session_settings = SessionSettings::default();
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
