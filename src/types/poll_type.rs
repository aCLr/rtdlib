
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the type of a poll
pub trait TDPollType: Debug + RObject {}

/// Describes the type of a poll
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PollType {
  #[doc(hidden)] _Default(()),
  /// A poll in quiz mode, which has exactly one correct answer option and can be answered only once
  Quiz(PollTypeQuiz),
  /// A regular poll
  Regular(PollTypeRegular),

}

impl Default for PollType {
  fn default() -> Self { PollType::_Default(()) }
}

impl<'de> Deserialize<'de> for PollType {
  fn deserialize<D>(deserializer: D) -> Result<PollType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PollType,
      (pollTypeQuiz, Quiz);
      (pollTypeRegular, Regular);

    )(deserializer)
  }
}

impl RObject for PollType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PollType::Quiz(t) => t.td_name(),
      PollType::Regular(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PollType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PollType::_Default(_) = self { true } else { false } }

  pub fn is_quiz(&self) -> bool { if let PollType::Quiz(_) = self { true } else { false } }
  pub fn is_regular(&self) -> bool { if let PollType::Regular(_) = self { true } else { false } }

  pub fn on_quiz<F: FnOnce(&PollTypeQuiz)>(&self, fnc: F) -> &Self { if let PollType::Quiz(t) = self { fnc(t) }; self }
  pub fn on_regular<F: FnOnce(&PollTypeRegular)>(&self, fnc: F) -> &Self { if let PollType::Regular(t) = self { fnc(t) }; self }

  pub fn as_quiz(&self) -> Option<&PollTypeQuiz> { if let PollType::Quiz(t) = self { return Some(t) } None }
  pub fn as_regular(&self) -> Option<&PollTypeRegular> { if let PollType::Regular(t) = self { return Some(t) } None }



  pub fn quiz<T: AsRef<PollTypeQuiz>>(t: T) -> Self { PollType::Quiz(t.as_ref().clone()) }

  pub fn regular<T: AsRef<PollTypeRegular>>(t: T) -> Self { PollType::Regular(t.as_ref().clone()) }

}

impl AsRef<PollType> for PollType {
  fn as_ref(&self) -> &PollType { self }
}







/// A poll in quiz mode, which has exactly one correct answer option and can be answered only once
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollTypeQuiz {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// 0-based identifier of the correct answer option; 1 for a yet unanswered poll
  correct_option_id: i64,
  /// Text that is shown when the user chooses an incorrect answer or taps on the lamp icon, 0-200 characters with at most 2 line feeds; empty for a yet unanswered poll
  explanation: FormattedText,
  
}

impl RObject for PollTypeQuiz {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pollTypeQuiz" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPollType for PollTypeQuiz {}



impl PollTypeQuiz {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPollTypeQuizBuilder {
    let mut inner = PollTypeQuiz::default();
    inner.td_name = "pollTypeQuiz".to_string();
    RTDPollTypeQuizBuilder { inner }
  }

  pub fn correct_option_id(&self) -> i64 { self.correct_option_id }

  pub fn explanation(&self) -> &FormattedText { &self.explanation }

}

#[doc(hidden)]
pub struct RTDPollTypeQuizBuilder {
  inner: PollTypeQuiz
}

impl RTDPollTypeQuizBuilder {
  pub fn build(&self) -> PollTypeQuiz { self.inner.clone() }

   
  pub fn correct_option_id(&mut self, correct_option_id: i64) -> &mut Self {
    self.inner.correct_option_id = correct_option_id;
    self
  }

   
  pub fn explanation<T: AsRef<FormattedText>>(&mut self, explanation: T) -> &mut Self {
    self.inner.explanation = explanation.as_ref().clone();
    self
  }

}

impl AsRef<PollTypeQuiz> for PollTypeQuiz {
  fn as_ref(&self) -> &PollTypeQuiz { self }
}

impl AsRef<PollTypeQuiz> for RTDPollTypeQuizBuilder {
  fn as_ref(&self) -> &PollTypeQuiz { &self.inner }
}







/// A regular poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if multiple answer options can be chosen simultaneously
  allow_multiple_answers: bool,
  
}

impl RObject for PollTypeRegular {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pollTypeRegular" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPollType for PollTypeRegular {}



impl PollTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPollTypeRegularBuilder {
    let mut inner = PollTypeRegular::default();
    inner.td_name = "pollTypeRegular".to_string();
    RTDPollTypeRegularBuilder { inner }
  }

  pub fn allow_multiple_answers(&self) -> bool { self.allow_multiple_answers }

}

#[doc(hidden)]
pub struct RTDPollTypeRegularBuilder {
  inner: PollTypeRegular
}

impl RTDPollTypeRegularBuilder {
  pub fn build(&self) -> PollTypeRegular { self.inner.clone() }

   
  pub fn allow_multiple_answers(&mut self, allow_multiple_answers: bool) -> &mut Self {
    self.inner.allow_multiple_answers = allow_multiple_answers;
    self
  }

}

impl AsRef<PollTypeRegular> for PollTypeRegular {
  fn as_ref(&self) -> &PollTypeRegular { self }
}

impl AsRef<PollTypeRegular> for RTDPollTypeRegularBuilder {
  fn as_ref(&self) -> &PollTypeRegular { &self.inner }
}



