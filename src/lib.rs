#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ doc = include_str!("../README.md") ]

#![ doc    ( html_root_url = "https://docs.rs/async_progress" ) ]
#![ deny   ( missing_docs                                     ) ]
#![ forbid ( unsafe_code                                      ) ]
#![ allow  ( clippy::suspicious_else_formatting               ) ]

#![ warn
(
	missing_debug_implementations ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unused_extern_crates          ,
	unused_qualifications         ,
	single_use_lifetimes          ,
	unreachable_pub               ,
	variant_size_differences      ,
)]


mod progress;

pub use
{
	progress::Progress,
};


// External dependencies
//
mod import
{
	pub(crate) use
	{
		std     :: { fmt, sync::Arc, any::type_name, future::Future                 } ,
		futures :: { lock::Mutex, SinkExt, StreamExt, FutureExt, executor::block_on } ,
		pharos  :: { Pharos, Observe, Observable, ObserveConfig, Events, Filter     } ,
		log     :: { trace                                                          } ,
	};


	// #[ cfg( test ) ]
	// //
	// pub(crate) use
	// {
	// 	pretty_assertions :: { assert_eq } ,
	// };
}


