// use std::{
//     io::{Read, Write},
//     sync::{atomic::AtomicBool, Arc, Mutex},
//     thread,
//     time::Duration,
// };

// use crate::message::Message;

// use super::{
//     Application, ApplicationError, Event, Session, SessionId,
// };

// pub struct Connection<A, RW>
// where
//     A: Application,
//     RW: Read + Write,
// {
//     application: Mutex<A>,
//     stream: Mutex<Session<RW>>,
//     write: Arc<Mutex<RW>>,
//     running: Arc<AtomicBool>,
// }
// //unsafe impl<'a, A: Application<'a>, RW: Read + Write> Send for Connection<'a, A, RW> {}

// #[derive(Debug)]
// pub enum ConnectionError {
//     ApplicationError(ApplicationError),
//     IoError(std::io::Error),
//     Logout,
//     FailedToSend,
// }

// impl From<ApplicationError> for ConnectionError {
//     fn from(err: ApplicationError) -> Self {
//         ConnectionError::ApplicationError(err)
//     }
// }
// impl From<std::io::Error> for ConnectionError {
//     fn from(err: std::io::Error) -> Self {
//         ConnectionError::IoError(err)
//     }
// }

// impl<A, RW>
//     Connection<A, RW>
// where
//     A: Application,
//     RW: Read + Write,
// {
//     pub fn new(read: RW, heartbeat_timeout: Duration, app: A) -> Self {
//         let mutex = Arc::new(Mutex::new(read));
//         Connection {
//             application: Mutex::new(app),
//             stream: Mutex::new(Session::new(false, mutex.clone(), heartbeat_timeout)),
//             write: mutex,
//             running: Arc::new(AtomicBool::new(true)),
//         }
//     }

//     pub fn event_loop(&self) {
//         while self.running.load(std::sync::atomic::Ordering::Relaxed) {
//             dbg!("loop");
//             let event = { self.stream.lock().unwrap().next() };
//             dbg!("event");
//             dbg!(&event);
//             let _action = match event {
//                 Some(event) => {
//                     match event {
//                         Event::Message(message) => self.on_message(message),
//                         Event::IoError => Err(ConnectionError::Logout),
//                         Event::Heartbeat => self.send_heartbeat(),
//                         Event::TestRequest => self.send_test_request(),
//                         Event::Logout => self.send_logout(),
//                         //_ => unreachable!(),
//                     }
//                 }
//                 None => Ok({
//                     dbg!("sleep 1");
//                     thread::sleep(Duration::from_millis(1))
//                 }),
//             };
//             dbg!(_action.unwrap());
//         }
//     }

//     fn send_logout(&self) -> Result<(), ConnectionError> {
//         dbg!("send_logout");
//         todo!()
//     }

//     fn send_test_request(&self) -> Result<(), ConnectionError> {
//         dbg!("send_test_request");
//         todo!()
//     }

//     fn send_heartbeat(&self) -> Result<(), ConnectionError> {
//         dbg!("send_heartbeat");
//         todo!()
//     }

//     fn on_message(&self, message: Vec<u8>) -> Result<(), ConnectionError> {
//         dbg!("on_message");
//         let message: &Vec<u8> = &message;
//         let msg = todo!("{:?}", message);
//         let session_id = get_id();
//         self.application
//             .lock()
//             .unwrap()
//             .from_app(msg, &session_id)?;

//         Ok(())
//     }

//     pub fn send_message(&self, message: Message) -> Result<(), ConnectionError> {
//         dbg!("send_message");
//         let session_id = get_id();

//         match if message.is_admin() {
//             self.application
//                 .lock()
//                 .unwrap()
//                 .to_admin(message, &session_id)
//         } else {
//             self.application
//                 .lock()
//                 .unwrap()
//                 .to_app(message, &session_id)
//         } {
//             Ok(msg) => self.send_raw(msg),
//             Err(ApplicationError::DoNotSend) => Ok(()),
//             Err(e) => Err(ConnectionError::ApplicationError(e)),
//         }
//     }

//     fn send_raw(&self, msg: Message) -> Result<(), ConnectionError> {
//         dbg!("send_raw");
//         dbg!(&msg);
//         let mut buffer = [0; 8192];
//         let encoded = self.encode(msg, &mut buffer)?;
//         let r = match self.write.lock() {
//             Ok(mut guard) => {
//                 dbg!("&guard");
//                 guard.write_all(&buffer[..encoded]).map_err(|_e| {
//                     println!("Error:\n{:?}", _e);
//                     ConnectionError::FailedToSend
//                 })
//             }
//             Err(_err) => Err(ConnectionError::Logout),
//         };
//         dbg!("send_raw end");
//         r
//     }

//     fn encode(
//         &self,
//         mut message: Message,
//         buffer: &mut [u8],
//     ) -> Result<usize, ConnectionError> {
//         let data = message.to_string();
//         let data = data.as_bytes();
//         let written = (&mut buffer[..data.len()]).write(data)?;
//         Ok(written)
//     }

//     pub fn stop(&self) {
//         dbg!("stop");
//         self.running
//             .store(false, std::sync::atomic::Ordering::SeqCst)
//     }
// }

// fn test_msg<'a, A: Application>(
//     _appl: &Mutex<A>,
//     _message: Vec<u8>,
// ) -> Message {
//     todo!()
// }

// fn get_id() -> super::SessionId {
//     let begin_string = "".into();
//     let sender_comp_id = "".into();
//     let sender_sub_id = "".into();
//     let sender_location_id = "".into();
//     let target_comp_id = "".into();
//     let target_sub_id = "".into();
//     let target_location_id = "".into();

//     SessionId::new(
//         begin_string,
//         sender_comp_id,
//         sender_sub_id,
//         sender_location_id,
//         target_comp_id,
//         target_sub_id,
//         target_location_id,
//     )
// }
