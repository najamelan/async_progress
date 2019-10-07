use crate::import::*;


/// A progress tracker.
///
/// You can call [`set_state`](Progress::set_state) and it will notify observers of the new state.
///
/// To wait for a specific state, you can call [`wait`](Progress::wait) or [`once`](Progress::once).
///
/// Progress is [`Clone`] and it's methods only require a shared reference for convenience.
//
#[ derive( Clone ) ]
//
pub struct Progress<State> where State: 'static + Clone + Send + Sync + Eq + fmt::Debug
{
	state : Arc<Mutex<State        >>,
	pharos: Arc<Mutex<Pharos<State>>>,
}


impl<State> Progress<State> where State: 'static + Clone + Send + Sync + Eq + fmt::Debug
{

	/// Create a Progress with an initial state.
	//
	pub fn new( state: State ) -> Self
	{
		Self
		{
			state : Arc::new( Mutex::new( state             )) ,
			pharos: Arc::new( Mutex::new( Pharos::default() )) ,
		}
	}


	/// Set a new state. Observers will be notified.
	//
	pub async fn set_state( &self, new_state: State )
	{
		let mut pharos = self.pharos.lock().await;
		let mut state  = self.state .lock().await;

		trace!( "Set progress to: {:?}", new_state );

		pharos.send( new_state.clone() ).await.expect( "notify" );
		*state = new_state;
	}


	/// Check the current state.
	//
	pub fn current( &self ) -> State
	{
		block_on( self.state.lock() ).clone()
	}


	/// Create an event stream that will only fire for the given state. This is a stream and if you call
	/// [`set_state`](Progress::set_state) several times with the given state, this will yield several events.
	///
	/// Note that events fired before you call this will not be delivered to the stream. It's recommended to
	/// set up all observers first and then start doing work that can call `set_state`.
	///
	/// Note that this method uses `block_on` to lock a mutex on the pharos object, so it might block the
	/// thread. All operations on `Progress` are really short, so this shouldn't be a problem as long as you
	/// haven't set up an observer by calling [`Progress::observe`] with a bounded channel with a low queue size.
	/// If the sending of an event (in `set_state`) returns pending, and this method is called, that will deadlock.
	//
	pub fn wait( &self, state: State ) -> Events<State>
	{
		block_on( self.pharos.lock() ).observe( Filter::Closure( Box::new( move |s| s == &state ) ).into() ).expect( "observe" )
	}


	/// Create a future that will resolve when a certain state is next triggered.
	///
	/// Note that events fired before you call this will not be delivered to the stream. It's recommended to
	/// set up all observers first and then start doing work that can call `set_state`.
	///
	/// Note that this method uses `block_on` to lock a mutex on the pharos object, so it might block the
	/// thread. All operations on `Progress` are really short, so this shouldn't be a problem as long as you
	/// haven't set up an observer by calling [`Progress::observe`] with a bounded channel with a low queue size.
	/// If the sending of an event (in `set_state`) returns pending, and this method is called, that will deadlock.
	///
	// It's important here that this is not an async method. We need to observe immediately as events that happen
	// before we start observing will not trigger.
	//
	pub fn once( &self, state: State ) -> impl Future + Send
	{
		let evts =
		{
			block_on( self.pharos.lock() )

				.observe( Filter::Closure( Box::new( move |s| s == &state ) ).into() )
				.expect( "observe" )
		};

		async { let _ = evts.into_future().await; }
	}
}



impl<State> Observable<State> for Progress<State> where State: 'static + Clone + Send + Sync + Eq + fmt::Debug
{
	type Error = pharos::Error;

	/// Avoid configuring pharos with a bounded channel of a low queue size. It is possible to create a
	/// deadlock if the send in [`Progress::set_state`] returns pending and you call another method on [`Progress`]
	/// that uses block_on, notably [`Progress::current`].
	//
	fn observe( &mut self, options: ObserveConfig<State> ) -> Result< Events<State>, Self::Error >
	{
		block_on( self.pharos.lock() ).observe( options )
	}
}



impl<State> fmt::Debug for Progress<State>  where State: 'static + Clone + Send + Sync + Eq + fmt::Debug
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		write!( f, "Progress<{}>", type_name::<State>() )
	}
}
