use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VObject {
   Current(Object),
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Object {
   pub account_id: AccountId,
}

impl From<VObject> for Object {
   fn from(v_object: VObject) -> Self {
      match v_object {
         VObject::Current(object) => object,
      }
   }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ObjectOutput {
   pub account_id: AccountId,
}

impl From<VObject> for ObjectOutput {
   fn from(v_object: VObject) -> Self {
      match v_object {
         VObject::Current(object) => ObjectOutput {
            account_id: object.account_id
         },
      }
   }
}
