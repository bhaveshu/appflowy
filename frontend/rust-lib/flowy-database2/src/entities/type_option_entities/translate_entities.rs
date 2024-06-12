use crate::services::field::translate_type_option::translate::TranslateTypeOption;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct TranslateTypeOptionPB {
  #[pb(index = 1)]
  pub auto_fill: bool,

  #[pb(index = 2)]
  pub language: TranslateLanguagePB,
}

impl From<TranslateTypeOption> for TranslateTypeOptionPB {
  fn from(value: TranslateTypeOption) -> Self {
    TranslateTypeOptionPB {
      auto_fill: value.auto_fill,
      language: value.language_type.into(),
    }
  }
}

impl From<TranslateTypeOptionPB> for TranslateTypeOption {
  fn from(value: TranslateTypeOptionPB) -> Self {
    TranslateTypeOption {
      auto_fill: value.auto_fill,
      language_type: value.language as i64,
    }
  }
}
#[derive(Clone, Debug, Copy, ProtoBuf_Enum, Default)]
#[repr(i64)]
pub enum TranslateLanguagePB {
  Chinese = 0,
  #[default]
  English = 1,
  French = 2,
  German = 3,
}

impl From<i64> for TranslateLanguagePB {
  fn from(data: i64) -> Self {
    match data {
      0 => TranslateLanguagePB::Chinese,
      1 => TranslateLanguagePB::English,
      2 => TranslateLanguagePB::French,
      3 => TranslateLanguagePB::German,
      _ => TranslateLanguagePB::English,
    }
  }
}
