
#![no_std]
use gstd::{msg,async_main, collections::HashMap , prelude::*,ActorId,debug};

use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


static mut STATE:Option<GNX> = None;

static mut ADDRESSFT:Option<InitFT> = None;

static mut INIT:Option<InitStruct> = None;



fn state_mut() -> &'static mut GNX {

    let state = unsafe {  STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }

}

#[warn(dead_code)]
fn init_state_mut() -> &'static mut InitStruct {

    let initstruct = unsafe { INIT.as_mut()};

    unsafe { initstruct.unwrap_unchecked() }

}

pub struct InitFT {
    pub ft_program_id: ActorId,
    pub nft_program_id: ActorId,

}


#[derive(Clone, Default)]
pub struct GNX {
    pub firstfield: String,
    pub secondfield: u128,
    pub owners: HashMap<ActorId, u128>,
    
}

fn MutarEstado() -> &'static mut InitFT {

    let addressft = unsafe { ADDRESSFT.as_mut()};

    unsafe { addressft.unwrap_unchecked() }

}

impl GNX {
    
    async fn useFunTo(&mut self, amount_tokens: u128){ 
        
        let currentstate = state_mut();
        let address_ft = MutarEstado();           
        let payload = FTAction::Mint(amount_tokens);     
        //let result =  msg::send_for_reply_as::<_, FTEvent>(address_ft.ft_program_id,payload,0,0).expect("Error in sending a message").await;
         
        let _ = msg::send(address_ft.ft_program_id, payload, 0);
        //ft_program_id: config.ft_program_id,
        //nft_program_id: config.nft_program_id
    }
    
}


#[no_mangle]
extern "C" fn init () {

    let config: InitStruct = msg::load().expect("Unable to decode InitStruct");

    if config.ft_program_id.is_zero() {
        panic!("InitStruct program address can't be 0");
    }

    if config.nft_program_id.is_zero() {
        panic!("InitStruct program address can't be 0");
    }

    let init = InitStruct {
        ft_program_id: config.ft_program_id,
        nft_program_id: config.nft_program_id
    };

    

    unsafe {
        INIT = Some(init);
    }



    let state = GNX {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };


}


#[async_main]
async fn main(){

        let action: Action = msg::load().expect("Could not load Action");
        let currentstate = state_mut();
        // Actualizar estado
        match action {
            Action::CollectPhotons => {
                msg::reply(String::from("Collected photons"),0).expect("Error ");
                
                currentstate.useFunTo(100).await;
                
                

                 let _ =msg::reply(Event::CollectPhotons,0);
                

            }
        };
    }

        


    
#[no_mangle]
extern "C" fn state() {
   
    let state = unsafe { STATE.take().expect("Unexpected error in taking state") };

    msg::reply::<IoGNX>(state.into(), 0)
    .expect("Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`");
    
}


impl From<Atenea> for IoGNX {

    fn from(value: GNX) -> Self {
        
        let GNX {
            firstfield,
            secondfield,
            owners,
        } = value;

        
        let owners = owners.iter().map(|(k, v)| (*k, v.clone())).collect();
   
   
        Self {
            firstfield,
            secondfield,
            owners,
        }
    }
}
