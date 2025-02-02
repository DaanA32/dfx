use std::println;

use base64::{engine::{general_purpose, self}, Engine, DecodeError, alphabet};
use dfx::{session::{Application, SessionSettings}, connection::SocketInitiator, message_store::DefaultStoreFactory, data_dictionary_provider::DefaultDataDictionaryProvider, logging::PrintlnLogFactory, message_factory::DefaultMessageFactory, tags::{self}};
use hmac_sha256::HMAC;

#[derive(Default, Clone, Debug)]
struct CoinbaseApp {
    password: String,
    secret: Vec<u8>,
}

impl CoinbaseApp {
    pub fn new(password: &str, secret: &str) -> Result<Self, DecodeError> {
        let engine = engine::GeneralPurpose::new(
             &alphabet::URL_SAFE,
             general_purpose::PAD);
        let secret = engine.decode(secret.replace("+", "-").replace("/", "_"))?;
        Ok(CoinbaseApp { password: password.into(), secret })
    }
}

impl Application for CoinbaseApp {
    fn on_create(&mut self, session_id: &dfx::session_id::SessionId) -> Result<(), dfx::session::DoNotAccept> {
        println!("Create {}", session_id);
        Ok(())
    }

    fn on_logon(&mut self, session_id: &dfx::session_id::SessionId) -> Result<(), dfx::session::LogonReject> {
        println!("Logon {}", session_id);
        Ok(())
    }

    fn on_logout(&mut self, session_id: &dfx::session_id::SessionId) -> Result<(), dfx::session::ApplicationError> {
        println!("Logout {}", session_id);
        Ok(())
    }

    fn to_admin(
        &mut self,
        mut message: dfx::message::Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<dfx::message::Message, dfx::field_map::FieldMapError> {
        let msg_type = message.header().get_string(tags::MsgType)?;
        match msg_type.as_str() {
            "A" => {
                message.set_tag_value(tags::ResetSeqNumFlag, "Y");
                message.set_tag_value(tags::Password, &self.password);
                let sending_time = message.header().get_string(tags::SendingTime)?;
                let msg_seq_num = message.header().get_string(tags::MsgSeqNum)?;
                let sender_comp_id = _session_id.sender_comp_id();
                let target_comp_id = _session_id.target_comp_id();
                let prehash = prehash(&sending_time, &msg_type, &msg_seq_num, sender_comp_id, target_comp_id, &self.password);
                let signature = sign(prehash, &self.secret);
                message.set_tag_value(tags::RawDataLength, signature.len());
                message.set_tag_value(tags::RawData, signature);
                message.set_tag_value(8013, "S");
                message.set_tag_value(9406, "N");
                Ok(message)
            },
            _ => Ok(message),
        }
    }

    fn from_admin(
        &mut self,
        _message: &dfx::message::Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::field_map::FieldMapError> {
        Ok(())
    }

    fn to_app(
        &mut self,
        _message: &mut dfx::message::Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::session::ApplicationError> {
        Ok(())
    }

    fn from_app(
        &mut self,
        _message: &dfx::message::Message,
        _session_id: &dfx::session_id::SessionId,
    ) -> Result<(), dfx::session::FromAppError> {
        Ok(())
    }
}

//
fn sign(prehash: String, key: &[u8]) -> String {
    let mut hmac = HMAC::new(key);
    hmac.update(prehash.as_bytes());
    let bytes = hmac.finalize();
    general_purpose::STANDARD.encode(bytes)
}

fn prehash(sending_time: &str, msg_type: &str, msg_seq_num: &str, sender_comp_id: &str, target_comp_id: &str, password: &str) -> String {
    format!("{}\x01{}\x01{}\x01{}\x01{}\x01{}", sending_time, msg_type, msg_seq_num, sender_comp_id, target_comp_id, password)
}

fn main() {
    let key_var_key = "CB_SECRET";
    let pass_var_key = "CB_PASSPHRASE";
    let key_var = std::env::var(key_var_key);
    let pass_var = std::env::var(pass_var_key);
    if let Err(err) = &key_var {
        println!("{err} for {key_var_key}");
    }
    if let Err(err) = &pass_var {
        println!("{err} for {pass_var_key}");
    }
    if key_var.is_err() || pass_var.is_err() {
        return;
    }

    let key = key_var.unwrap();
    let pass = pass_var.unwrap();
    let coinbase_app = CoinbaseApp::new(&pass, &key);
    if let Err(err) = coinbase_app {
        println!("Failed to decode secret: {err:?}, {}", err);
        return;
    }
    let app = coinbase_app.unwrap();
    let session_settings = SessionSettings::from_file("fix.cfg").unwrap();
    let mut initiator = SocketInitiator::new(
        session_settings.clone(),
        app,
        DefaultStoreFactory::new(&session_settings),
        DefaultDataDictionaryProvider::new(),
        PrintlnLogFactory::new(),
        DefaultMessageFactory::new(),
    );

    initiator.start();
    initiator.join()
}
