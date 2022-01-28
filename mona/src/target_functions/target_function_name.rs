use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TargetFunctionName {
    // Expectation,
    // Max,
    AlbedoDefault,
    AmberDefault,
    AratakiIttoDefault,
    BarbaraDefault,
    BeidouDefault,
    BennettDefault,
    ChongyunDefault,
    DilucDefault,
    DionaDefault,
    EulaDefault,
    FischlDefault,
    GanyuDefault,
    GorouDefault,
    HuTaoDefault,
    JeanDefault,
    KaedeharaKazuhaDefault,
    KaeyaDefault,
    KamisatoAyakaDefault,
}