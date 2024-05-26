
use frame_support::pallet_prelude::*;


use super::*;



#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct EventAuction {
	pub id: u32,

    pub dao_id: String,
    pub user_id: String,
    pub event_uri: String,
    pub event_wallet: String,
    pub raised: String,
    pub status: String
}

impl EventAuction {
    pub fn new(
		_id: u32,
		_dao_id: String,
		_user_id: String,
		_event_uri: String,
		_event_wallet: String,
    ) -> Self {
        EventAuction {
            id:_id,
            dao_id:_dao_id,
            user_id:_user_id,
            event_uri:_event_uri.clone(),
			event_wallet:_event_uri.clone(),
			raised: String::from("0"),
			status:String::from("0"),
        }
    }
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct TokenAuction {
	pub id: u32,

    pub event_id: u32,
    pub token_uri: String,
    pub token_wallet: String
}

impl TokenAuction {
    pub fn new(
		_id: u32,
		_event_id: u32,
		_token_uri: String,
		_token_wallet: String,
    ) -> Self {
        TokenAuction {
            id:_id,
            event_id:_event_id,
			token_uri:_token_uri.clone(),
            token_wallet:_token_wallet.clone() 
        }
    }
}


