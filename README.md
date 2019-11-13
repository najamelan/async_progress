# async_progress

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_progress.svg?branch=master)](https://travis-ci.org/najamelan/async_progress)
[![Docs](https://docs.rs/async_progress/badge.svg)](https://docs.rs/async_progress)
[![crates.io](https://img.shields.io/crates/v/async_progress.svg)](https://crates.io/crates/async_progress)


> Create synchronization points between concurrent async tasks.

Sometimes, especially in order to test async code, we need code to run in a specific order. Making certain tasks wait on things that happen in others. You can create such synchronization by creating (oneshot) channels. When you have more than 2 steps in your flow, channels quickly become quite unwieldy to keep track of and to name. _async_progress_ allows you to create a state enum with steps and simply trigger them with [`Progress::set_state`] and wait on them with [`Progress::once`] or [`Progress::wait`].

__Warning:__ Since this is a convenience crate for testing, I haven't yet been bothered to write tests for it. Some things could be buggy.

## Table of Contents

- [Install](#install)
   - [Upgrade](#upgrade)
   - [Dependencies](#dependencies)
   - [Security](#security)
- [Usage](#usage)
   - [Basic Example](#basic-example)
   - [API](#api)
- [Contributing](#contributing)
   - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add async_progress`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   async_progress: ^0.1
```

With raw Cargo.toml
```toml
[dependencies]

    async_progress = "^0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_progress/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


### Security

This crate has `#![ forbid( unsafe_code ) ]`, but notably the futures library on which it depends uses quite some unsafe. It is mainly meant for convenience in tests, so it hasn't been scrutinized for security or performance.


## Usage

It's important to understand that [`Progress`] uses _pharos_ to be observable, and that observers that subscribe after
an event is triggered will not get that event.

Therefor it's recommended to make all calls to [`Progress::once`], [`Progress::wait`] and [`Progress::observe`] before you start any work that might call [`Progress::set_state`]. You can then pass those futures to the tasks that need
to await them. This also allows triggering events multiple times, which wouldn't be possible otherwise.

Sometimes your next call will be pending, but you need to give green light to some other task to do some stuff. In general it's safe to call [`Progress::set_state`] before the call that will pend. Your pending call will be polled before the other task will observe the new state.


### Basic example

```rust
use
{
   async_progress :: Progress,
   futures        :: { executor::block_on, future::join } ,
};

// Some hypothetical steps in our flow.
//
#[ derive( Debug, Clone, PartialEq, Eq )]
//
enum Step
{
   FillQueue,
   SendText,
   ReadText,
}


#[ test ]
//
fn test_something()
{
   let steps     = Progress::new( Step::FillQueue );
   let send_text = steps.once( Step::SendText );
   let read_text = steps.once( Step::ReadText );

   // Remark we don't need to move here, we can work on shared references of the local vars.
   // We also don't need to clone steps, since all the methods on it only require a shared reference.
   //
   let server = async
   {
      // Fill some queue...

      steps.set_state( Step::SendText ).await;

      read_text.await;

      // Now we can read the text
   };

   let client = async
   {
      send_text.await;

      // Now we can send some text...

      steps.set_state( Step::ReadText ).await;
   };


   block_on( join( server, client ) );
}
```

## API

API documentation can be found on [docs.rs](https://docs.rs/async_progress).


## Contributing

This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through Github issues.

Pull Requests are welcome on Github. By committing pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.

Please file PR's against the `dev` branch, don't forget to update the changelog and the documentation.

### Testing

There are no tests for the moment. `cargo doc --no-deps --all-features` will test the example in this readme.


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

