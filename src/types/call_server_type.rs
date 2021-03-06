
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the type of a call server
pub trait TDCallServerType: Debug + RObject {}

/// Describes the type of a call server
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallServerType {
  #[doc(hidden)] _Default(()),
  /// A Telegram call reflector
  TelegramReflector(CallServerTypeTelegramReflector),
  /// A WebRTC server
  Webrtc(CallServerTypeWebrtc),

}

impl Default for CallServerType {
  fn default() -> Self { CallServerType::_Default(()) }
}

impl<'de> Deserialize<'de> for CallServerType {
  fn deserialize<D>(deserializer: D) -> Result<CallServerType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CallServerType,
      (callServerTypeTelegramReflector, TelegramReflector);
      (callServerTypeWebrtc, Webrtc);

    )(deserializer)
  }
}

impl RObject for CallServerType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      CallServerType::TelegramReflector(t) => t.td_name(),
      CallServerType::Webrtc(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl CallServerType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let CallServerType::_Default(_) = self { true } else { false } }

  pub fn is_telegram_reflector(&self) -> bool { if let CallServerType::TelegramReflector(_) = self { true } else { false } }
  pub fn is_webrtc(&self) -> bool { if let CallServerType::Webrtc(_) = self { true } else { false } }

  pub fn on_telegram_reflector<F: FnOnce(&CallServerTypeTelegramReflector)>(&self, fnc: F) -> &Self { if let CallServerType::TelegramReflector(t) = self { fnc(t) }; self }
  pub fn on_webrtc<F: FnOnce(&CallServerTypeWebrtc)>(&self, fnc: F) -> &Self { if let CallServerType::Webrtc(t) = self { fnc(t) }; self }

  pub fn as_telegram_reflector(&self) -> Option<&CallServerTypeTelegramReflector> { if let CallServerType::TelegramReflector(t) = self { return Some(t) } None }
  pub fn as_webrtc(&self) -> Option<&CallServerTypeWebrtc> { if let CallServerType::Webrtc(t) = self { return Some(t) } None }



  pub fn telegram_reflector<T: AsRef<CallServerTypeTelegramReflector>>(t: T) -> Self { CallServerType::TelegramReflector(t.as_ref().clone()) }

  pub fn webrtc<T: AsRef<CallServerTypeWebrtc>>(t: T) -> Self { CallServerType::Webrtc(t.as_ref().clone()) }

}

impl AsRef<CallServerType> for CallServerType {
  fn as_ref(&self) -> &CallServerType { self }
}







/// A Telegram call reflector
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServerTypeTelegramReflector {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// A peer tag to be used with the reflector
  peer_tag: String,
  
}

impl RObject for CallServerTypeTelegramReflector {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callServerTypeTelegramReflector" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallServerType for CallServerTypeTelegramReflector {}



impl CallServerTypeTelegramReflector {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallServerTypeTelegramReflectorBuilder {
    let mut inner = CallServerTypeTelegramReflector::default();
    inner.td_name = "callServerTypeTelegramReflector".to_string();
    RTDCallServerTypeTelegramReflectorBuilder { inner }
  }

  pub fn peer_tag(&self) -> &String { &self.peer_tag }

}

#[doc(hidden)]
pub struct RTDCallServerTypeTelegramReflectorBuilder {
  inner: CallServerTypeTelegramReflector
}

impl RTDCallServerTypeTelegramReflectorBuilder {
  pub fn build(&self) -> CallServerTypeTelegramReflector { self.inner.clone() }

   
  pub fn peer_tag<T: AsRef<str>>(&mut self, peer_tag: T) -> &mut Self {
    self.inner.peer_tag = peer_tag.as_ref().to_string();
    self
  }

}

impl AsRef<CallServerTypeTelegramReflector> for CallServerTypeTelegramReflector {
  fn as_ref(&self) -> &CallServerTypeTelegramReflector { self }
}

impl AsRef<CallServerTypeTelegramReflector> for RTDCallServerTypeTelegramReflectorBuilder {
  fn as_ref(&self) -> &CallServerTypeTelegramReflector { &self.inner }
}







/// A WebRTC server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServerTypeWebrtc {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Username to be used for authentification
  username: String,
  /// Authentication password
  password: String,
  /// True, if the server supports TURN
  supports_turn: bool,
  /// True, if the server supports STUN
  supports_stun: bool,
  
}

impl RObject for CallServerTypeWebrtc {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callServerTypeWebrtc" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallServerType for CallServerTypeWebrtc {}



impl CallServerTypeWebrtc {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallServerTypeWebrtcBuilder {
    let mut inner = CallServerTypeWebrtc::default();
    inner.td_name = "callServerTypeWebrtc".to_string();
    RTDCallServerTypeWebrtcBuilder { inner }
  }

  pub fn username(&self) -> &String { &self.username }

  pub fn password(&self) -> &String { &self.password }

  pub fn supports_turn(&self) -> bool { self.supports_turn }

  pub fn supports_stun(&self) -> bool { self.supports_stun }

}

#[doc(hidden)]
pub struct RTDCallServerTypeWebrtcBuilder {
  inner: CallServerTypeWebrtc
}

impl RTDCallServerTypeWebrtcBuilder {
  pub fn build(&self) -> CallServerTypeWebrtc { self.inner.clone() }

   
  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

   
  pub fn supports_turn(&mut self, supports_turn: bool) -> &mut Self {
    self.inner.supports_turn = supports_turn;
    self
  }

   
  pub fn supports_stun(&mut self, supports_stun: bool) -> &mut Self {
    self.inner.supports_stun = supports_stun;
    self
  }

}

impl AsRef<CallServerTypeWebrtc> for CallServerTypeWebrtc {
  fn as_ref(&self) -> &CallServerTypeWebrtc { self }
}

impl AsRef<CallServerTypeWebrtc> for RTDCallServerTypeWebrtcBuilder {
  fn as_ref(&self) -> &CallServerTypeWebrtc { &self.inner }
}



