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
      > Requires ordered fields to fix (so technically working, but can discuss)
  - [x] FIXT11
    - [x] FIX50
    - [x] FIX50SP1
    - [x] FIX50SP2
  - [ ] FUTURE
    > Currently not supported by Quickfix or Quickfix/N
  - [x] MISC

## TODO

- [ ] Remove useless comments
- [ ] FileStore for messages
- [ ] FileLogger
  - [ ] log4rs Logger
- [ ] Add inline and doc comments
- [ ] Add message factory from data dictionary.
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
