
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a sticker that needs to be added to a sticker set
pub trait TDInputSticker: Debug + RObject {}

/// Describes a sticker that needs to be added to a sticker set
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputSticker {
  #[doc(hidden)] _Default(()),
  /// An animated sticker in TGS format
  Animated(InputStickerAnimated),
  /// A static sticker in PNG format, which will be converted to WEBP server-side
  Static(InputStickerStatic),

}

impl Default for InputSticker {
  fn default() -> Self { InputSticker::_Default(()) }
}

impl<'de> Deserialize<'de> for InputSticker {
  fn deserialize<D>(deserializer: D) -> Result<InputSticker, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputSticker,
      (inputStickerAnimated, Animated);
      (inputStickerStatic, Static);

    )(deserializer)
  }
}

impl RObject for InputSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InputSticker::Animated(t) => t.td_name(),
      InputSticker::Static(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InputSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InputSticker::_Default(_) = self { true } else { false } }

  pub fn is_animated(&self) -> bool { if let InputSticker::Animated(_) = self { true } else { false } }
  pub fn is_static(&self) -> bool { if let InputSticker::Static(_) = self { true } else { false } }

  pub fn on_animated<F: FnOnce(&InputStickerAnimated)>(&self, fnc: F) -> &Self { if let InputSticker::Animated(t) = self { fnc(t) }; self }
  pub fn on_static<F: FnOnce(&InputStickerStatic)>(&self, fnc: F) -> &Self { if let InputSticker::Static(t) = self { fnc(t) }; self }

  pub fn as_animated(&self) -> Option<&InputStickerAnimated> { if let InputSticker::Animated(t) = self { return Some(t) } None }
  pub fn as_static(&self) -> Option<&InputStickerStatic> { if let InputSticker::Static(t) = self { return Some(t) } None }



  pub fn animated<T: AsRef<InputStickerAnimated>>(t: T) -> Self { InputSticker::Animated(t.as_ref().clone()) }

  pub fn static_<T: AsRef<InputStickerStatic>>(t: T) -> Self { InputSticker::Static(t.as_ref().clone()) }

}

impl AsRef<InputSticker> for InputSticker {
  fn as_ref(&self) -> &InputSticker { self }
}







/// An animated sticker in TGS format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStickerAnimated {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// File with the animated sticker. Only local or uploaded within a week files are supported. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
  sticker: InputFile,
  /// Emojis corresponding to the sticker
  emojis: String,
  
}

impl RObject for InputStickerAnimated {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputStickerAnimated" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputSticker for InputStickerAnimated {}



impl InputStickerAnimated {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputStickerAnimatedBuilder {
    let mut inner = InputStickerAnimated::default();
    inner.td_name = "inputStickerAnimated".to_string();
    RTDInputStickerAnimatedBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

  pub fn emojis(&self) -> &String { &self.emojis }

}

#[doc(hidden)]
pub struct RTDInputStickerAnimatedBuilder {
  inner: InputStickerAnimated
}

impl RTDInputStickerAnimatedBuilder {
  pub fn build(&self) -> InputStickerAnimated { self.inner.clone() }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

   
  pub fn emojis<T: AsRef<str>>(&mut self, emojis: T) -> &mut Self {
    self.inner.emojis = emojis.as_ref().to_string();
    self
  }

}

impl AsRef<InputStickerAnimated> for InputStickerAnimated {
  fn as_ref(&self) -> &InputStickerAnimated { self }
}

impl AsRef<InputStickerAnimated> for RTDInputStickerAnimatedBuilder {
  fn as_ref(&self) -> &InputStickerAnimated { &self.inner }
}







/// A static sticker in PNG format, which will be converted to WEBP server-side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStickerStatic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// PNG image with the sticker; must be up to 512 KB in size and fit in a 512x512 square
  sticker: InputFile,
  /// Emojis corresponding to the sticker
  emojis: String,
  /// For masks, position where the mask should be placed; may be null
  mask_position: Option<MaskPosition>,
  
}

impl RObject for InputStickerStatic {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputStickerStatic" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputSticker for InputStickerStatic {}



impl InputStickerStatic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputStickerStaticBuilder {
    let mut inner = InputStickerStatic::default();
    inner.td_name = "inputStickerStatic".to_string();
    RTDInputStickerStaticBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

  pub fn emojis(&self) -> &String { &self.emojis }

  pub fn mask_position(&self) -> &Option<MaskPosition> { &self.mask_position }

}

#[doc(hidden)]
pub struct RTDInputStickerStaticBuilder {
  inner: InputStickerStatic
}

impl RTDInputStickerStaticBuilder {
  pub fn build(&self) -> InputStickerStatic { self.inner.clone() }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

   
  pub fn emojis<T: AsRef<str>>(&mut self, emojis: T) -> &mut Self {
    self.inner.emojis = emojis.as_ref().to_string();
    self
  }

   
  pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
    self.inner.mask_position = Some(mask_position.as_ref().clone());
    self
  }

}

impl AsRef<InputStickerStatic> for InputStickerStatic {
  fn as_ref(&self) -> &InputStickerStatic { self }
}

impl AsRef<InputStickerStatic> for RTDInputStickerStaticBuilder {
  fn as_ref(&self) -> &InputStickerStatic { &self.inner }
}



