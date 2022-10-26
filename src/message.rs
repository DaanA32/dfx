use crate::field_map::FieldMap;
use std::ops::Deref;
use std::ops::DerefMut;
use crate::data_dictionary::MessageValidationError;
use crate::data_dictionary::DataDictionary;
use crate::data_dictionary::DDMap;
use crate::data_dictionary::DDGroup;
use crate::field_map::Tag;
use crate::field_map::FieldBase;
use crate::field_map::Group;
use crate::tags;

#[derive(Default, Clone, Debug)]
pub struct Header(FieldMap);

const HEADER_FIELD_ORDER: [Tag; 3] = [ tags::BeginString, tags::BodyLength, tags::MsgType ];

impl Deref for Header {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone, Debug)]
pub struct Trailer(FieldMap);

impl Deref for Trailer {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Trailer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone, Debug)]
pub struct Message {
    header: Header,
    body: FieldMap,
    trailer: Trailer,
    application_data_dictionary: Option<DataDictionary>,
    field_: u32,
    valid_structure_: bool,
}

impl Message {
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
    pub fn trailer(&self) -> &Trailer {
        &self.trailer
    }
    pub fn trailer_mut(&mut self) -> &mut Trailer {
        &mut self.trailer
    }

    pub fn has_valid_structure(&self) -> Result<(), MessageValidationError> {
        if self.valid_structure_ {
            Ok(())
        }else{
            Err(MessageValidationError::InvalidStructure(self.field_))
        }
    }

    pub fn is_header_field(tag: Tag, data_dictionary: Option<&DataDictionary>) -> bool {
        // switch (tag)
        // {
        //     case Tags.BeginString:
        //     case Tags.BodyLength:
        //     case Tags.MsgType:
        //     case Tags.SenderCompID:
        //     case Tags.TargetCompID:
        //     case Tags.OnBehalfOfCompID:
        //     case Tags.DeliverToCompID:
        //     case Tags.SecureDataLen:
        //     case Tags.MsgSeqNum:
        //     case Tags.SenderSubID:
        //     case Tags.SenderLocationID:
        //     case Tags.TargetSubID:
        //     case Tags.TargetLocationID:
        //     case Tags.OnBehalfOfSubID:
        //     case Tags.OnBehalfOfLocationID:
        //     case Tags.DeliverToSubID:
        //     case Tags.DeliverToLocationID:
        //     case Tags.PossDupFlag:
        //     case Tags.PossResend:
        //     case Tags.SendingTime:
        //     case Tags.OrigSendingTime:
        //     case Tags.XmlDataLen:
        //     case Tags.XmlData:
        //     case Tags.MessageEncoding:
        //     case Tags.LastMsgSeqNumProcessed:
        //         // case Tags.OnBehalfOfSendingTime: TODO
        //         return true;
        //     default:
        //         return false;
        // }
        match tag {
            tags::BeginString => true,
            tags::BodyLength => true,
            tags::MsgType => true,
            tags::SenderCompID => true,
            tags::TargetCompID => true,
            tags::OnBehalfOfCompID => true,
            tags::DeliverToCompID => true,
            tags::SecureDataLen => true,
            tags::MsgSeqNum => true,
            tags::SenderSubID => true,
            tags::SenderLocationID => true,
            tags::TargetSubID => true,
            tags::TargetLocationID => true,
            tags::OnBehalfOfSubID => true,
            tags::OnBehalfOfLocationID => true,
            tags::DeliverToSubID => true,
            tags::DeliverToLocationID => true,
            tags::PossDupFlag => true,
            tags::PossResend => true,
            tags::SendingTime => true,
            tags::OrigSendingTime => true,
            tags::XmlDataLen => true,
            tags::XmlData => true,
            tags::MessageEncoding => true,
            tags::LastMsgSeqNumProcessed => true,
            //tags::OnBehalfOfSendingTime => true, TODO
            _ => match data_dictionary {
                Some(dd) => dd.is_trailer_field(tag),
                None => false,
            }
        }
    }

    pub fn is_trailer_field(tag: Tag, data_dictionary: Option<&DataDictionary>) -> bool {
        // switch (tag)
        // {
        //     case Tags.SignatureLength:
        //     case Tags.Signature:
        //     case Tags.CheckSum:
        //         return true;
        //     default:
        //         return false;
        // }
        match tag {
            tags::SignatureLength => true,
            tags::Signature => true,
            tags::CheckSum => true,
            _ => match data_dictionary {
                Some(dd) => dd.is_trailer_field(tag),
                None => false,
            }
        }
    }

    // public static StringField ExtractField(string msgstr, ref int pos, DataDictionary.DataDictionary sessionDD, DataDictionary.DataDictionary appDD)
    fn extract_field(msgstr: &str, pos: &mut usize, session_dd: Option<&DataDictionary>, app_dd: Option<&DataDictionary>) -> Result<FieldBase, MessageParseError> {
        // int tagend = msgstr.IndexOf('=', pos);
        let tagend = msgstr[*pos..].chars().position(|c| c == '=');
        if tagend.is_none() {
            return Err(MessageParseError::FailedToFindEqualsAt(*pos))
        }

        let tagend = tagend.unwrap();
        // int tag = Convert.ToInt32(msgstr.Substring(pos, tagend - pos));
        let tag: Result<u32, _> = msgstr[*pos..tagend].parse();
        if tag.is_err() {
            return Err(MessageParseError::FailedToConvertTagToInt(msgstr[*pos..tagend].into()))
        }
        let tag = tag.unwrap();

        //     pos = tagend + 1;
        *pos = tagend + 1;

        //     int fieldvalend = msgstr.IndexOf((char)1, pos);
        let fieldend = msgstr[*pos..].chars().position(|c| c == '\x01');
        if fieldend.is_none() {
            return Err(MessageParseError::FailedToFindSohAt(*pos))
        }
        let fieldend = fieldend.unwrap();
        //     StringField field =  new StringField(tag, msgstr.Substring(pos, fieldvalend - pos));
        let value = &msgstr[*pos..fieldend];
        let field = FieldBase::new(tag, value.into());

        /*
         TODO data dict stuff
        if (((null != sessionDD) && sessionDD.IsDataField(field.Tag)) || ((null != appDD) && appDD.IsDataField(field.Tag)))
        {
            string fieldLength = "";
            // Assume length field is 1 less
            int lenField = field.Tag - 1;
            // Special case for Signature which violates above assumption
            if (Tags.Signature.Equals(field.Tag))
                lenField = Tags.SignatureLength;
            if ((null != group) && group.isSetField(lenField))
            {
                fieldLength = group.GetField(lenField);
                soh = equalSign + 1 + atol(fieldLength.c_str());
            }
            else if (isSetField(lenField))
            {
                fieldLength = getField(lenField);
                soh = equalSign + 1 + atol(fieldLength.c_str());
            }
        }
        */

        // pos = fieldvalend + 1;
        *pos = fieldend + 1;
        Ok(field)
    }

/// <summary>
/// Creates a Message from a FIX string
/// </summary>
/// <param name="msgstr"></param>
/// <param name="validate"></param>
/// <param name="sessionDD"></param>
/// <param name="appDD"></param>
/// <param name="msgFactory">If null, any groups will be constructed as generic Group objects</param>
/// <param name="ignoreBody">(default false) if true, ignores all non-header non-trailer fields.
///   Intended for callers that only need rejection-related information from the header.
/// </param>
// public void FromString(string msgstr, bool validate,
//     DataDictionary.DataDictionary sessionDD, DataDictionary.DataDictionary appDD, IMessageFactory msgFactory,
//     bool ignoreBody)
    pub fn from_string(
        &mut self,
        msgstr: &str,
        validate: bool,
        session_dd: Option<&DataDictionary>,
        app_dd: Option<&DataDictionary>,
        msg_factory: Option<bool>,
        ignore_body: bool
    ) -> Result<(), MessageParseError> {
//      this.ApplicationDataDictionary = appDD;
        self.application_data_dictionary = app_dd.map(|d| d.clone());
//      Clear();
        self.clear();

//      string msgType = "";
        let mut msg_type = "";
//      bool expectingHeader = true;
        let mut expecting_header = true;
//      bool expectingBody = true;
        let mut expecting_body = true;
//      int count = 0;
        let mut count = 0;
//      int pos = 0;
        let mut pos = 0;
//      DataDictionary.IFieldMapSpec msgMap = null;
        let mut msg_map: Option<&DDMap> = None;

//      while (pos < msgstr.Length)
        while pos < msgstr.len() {
//          StringField f = ExtractField(msgstr, ref pos, sessionDD, appDD);
            let f = Message::extract_field(msgstr, &mut pos, session_dd, app_dd )?;

//          if (validate && (count < 3) && (Header.HEADER_FIELD_ORDER[count++] != f.Tag))
            if validate && count < 3 && HEADER_FIELD_ORDER[count] != f.tag() {
//              throw new InvalidMessage("Header fields out of order");
                return Err(MessageParseError::InvalidMessage("Header fields out of order".into()));
            }
            count += 1;

//          if (IsHeaderField(f.Tag, sessionDD))
//          {
            if Message::is_header_field(f.tag(), session_dd) {
//              if (!expectingHeader)
                if !expecting_header {
//                  if (0 == field_)
                    if 0 == self.field_ {
//                      field_ = f.Tag;
                        self.field_ = f.tag();
                    }
//                  validStructure_ = false;
                    self.valid_structure_ = false;
                }

//              if (Tags.MsgType.Equals(f.Tag))
                if tags::MsgType == f.tag() {
//                  msgType = string.Copy(f.Obj);
                    msg_type = f.value();
//                  if (appDD != null)
                    if app_dd.is_some() {
//                      msgMap = appDD.GetMapForMessage(msgType);
                        msg_map = app_dd.unwrap().get_map_for_message(msg_type);
                    }
                }

//              if (!this.Header.SetField(f, false))
                if !self.header.set_field_base(f.clone(), Some(false)) {
//                  this.Header.RepeatedTags.Add(f);
                    self.header.repeated_tags_mut().push(f.clone());
                }

//              if ((null != sessionDD) && sessionDD.Header.IsGroup(f.Tag))
                if matches!(session_dd, Some(dd) if dd.header().is_group(f.tag())) {
                    let dd = session_dd.unwrap();
//                  pos = SetGroup(f, msgstr, pos, this.Header, sessionDD.Header.GetGroupSpec(f.Tag), sessionDD, appDD, msgFactory);
                    pos = Message::set_group(f.clone(), msgstr, pos, &mut self.header, dd.header().get_group(f.tag()), session_dd, app_dd, msg_factory)?;
                }
//          else if (IsTrailerField(f.Tag, sessionDD))
            }else if Message::is_trailer_field(f.tag(), session_dd) {
//              expectingHeader = false;
                expecting_header = false;
//              expectingBody = false;
                expecting_body = false;
//              if (!this.Trailer.SetField(f, false))
                if !self.header.set_field_base(f.clone(), Some(false)) {
//                  this.Trailer.RepeatedTags.Add(f);
                    self.trailer.repeated_tags_mut().push(f.clone());
                }

//              if ((null != sessionDD) && sessionDD.Trailer.IsGroup(f.Tag))
                if matches!(session_dd, Some(dd) if dd.trailer().is_group(f.tag())) {
                    let dd = session_dd.unwrap();
//                  pos = SetGroup(f, msgstr, pos, this.Trailer, sessionDD.Trailer.GetGroup(f.Tag), sessionDD, appDD, msgFactory);
                    pos = Message::set_group(f.clone(), msgstr, pos, &mut self.trailer, dd.trailer().get_group(f.tag()), session_dd, app_dd, msg_factory)?;
                }

//          else if (ignoreBody==false)
            } else if !ignore_body {
//              if (!expectingBody)
                if !expecting_body {
//                  if (0 == field_)
                    if self.field_ == 0 {
//                      field_ = f.Tag;
                        self.field_ = f.tag();
                    }
//                  validStructure_ = false;
                    self.valid_structure_ = false;
                }

//              expectingHeader = false;
                expecting_header = false;
//              if (!SetField(f, false))
                if !self.set_field_base(f.clone(), Some(false)) {
//                  this.RepeatedTags.Add(f);
                    self.repeated_tags_mut().push(f.clone());
                }

//              if((null != msgMap) && (msgMap.IsGroup(f.Tag)))
                if matches!(msg_map, Some(map) if map.is_group(f.tag())) {
                    let map = msg_map.unwrap();
//                  pos = SetGroup(f, msgstr, pos, this, msgMap.GetGroupSpec(f.Tag), sessionDD, appDD, msgFactory);
                    pos = Message::set_group(f.clone(), msgstr, pos, self, map.get_group(f.tag()), session_dd, app_dd, msg_factory)?;
                }

            }
        }

//      if (validate)
        if validate {
//          Validate();
            self.validate()?;
        }
        Ok(())
    }

    fn set_group(
        grp_no_fld: FieldBase,
        msgstr: &str,
        pos: usize,
        map: &mut FieldMap,
        group_dd: Option<&DDGroup>,
        session_dd: Option<&DataDictionary>,
        app_dd: Option<&DataDictionary>,
        msg_factory: Option<bool>
    ) -> Result<usize, MessageParseError> {
        // TODO fix
        let group_dd = group_dd.unwrap();

        let mut pos = pos;
        // int grpEntryDelimiterTag = groupDD.Delim;
        let grp_entry_delimiter_tag = group_dd.delim();
        // int grpPos = pos;
        let grp_pos = pos;
        // Group grp = null; // the group entry being constructed
        let mut group: Option<Group> = None;

        // while (pos < msgstr.Length)
        while pos < msgstr.len() {
            // grpPos = pos;
            let grp_pos = pos;
            // StringField f = ExtractField(msgstr, ref pos, sessionDataDictionary, appDD);
            let f = Message::extract_field(msgstr, &mut pos, session_dd, app_dd)?;
            // if (f.Tag == grpEntryDelimiterTag)
            if f.tag() == grp_entry_delimiter_tag {
                // This is the start of a group entry.

                // if (grp != null)
                if group.is_some() {
                    // // We were already building an entry, so the delimiter means it's done.
                    // fieldMap.AddGroup(grp, false);
                    map.add_group(f.tag(), group.as_ref().unwrap(), Some(false));
                    // grp = null; // prepare for new Group
                    group = None;
                }

                // Create a new group!
                // if (msgFactory != null)
                if let Some(factory) = msg_factory {
                    // grp = msgFactory.Create(Message.ExtractBeginString(msgstr), Message.GetMsgType(msgstr), grpNoFld.Tag);
                    todo!();
                }

                //If above failed (shouldn't ever happen), just use a generic Group.
                // if (grp == null)
                if group.is_none() {
                    // grp = new Group(grpNoFld.Tag, grpEntryDelimiterTag);
                    group = Some(Group::new(grp_no_fld.tag(), grp_entry_delimiter_tag));
                }

            //} else if (!groupDD.IsField(f.Tag)) {
            } else if !group_dd.is_field(f.tag()) {
                // This field is not in the group, thus the repeating group is done.

                // if (grp != null)
                if let Some(group) = group {
                   // fieldMap.AddGroup(grp, false);
                   map.add_group(f.tag(), &group, Some(false));
                }
                return Ok(grp_pos);
            // else if(groupDD.IsField(f.Tag) && grp != null && grp.IsSetField(f.Tag))
            } else if group_dd.is_field(f.tag()) && group.is_some() && group.as_ref().unwrap().is_field_set(f.tag()) {
                // Tag is appearing for the second time within a group element.
                // Presumably the sender didn't set the delimiter (or their DD has a different delimiter).

                // throw new RepeatedTagWithoutGroupDelimiterTagException(grpNoFld.Tag, f.Tag);
                return Err(MessageParseError::RepeatedTagWithoutGroupDelimiterTagException(grp_no_fld.tag(), f.tag()));
            } else {
                // if (grp == null)
                if group.is_none() {
                    // This means we got into the group's fields without finding a delimiter tag.

                    //throw new GroupDelimiterTagException(grpNoFld.Tag, grpEntryDelimiterTag);
                    return Err(MessageParseError::GroupDelimiterTagException(grp_no_fld.tag(), f.tag()));
                }
                let group: &mut Group = group.as_mut().unwrap();

                // f is just a field in our group entry.  Add it and iterate again.
                // grp.SetField(f);
                group.set_field_base(f.clone(), None);

                // if(groupDD.IsGroup(f.Tag))
                if group_dd.is_group(f.tag()) {
                    // f is a counter for a nested group.  Recurse!

                    //pos = SetGroup(f, msgstr, pos, grp, groupDD.GetGroupSpec(f.Tag), sessionDataDictionary, appDD, msgFactory);
                    pos = Message::set_group(f.clone(), msgstr, pos, group, group_dd.get_group(f.tag()), session_dd, app_dd, msg_factory)?;
                }
            }

        }

        Ok(grp_pos)
    }

    fn validate(&self) -> Result<(), MessageParseError> {
        // try
        // {
        //     int receivedBodyLength = this.Header.GetInt(Tags.BodyLength);
        //     if (BodyLength() != receivedBodyLength)
        //         throw new InvalidMessage("Expected BodyLength=" + BodyLength() + ", Received BodyLength=" + receivedBodyLength + ", Message.SeqNum=" + this.Header.GetInt(Tags.MsgSeqNum));

        //     int receivedCheckSum = this.Trailer.GetInt(Tags.CheckSum);
        //     if (CheckSum() != receivedCheckSum)
        //         throw new InvalidMessage("Expected CheckSum=" + CheckSum() + ", Received CheckSum=" + receivedCheckSum + ", Message.SeqNum=" + this.Header.GetInt(Tags.MsgSeqNum));
        // }
        // catch (FieldNotFoundException e)
        // {
        //     throw new InvalidMessage("BodyLength or CheckSum missing", e);
        // }
        // catch (FieldConvertError e)
        // {
        //     throw new InvalidMessage("BodyLength or Checksum has wrong format", e);
        // }

        let received_body_length = self.header.get_int(tags::BodyLength);
        if self.body_length() != received_body_length {
            return Err(MessageParseError::InvalidMessage(format!("Expected BodyLength={}, Received BodyLength={}, Message.SeqNum={}", self.body_length(), received_body_length, self.header.get_int(tags::MsgSeqNum))));
        }
        let received_checksum = self.header.get_int(tags::CheckSum);
        if self.checksum() != received_checksum {
            return Err(MessageParseError::InvalidMessage(format!("Expected CheckSum={}, Received CheckSum={}, Message.SeqNum={}", self.checksum(), received_checksum, self.header.get_int(tags::MsgSeqNum))));
        }
        Ok(())
    }

    fn body_length(&self) -> u32 {
        self.header.len() +
        self.len() +
        self.trailer.len()
    }

    fn checksum(&self) -> u32 {
        (
            self.header.calculate_total() +
            self.calculate_total() +
            self.trailer.calculate_total()
        ) % 256
    }

    fn clear(&mut self) {
        self.field_ = 0;
        self.header.clear();
        self.body.clear();
        self.trailer.clear();
    }
}

impl Deref for Message {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}

pub enum MessageParseError {
    InvalidMessage(String),
    FailedToConvertTagToInt(String),
    FailedToFindEqualsAt(usize),
    FailedToFindSohAt(usize),
    RepeatedTagWithoutGroupDelimiterTagException(Tag, Tag),
    GroupDelimiterTagException(Tag, Tag),
}
