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
    - [ ] FIX: fix44::test_resend_repeating_group
  - [ ] FIX50SP1
  - [ ] FIX50SP2
  - [ ] FIX50
  - [ ] FIXT11

## TODO

- [ ] FileStore for messages
- [ ] FileLogger
  - [ ] log4rs Logger
- [ ] Codegen static data dictionary from xml.
- [ ] Replace with Traits where possible
- [ ] Allow compile time message definitions
- [ ] MessageCracker
- [ ] Cleanup session.rs
  - [ ] Simplify message handling
  - [ ] Simplify next / next_msg()
- [ ] Generate report from test suite (For easier tracking)

## Credits
Heavily derived / inspired from [QuickfixN](https://github.com/connamara/quickfixn/)
