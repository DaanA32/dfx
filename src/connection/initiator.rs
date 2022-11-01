use crate::session::Session;

pub struct SocketInitiator {

}

pub type Error = u32; //TODO

impl SocketInitiator {
    pub fn get_session_mut(&mut self) -> Option<&mut Session> {
        todo!();
    }

    pub fn event_loop(mut self) {
        //let session: &mut Session = self.get_session_mut();
        while self.read().unwrap() { //TODO handle Error
            //...
        }
        todo!();
    }

    pub fn read(&mut self) -> Result<bool, Error> {

        let read = self.read_some()?;
        if read > 0 {
            //self.add_to_stream()
        }else if let Some(session) = self.get_session_mut() {
            session.next();
        }else{
            // throw new QuickFIXException("Initiator timed out while reading socket");
            todo!("timeout");
        }

        self.process_stream()?;
        todo!("exceptions!")
    }

    pub fn read_some(&mut self) -> Result<usize, Error> {
        todo!()
    }

    pub fn process_stream(&mut self) -> Result<(), Error> {
        todo!()
    }
}
