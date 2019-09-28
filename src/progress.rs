use crate::import::*;

#[ derive( Clone ) ]
//
pub struct Progress<State> where State: 'static + Clone + Send + Sync + Eq + fmt::Debug
{
	state : Arc<Mutex<State        >>,
	pharos: Arc<Mutex<Pharos<State>>>,
}

impl<State> Progress<State> where State: 'static + Clone + Send + Sync + Eq + fmt::Debug
{
	pub fn new( state: State ) -> Self
	{
		Self
		{
			state : Arc::new( Mutex::new( state             )) ,
			pharos: Arc::new( Mutex::new( Pharos::default() )) ,
		}
	}

	pub async fn set_state( &self, new_state: State )
	{
		let mut pharos = self.pharos.lock().await;
		let mut state  = self.state .lock().await;

		trace!( "Set progress to: {:?}", new_state );

		pharos.send( new_state.clone() ).await.expect( "notify" );
		*state = new_state;
	}


	pub fn wait( &self, state: State ) -> Events<State>
	{
		block_on( self.pharos.lock() ).observe( Filter::Closure( Box::new( move |s| s == &state ) ).into() ).expect( "observe" )
	}
}



impl<State> Observable<State> for Progress<State> where State: 'static + Clone + Send + Sync + Eq + fmt::Debug
{
	type Error = pharos::Error;

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
