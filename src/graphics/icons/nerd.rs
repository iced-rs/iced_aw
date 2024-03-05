//! This file was automatically generated
//! by [Mamba Bronze](https://github.com/Redhawk18/mamba-bronze)

use crate::NERD_FONT;
use std::{
    fmt::{Display, Formatter, Result},
    string::String,
};

use iced::widget::{text, Text};

/// Holds all glyphs of the Nerd font
#[derive(Debug, Clone, Copy)]
pub enum Nerd {
    /// AbTesting 󰇉
    AbTesting,
    /// Abacus 󱛠
    Abacus,
    /// AbjadArabic 󱌨
    AbjadArabic,
    /// AbjadHebrew 󱌩
    AbjadHebrew,
    /// AbugidaDevanagari 󱌪
    AbugidaDevanagari,
    /// AbugidaThai 󱌫
    AbugidaThai,
    /// AccessPoint 󰀃
    AccessPoint,
    /// AccessPointCheck 󱔸
    AccessPointCheck,
    /// AccessPointMinus 󱔹
    AccessPointMinus,
    /// AccessPointNetwork 󰀂
    AccessPointNetwork,
    /// AccessPointNetworkOff 󰯡
    AccessPointNetworkOff,
    /// AccessPointOff 󱔑
    AccessPointOff,
    /// AccessPointPlus 󱔺
    AccessPointPlus,
    /// AccessPointRemove 󱔻
    AccessPointRemove,
    /// Accessibility 
    Accessibility,
    /// AccessibilityInset 
    AccessibilityInset,
    /// Account 
    Account,
    /// AccountAlert 󰀅
    AccountAlert,
    /// AccountAlertOutline 󰭐
    AccountAlertOutline,
    /// AccountArrowDown 󱡨
    AccountArrowDown,
    /// AccountArrowDownOutline 󱡩
    AccountArrowDownOutline,
    /// AccountArrowLeft 󰭑
    AccountArrowLeft,
    /// AccountArrowLeftOutline 󰭒
    AccountArrowLeftOutline,
    /// AccountArrowRight 󰭓
    AccountArrowRight,
    /// AccountArrowRightOutline 󰭔
    AccountArrowRightOutline,
    /// AccountArrowUp 󱡧
    AccountArrowUp,
    /// AccountArrowUpOutline 󱡪
    AccountArrowUpOutline,
    /// AccountBox 󰀆
    AccountBox,
    /// AccountBoxMultiple 󰤴
    AccountBoxMultiple,
    /// AccountBoxMultipleOutline 󱀊
    AccountBoxMultipleOutline,
    /// AccountBoxOutline 󰀇
    AccountBoxOutline,
    /// AccountCancel 󱋟
    AccountCancel,
    /// AccountCancelOutline 󱋠
    AccountCancelOutline,
    /// AccountCash 󱂗
    AccountCash,
    /// AccountCashOutline 󱂘
    AccountCashOutline,
    /// AccountCheck 󰀈
    AccountCheck,
    /// AccountCheckOutline 󰯢
    AccountCheckOutline,
    /// AccountChild 󰪉
    AccountChild,
    /// AccountChildCircle 󰪊
    AccountChildCircle,
    /// AccountChildOutline 󱃈
    AccountChildOutline,
    /// AccountCircle 󰀉
    AccountCircle,
    /// AccountCircleOutline 󰭕
    AccountCircleOutline,
    /// AccountClock 󰭖
    AccountClock,
    /// AccountClockOutline 󰭗
    AccountClockOutline,
    /// AccountCog 󱍰
    AccountCog,
    /// AccountCogOutline 󱍱
    AccountCogOutline,
    /// AccountConvert 󰀊
    AccountConvert,
    /// AccountConvertOutline 󱌁
    AccountConvertOutline,
    /// AccountCowboyHat 󰺛
    AccountCowboyHat,
    /// AccountCowboyHatOutline 󱟳
    AccountCowboyHatOutline,
    /// AccountDetails 󰘱
    AccountDetails,
    /// AccountDetailsOutline 󱍲
    AccountDetailsOutline,
    /// AccountEdit 󰚼
    AccountEdit,
    /// AccountEditOutline 󰿻
    AccountEditOutline,
    /// AccountEye 󰐠
    AccountEye,
    /// AccountEyeOutline 󱉻
    AccountEyeOutline,
    /// AccountFilter 󰤶
    AccountFilter,
    /// AccountFilterOutline 󰾝
    AccountFilterOutline,
    /// AccountGroup 󰡉
    AccountGroup,
    /// AccountGroupOutline 󰭘
    AccountGroupOutline,
    /// AccountHardHat 󰖵
    AccountHardHat,
    /// AccountHardHatOutline 󱨟
    AccountHardHatOutline,
    /// AccountHeart 󰢙
    AccountHeart,
    /// AccountHeartOutline 󰯣
    AccountHeartOutline,
    /// AccountInjury 󱠕
    AccountInjury,
    /// AccountInjuryOutline 󱠖
    AccountInjuryOutline,
    /// AccountKey 󰀋
    AccountKey,
    /// AccountKeyOutline 󰯤
    AccountKeyOutline,
    /// AccountLock 󱅞
    AccountLock,
    /// AccountLockOpen 󱥠
    AccountLockOpen,
    /// AccountLockOpenOutline 󱥡
    AccountLockOpenOutline,
    /// AccountLockOutline 󱅟
    AccountLockOutline,
    /// AccountMinus 󰀍
    AccountMinus,
    /// AccountMinusOutline 󰫬
    AccountMinusOutline,
    /// AccountMultiple 󰀎
    AccountMultiple,
    /// AccountMultipleCheck 󰣅
    AccountMultipleCheck,
    /// AccountMultipleCheckOutline 󱇾
    AccountMultipleCheckOutline,
    /// AccountMultipleMinus 󰗓
    AccountMultipleMinus,
    /// AccountMultipleMinusOutline 󰯥
    AccountMultipleMinusOutline,
    /// AccountMultipleOutline 󰀏
    AccountMultipleOutline,
    /// AccountMultiplePlus 󰀐
    AccountMultiplePlus,
    /// AccountMultiplePlusOutline 󰠀
    AccountMultiplePlusOutline,
    /// AccountMultipleRemove 󱈊
    AccountMultipleRemove,
    /// AccountMultipleRemoveOutline 󱈋
    AccountMultipleRemoveOutline,
    /// AccountMusic 󰠃
    AccountMusic,
    /// AccountMusicOutline 󰳩
    AccountMusicOutline,
    /// AccountNetwork 󰀑
    AccountNetwork,
    /// AccountNetworkOutline 󰯦
    AccountNetworkOutline,
    /// AccountOff 󰀒
    AccountOff,
    /// AccountOffOutline 󰯧
    AccountOffOutline,
    /// AccountOne 󰀄
    AccountOne,
    /// AccountOutline 󰀓
    AccountOutline,
    /// AccountPlus 󰀔
    AccountPlus,
    /// AccountPlusOutline 󰠁
    AccountPlusOutline,
    /// AccountQuestion 󰭙
    AccountQuestion,
    /// AccountQuestionOutline 󰭚
    AccountQuestionOutline,
    /// AccountReactivate 󱔫
    AccountReactivate,
    /// AccountReactivateOutline 󱔬
    AccountReactivateOutline,
    /// AccountRemove 󰀕
    AccountRemove,
    /// AccountRemoveOutline 󰫭
    AccountRemoveOutline,
    /// AccountSchool 󱨠
    AccountSchool,
    /// AccountSchoolOutline 󱨡
    AccountSchoolOutline,
    /// AccountSearch 󰀖
    AccountSearch,
    /// AccountSearchOutline 󰤵
    AccountSearchOutline,
    /// AccountSettings 󰘰
    AccountSettings,
    /// AccountSettingsOutline 󱃉
    AccountSettingsOutline,
    /// AccountStar 󰀗
    AccountStar,
    /// AccountStarOutline 󰯨
    AccountStarOutline,
    /// AccountSupervisor 󰪋
    AccountSupervisor,
    /// AccountSupervisorCircle 󰪌
    AccountSupervisorCircle,
    /// AccountSupervisorCircleOutline 󱓬
    AccountSupervisorCircleOutline,
    /// AccountSupervisorOutline 󱄭
    AccountSupervisorOutline,
    /// AccountSwitch 󰀙
    AccountSwitch,
    /// AccountSwitchOutline 󰓋
    AccountSwitchOutline,
    /// AccountSync 󱤛
    AccountSync,
    /// AccountSyncOutline 󱤜
    AccountSyncOutline,
    /// AccountTie 󰳣
    AccountTie,
    /// AccountTieHat 󱢘
    AccountTieHat,
    /// AccountTieHatOutline 󱢙
    AccountTieHatOutline,
    /// AccountTieOutline 󱃊
    AccountTieOutline,
    /// AccountTieVoice 󱌈
    AccountTieVoice,
    /// AccountTieVoiceOff 󱌊
    AccountTieVoiceOff,
    /// AccountTieVoiceOffOutline 󱌋
    AccountTieVoiceOffOutline,
    /// AccountTieVoiceOutline 󱌉
    AccountTieVoiceOutline,
    /// AccountTieWoman 󱪌
    AccountTieWoman,
    /// AccountVoice 󰗋
    AccountVoice,
    /// AccountVoiceOff 󰻔
    AccountVoiceOff,
    /// AccountWrench 󱢚
    AccountWrench,
    /// AccountWrenchOutline 󱢛
    AccountWrenchOutline,
    /// ActivateBreakpoints 
    ActivateBreakpoints,
    /// Add 
    Add,
    /// Adjust 
    Adjust,
    /// AdjustOne 󰀚
    AdjustOne,
    /// Adn 
    Adn,
    /// Advertisements 󱤪
    Advertisements,
    /// AdvertisementsOff 󱤫
    AdvertisementsOff,
    /// AirConditioner 󰀛
    AirConditioner,
    /// AirFilter 󰵃
    AirFilter,
    /// AirHorn 󰶬
    AirHorn,
    /// AirHumidifier 󱂙
    AirHumidifier,
    /// AirHumidifierOff 󱑦
    AirHumidifierOff,
    /// AirPurifier 󰵄
    AirPurifier,
    /// Airbag 󰯩
    Airbag,
    /// Airballoon 󰀜
    Airballoon,
    /// AirballoonOutline 󱀋
    AirballoonOutline,
    /// Airplane 󰀝
    Airplane,
    /// AirplaneAlert 󱡺
    AirplaneAlert,
    /// AirplaneCheck 󱡻
    AirplaneCheck,
    /// AirplaneClock 󱡼
    AirplaneClock,
    /// AirplaneCog 󱡽
    AirplaneCog,
    /// AirplaneEdit 󱡾
    AirplaneEdit,
    /// AirplaneLanding 󰗔
    AirplaneLanding,
    /// AirplaneMarker 󱡿
    AirplaneMarker,
    /// AirplaneMinus 󱢀
    AirplaneMinus,
    /// AirplaneOff 󰀞
    AirplaneOff,
    /// AirplanePlus 󱢁
    AirplanePlus,
    /// AirplaneRemove 󱢂
    AirplaneRemove,
    /// AirplaneSearch 󱢃
    AirplaneSearch,
    /// AirplaneSettings 󱢄
    AirplaneSettings,
    /// AirplaneTakeoff 󰗕
    AirplaneTakeoff,
    /// Airport 󰡋
    Airport,
    /// Alarm 󰀠
    Alarm,
    /// AlarmBell 󰞎
    AlarmBell,
    /// AlarmCheck 󰀡
    AlarmCheck,
    /// AlarmLight 󰞏
    AlarmLight,
    /// AlarmLightOff 󱜞
    AlarmLightOff,
    /// AlarmLightOffOutline 󱜟
    AlarmLightOffOutline,
    /// AlarmLightOutline 󰯪
    AlarmLightOutline,
    /// AlarmMultiple 󰀢
    AlarmMultiple,
    /// AlarmNote 󰹱
    AlarmNote,
    /// AlarmNoteOff 󰹲
    AlarmNoteOff,
    /// AlarmOff 󰀣
    AlarmOff,
    /// AlarmPanel 󱗄
    AlarmPanel,
    /// AlarmPanelOutline 󱗅
    AlarmPanelOutline,
    /// AlarmPlus 󰀤
    AlarmPlus,
    /// AlarmSnooze 󰚎
    AlarmSnooze,
    /// Album 󰀥
    Album,
    /// Alert 
    Alert,
    /// AlertBox 󰀧
    AlertBox,
    /// AlertBoxOutline 󰳤
    AlertBoxOutline,
    /// AlertCircle 󰀨
    AlertCircle,
    /// AlertCircleCheck 󱇭
    AlertCircleCheck,
    /// AlertCircleCheckOutline 󱇮
    AlertCircleCheckOutline,
    /// AlertCircleOutline 󰗖
    AlertCircleOutline,
    /// AlertDecagram 󰚽
    AlertDecagram,
    /// AlertDecagramOutline 󰳥
    AlertDecagramOutline,
    /// AlertFill 
    AlertFill,
    /// AlertMinus 󱒻
    AlertMinus,
    /// AlertMinusOutline 󱒾
    AlertMinusOutline,
    /// AlertOctagon 󰀩
    AlertOctagon,
    /// AlertOctagonOutline 󰳦
    AlertOctagonOutline,
    /// AlertOctagram 󰝧
    AlertOctagram,
    /// AlertOctagramOutline 󰳧
    AlertOctagramOutline,
    /// AlertOne 󰀦
    AlertOne,
    /// AlertOutline 󰀪
    AlertOutline,
    /// AlertPlus 󱒺
    AlertPlus,
    /// AlertPlusOutline 󱒽
    AlertPlusOutline,
    /// AlertRemove 󱒼
    AlertRemove,
    /// AlertRemoveOutline 󱒿
    AlertRemoveOutline,
    /// AlertRhombus 󱇎
    AlertRhombus,
    /// AlertRhombusOutline 󱇏
    AlertRhombusOutline,
    /// Alien 󰢚
    Alien,
    /// AlienOutline 󱃋
    AlienOutline,
    /// AlignCenter 
    AlignCenter,
    /// AlignHorizontalCenter 󱇃
    AlignHorizontalCenter,
    /// AlignHorizontalDistribute 󱥢
    AlignHorizontalDistribute,
    /// AlignHorizontalLeft 󱇂
    AlignHorizontalLeft,
    /// AlignHorizontalRight 󱇄
    AlignHorizontalRight,
    /// AlignJustify 
    AlignJustify,
    /// AlignLeft 
    AlignLeft,
    /// AlignRight 
    AlignRight,
    /// AlignVerticalBottom 󱇅
    AlignVerticalBottom,
    /// AlignVerticalCenter 󱇆
    AlignVerticalCenter,
    /// AlignVerticalDistribute 󱥣
    AlignVerticalDistribute,
    /// AlignVerticalTop 󱇇
    AlignVerticalTop,
    /// AllInclusive 󰚾
    AllInclusive,
    /// AllInclusiveBox 󱢍
    AllInclusiveBox,
    /// AllInclusiveBoxOutline 󱢎
    AllInclusiveBoxOutline,
    /// Allergy 󱉘
    Allergy,
    /// AlmaLinux 
    AlmaLinux,
    /// Alpha 󰀫
    Alpha,
    /// AlphaA 󰫮
    AlphaA,
    /// AlphaABox 󰬈
    AlphaABox,
    /// AlphaABoxOutline 󰯫
    AlphaABoxOutline,
    /// AlphaACircle 󰯬
    AlphaACircle,
    /// AlphaACircleOutline 󰯭
    AlphaACircleOutline,
    /// AlphaB 󰫯
    AlphaB,
    /// AlphaBBox 󰬉
    AlphaBBox,
    /// AlphaBBoxOutline 󰯮
    AlphaBBoxOutline,
    /// AlphaBCircle 󰯯
    AlphaBCircle,
    /// AlphaBCircleOutline 󰯰
    AlphaBCircleOutline,
    /// AlphaC 󰫰
    AlphaC,
    /// AlphaCBox 󰬊
    AlphaCBox,
    /// AlphaCBoxOutline 󰯱
    AlphaCBoxOutline,
    /// AlphaCCircle 󰯲
    AlphaCCircle,
    /// AlphaCCircleOutline 󰯳
    AlphaCCircleOutline,
    /// AlphaD 󰫱
    AlphaD,
    /// AlphaDBox 󰬋
    AlphaDBox,
    /// AlphaDBoxOutline 󰯴
    AlphaDBoxOutline,
    /// AlphaDCircle 󰯵
    AlphaDCircle,
    /// AlphaDCircleOutline 󰯶
    AlphaDCircleOutline,
    /// AlphaE 󰫲
    AlphaE,
    /// AlphaEBox 󰬌
    AlphaEBox,
    /// AlphaEBoxOutline 󰯷
    AlphaEBoxOutline,
    /// AlphaECircle 󰯸
    AlphaECircle,
    /// AlphaECircleOutline 󰯹
    AlphaECircleOutline,
    /// AlphaF 󰫳
    AlphaF,
    /// AlphaFBox 󰬍
    AlphaFBox,
    /// AlphaFBoxOutline 󰯺
    AlphaFBoxOutline,
    /// AlphaFCircle 󰯻
    AlphaFCircle,
    /// AlphaFCircleOutline 󰯼
    AlphaFCircleOutline,
    /// AlphaG 󰫴
    AlphaG,
    /// AlphaGBox 󰬎
    AlphaGBox,
    /// AlphaGBoxOutline 󰯽
    AlphaGBoxOutline,
    /// AlphaGCircle 󰯾
    AlphaGCircle,
    /// AlphaGCircleOutline 󰯿
    AlphaGCircleOutline,
    /// AlphaH 󰫵
    AlphaH,
    /// AlphaHBox 󰬏
    AlphaHBox,
    /// AlphaHBoxOutline 󰰀
    AlphaHBoxOutline,
    /// AlphaHCircle 󰰁
    AlphaHCircle,
    /// AlphaHCircleOutline 󰰂
    AlphaHCircleOutline,
    /// AlphaI 󰫶
    AlphaI,
    /// AlphaIBox 󰬐
    AlphaIBox,
    /// AlphaIBoxOutline 󰰃
    AlphaIBoxOutline,
    /// AlphaICircle 󰰄
    AlphaICircle,
    /// AlphaICircleOutline 󰰅
    AlphaICircleOutline,
    /// AlphaIOne 󱂈
    AlphaIOne,
    /// AlphaJ 󰫷
    AlphaJ,
    /// AlphaJBox 󰬑
    AlphaJBox,
    /// AlphaJBoxOutline 󰰆
    AlphaJBoxOutline,
    /// AlphaJCircle 󰰇
    AlphaJCircle,
    /// AlphaJCircleOutline 󰰈
    AlphaJCircleOutline,
    /// AlphaK 󰫸
    AlphaK,
    /// AlphaKBox 󰬒
    AlphaKBox,
    /// AlphaKBoxOutline 󰰉
    AlphaKBoxOutline,
    /// AlphaKCircle 󰰊
    AlphaKCircle,
    /// AlphaKCircleOutline 󰰋
    AlphaKCircleOutline,
    /// AlphaL 󰫹
    AlphaL,
    /// AlphaLBox 󰬓
    AlphaLBox,
    /// AlphaLBoxOutline 󰰌
    AlphaLBoxOutline,
    /// AlphaLCircle 󰰍
    AlphaLCircle,
    /// AlphaLCircleOutline 󰰎
    AlphaLCircleOutline,
    /// AlphaLOne 󱎦
    AlphaLOne,
    /// AlphaM 󰫺
    AlphaM,
    /// AlphaMBox 󰬔
    AlphaMBox,
    /// AlphaMBoxOutline 󰰏
    AlphaMBoxOutline,
    /// AlphaMCircle 󰰐
    AlphaMCircle,
    /// AlphaMCircleOutline 󰰑
    AlphaMCircleOutline,
    /// AlphaN 󰫻
    AlphaN,
    /// AlphaNBox 󰬕
    AlphaNBox,
    /// AlphaNBoxOutline 󰰒
    AlphaNBoxOutline,
    /// AlphaNCircle 󰰓
    AlphaNCircle,
    /// AlphaNCircleOutline 󰰔
    AlphaNCircleOutline,
    /// AlphaO 󰫼
    AlphaO,
    /// AlphaOBox 󰬖
    AlphaOBox,
    /// AlphaOBoxOutline 󰰕
    AlphaOBoxOutline,
    /// AlphaOCircle 󰰖
    AlphaOCircle,
    /// AlphaOCircleOne 󰲞
    AlphaOCircleOne,
    /// AlphaOCircleOutline 󰰗
    AlphaOCircleOutline,
    /// AlphaOCircleOutlineOne 󰲟
    AlphaOCircleOutlineOne,
    /// AlphaOOne 󰬹
    AlphaOOne,
    /// AlphaP 󰫽
    AlphaP,
    /// AlphaPBox 󰬗
    AlphaPBox,
    /// AlphaPBoxOutline 󰰘
    AlphaPBoxOutline,
    /// AlphaPCircle 󰰙
    AlphaPCircle,
    /// AlphaPCircleOutline 󰰚
    AlphaPCircleOutline,
    /// AlphaQ 󰫾
    AlphaQ,
    /// AlphaQBox 󰬘
    AlphaQBox,
    /// AlphaQBoxOutline 󰰛
    AlphaQBoxOutline,
    /// AlphaQCircle 󰰜
    AlphaQCircle,
    /// AlphaQCircleOutline 󰰝
    AlphaQCircleOutline,
    /// AlphaR 󰫿
    AlphaR,
    /// AlphaRBox 󰬙
    AlphaRBox,
    /// AlphaRBoxOutline 󰰞
    AlphaRBoxOutline,
    /// AlphaRCircle 󰰟
    AlphaRCircle,
    /// AlphaRCircleOutline 󰰠
    AlphaRCircleOutline,
    /// AlphaS 󰬀
    AlphaS,
    /// AlphaSBox 󰬚
    AlphaSBox,
    /// AlphaSBoxOutline 󰰡
    AlphaSBoxOutline,
    /// AlphaSCircle 󰰢
    AlphaSCircle,
    /// AlphaSCircleOutline 󰰣
    AlphaSCircleOutline,
    /// AlphaT 󰬁
    AlphaT,
    /// AlphaTBox 󰬛
    AlphaTBox,
    /// AlphaTBoxOutline 󰰤
    AlphaTBoxOutline,
    /// AlphaTCircle 󰰥
    AlphaTCircle,
    /// AlphaTCircleOutline 󰰦
    AlphaTCircleOutline,
    /// AlphaU 󰬂
    AlphaU,
    /// AlphaUBox 󰬜
    AlphaUBox,
    /// AlphaUBoxOutline 󰰧
    AlphaUBoxOutline,
    /// AlphaUCircle 󰰨
    AlphaUCircle,
    /// AlphaUCircleOutline 󰰩
    AlphaUCircleOutline,
    /// AlphaV 󰬃
    AlphaV,
    /// AlphaVBox 󰬝
    AlphaVBox,
    /// AlphaVBoxOutline 󰰪
    AlphaVBoxOutline,
    /// AlphaVCircle 󰰫
    AlphaVCircle,
    /// AlphaVCircleOutline 󰰬
    AlphaVCircleOutline,
    /// AlphaVOne 󱂌
    AlphaVOne,
    /// AlphaW 󰬄
    AlphaW,
    /// AlphaWBox 󰬞
    AlphaWBox,
    /// AlphaWBoxOutline 󰰭
    AlphaWBoxOutline,
    /// AlphaWCircle 󰰮
    AlphaWCircle,
    /// AlphaWCircleOutline 󰰯
    AlphaWCircleOutline,
    /// AlphaX 󰬅
    AlphaX,
    /// AlphaXBox 󰬟
    AlphaXBox,
    /// AlphaXBoxOutline 󰰰
    AlphaXBoxOutline,
    /// AlphaXCircle 󰰱
    AlphaXCircle,
    /// AlphaXCircleOutline 󰰲
    AlphaXCircleOutline,
    /// AlphaXOne 󱂑
    AlphaXOne,
    /// AlphaY 󰬆
    AlphaY,
    /// AlphaYBox 󰬠
    AlphaYBox,
    /// AlphaYBoxOutline 󰰳
    AlphaYBoxOutline,
    /// AlphaYCircle 󰰴
    AlphaYCircle,
    /// AlphaYCircleOutline 󰰵
    AlphaYCircleOutline,
    /// AlphaZ 󰬇
    AlphaZ,
    /// AlphaZBox 󰬡
    AlphaZBox,
    /// AlphaZBoxOutline 󰰶
    AlphaZBoxOutline,
    /// AlphaZCircle 󰰷
    AlphaZCircle,
    /// AlphaZCircleOutline 󰰸
    AlphaZCircleOutline,
    /// AlphabetAurebesh 󱌬
    AlphabetAurebesh,
    /// AlphabetCyrillic 󱌭
    AlphabetCyrillic,
    /// AlphabetGreek 󱌮
    AlphabetGreek,
    /// AlphabetLatin 󱌯
    AlphabetLatin,
    /// AlphabetPiqad 󱌰
    AlphabetPiqad,
    /// AlphabetTengwar 󱌷
    AlphabetTengwar,
    /// Alphabetical 󰀬
    Alphabetical,
    /// AlphabeticalOff 󱀌
    AlphabeticalOff,
    /// AlphabeticalVariant 󱀍
    AlphabeticalVariant,
    /// AlphabeticalVariantOff 󱀎
    AlphabeticalVariantOff,
    /// Alpine 
    Alpine,
    /// Altimeter 󰗗
    Altimeter,
    /// Ambulance 
    Ambulance,
    /// AmbulanceOne 󰀯
    AmbulanceOne,
    /// Ammunition 󰳨
    Ammunition,
    /// Ampersand 󰪍
    Ampersand,
    /// Amplifier 󰀰
    Amplifier,
    /// AmplifierOff 󱆵
    AmplifierOff,
    /// Anchor 
    Anchor,
    /// AnchorOne 󰀱
    AnchorOne,
    /// Android 
    Android,
    /// AndroidMessages 󰵅
    AndroidMessages,
    /// AndroidOne 󰀲
    AndroidOne,
    /// AndroidStudio 󰀴
    AndroidStudio,
    /// AngleAcute 󰤷
    AngleAcute,
    /// AngleDown 
    AngleDown,
    /// AngleLeft 
    AngleLeft,
    /// AngleObtuse 󰤸
    AngleObtuse,
    /// AngleRight 󰤹
    AngleRight,
    /// AngleUp 
    AngleUp,
    /// Angular 󰚲
    Angular,
    /// Angularjs 󰚿
    Angularjs,
    /// Animation 󰗘
    Animation,
    /// AnimationOutline 󰪏
    AnimationOutline,
    /// AnimationPlay 󰤺
    AnimationPlay,
    /// AnimationPlayOutline 󰪐
    AnimationPlayOutline,
    /// Ansible 󱂚
    Ansible,
    /// Antenna 󱄙
    Antenna,
    /// Anvil 󰢛
    Anvil,
    /// AoscOs 
    AoscOs,
    /// ApacheKafka 󱀏
    ApacheKafka,
    /// Api 󱂛
    Api,
    /// ApiOff 󱉗
    ApiOff,
    /// Apple 
    Apple,
    /// AppleFinder 󰀶
    AppleFinder,
    /// AppleFruit 
    AppleFruit,
    /// AppleIcloud 󰀸
    AppleIcloud,
    /// AppleIos 󰀷
    AppleIos,
    /// AppleKeyboardCaps 󰘲
    AppleKeyboardCaps,
    /// AppleKeyboardCommand 󰘳
    AppleKeyboardCommand,
    /// AppleKeyboardControl 󰘴
    AppleKeyboardControl,
    /// AppleKeyboardOption 󰘵
    AppleKeyboardOption,
    /// AppleKeyboardShift 󰘶
    AppleKeyboardShift,
    /// AppleOne 󰀵
    AppleOne,
    /// AppleSafari 󰀹
    AppleSafari,
    /// Application 󰣆
    Application,
    /// ApplicationArray 󱃵
    ApplicationArray,
    /// ApplicationArrayOutline 󱃶
    ApplicationArrayOutline,
    /// ApplicationBraces 󱃷
    ApplicationBraces,
    /// ApplicationBracesOutline 󱃸
    ApplicationBracesOutline,
    /// ApplicationBrackets 󰲋
    ApplicationBrackets,
    /// ApplicationBracketsOutline 󰲌
    ApplicationBracketsOutline,
    /// ApplicationCog 󰙵
    ApplicationCog,
    /// ApplicationCogOutline 󱕷
    ApplicationCogOutline,
    /// ApplicationEdit 󰂮
    ApplicationEdit,
    /// ApplicationEditOutline 󰘙
    ApplicationEditOutline,
    /// ApplicationExport 󰶭
    ApplicationExport,
    /// ApplicationImport 󰶮
    ApplicationImport,
    /// ApplicationOutline 󰘔
    ApplicationOutline,
    /// ApplicationParentheses 󱃹
    ApplicationParentheses,
    /// ApplicationParenthesesOutline 󱃺
    ApplicationParenthesesOutline,
    /// ApplicationSettings 󰭠
    ApplicationSettings,
    /// ApplicationSettingsOutline 󱕕
    ApplicationSettingsOutline,
    /// ApplicationVariable 󱃻
    ApplicationVariable,
    /// ApplicationVariableOutline 󱃼
    ApplicationVariableOutline,
    /// ApproximatelyEqual 󰾞
    ApproximatelyEqual,
    /// ApproximatelyEqualBox 󰾟
    ApproximatelyEqualBox,
    /// Apps 
    Apps,
    /// AppsBox 󰵆
    AppsBox,
    /// AppsOne 󰀻
    AppsOne,
    /// Arch 󰣇
    Arch,
    /// ArchLinux 
    ArchLinux,
    /// Archcraft 
    Archcraft,
    /// Archive 
    Archive,
    /// ArchiveAlert 󱓽
    ArchiveAlert,
    /// ArchiveAlertOutline 󱓾
    ArchiveAlertOutline,
    /// ArchiveArrowDown 󱉙
    ArchiveArrowDown,
    /// ArchiveArrowDownOutline 󱉚
    ArchiveArrowDownOutline,
    /// ArchiveArrowUp 󱉛
    ArchiveArrowUp,
    /// ArchiveArrowUpOutline 󱉜
    ArchiveArrowUpOutline,
    /// ArchiveCancel 󱝋
    ArchiveCancel,
    /// ArchiveCancelOutline 󱝌
    ArchiveCancelOutline,
    /// ArchiveCheck 󱝍
    ArchiveCheck,
    /// ArchiveCheckOutline 󱝎
    ArchiveCheckOutline,
    /// ArchiveClock 󱝏
    ArchiveClock,
    /// ArchiveClockOutline 󱝐
    ArchiveClockOutline,
    /// ArchiveCog 󱝑
    ArchiveCog,
    /// ArchiveCogOutline 󱝒
    ArchiveCogOutline,
    /// ArchiveEdit 󱝓
    ArchiveEdit,
    /// ArchiveEditOutline 󱝔
    ArchiveEditOutline,
    /// ArchiveEye 󱝕
    ArchiveEye,
    /// ArchiveEyeOutline 󱝖
    ArchiveEyeOutline,
    /// ArchiveLock 󱝗
    ArchiveLock,
    /// ArchiveLockOpen 󱝘
    ArchiveLockOpen,
    /// ArchiveLockOpenOutline 󱝙
    ArchiveLockOpenOutline,
    /// ArchiveLockOutline 󱝚
    ArchiveLockOutline,
    /// ArchiveMarker 󱝛
    ArchiveMarker,
    /// ArchiveMarkerOutline 󱝜
    ArchiveMarkerOutline,
    /// ArchiveMinus 󱝝
    ArchiveMinus,
    /// ArchiveMinusOutline 󱝞
    ArchiveMinusOutline,
    /// ArchiveMusic 󱝟
    ArchiveMusic,
    /// ArchiveMusicOutline 󱝠
    ArchiveMusicOutline,
    /// ArchiveOff 󱝡
    ArchiveOff,
    /// ArchiveOffOutline 󱝢
    ArchiveOffOutline,
    /// ArchiveOne 
    ArchiveOne,
    /// ArchiveOutline 󱈎
    ArchiveOutline,
    /// ArchivePlus 󱝣
    ArchivePlus,
    /// ArchivePlusOutline 󱝤
    ArchivePlusOutline,
    /// ArchiveRefresh 󱝥
    ArchiveRefresh,
    /// ArchiveRefreshOutline 󱝦
    ArchiveRefreshOutline,
    /// ArchiveRemove 󱝧
    ArchiveRemove,
    /// ArchiveRemoveOutline 󱝨
    ArchiveRemoveOutline,
    /// ArchiveSearch 󱝩
    ArchiveSearch,
    /// ArchiveSearchOutline 󱝪
    ArchiveSearchOutline,
    /// ArchiveSettings 󱝫
    ArchiveSettings,
    /// ArchiveSettingsOutline 󱝬
    ArchiveSettingsOutline,
    /// ArchiveStar 󱝭
    ArchiveStar,
    /// ArchiveStarOutline 󱝮
    ArchiveStarOutline,
    /// ArchiveSync 󱝯
    ArchiveSync,
    /// ArchiveSyncOutline 󱝰
    ArchiveSyncOutline,
    /// ArchiveThree 󰀼
    ArchiveThree,
    /// ArchiveTwo 
    ArchiveTwo,
    /// Archlabs 
    Archlabs,
    /// Arcolinux 
    Arcolinux,
    /// Arduino 
    Arduino,
    /// ArmFlex 󰿗
    ArmFlex,
    /// ArmFlexOutline 󰿖
    ArmFlexOutline,
    /// ArrangeBringForward 󰀽
    ArrangeBringForward,
    /// ArrangeBringToFront 󰀾
    ArrangeBringToFront,
    /// ArrangeSendBackward 󰀿
    ArrangeSendBackward,
    /// ArrangeSendToBack 󰁀
    ArrangeSendToBack,
    /// ArrowAll 󰁁
    ArrowAll,
    /// ArrowBoth 
    ArrowBoth,
    /// ArrowBothOne 
    ArrowBothOne,
    /// ArrowBottomLeft 󰁂
    ArrowBottomLeft,
    /// ArrowBottomLeftBoldBox 󱥤
    ArrowBottomLeftBoldBox,
    /// ArrowBottomLeftBoldBoxOutline 󱥥
    ArrowBottomLeftBoldBoxOutline,
    /// ArrowBottomLeftBoldOutline 󰦷
    ArrowBottomLeftBoldOutline,
    /// ArrowBottomLeftThick 󰦸
    ArrowBottomLeftThick,
    /// ArrowBottomLeftThin 󱦶
    ArrowBottomLeftThin,
    /// ArrowBottomLeftThinCircleOutline 󱖖
    ArrowBottomLeftThinCircleOutline,
    /// ArrowBottomRight 󰁃
    ArrowBottomRight,
    /// ArrowBottomRightBoldBox 󱥦
    ArrowBottomRightBoldBox,
    /// ArrowBottomRightBoldBoxOutline 󱥧
    ArrowBottomRightBoldBoxOutline,
    /// ArrowBottomRightBoldOutline 󰦹
    ArrowBottomRightBoldOutline,
    /// ArrowBottomRightThick 󰦺
    ArrowBottomRightThick,
    /// ArrowBottomRightThin 󱦷
    ArrowBottomRightThin,
    /// ArrowBottomRightThinCircleOutline 󱖕
    ArrowBottomRightThinCircleOutline,
    /// ArrowCircleAltLeft 
    ArrowCircleAltLeft,
    /// ArrowCollapse 󰘕
    ArrowCollapse,
    /// ArrowCollapseAll 󰁄
    ArrowCollapseAll,
    /// ArrowCollapseDown 󰞒
    ArrowCollapseDown,
    /// ArrowCollapseHorizontal 󰡌
    ArrowCollapseHorizontal,
    /// ArrowCollapseLeft 󰞓
    ArrowCollapseLeft,
    /// ArrowCollapseRight 󰞔
    ArrowCollapseRight,
    /// ArrowCollapseUp 󰞕
    ArrowCollapseUp,
    /// ArrowCollapseVertical 󰡍
    ArrowCollapseVertical,
    /// ArrowDecision 󰦻
    ArrowDecision,
    /// ArrowDecisionAuto 󰦼
    ArrowDecisionAuto,
    /// ArrowDecisionAutoOutline 󰦽
    ArrowDecisionAutoOutline,
    /// ArrowDecisionOutline 󰦾
    ArrowDecisionOutline,
    /// ArrowDown 
    ArrowDown,
    /// ArrowDownBold 󰜮
    ArrowDownBold,
    /// ArrowDownBoldBox 󰜯
    ArrowDownBoldBox,
    /// ArrowDownBoldBoxOutline 󰜰
    ArrowDownBoldBoxOutline,
    /// ArrowDownBoldCircle 󰁇
    ArrowDownBoldCircle,
    /// ArrowDownBoldCircleOutline 󰁈
    ArrowDownBoldCircleOutline,
    /// ArrowDownBoldHexagonOutline 󰁉
    ArrowDownBoldHexagonOutline,
    /// ArrowDownBoldOutline 󰦿
    ArrowDownBoldOutline,
    /// ArrowDownBox 󰛀
    ArrowDownBox,
    /// ArrowDownCircle 󰳛
    ArrowDownCircle,
    /// ArrowDownCircleOutline 󰳜
    ArrowDownCircleOutline,
    /// ArrowDownDropCircle 󰁊
    ArrowDownDropCircle,
    /// ArrowDownDropCircleOutline 󰁋
    ArrowDownDropCircleOutline,
    /// ArrowDownLeft 
    ArrowDownLeft,
    /// ArrowDownLeftBold 󱞢
    ArrowDownLeftBold,
    /// ArrowDownLeftOne 󱞡
    ArrowDownLeftOne,
    /// ArrowDownOne 
    ArrowDownOne,
    /// ArrowDownRight 
    ArrowDownRight,
    /// ArrowDownRightBold 󱞤
    ArrowDownRightBold,
    /// ArrowDownRightOne 󱞣
    ArrowDownRightOne,
    /// ArrowDownThick 󰁆
    ArrowDownThick,
    /// ArrowDownThin 󱦳
    ArrowDownThin,
    /// ArrowDownThinCircleOutline 󱖙
    ArrowDownThinCircleOutline,
    /// ArrowDownTwo 󰁅
    ArrowDownTwo,
    /// ArrowExpand 󰘖
    ArrowExpand,
    /// ArrowExpandAll 󰁌
    ArrowExpandAll,
    /// ArrowExpandDown 󰞖
    ArrowExpandDown,
    /// ArrowExpandHorizontal 󰡎
    ArrowExpandHorizontal,
    /// ArrowExpandLeft 󰞗
    ArrowExpandLeft,
    /// ArrowExpandRight 󰞘
    ArrowExpandRight,
    /// ArrowExpandUp 󰞙
    ArrowExpandUp,
    /// ArrowExpandVertical 󰡏
    ArrowExpandVertical,
    /// ArrowHorizontalLock 󱅛
    ArrowHorizontalLock,
    /// ArrowLeft 
    ArrowLeft,
    /// ArrowLeftBold 󰜱
    ArrowLeftBold,
    /// ArrowLeftBoldBox 󰜲
    ArrowLeftBoldBox,
    /// ArrowLeftBoldBoxOutline 󰜳
    ArrowLeftBoldBoxOutline,
    /// ArrowLeftBoldCircle 󰁏
    ArrowLeftBoldCircle,
    /// ArrowLeftBoldCircleOutline 󰁐
    ArrowLeftBoldCircleOutline,
    /// ArrowLeftBoldHexagonOutline 󰁑
    ArrowLeftBoldHexagonOutline,
    /// ArrowLeftBoldOutline 󰧀
    ArrowLeftBoldOutline,
    /// ArrowLeftBottom 󱞥
    ArrowLeftBottom,
    /// ArrowLeftBottomBold 󱞦
    ArrowLeftBottomBold,
    /// ArrowLeftBox 󰛁
    ArrowLeftBox,
    /// ArrowLeftCircle 󰳝
    ArrowLeftCircle,
    /// ArrowLeftCircleOutline 󰳞
    ArrowLeftCircleOutline,
    /// ArrowLeftDropCircle 󰁒
    ArrowLeftDropCircle,
    /// ArrowLeftDropCircleOutline 󰁓
    ArrowLeftDropCircleOutline,
    /// ArrowLeftOne 
    ArrowLeftOne,
    /// ArrowLeftRight 󰹳
    ArrowLeftRight,
    /// ArrowLeftRightBold 󰹴
    ArrowLeftRightBold,
    /// ArrowLeftRightBoldOutline 󰧁
    ArrowLeftRightBoldOutline,
    /// ArrowLeftThick 󰁎
    ArrowLeftThick,
    /// ArrowLeftThin 󱦱
    ArrowLeftThin,
    /// ArrowLeftThinCircleOutline 󱖚
    ArrowLeftThinCircleOutline,
    /// ArrowLeftTop 󱞧
    ArrowLeftTop,
    /// ArrowLeftTopBold 󱞨
    ArrowLeftTopBold,
    /// ArrowLeftTwo 󰁍
    ArrowLeftTwo,
    /// ArrowProjectile 󱡀
    ArrowProjectile,
    /// ArrowProjectileMultiple 󱠿
    ArrowProjectileMultiple,
    /// ArrowRight 
    ArrowRight,
    /// ArrowRightBold 󰜴
    ArrowRightBold,
    /// ArrowRightBoldBox 󰜵
    ArrowRightBoldBox,
    /// ArrowRightBoldBoxOutline 󰜶
    ArrowRightBoldBoxOutline,
    /// ArrowRightBoldCircle 󰁖
    ArrowRightBoldCircle,
    /// ArrowRightBoldCircleOutline 󰁗
    ArrowRightBoldCircleOutline,
    /// ArrowRightBoldHexagonOutline 󰁘
    ArrowRightBoldHexagonOutline,
    /// ArrowRightBoldOutline 󰧂
    ArrowRightBoldOutline,
    /// ArrowRightBottom 󱞩
    ArrowRightBottom,
    /// ArrowRightBottomBold 󱞪
    ArrowRightBottomBold,
    /// ArrowRightBox 󰛂
    ArrowRightBox,
    /// ArrowRightCircle 󰳟
    ArrowRightCircle,
    /// ArrowRightCircleOutline 󰳠
    ArrowRightCircleOutline,
    /// ArrowRightDropCircle 󰁙
    ArrowRightDropCircle,
    /// ArrowRightDropCircleOutline 󰁚
    ArrowRightDropCircleOutline,
    /// ArrowRightOne 
    ArrowRightOne,
    /// ArrowRightThick 󰁕
    ArrowRightThick,
    /// ArrowRightThin 󱦰
    ArrowRightThin,
    /// ArrowRightThinCircleOutline 󱖘
    ArrowRightThinCircleOutline,
    /// ArrowRightTop 󱞫
    ArrowRightTop,
    /// ArrowRightTopBold 󱞬
    ArrowRightTopBold,
    /// ArrowRightTwo 󰁔
    ArrowRightTwo,
    /// ArrowSmallDown 
    ArrowSmallDown,
    /// ArrowSmallLeft 
    ArrowSmallLeft,
    /// ArrowSmallRight 
    ArrowSmallRight,
    /// ArrowSmallUp 
    ArrowSmallUp,
    /// ArrowSplitHorizontal 󰤻
    ArrowSplitHorizontal,
    /// ArrowSplitVertical 󰤼
    ArrowSplitVertical,
    /// ArrowSwap 
    ArrowSwap,
    /// ArrowSwitch 
    ArrowSwitch,
    /// ArrowTopLeft 󰁛
    ArrowTopLeft,
    /// ArrowTopLeftBoldBox 󱥨
    ArrowTopLeftBoldBox,
    /// ArrowTopLeftBoldBoxOutline 󱥩
    ArrowTopLeftBoldBoxOutline,
    /// ArrowTopLeftBoldOutline 󰧃
    ArrowTopLeftBoldOutline,
    /// ArrowTopLeftBottomRight 󰹵
    ArrowTopLeftBottomRight,
    /// ArrowTopLeftBottomRightBold 󰹶
    ArrowTopLeftBottomRightBold,
    /// ArrowTopLeftThick 󰧄
    ArrowTopLeftThick,
    /// ArrowTopLeftThin 󱦵
    ArrowTopLeftThin,
    /// ArrowTopLeftThinCircleOutline 󱖓
    ArrowTopLeftThinCircleOutline,
    /// ArrowTopRight 󰁜
    ArrowTopRight,
    /// ArrowTopRightBoldBox 󱥪
    ArrowTopRightBoldBox,
    /// ArrowTopRightBoldBoxOutline 󱥫
    ArrowTopRightBoldBoxOutline,
    /// ArrowTopRightBoldOutline 󰧅
    ArrowTopRightBoldOutline,
    /// ArrowTopRightBottomLeft 󰹷
    ArrowTopRightBottomLeft,
    /// ArrowTopRightBottomLeftBold 󰹸
    ArrowTopRightBottomLeftBold,
    /// ArrowTopRightThick 󰧆
    ArrowTopRightThick,
    /// ArrowTopRightThin 󱦴
    ArrowTopRightThin,
    /// ArrowTopRightThinCircleOutline 󱖔
    ArrowTopRightThinCircleOutline,
    /// ArrowUDownLeft 󱞭
    ArrowUDownLeft,
    /// ArrowUDownLeftBold 󱞮
    ArrowUDownLeftBold,
    /// ArrowUDownRight 󱞯
    ArrowUDownRight,
    /// ArrowUDownRightBold 󱞰
    ArrowUDownRightBold,
    /// ArrowULeftBottom 󱞱
    ArrowULeftBottom,
    /// ArrowULeftBottomBold 󱞲
    ArrowULeftBottomBold,
    /// ArrowULeftTop 󱞳
    ArrowULeftTop,
    /// ArrowULeftTopBold 󱞴
    ArrowULeftTopBold,
    /// ArrowURightBottom 󱞵
    ArrowURightBottom,
    /// ArrowURightBottomBold 󱞶
    ArrowURightBottomBold,
    /// ArrowURightTop 󱞷
    ArrowURightTop,
    /// ArrowURightTopBold 󱞸
    ArrowURightTopBold,
    /// ArrowUUpLeft 󱞹
    ArrowUUpLeft,
    /// ArrowUUpLeftBold 󱞺
    ArrowUUpLeftBold,
    /// ArrowUUpRight 󱞻
    ArrowUUpRight,
    /// ArrowUUpRightBold 󱞼
    ArrowUUpRightBold,
    /// ArrowUp 
    ArrowUp,
    /// ArrowUpBold 󰜷
    ArrowUpBold,
    /// ArrowUpBoldBox 󰜸
    ArrowUpBoldBox,
    /// ArrowUpBoldBoxOutline 󰜹
    ArrowUpBoldBoxOutline,
    /// ArrowUpBoldCircle 󰁟
    ArrowUpBoldCircle,
    /// ArrowUpBoldCircleOutline 󰁠
    ArrowUpBoldCircleOutline,
    /// ArrowUpBoldHexagonOutline 󰁡
    ArrowUpBoldHexagonOutline,
    /// ArrowUpBoldOutline 󰧇
    ArrowUpBoldOutline,
    /// ArrowUpBox 󰛃
    ArrowUpBox,
    /// ArrowUpCircle 󰳡
    ArrowUpCircle,
    /// ArrowUpCircleOutline 󰳢
    ArrowUpCircleOutline,
    /// ArrowUpDown 󰹹
    ArrowUpDown,
    /// ArrowUpDownBold 󰹺
    ArrowUpDownBold,
    /// ArrowUpDownBoldOutline 󰧈
    ArrowUpDownBoldOutline,
    /// ArrowUpDropCircle 󰁢
    ArrowUpDropCircle,
    /// ArrowUpDropCircleOutline 󰁣
    ArrowUpDropCircleOutline,
    /// ArrowUpLeft 
    ArrowUpLeft,
    /// ArrowUpLeftBold 󱞾
    ArrowUpLeftBold,
    /// ArrowUpLeftOne 󱞽
    ArrowUpLeftOne,
    /// ArrowUpOne 
    ArrowUpOne,
    /// ArrowUpRight 
    ArrowUpRight,
    /// ArrowUpRightBold 󱟀
    ArrowUpRightBold,
    /// ArrowUpRightOne 󱞿
    ArrowUpRightOne,
    /// ArrowUpThick 󰁞
    ArrowUpThick,
    /// ArrowUpThin 󱦲
    ArrowUpThin,
    /// ArrowUpThinCircleOutline 󱖗
    ArrowUpThinCircleOutline,
    /// ArrowUpTwo 󰁝
    ArrowUpTwo,
    /// ArrowVerticalLock 󱅜
    ArrowVerticalLock,
    /// ArtixLinux 
    ArtixLinux,
    /// Artstation 󰭛
    Artstation,
    /// AspectRatio 󰨤
    AspectRatio,
    /// Assistant 󰁤
    Assistant,
    /// Asterisk 
    Asterisk,
    /// AsteriskCircleOutline 󱨧
    AsteriskCircleOutline,
    /// AsteriskOne 󰛄
    AsteriskOne,
    /// At 󰁥
    At,
    /// Atlassian 󰠄
    Atlassian,
    /// Atm 󰵇
    Atm,
    /// Atom 
    Atom,
    /// AtomOne 󰝨
    AtomOne,
    /// AtomVariant 󰹻
    AtomVariant,
    /// Attachment 󰁦
    Attachment,
    /// AttachmentCheck 󱫁
    AttachmentCheck,
    /// AttachmentLock 󱧄
    AttachmentLock,
    /// AttachmentMinus 󱫂
    AttachmentMinus,
    /// AttachmentOff 󱫃
    AttachmentOff,
    /// AttachmentPlus 󱫄
    AttachmentPlus,
    /// AttachmentRemove 󱫅
    AttachmentRemove,
    /// AudioInputRca 󱡫
    AudioInputRca,
    /// AudioInputStereoMinijack 󱡬
    AudioInputStereoMinijack,
    /// AudioInputXlr 󱡭
    AudioInputXlr,
    /// AudioVideo 󰤽
    AudioVideo,
    /// AudioVideoOff 󱆶
    AudioVideoOff,
    /// AugmentedReality 󰡐
    AugmentedReality,
    /// AutoDownload 󱍾
    AutoDownload,
    /// AutoFix 󰁨
    AutoFix,
    /// AutoUpload 󰁩
    AutoUpload,
    /// Autorenew 󰁪
    Autorenew,
    /// AutorenewOff 󱧧
    AutorenewOff,
    /// AvTimer 󰁫
    AvTimer,
    /// Away 
    Away,
    /// AwesomeWm 
    AwesomeWm,
    /// Aws 󰸏
    Aws,
    /// Axe 󰣈
    Axe,
    /// AxeBattle 󱡂
    AxeBattle,
    /// Axis 󰵈
    Axis,
    /// AxisArrow 󰵉
    AxisArrow,
    /// AxisArrowInfo 󱐎
    AxisArrowInfo,
    /// AxisArrowLock 󰵊
    AxisArrowLock,
    /// AxisLock 󰵋
    AxisLock,
    /// AxisXArrow 󰵌
    AxisXArrow,
    /// AxisXArrowLock 󰵍
    AxisXArrowLock,
    /// AxisXRotateClockwise 󰵎
    AxisXRotateClockwise,
    /// AxisXRotateCounterclockwise 󰵏
    AxisXRotateCounterclockwise,
    /// AxisXYArrowLock 󰵐
    AxisXYArrowLock,
    /// AxisYArrow 󰵑
    AxisYArrow,
    /// AxisYArrowLock 󰵒
    AxisYArrowLock,
    /// AxisYRotateClockwise 󰵓
    AxisYRotateClockwise,
    /// AxisYRotateCounterclockwise 󰵔
    AxisYRotateCounterclockwise,
    /// AxisZArrow 󰵕
    AxisZArrow,
    /// AxisZArrowLock 󰵖
    AxisZArrowLock,
    /// AxisZRotateClockwise 󰵗
    AxisZRotateClockwise,
    /// AxisZRotateCounterclockwise 󰵘
    AxisZRotateCounterclockwise,
    /// Azure 
    Azure,
    /// AzureDevops 
    AzureDevops,
    /// Babel 󰨥
    Babel,
    /// Baby 󰁬
    Baby,
    /// BabyBottle 󰼹
    BabyBottle,
    /// BabyBottleOutline 󰼺
    BabyBottleOutline,
    /// BabyBuggy 󱏠
    BabyBuggy,
    /// BabyCarriage 󰚏
    BabyCarriage,
    /// BabyCarriageOff 󰾠
    BabyCarriageOff,
    /// BabyFace 󰹼
    BabyFace,
    /// BabyFaceOutline 󰹽
    BabyFaceOutline,
    /// Backburger 󰁭
    Backburger,
    /// Backspace 󰁮
    Backspace,
    /// BackspaceOutline 󰭜
    BackspaceOutline,
    /// BackspaceReverse 󰹾
    BackspaceReverse,
    /// BackspaceReverseOutline 󰹿
    BackspaceReverseOutline,
    /// BackupRestore 󰁯
    BackupRestore,
    /// Backward 
    Backward,
    /// Bacteria 
    Bacteria,
    /// BacteriaOne 󰻕
    BacteriaOne,
    /// BacteriaOutline 󰻖
    BacteriaOutline,
    /// BadgeAccount 󰶧
    BadgeAccount,
    /// BadgeAccountAlert 󰶨
    BadgeAccountAlert,
    /// BadgeAccountAlertOutline 󰶩
    BadgeAccountAlertOutline,
    /// BadgeAccountHorizontal 󰸍
    BadgeAccountHorizontal,
    /// BadgeAccountHorizontalOutline 󰸎
    BadgeAccountHorizontalOutline,
    /// BadgeAccountOutline 󰶪
    BadgeAccountOutline,
    /// Badminton 󰡑
    Badminton,
    /// BagCarryOn 󰼻
    BagCarryOn,
    /// BagCarryOnCheck 󰵥
    BagCarryOnCheck,
    /// BagCarryOnOff 󰼼
    BagCarryOnOff,
    /// BagChecked 󰼽
    BagChecked,
    /// BagPersonal 󰸐
    BagPersonal,
    /// BagPersonalOff 󰸑
    BagPersonalOff,
    /// BagPersonalOffOutline 󰸒
    BagPersonalOffOutline,
    /// BagPersonalOutline 󰸓
    BagPersonalOutline,
    /// BagSuitcase 󱖋
    BagSuitcase,
    /// BagSuitcaseOff 󱖍
    BagSuitcaseOff,
    /// BagSuitcaseOffOutline 󱖎
    BagSuitcaseOffOutline,
    /// BagSuitcaseOutline 󱖌
    BagSuitcaseOutline,
    /// Baguette 󰼾
    Baguette,
    /// Balcony 󱠗
    Balcony,
    /// Balloon 󰨦
    Balloon,
    /// Ballot 󰧉
    Ballot,
    /// BallotOutline 󰧊
    BallotOutline,
    /// BallotRecount 󰰹
    BallotRecount,
    /// BallotRecountOutline 󰰺
    BallotRecountOutline,
    /// BanCircle 
    BanCircle,
    /// Banana 
    Banana,
    /// Bandage 󰶯
    Bandage,
    /// Bank 󰁰
    Bank,
    /// BankCheck 󱙕
    BankCheck,
    /// BankMinus 󰶰
    BankMinus,
    /// BankOff 󱙖
    BankOff,
    /// BankOffOutline 󱙗
    BankOffOutline,
    /// BankOutline 󰺀
    BankOutline,
    /// BankPlus 󰶱
    BankPlus,
    /// BankRemove 󰶲
    BankRemove,
    /// BankTransfer 󰨧
    BankTransfer,
    /// BankTransferIn 󰨨
    BankTransferIn,
    /// BankTransferOut 󰨩
    BankTransferOut,
    /// BarChart 
    BarChart,
    /// Barcode 
    Barcode,
    /// BarcodeOff 󱈶
    BarcodeOff,
    /// BarcodeOne 󰁱
    BarcodeOne,
    /// BarcodeScan 󰁲
    BarcodeScan,
    /// Barley 󰁳
    Barley,
    /// BarleyOff 󰭝
    BarleyOff,
    /// Barn 󰭞
    Barn,
    /// Barrel 󰁴
    Barrel,
    /// BarrelOutline 󱨨
    BarrelOutline,
    /// Baseball 󰡒
    Baseball,
    /// BaseballBat 󰡓
    BaseballBat,
    /// BaseballDiamond 󱗬
    BaseballDiamond,
    /// BaseballDiamondOutline 󱗭
    BaseballDiamondOutline,
    /// Bash 󱆃
    Bash,
    /// Basket 󰁶
    Basket,
    /// BasketCheck 󱣥
    BasketCheck,
    /// BasketCheckOutline 󱣦
    BasketCheckOutline,
    /// BasketFill 󰁷
    BasketFill,
    /// BasketMinus 󱔣
    BasketMinus,
    /// BasketMinusOutline 󱔤
    BasketMinusOutline,
    /// BasketOff 󱔥
    BasketOff,
    /// BasketOffOutline 󱔦
    BasketOffOutline,
    /// BasketOutline 󱆁
    BasketOutline,
    /// BasketPlus 󱔧
    BasketPlus,
    /// BasketPlusOutline 󱔨
    BasketPlusOutline,
    /// BasketRemove 󱔩
    BasketRemove,
    /// BasketRemoveOutline 󱔪
    BasketRemoveOutline,
    /// BasketUnfill 󰁸
    BasketUnfill,
    /// Basketball 󰠆
    Basketball,
    /// BasketballHoop 󰰻
    BasketballHoop,
    /// BasketballHoopOutline 󰰼
    BasketballHoopOutline,
    /// Bat 󰭟
    Bat,
    /// Bath 
    Bath,
    /// Bathtub 󱠘
    Bathtub,
    /// BathtubOutline 󱠙
    BathtubOutline,
    /// Battery 󰁹
    Battery,
    /// BatteryAlert 󰂃
    BatteryAlert,
    /// BatteryAlertBluetooth 󰥇
    BatteryAlertBluetooth,
    /// BatteryAlertVariant 󱃌
    BatteryAlertVariant,
    /// BatteryAlertVariantOutline 󱃍
    BatteryAlertVariantOutline,
    /// BatteryArrowDown 󱟞
    BatteryArrowDown,
    /// BatteryArrowDownOutline 󱟟
    BatteryArrowDownOutline,
    /// BatteryArrowUp 󱟠
    BatteryArrowUp,
    /// BatteryArrowUpOutline 󱟡
    BatteryArrowUpOutline,
    /// BatteryBluetooth 󰥈
    BatteryBluetooth,
    /// BatteryBluetoothVariant 󰥉
    BatteryBluetoothVariant,
    /// BatteryCharging 󰂄
    BatteryCharging,
    /// BatteryChargingEightzero 󰂊
    BatteryChargingEightzero,
    /// BatteryChargingFivezero 󰢝
    BatteryChargingFivezero,
    /// BatteryChargingFourzero 󰂈
    BatteryChargingFourzero,
    /// BatteryChargingHigh 󱊦
    BatteryChargingHigh,
    /// BatteryChargingLow 󱊤
    BatteryChargingLow,
    /// BatteryChargingMedium 󱊥
    BatteryChargingMedium,
    /// BatteryChargingNinezero 󰂋
    BatteryChargingNinezero,
    /// BatteryChargingOnezero 󰢜
    BatteryChargingOnezero,
    /// BatteryChargingOnezerozero 󰂅
    BatteryChargingOnezerozero,
    /// BatteryChargingOutline 󰢟
    BatteryChargingOutline,
    /// BatteryChargingSevenzero 󰢞
    BatteryChargingSevenzero,
    /// BatteryChargingSixzero 󰂉
    BatteryChargingSixzero,
    /// BatteryChargingThreezero 󰂇
    BatteryChargingThreezero,
    /// BatteryChargingTwozero 󰂆
    BatteryChargingTwozero,
    /// BatteryChargingWireless 󰠇
    BatteryChargingWireless,
    /// BatteryChargingWirelessAlert 󰠑
    BatteryChargingWirelessAlert,
    /// BatteryChargingWirelessEightzero 󰠏
    BatteryChargingWirelessEightzero,
    /// BatteryChargingWirelessFivezero 󰠌
    BatteryChargingWirelessFivezero,
    /// BatteryChargingWirelessFourzero 󰠋
    BatteryChargingWirelessFourzero,
    /// BatteryChargingWirelessNinezero 󰠐
    BatteryChargingWirelessNinezero,
    /// BatteryChargingWirelessOnezero 󰠈
    BatteryChargingWirelessOnezero,
    /// BatteryChargingWirelessOutline 󰠒
    BatteryChargingWirelessOutline,
    /// BatteryChargingWirelessSevenzero 󰠎
    BatteryChargingWirelessSevenzero,
    /// BatteryChargingWirelessSixzero 󰠍
    BatteryChargingWirelessSixzero,
    /// BatteryChargingWirelessThreezero 󰠊
    BatteryChargingWirelessThreezero,
    /// BatteryChargingWirelessTwozero 󰠉
    BatteryChargingWirelessTwozero,
    /// BatteryCheck 󱟢
    BatteryCheck,
    /// BatteryCheckOutline 󱟣
    BatteryCheckOutline,
    /// BatteryClock 󱧥
    BatteryClock,
    /// BatteryClockOutline 󱧦
    BatteryClockOutline,
    /// BatteryEightzero 󰂁
    BatteryEightzero,
    /// BatteryEightzeroBluetooth 󰥅
    BatteryEightzeroBluetooth,
    /// BatteryFivezero 󰁾
    BatteryFivezero,
    /// BatteryFivezeroBluetooth 󰥂
    BatteryFivezeroBluetooth,
    /// BatteryFourzero 󰁽
    BatteryFourzero,
    /// BatteryFourzeroBluetooth 󰥁
    BatteryFourzeroBluetooth,
    /// BatteryHeart 󱈏
    BatteryHeart,
    /// BatteryHeartOutline 󱈐
    BatteryHeartOutline,
    /// BatteryHeartVariant 󱈑
    BatteryHeartVariant,
    /// BatteryHigh 󱊣
    BatteryHigh,
    /// BatteryLock 󱞜
    BatteryLock,
    /// BatteryLockOpen 󱞝
    BatteryLockOpen,
    /// BatteryLow 󱊡
    BatteryLow,
    /// BatteryMedium 󱊢
    BatteryMedium,
    /// BatteryMinus 󱟤
    BatteryMinus,
    /// BatteryMinusOutline 󱟥
    BatteryMinusOutline,
    /// BatteryMinusVariant 󰂌
    BatteryMinusVariant,
    /// BatteryNegative 󰂍
    BatteryNegative,
    /// BatteryNinezero 󰂂
    BatteryNinezero,
    /// BatteryNinezeroBluetooth 󰥆
    BatteryNinezeroBluetooth,
    /// BatteryOff 󱉝
    BatteryOff,
    /// BatteryOffOutline 󱉞
    BatteryOffOutline,
    /// BatteryOnezero 󰁺
    BatteryOnezero,
    /// BatteryOnezeroBluetooth 󰤾
    BatteryOnezeroBluetooth,
    /// BatteryOutline 󰂎
    BatteryOutline,
    /// BatteryPlus 󱟦
    BatteryPlus,
    /// BatteryPlusOutline 󱟧
    BatteryPlusOutline,
    /// BatteryPlusVariant 󰂏
    BatteryPlusVariant,
    /// BatteryPositive 󰂐
    BatteryPositive,
    /// BatteryRemove 󱟨
    BatteryRemove,
    /// BatteryRemoveOutline 󱟩
    BatteryRemoveOutline,
    /// BatterySevenzero 󰂀
    BatterySevenzero,
    /// BatterySevenzeroBluetooth 󰥄
    BatterySevenzeroBluetooth,
    /// BatterySixzero 󰁿
    BatterySixzero,
    /// BatterySixzeroBluetooth 󰥃
    BatterySixzeroBluetooth,
    /// BatterySync 󱠴
    BatterySync,
    /// BatterySyncOutline 󱠵
    BatterySyncOutline,
    /// BatteryThreezero 󰁼
    BatteryThreezero,
    /// BatteryThreezeroBluetooth 󰥀
    BatteryThreezeroBluetooth,
    /// BatteryTwozero 󰁻
    BatteryTwozero,
    /// BatteryTwozeroBluetooth 󰤿
    BatteryTwozeroBluetooth,
    /// BatteryUnknown 󰂑
    BatteryUnknown,
    /// BatteryUnknownBluetooth 󰥊
    BatteryUnknownBluetooth,
    /// Beach 󰂒
    Beach,
    /// Beaker 
    Beaker,
    /// BeakerAlert 󱈩
    BeakerAlert,
    /// BeakerAlertOutline 󱈪
    BeakerAlertOutline,
    /// BeakerCheck 󱈫
    BeakerCheck,
    /// BeakerCheckOutline 󱈬
    BeakerCheckOutline,
    /// BeakerMinus 󱈭
    BeakerMinus,
    /// BeakerMinusOutline 󱈮
    BeakerMinusOutline,
    /// BeakerOne 
    BeakerOne,
    /// BeakerOutline 󰚐
    BeakerOutline,
    /// BeakerPlus 󱈯
    BeakerPlus,
    /// BeakerPlusOutline 󱈰
    BeakerPlusOutline,
    /// BeakerQuestion 󱈱
    BeakerQuestion,
    /// BeakerQuestionOutline 󱈲
    BeakerQuestionOutline,
    /// BeakerRemove 󱈳
    BeakerRemove,
    /// BeakerRemoveOutline 󱈴
    BeakerRemoveOutline,
    /// BeakerStop 
    BeakerStop,
    /// BeakerThree 󰳪
    BeakerThree,
    /// BeakerTwo 
    BeakerTwo,
    /// Bed 
    Bed,
    /// BedDouble 󰿔
    BedDouble,
    /// BedDoubleOutline 󰿓
    BedDoubleOutline,
    /// BedEmpty 󰢠
    BedEmpty,
    /// BedKing 󰿒
    BedKing,
    /// BedKingOutline 󰿑
    BedKingOutline,
    /// BedOne 󰋣
    BedOne,
    /// BedOutline 󰂙
    BedOutline,
    /// BedQueen 󰿐
    BedQueen,
    /// BedQueenOutline 󰿛
    BedQueenOutline,
    /// BedSingle 󱁭
    BedSingle,
    /// BedSingleOutline 󱁮
    BedSingleOutline,
    /// Bee 󰾡
    Bee,
    /// BeeFlower 󰾢
    BeeFlower,
    /// BeehiveOffOutline 󱏭
    BeehiveOffOutline,
    /// BeehiveOutline 󱃎
    BeehiveOutline,
    /// Beekeeper 󱓢
    Beekeeper,
    /// Beer 
    Beer,
    /// BeerOne 󰂘
    BeerOne,
    /// BeerOutline 󱌌
    BeerOutline,
    /// Bell 
    Bell,
    /// BellAlert 󰵙
    BellAlert,
    /// BellAlertOutline 󰺁
    BellAlertOutline,
    /// BellAlt 
    BellAlt,
    /// BellBadge 󱅫
    BellBadge,
    /// BellBadgeOutline 󰅸
    BellBadgeOutline,
    /// BellCancel 󱏧
    BellCancel,
    /// BellCancelOutline 󱏨
    BellCancelOutline,
    /// BellCheck 󱇥
    BellCheck,
    /// BellCheckOutline 󱇦
    BellCheckOutline,
    /// BellCircle 󰵚
    BellCircle,
    /// BellCircleOutline 󰵛
    BellCircleOutline,
    /// BellCog 󱨩
    BellCog,
    /// BellCogOutline 󱨪
    BellCogOutline,
    /// BellDot 
    BellDot,
    /// BellFill 
    BellFill,
    /// BellMinus 󱏩
    BellMinus,
    /// BellMinusOutline 󱏪
    BellMinusOutline,
    /// BellOff 󰂛
    BellOff,
    /// BellOffOutline 󰪑
    BellOffOutline,
    /// BellOne 
    BellOne,
    /// BellOutline 󰂜
    BellOutline,
    /// BellPlus 󰂝
    BellPlus,
    /// BellPlusOutline 󰪒
    BellPlusOutline,
    /// BellRemove 󱏫
    BellRemove,
    /// BellRemoveOutline 󱏬
    BellRemoveOutline,
    /// BellRing 󰂞
    BellRing,
    /// BellRingOutline 󰂟
    BellRingOutline,
    /// BellSlash 
    BellSlash,
    /// BellSleep 󰂠
    BellSleep,
    /// BellSleepOutline 󰪓
    BellSleepOutline,
    /// BellThree 󰂚
    BellThree,
    /// BellTwo 
    BellTwo,
    /// Benzene 
    Benzene,
    /// Beta 󰂡
    Beta,
    /// Betamax 󰧋
    Betamax,
    /// Biathlon 󰸔
    Biathlon,
    /// Bicycle 󱂜
    Bicycle,
    /// BicycleBasket 󱈵
    BicycleBasket,
    /// BicycleCargo 󱢜
    BicycleCargo,
    /// BicycleElectric 󱖴
    BicycleElectric,
    /// BicyclePennyFarthing 󱗩
    BicyclePennyFarthing,
    /// Bigger 
    Bigger,
    /// Biglinux 
    Biglinux,
    /// Bike 󰂣
    Bike,
    /// BikeFast 󱄟
    BikeFast,
    /// Billboard 󱀐
    Billboard,
    /// Billiards 󰭡
    Billiards,
    /// BilliardsRack 󰭢
    BilliardsRack,
    /// Binoculars 󰂥
    Binoculars,
    /// Bio 󰂦
    Bio,
    /// Biohazard 
    Biohazard,
    /// BiohazardOne 󰂧
    BiohazardOne,
    /// Bird 󱗆
    Bird,
    /// Bitbucket 󰂨
    Bitbucket,
    /// BitbucketSign 
    BitbucketSign,
    /// Bitcoin 󰠓
    Bitcoin,
    /// BlackMesa 󰂩
    BlackMesa,
    /// Blender 󰳫
    Blender,
    /// BlenderOutline 󱠚
    BlenderOutline,
    /// BlenderSoftware 󰂫
    BlenderSoftware,
    /// Blinds 󰂬
    Blinds,
    /// BlindsHorizontal 󱨫
    BlindsHorizontal,
    /// BlindsHorizontalClosed 󱨬
    BlindsHorizontalClosed,
    /// BlindsOpen 󱀑
    BlindsOpen,
    /// BlindsVertical 󱨭
    BlindsVertical,
    /// BlindsVerticalClosed 󱨮
    BlindsVerticalClosed,
    /// BlockHelper 󰂭
    BlockHelper,
    /// Blocked 
    Blocked,
    /// BloggerCircle 
    BloggerCircle,
    /// BloggerSquare 
    BloggerSquare,
    /// BloodBag 󰳬
    BloodBag,
    /// Bluetooth 󰂯
    Bluetooth,
    /// BluetoothAudio 󰂰
    BluetoothAudio,
    /// BluetoothConnect 󰂱
    BluetoothConnect,
    /// BluetoothOff 󰂲
    BluetoothOff,
    /// BluetoothSettings 󰂳
    BluetoothSettings,
    /// BluetoothTransfer 󰂴
    BluetoothTransfer,
    /// Blur 󰂵
    Blur,
    /// BlurLinear 󰂶
    BlurLinear,
    /// BlurOff 󰂷
    BlurOff,
    /// BlurRadial 󰂸
    BlurRadial,
    /// Bold 
    Bold,
    /// BoldOne 
    BoldOne,
    /// BoldTwo 
    BoldTwo,
    /// Bolt 
    Bolt,
    /// BoltOne 󰶳
    BoltOne,
    /// Bomb 󰚑
    Bomb,
    /// BombOff 󰛅
    BombOff,
    /// Bone 󰂹
    Bone,
    /// BoneOff 󱧠
    BoneOff,
    /// Bones 
    Bones,
    /// Book 
    Book,
    /// BookAccount 󱎭
    BookAccount,
    /// BookAccountOutline 󱎮
    BookAccountOutline,
    /// BookAlert 󱙼
    BookAlert,
    /// BookAlertOutline 󱙽
    BookAlertOutline,
    /// BookAlphabet 󰘝
    BookAlphabet,
    /// BookArrowDown 󱙾
    BookArrowDown,
    /// BookArrowDownOutline 󱙿
    BookArrowDownOutline,
    /// BookArrowLeft 󱚀
    BookArrowLeft,
    /// BookArrowLeftOutline 󱚁
    BookArrowLeftOutline,
    /// BookArrowRight 󱚂
    BookArrowRight,
    /// BookArrowRightOutline 󱚃
    BookArrowRightOutline,
    /// BookArrowUp 󱚄
    BookArrowUp,
    /// BookArrowUpOutline 󱚅
    BookArrowUpOutline,
    /// BookCancel 󱚆
    BookCancel,
    /// BookCancelOutline 󱚇
    BookCancelOutline,
    /// BookCheck 󱓳
    BookCheck,
    /// BookCheckOutline 󱓴
    BookCheckOutline,
    /// BookClock 󱚈
    BookClock,
    /// BookClockOutline 󱚉
    BookClockOutline,
    /// BookCog 󱚊
    BookCog,
    /// BookCogOutline 󱚋
    BookCogOutline,
    /// BookCross 󰂢
    BookCross,
    /// BookEdit 󱚌
    BookEdit,
    /// BookEditOutline 󱚍
    BookEditOutline,
    /// BookEducation 󱛉
    BookEducation,
    /// BookEducationOutline 󱛊
    BookEducationOutline,
    /// BookHeart 󱨝
    BookHeart,
    /// BookHeartOutline 󱨞
    BookHeartOutline,
    /// BookInformationVariant 󱁯
    BookInformationVariant,
    /// BookLock 󰞚
    BookLock,
    /// BookLockOpen 󰞛
    BookLockOpen,
    /// BookLockOpenOutline 󱚎
    BookLockOpenOutline,
    /// BookLockOutline 󱚏
    BookLockOutline,
    /// BookMarker 󱚐
    BookMarker,
    /// BookMarkerOutline 󱚑
    BookMarkerOutline,
    /// BookMinus 󰗙
    BookMinus,
    /// BookMinusMultiple 󰪔
    BookMinusMultiple,
    /// BookMinusMultipleOutline 󰤋
    BookMinusMultipleOutline,
    /// BookMinusOutline 󱚒
    BookMinusOutline,
    /// BookMultiple 󰂻
    BookMultiple,
    /// BookMultipleOutline 󰐶
    BookMultipleOutline,
    /// BookMusic 󰁧
    BookMusic,
    /// BookMusicOutline 󱚓
    BookMusicOutline,
    /// BookOff 󱚔
    BookOff,
    /// BookOffOutline 󱚕
    BookOffOutline,
    /// BookOne 
    BookOne,
    /// BookOpen 
    BookOpen,
    /// BookOpenBlankVariant 󰂾
    BookOpenBlankVariant,
    /// BookOpenO 
    BookOpenO,
    /// BookOpenOne 󰂽
    BookOpenOne,
    /// BookOpenOutline 󰭣
    BookOpenOutline,
    /// BookOpenPageVariant 󰗚
    BookOpenPageVariant,
    /// BookOpenPageVariantOutline 󱗖
    BookOpenPageVariantOutline,
    /// BookOpenVariant 󱓷
    BookOpenVariant,
    /// BookOutline 󰭤
    BookOutline,
    /// BookPlay 󰺂
    BookPlay,
    /// BookPlayOutline 󰺃
    BookPlayOutline,
    /// BookPlus 󰗛
    BookPlus,
    /// BookPlusMultiple 󰪕
    BookPlusMultiple,
    /// BookPlusMultipleOutline 󰫞
    BookPlusMultipleOutline,
    /// BookPlusOutline 󱚖
    BookPlusOutline,
    /// BookRefresh 󱚗
    BookRefresh,
    /// BookRefreshOutline 󱚘
    BookRefreshOutline,
    /// BookRemove 󰪗
    BookRemove,
    /// BookRemoveMultiple 󰪖
    BookRemoveMultiple,
    /// BookRemoveMultipleOutline 󰓊
    BookRemoveMultipleOutline,
    /// BookRemoveOutline 󱚙
    BookRemoveOutline,
    /// BookSearch 󰺄
    BookSearch,
    /// BookSearchOutline 󰺅
    BookSearchOutline,
    /// BookSettings 󱚚
    BookSettings,
    /// BookSettingsOutline 󱚛
    BookSettingsOutline,
    /// BookSync 󱚜
    BookSync,
    /// BookSyncOutline 󱛈
    BookSyncOutline,
    /// BookThree 󰂺
    BookThree,
    /// BookTwo 
    BookTwo,
    /// BookVariant 󰂿
    BookVariant,
    /// BookVariantMultiple 󰂼
    BookVariantMultiple,
    /// Bookmark 
    Bookmark,
    /// BookmarkBoxMultiple 󱥬
    BookmarkBoxMultiple,
    /// BookmarkBoxMultipleOutline 󱥭
    BookmarkBoxMultipleOutline,
    /// BookmarkCheck 󰃁
    BookmarkCheck,
    /// BookmarkCheckOutline 󱍻
    BookmarkCheckOutline,
    /// BookmarkEmpty 
    BookmarkEmpty,
    /// BookmarkFill 
    BookmarkFill,
    /// BookmarkMinus 󰧌
    BookmarkMinus,
    /// BookmarkMinusOutline 󰧍
    BookmarkMinusOutline,
    /// BookmarkMultiple 󰸕
    BookmarkMultiple,
    /// BookmarkMultipleOutline 󰸖
    BookmarkMultipleOutline,
    /// BookmarkMusic 󰃂
    BookmarkMusic,
    /// BookmarkMusicOutline 󱍹
    BookmarkMusicOutline,
    /// BookmarkOff 󰧎
    BookmarkOff,
    /// BookmarkOffOutline 󰧏
    BookmarkOffOutline,
    /// BookmarkOne 
    BookmarkOne,
    /// BookmarkOutline 󰃃
    BookmarkOutline,
    /// BookmarkPlus 󰃅
    BookmarkPlus,
    /// BookmarkPlusOutline 󰃄
    BookmarkPlusOutline,
    /// BookmarkRemove 󰃆
    BookmarkRemove,
    /// BookmarkRemoveOutline 󱍺
    BookmarkRemoveOutline,
    /// BookmarkSlash 
    BookmarkSlash,
    /// BookmarkSlashFill 
    BookmarkSlashFill,
    /// BookmarkThree 󰃀
    BookmarkThree,
    /// BookmarkTwo 
    BookmarkTwo,
    /// Bookshelf 󱉟
    Bookshelf,
    /// BoomGate 󰺆
    BoomGate,
    /// BoomGateAlert 󰺇
    BoomGateAlert,
    /// BoomGateAlertOutline 󰺈
    BoomGateAlertOutline,
    /// BoomGateArrowDown 󰺉
    BoomGateArrowDown,
    /// BoomGateArrowDownOutline 󰺊
    BoomGateArrowDownOutline,
    /// BoomGateArrowUp 󰺌
    BoomGateArrowUp,
    /// BoomGateArrowUpOutline 󰺍
    BoomGateArrowUpOutline,
    /// BoomGateOutline 󰺋
    BoomGateOutline,
    /// BoomGateUp 󱟹
    BoomGateUp,
    /// BoomGateUpOutline 󱟺
    BoomGateUpOutline,
    /// Boombox 󰗜
    Boombox,
    /// Boomerang 󱃏
    Boomerang,
    /// Bootstrap 󰛆
    Bootstrap,
    /// BorderAll 󰃇
    BorderAll,
    /// BorderAllVariant 󰢡
    BorderAllVariant,
    /// BorderBottom 󰃈
    BorderBottom,
    /// BorderBottomVariant 󰢢
    BorderBottomVariant,
    /// BorderColor 󰃉
    BorderColor,
    /// BorderHorizontal 󰃊
    BorderHorizontal,
    /// BorderInside 󰃋
    BorderInside,
    /// BorderLeft 󰃌
    BorderLeft,
    /// BorderLeftVariant 󰢣
    BorderLeftVariant,
    /// BorderNone 󰃍
    BorderNone,
    /// BorderNoneVariant 󰢤
    BorderNoneVariant,
    /// BorderOutside 󰃎
    BorderOutside,
    /// BorderRight 󰃏
    BorderRight,
    /// BorderRightVariant 󰢥
    BorderRightVariant,
    /// BorderStyle 󰃐
    BorderStyle,
    /// BorderTop 󰃑
    BorderTop,
    /// BorderTopVariant 󰢦
    BorderTopVariant,
    /// BorderVertical 󰃒
    BorderVertical,
    /// BottleSoda 󱁰
    BottleSoda,
    /// BottleSodaClassic 󱁱
    BottleSodaClassic,
    /// BottleSodaClassicOutline 󱍣
    BottleSodaClassicOutline,
    /// BottleSodaOutline 󱁲
    BottleSodaOutline,
    /// BottleTonic 󱄮
    BottleTonic,
    /// BottleTonicOutline 󱄯
    BottleTonicOutline,
    /// BottleTonicPlus 󱄰
    BottleTonicPlus,
    /// BottleTonicPlusOutline 󱄱
    BottleTonicPlusOutline,
    /// BottleTonicSkull 󱄲
    BottleTonicSkull,
    /// BottleTonicSkullOutline 󱄳
    BottleTonicSkullOutline,
    /// BottleWine 󰡔
    BottleWine,
    /// BottleWineOutline 󱌐
    BottleWineOutline,
    /// BowArrow 󱡁
    BowArrow,
    /// BowTie 󰙸
    BowTie,
    /// Bowl 󰊎
    Bowl,
    /// BowlMix 󰘗
    BowlMix,
    /// BowlMixOutline 󰋤
    BowlMixOutline,
    /// BowlOutline 󰊩
    BowlOutline,
    /// Bowling 󰃓
    Bowling,
    /// Box 󰃔
    Box,
    /// BoxCutter 󰃕
    BoxCutter,
    /// BoxCutterOff 󰭊
    BoxCutterOff,
    /// BoxShadow 󰘷
    BoxShadow,
    /// BoxingGlove 󰭥
    BoxingGlove,
    /// BracketDot 
    BracketDot,
    /// BracketError 
    BracketError,
    /// Braille 󰧐
    Braille,
    /// Brain 
    Brain,
    /// BrainOne 󰧑
    BrainOne,
    /// Bread 
    Bread,
    /// BreadSlice 󰳮
    BreadSlice,
    /// BreadSliceOutline 󰳯
    BreadSliceOutline,
    /// Bridge 󰘘
    Bridge,
    /// Briefcase 
    Briefcase,
    /// BriefcaseAccount 󰳰
    BriefcaseAccount,
    /// BriefcaseAccountOutline 󰳱
    BriefcaseAccountOutline,
    /// BriefcaseArrowLeftRight 󱪍
    BriefcaseArrowLeftRight,
    /// BriefcaseArrowLeftRightOutline 󱪎
    BriefcaseArrowLeftRightOutline,
    /// BriefcaseArrowUpDown 󱪏
    BriefcaseArrowUpDown,
    /// BriefcaseArrowUpDownOutline 󱪐
    BriefcaseArrowUpDownOutline,
    /// BriefcaseCheck 󰃗
    BriefcaseCheck,
    /// BriefcaseCheckOutline 󱌞
    BriefcaseCheckOutline,
    /// BriefcaseClock 󱃐
    BriefcaseClock,
    /// BriefcaseClockOutline 󱃑
    BriefcaseClockOutline,
    /// BriefcaseDownload 󰃘
    BriefcaseDownload,
    /// BriefcaseDownloadOutline 󰰽
    BriefcaseDownloadOutline,
    /// BriefcaseEdit 󰪘
    BriefcaseEdit,
    /// BriefcaseEditOutline 󰰾
    BriefcaseEditOutline,
    /// BriefcaseEye 󱟙
    BriefcaseEye,
    /// BriefcaseEyeOutline 󱟚
    BriefcaseEyeOutline,
    /// BriefcaseMinus 󰨪
    BriefcaseMinus,
    /// BriefcaseMinusOutline 󰰿
    BriefcaseMinusOutline,
    /// BriefcaseOff 󱙘
    BriefcaseOff,
    /// BriefcaseOffOutline 󱙙
    BriefcaseOffOutline,
    /// BriefcaseOne 
    BriefcaseOne,
    /// BriefcaseOutline 󰠔
    BriefcaseOutline,
    /// BriefcasePlus 󰨫
    BriefcasePlus,
    /// BriefcasePlusOutline 󰱀
    BriefcasePlusOutline,
    /// BriefcaseRemove 󰨬
    BriefcaseRemove,
    /// BriefcaseRemoveOutline 󰱁
    BriefcaseRemoveOutline,
    /// BriefcaseSearch 󰨭
    BriefcaseSearch,
    /// BriefcaseSearchOutline 󰱂
    BriefcaseSearchOutline,
    /// BriefcaseThree 󰃖
    BriefcaseThree,
    /// BriefcaseTwo 
    BriefcaseTwo,
    /// BriefcaseUpload 󰃙
    BriefcaseUpload,
    /// BriefcaseUploadOutline 󰱃
    BriefcaseUploadOutline,
    /// BriefcaseVariant 󱒔
    BriefcaseVariant,
    /// BriefcaseVariantOff 󱙚
    BriefcaseVariantOff,
    /// BriefcaseVariantOffOutline 󱙛
    BriefcaseVariantOffOutline,
    /// BriefcaseVariantOutline 󱒕
    BriefcaseVariantOutline,
    /// BrightnessAuto 󰃡
    BrightnessAuto,
    /// BrightnessFive 󰃞
    BrightnessFive,
    /// BrightnessFour 󰃝
    BrightnessFour,
    /// BrightnessOne 󰃚
    BrightnessOne,
    /// BrightnessPercent 󰳲
    BrightnessPercent,
    /// BrightnessSeven 󰃠
    BrightnessSeven,
    /// BrightnessSix 󰃟
    BrightnessSix,
    /// BrightnessThree 󰃜
    BrightnessThree,
    /// BrightnessTwo 󰃛
    BrightnessTwo,
    /// Broadcast 
    Broadcast,
    /// BroadcastOff 󱜡
    BroadcastOff,
    /// BroadcastOne 
    BroadcastOne,
    /// BroadcastTwo 󱜠
    BroadcastTwo,
    /// Broom 󰃢
    Broom,
    /// Browser 
    Browser,
    /// BrowserOne 
    BrowserOne,
    /// Brush 󰃣
    Brush,
    /// BrushOff 󱝱
    BrushOff,
    /// BrushOutline 󱨍
    BrushOutline,
    /// BrushVariant 󱠓
    BrushVariant,
    /// Bspwm 
    Bspwm,
    /// Btc 
    Btc,
    /// Bucket 󱐕
    Bucket,
    /// BucketOutline 󱐖
    BucketOutline,
    /// Budgie 
    Budgie,
    /// Buffet 󰕸
    Buffet,
    /// Bug 
    Bug,
    /// BugCheck 󰨮
    BugCheck,
    /// BugCheckOutline 󰨯
    BugCheckOutline,
    /// BugOne 
    BugOne,
    /// BugOutline 󰨰
    BugOutline,
    /// BugThree 󰃤
    BugThree,
    /// BugTwo 
    BugTwo,
    /// Bugle 󰶴
    Bugle,
    /// Building 
    Building,
    /// BulkheadLight 󱨯
    BulkheadLight,
    /// Bulldozer 󰬢
    Bulldozer,
    /// Bullet 󰳳
    Bullet,
    /// BulletinBoard 󰃥
    BulletinBoard,
    /// Bullhorn 
    Bullhorn,
    /// BullhornOne 󰃦
    BullhornOne,
    /// BullhornOutline 󰬣
    BullhornOutline,
    /// BullhornVariant 󱥮
    BullhornVariant,
    /// BullhornVariantOutline 󱥯
    BullhornVariantOutline,
    /// Bullseye 
    Bullseye,
    /// BullseyeArrow 󰣉
    BullseyeArrow,
    /// BullseyeOne 󰗝
    BullseyeOne,
    /// Bulma 󱋧
    Bulma,
    /// BunkBed 󱌂
    BunkBed,
    /// BunkBedOutline 󰂗
    BunkBedOutline,
    /// Bus 󰃧
    Bus,
    /// BusAlert 󰪙
    BusAlert,
    /// BusArticulatedEnd 󰞜
    BusArticulatedEnd,
    /// BusArticulatedFront 󰞝
    BusArticulatedFront,
    /// BusClock 󰣊
    BusClock,
    /// BusDoubleDecker 󰞞
    BusDoubleDecker,
    /// BusElectric 󱤝
    BusElectric,
    /// BusMarker 󱈒
    BusMarker,
    /// BusMultiple 󰼿
    BusMultiple,
    /// BusSchool 󰞟
    BusSchool,
    /// BusSide 󰞠
    BusSide,
    /// BusStop 󱀒
    BusStop,
    /// BusStopCovered 󱀓
    BusStopCovered,
    /// BusStopUncovered 󱀔
    BusStopUncovered,
    /// Butterfly 
    Butterfly,
    /// ButterflyOne 󱖉
    ButterflyOne,
    /// ButterflyOutline 󱖊
    ButterflyOutline,
    /// CabinAFrame 󱢌
    CabinAFrame,
    /// CableData 󱎔
    CableData,
    /// Cache 
    Cache,
    /// Cached 󰃨
    Cached,
    /// Cactus 󰶵
    Cactus,
    /// Cake 󰃩
    Cake,
    /// CakeLayered 󰃪
    CakeLayered,
    /// CakeVariant 󰃫
    CakeVariant,
    /// CakeVariantOutline 󱟰
    CakeVariantOutline,
    /// Calculator 󰃬
    Calculator,
    /// CalculatorVariant 󰪚
    CalculatorVariant,
    /// CalculatorVariantOutline 󱖦
    CalculatorVariantOutline,
    /// Calendar 
    Calendar,
    /// CalendarAccount 󰻗
    CalendarAccount,
    /// CalendarAccountOutline 󰻘
    CalendarAccountOutline,
    /// CalendarAlert 󰨱
    CalendarAlert,
    /// CalendarArrowLeft 󱄴
    CalendarArrowLeft,
    /// CalendarArrowRight 󱄵
    CalendarArrowRight,
    /// CalendarBlank 󰃮
    CalendarBlank,
    /// CalendarBlankMultiple 󱁳
    CalendarBlankMultiple,
    /// CalendarBlankOutline 󰭦
    CalendarBlankOutline,
    /// CalendarCheck 󰃯
    CalendarCheck,
    /// CalendarCheckOutline 󰱄
    CalendarCheckOutline,
    /// CalendarClock 󰃰
    CalendarClock,
    /// CalendarClockOutline 󱛡
    CalendarClockOutline,
    /// CalendarCollapseHorizontal 󱢝
    CalendarCollapseHorizontal,
    /// CalendarCursor 󱕻
    CalendarCursor,
    /// CalendarEdit 󰢧
    CalendarEdit,
    /// CalendarEmpty 
    CalendarEmpty,
    /// CalendarEnd 󱙬
    CalendarEnd,
    /// CalendarExpandHorizontal 󱢞
    CalendarExpandHorizontal,
    /// CalendarExport 󰬤
    CalendarExport,
    /// CalendarHeart 󰧒
    CalendarHeart,
    /// CalendarImport 󰬥
    CalendarImport,
    /// CalendarLock 󱙁
    CalendarLock,
    /// CalendarLockOutline 󱙂
    CalendarLockOutline,
    /// CalendarMinus 󰵜
    CalendarMinus,
    /// CalendarMonth 󰸗
    CalendarMonth,
    /// CalendarMonthOutline 󰸘
    CalendarMonthOutline,
    /// CalendarMultiple 󰃱
    CalendarMultiple,
    /// CalendarMultipleCheck 󰃲
    CalendarMultipleCheck,
    /// CalendarMultiselect 󰨲
    CalendarMultiselect,
    /// CalendarOne 
    CalendarOne,
    /// CalendarOutline 󰭧
    CalendarOutline,
    /// CalendarPlus 󰃳
    CalendarPlus,
    /// CalendarQuestion 󰚒
    CalendarQuestion,
    /// CalendarRange 󰙹
    CalendarRange,
    /// CalendarRangeOutline 󰭨
    CalendarRangeOutline,
    /// CalendarRefresh 󰇡
    CalendarRefresh,
    /// CalendarRefreshOutline 󰈃
    CalendarRefreshOutline,
    /// CalendarRemove 󰃴
    CalendarRemove,
    /// CalendarRemoveOutline 󰱅
    CalendarRemoveOutline,
    /// CalendarSearch 󰥌
    CalendarSearch,
    /// CalendarStar 󰧓
    CalendarStar,
    /// CalendarStart 󱙭
    CalendarStart,
    /// CalendarSync 󰺎
    CalendarSync,
    /// CalendarSyncOutline 󰺏
    CalendarSyncOutline,
    /// CalendarText 󰃵
    CalendarText,
    /// CalendarTextOutline 󰱆
    CalendarTextOutline,
    /// CalendarThree 󰃭
    CalendarThree,
    /// CalendarToday 󰃶
    CalendarToday,
    /// CalendarTodayOutline 󱨰
    CalendarTodayOutline,
    /// CalendarTwo 
    CalendarTwo,
    /// CalendarWeek 󰨳
    CalendarWeek,
    /// CalendarWeekBegin 󰨴
    CalendarWeekBegin,
    /// CalendarWeekBeginOutline 󱨱
    CalendarWeekBeginOutline,
    /// CalendarWeekEnd 󱨲
    CalendarWeekEnd,
    /// CalendarWeekEndOutline 󱨳
    CalendarWeekEndOutline,
    /// CalendarWeekOutline 󱨴
    CalendarWeekOutline,
    /// CalendarWeekend 󰻙
    CalendarWeekend,
    /// CalendarWeekendOutline 󰻚
    CalendarWeekendOutline,
    /// CallIncoming 
    CallIncoming,
    /// CallMade 󰃷
    CallMade,
    /// CallMerge 󰃸
    CallMerge,
    /// CallMissed 󰃹
    CallMissed,
    /// CallOutgoing 
    CallOutgoing,
    /// CallReceived 󰃺
    CallReceived,
    /// CallSplit 󰃻
    CallSplit,
    /// Camcorder 󰃼
    Camcorder,
    /// CamcorderOff 󰃿
    CamcorderOff,
    /// Camera 
    Camera,
    /// CameraAccount 󰣋
    CameraAccount,
    /// CameraBurst 󰚓
    CameraBurst,
    /// CameraControl 󰭩
    CameraControl,
    /// CameraDocument 󱡱
    CameraDocument,
    /// CameraDocumentOff 󱡲
    CameraDocumentOff,
    /// CameraEnhance 󰄁
    CameraEnhance,
    /// CameraEnhanceOutline 󰭪
    CameraEnhanceOutline,
    /// CameraFlip 󱗙
    CameraFlip,
    /// CameraFlipOutline 󱗚
    CameraFlipOutline,
    /// CameraFront 󰄂
    CameraFront,
    /// CameraFrontVariant 󰄃
    CameraFrontVariant,
    /// CameraGopro 󰞡
    CameraGopro,
    /// CameraImage 󰣌
    CameraImage,
    /// CameraIris 󰄄
    CameraIris,
    /// CameraLock 󱨔
    CameraLock,
    /// CameraLockOutline 󱨕
    CameraLockOutline,
    /// CameraMarker 󱦧
    CameraMarker,
    /// CameraMarkerOutline 󱦨
    CameraMarkerOutline,
    /// CameraMeteringCenter 󰞢
    CameraMeteringCenter,
    /// CameraMeteringMatrix 󰞣
    CameraMeteringMatrix,
    /// CameraMeteringPartial 󰞤
    CameraMeteringPartial,
    /// CameraMeteringSpot 󰞥
    CameraMeteringSpot,
    /// CameraOff 󰗟
    CameraOff,
    /// CameraOffOutline 󱦿
    CameraOffOutline,
    /// CameraOne 󰄀
    CameraOne,
    /// CameraOutline 󰵝
    CameraOutline,
    /// CameraPartyMode 󰄅
    CameraPartyMode,
    /// CameraPlus 󰻛
    CameraPlus,
    /// CameraPlusOutline 󰻜
    CameraPlusOutline,
    /// CameraRear 󰄆
    CameraRear,
    /// CameraRearVariant 󰄇
    CameraRearVariant,
    /// CameraRetake 󰸙
    CameraRetake,
    /// CameraRetakeOutline 󰸚
    CameraRetakeOutline,
    /// CameraRetro 
    CameraRetro,
    /// CameraSwitch 󰄈
    CameraSwitch,
    /// CameraSwitchOutline 󰡊
    CameraSwitchOutline,
    /// CameraTimer 󰄉
    CameraTimer,
    /// CameraWireless 󰶶
    CameraWireless,
    /// CameraWirelessOutline 󰶷
    CameraWirelessOutline,
    /// Campfire 󰻝
    Campfire,
    /// Cancel 󰜺
    Cancel,
    /// Candelabra 󱟒
    Candelabra,
    /// CandelabraFire 󱟓
    CandelabraFire,
    /// Candle 󰗢
    Candle,
    /// Candy 󱥰
    Candy,
    /// CandyOff 󱥱
    CandyOff,
    /// CandyOffOutline 󱥲
    CandyOffOutline,
    /// CandyOutline 󱥳
    CandyOutline,
    /// Candycane 󰄊
    Candycane,
    /// Cannabis 󰞦
    Cannabis,
    /// CannabisOff 󱙮
    CannabisOff,
    /// CapsLock 󰪛
    CapsLock,
    /// Car 󰄋
    Car,
    /// CarArrowLeft 󱎲
    CarArrowLeft,
    /// CarArrowRight 󱎳
    CarArrowRight,
    /// CarBack 󰸛
    CarBack,
    /// CarBattery 󰄌
    CarBattery,
    /// CarBrakeAbs 󰱇
    CarBrakeAbs,
    /// CarBrakeAlert 󰱈
    CarBrakeAlert,
    /// CarBrakeFluidLevel 󱤉
    CarBrakeFluidLevel,
    /// CarBrakeHold 󰵞
    CarBrakeHold,
    /// CarBrakeLowPressure 󱤊
    CarBrakeLowPressure,
    /// CarBrakeParking 󰵟
    CarBrakeParking,
    /// CarBrakeRetarder 󱀗
    CarBrakeRetarder,
    /// CarBrakeTemperature 󱤋
    CarBrakeTemperature,
    /// CarBrakeWornLinings 󱤌
    CarBrakeWornLinings,
    /// CarChildSeat 󰾣
    CarChildSeat,
    /// CarClock 󱥴
    CarClock,
    /// CarClutch 󱀘
    CarClutch,
    /// CarCog 󱏌
    CarCog,
    /// CarConnected 󰄍
    CarConnected,
    /// CarConvertible 󰞧
    CarConvertible,
    /// CarCoolantLevel 󱀙
    CarCoolantLevel,
    /// CarCruiseControl 󰵠
    CarCruiseControl,
    /// CarDefrostFront 󰵡
    CarDefrostFront,
    /// CarDefrostRear 󰵢
    CarDefrostRear,
    /// CarDoor 󰭫
    CarDoor,
    /// CarDoorLock 󱂝
    CarDoorLock,
    /// CarElectric 󰭬
    CarElectric,
    /// CarElectricOutline 󱖵
    CarElectricOutline,
    /// CarEmergency 󱘏
    CarEmergency,
    /// CarEsp 󰱉
    CarEsp,
    /// CarEstate 󰞨
    CarEstate,
    /// CarHatchback 󰞩
    CarHatchback,
    /// CarInfo 󱆾
    CarInfo,
    /// CarKey 󰭭
    CarKey,
    /// CarLiftedPickup 󱔭
    CarLiftedPickup,
    /// CarLightAlert 󱤍
    CarLightAlert,
    /// CarLightDimmed 󰱊
    CarLightDimmed,
    /// CarLightFog 󰱋
    CarLightFog,
    /// CarLightHigh 󰱌
    CarLightHigh,
    /// CarLimousine 󰣍
    CarLimousine,
    /// CarMultiple 󰭮
    CarMultiple,
    /// CarOff 󰸜
    CarOff,
    /// CarOutline 󱓭
    CarOutline,
    /// CarParkingLights 󰵣
    CarParkingLights,
    /// CarPickup 󰞪
    CarPickup,
    /// CarSeat 󰾤
    CarSeat,
    /// CarSeatCooler 󰾥
    CarSeatCooler,
    /// CarSeatHeater 󰾦
    CarSeatHeater,
    /// CarSelect 󱡹
    CarSelect,
    /// CarSettings 󱏍
    CarSettings,
    /// CarShiftPattern 󰽀
    CarShiftPattern,
    /// CarSide 󰞫
    CarSide,
    /// CarSpeedLimiter 󱤎
    CarSpeedLimiter,
    /// CarSports 󰞬
    CarSports,
    /// CarThreePlus 󱀖
    CarThreePlus,
    /// CarTireAlert 󰱍
    CarTireAlert,
    /// CarTractionControl 󰵤
    CarTractionControl,
    /// CarTurbocharger 󱀚
    CarTurbocharger,
    /// CarTwoPlus 󱀕
    CarTwoPlus,
    /// CarWash 󰄎
    CarWash,
    /// CarWindshield 󱀛
    CarWindshield,
    /// CarWindshieldOutline 󱀜
    CarWindshieldOutline,
    /// CarWireless 󱡸
    CarWireless,
    /// CarWrench 󱠔
    CarWrench,
    /// Carabiner 󱓀
    Carabiner,
    /// Caravan 󰞭
    Caravan,
    /// Card 󰭯
    Card,
    /// CardAccountDetails 󰗒
    CardAccountDetails,
    /// CardAccountDetailsOutline 󰶫
    CardAccountDetailsOutline,
    /// CardAccountDetailsStar 󰊣
    CardAccountDetailsStar,
    /// CardAccountDetailsStarOutline 󰛛
    CardAccountDetailsStarOutline,
    /// CardAccountMail 󰆎
    CardAccountMail,
    /// CardAccountMailOutline 󰺘
    CardAccountMailOutline,
    /// CardAccountPhone 󰺙
    CardAccountPhone,
    /// CardAccountPhoneOutline 󰺚
    CardAccountPhoneOutline,
    /// CardBulleted 󰭰
    CardBulleted,
    /// CardBulletedOff 󰭱
    CardBulletedOff,
    /// CardBulletedOffOutline 󰭲
    CardBulletedOffOutline,
    /// CardBulletedOutline 󰭳
    CardBulletedOutline,
    /// CardBulletedSettings 󰭴
    CardBulletedSettings,
    /// CardBulletedSettingsOutline 󰭵
    CardBulletedSettingsOutline,
    /// CardMinus 󱘀
    CardMinus,
    /// CardMinusOutline 󱘁
    CardMinusOutline,
    /// CardMultiple 󱟱
    CardMultiple,
    /// CardMultipleOutline 󱟲
    CardMultipleOutline,
    /// CardOff 󱘂
    CardOff,
    /// CardOffOutline 󱘃
    CardOffOutline,
    /// CardOutline 󰭶
    CardOutline,
    /// CardPlus 󱇿
    CardPlus,
    /// CardPlusOutline 󱈀
    CardPlusOutline,
    /// CardRemove 󱘄
    CardRemove,
    /// CardRemoveOutline 󱘅
    CardRemoveOutline,
    /// CardSearch 󱁴
    CardSearch,
    /// CardSearchOutline 󱁵
    CardSearchOutline,
    /// CardText 󰭷
    CardText,
    /// CardTextOutline 󰭸
    CardTextOutline,
    /// Cards 󰘸
    Cards,
    /// CardsClub 󰣎
    CardsClub,
    /// CardsClubOutline 󱢟
    CardsClubOutline,
    /// CardsDiamond 󰣏
    CardsDiamond,
    /// CardsDiamondOutline 󱀝
    CardsDiamondOutline,
    /// CardsOutline 󰘹
    CardsOutline,
    /// CardsPlaying 󱢡
    CardsPlaying,
    /// CardsPlayingClub 󱢢
    CardsPlayingClub,
    /// CardsPlayingClubMultiple 󱢣
    CardsPlayingClubMultiple,
    /// CardsPlayingClubMultipleOutline 󱢤
    CardsPlayingClubMultipleOutline,
    /// CardsPlayingClubOutline 󱢥
    CardsPlayingClubOutline,
    /// CardsPlayingDiamond 󱢦
    CardsPlayingDiamond,
    /// CardsPlayingDiamondMultiple 󱢧
    CardsPlayingDiamondMultiple,
    /// CardsPlayingDiamondMultipleOutline 󱢨
    CardsPlayingDiamondMultipleOutline,
    /// CardsPlayingDiamondOutline 󱢩
    CardsPlayingDiamondOutline,
    /// CardsPlayingHeart 󱢪
    CardsPlayingHeart,
    /// CardsPlayingHeartMultiple 󱢫
    CardsPlayingHeartMultiple,
    /// CardsPlayingHeartMultipleOutline 󱢬
    CardsPlayingHeartMultipleOutline,
    /// CardsPlayingHeartOutline 󱢭
    CardsPlayingHeartOutline,
    /// CardsPlayingOutline 󰘺
    CardsPlayingOutline,
    /// CardsPlayingSpade 󱢮
    CardsPlayingSpade,
    /// CardsPlayingSpadeMultiple 󱢯
    CardsPlayingSpadeMultiple,
    /// CardsPlayingSpadeMultipleOutline 󱢰
    CardsPlayingSpadeMultipleOutline,
    /// CardsPlayingSpadeOutline 󱢱
    CardsPlayingSpadeOutline,
    /// CardsSpade 󰣑
    CardsSpade,
    /// CardsSpadeOutline 󱢲
    CardsSpadeOutline,
    /// CardsVariant 󰛇
    CardsVariant,
    /// CaretDown 
    CaretDown,
    /// CaretLeft 
    CaretLeft,
    /// CaretRight 
    CaretRight,
    /// CaretUp 
    CaretUp,
    /// Carot 
    Carot,
    /// Carrot 󰄏
    Carrot,
    /// Cart 󰄐
    Cart,
    /// CartArrowDown 󰵦
    CartArrowDown,
    /// CartArrowRight 󰱎
    CartArrowRight,
    /// CartArrowUp 󰵧
    CartArrowUp,
    /// CartCheck 󱗪
    CartCheck,
    /// CartHeart 󱣠
    CartHeart,
    /// CartMinus 󰵨
    CartMinus,
    /// CartOff 󰙫
    CartOff,
    /// CartOutline 󰄑
    CartOutline,
    /// CartPlus 󰄒
    CartPlus,
    /// CartRemove 󰵩
    CartRemove,
    /// CartVariant 󱗫
    CartVariant,
    /// CaseSensitive 
    CaseSensitive,
    /// CaseSensitiveAlt 󰄓
    CaseSensitiveAlt,
    /// Cash 󰄔
    Cash,
    /// CashCheck 󱓮
    CashCheck,
    /// CashClock 󱪑
    CashClock,
    /// CashFast 󱡜
    CashFast,
    /// CashLock 󱓪
    CashLock,
    /// CashLockOpen 󱓫
    CashLockOpen,
    /// CashMarker 󰶸
    CashMarker,
    /// CashMinus 󱉠
    CashMinus,
    /// CashMultiple 󰄖
    CashMultiple,
    /// CashOnezerozero 󰄕
    CashOnezerozero,
    /// CashPlus 󱉡
    CashPlus,
    /// CashRefund 󰪜
    CashRefund,
    /// CashRegister 󰳴
    CashRegister,
    /// CashRemove 󱉢
    CashRemove,
    /// CashSync 󱪒
    CashSync,
    /// Cassette 󰧔
    Cassette,
    /// Cast 󰄘
    Cast,
    /// CastAudio 󱀞
    CastAudio,
    /// CastAudioVariant 󱝉
    CastAudioVariant,
    /// CastConnected 󰄙
    CastConnected,
    /// CastEducation 󰸝
    CastEducation,
    /// CastOff 󰞊
    CastOff,
    /// CastVariant 󰀟
    CastVariant,
    /// Castle 󰄚
    Castle,
    /// Cat 󰄛
    Cat,
    /// CcBy 
    CcBy,
    /// CcCc 
    CcCc,
    /// CcNc 
    CcNc,
    /// CcNcEu 
    CcNcEu,
    /// CcNcJp 
    CcNcJp,
    /// CcNd 
    CcNd,
    /// CcRemix 
    CcRemix,
    /// CcSa 
    CcSa,
    /// CcShare 
    CcShare,
    /// CcZero 
    CcZero,
    /// Cctv 󰞮
    Cctv,
    /// CctvOff 󱡟
    CctvOff,
    /// CeilingFan 󱞗
    CeilingFan,
    /// CeilingFanLight 󱞘
    CeilingFanLight,
    /// CeilingLight 󰝩
    CeilingLight,
    /// CeilingLightMultiple 󱣝
    CeilingLightMultiple,
    /// CeilingLightMultipleOutline 󱣞
    CeilingLightMultipleOutline,
    /// CeilingLightOutline 󱟇
    CeilingLightOutline,
    /// Cellphone 󰄜
    Cellphone,
    /// CellphoneArrowDown 󰧕
    CellphoneArrowDown,
    /// CellphoneArrowDownVariant 󱧅
    CellphoneArrowDownVariant,
    /// CellphoneBasic 󰄞
    CellphoneBasic,
    /// CellphoneCharging 󱎗
    CellphoneCharging,
    /// CellphoneCheck 󱟽
    CellphoneCheck,
    /// CellphoneCog 󰥑
    CellphoneCog,
    /// CellphoneDock 󰄟
    CellphoneDock,
    /// CellphoneInformation 󰽁
    CellphoneInformation,
    /// CellphoneKey 󰥎
    CellphoneKey,
    /// CellphoneLink 󰄡
    CellphoneLink,
    /// CellphoneLinkOff 󰄢
    CellphoneLinkOff,
    /// CellphoneLock 󰥏
    CellphoneLock,
    /// CellphoneMarker 󱠺
    CellphoneMarker,
    /// CellphoneMessage 󰣓
    CellphoneMessage,
    /// CellphoneMessageOff 󱃒
    CellphoneMessageOff,
    /// CellphoneNfc 󰺐
    CellphoneNfc,
    /// CellphoneNfcOff 󱋘
    CellphoneNfcOff,
    /// CellphoneOff 󰥐
    CellphoneOff,
    /// CellphonePlay 󱀟
    CellphonePlay,
    /// CellphoneRemove 󰥍
    CellphoneRemove,
    /// CellphoneScreenshot 󰨵
    CellphoneScreenshot,
    /// CellphoneSettings 󰄣
    CellphoneSettings,
    /// CellphoneSound 󰥒
    CellphoneSound,
    /// CellphoneText 󰣒
    CellphoneText,
    /// CellphoneWireless 󰠕
    CellphoneWireless,
    /// Centos 󱄚
    Centos,
    /// Certificate 
    Certificate,
    /// CertificateOne 󰄤
    CertificateOne,
    /// CertificateOutline 󱆈
    CertificateOutline,
    /// ChairRolling 󰽈
    ChairRolling,
    /// ChairSchool 󰄥
    ChairSchool,
    /// Chandelier 󱞓
    Chandelier,
    /// Charity 󰱏
    Charity,
    /// ChartArc 󰄦
    ChartArc,
    /// ChartAreaspline 󰄧
    ChartAreaspline,
    /// ChartAreasplineVariant 󰺑
    ChartAreasplineVariant,
    /// ChartBar 󰄨
    ChartBar,
    /// ChartBarStacked 󰝪
    ChartBarStacked,
    /// ChartBellCurve 󰱐
    ChartBellCurve,
    /// ChartBellCurveCumulative 󰾧
    ChartBellCurveCumulative,
    /// ChartBox 󱕍
    ChartBox,
    /// ChartBoxOutline 󱕎
    ChartBoxOutline,
    /// ChartBoxPlusOutline 󱕏
    ChartBoxPlusOutline,
    /// ChartBubble 󰗣
    ChartBubble,
    /// ChartDonut 󰞯
    ChartDonut,
    /// ChartDonutVariant 󰞰
    ChartDonutVariant,
    /// ChartGantt 󰙬
    ChartGantt,
    /// ChartHistogram 󰄩
    ChartHistogram,
    /// ChartLine 󰄪
    ChartLine,
    /// ChartLineStacked 󰝫
    ChartLineStacked,
    /// ChartLineVariant 󰞱
    ChartLineVariant,
    /// ChartMultiline 󰣔
    ChartMultiline,
    /// ChartMultiple 󱈓
    ChartMultiple,
    /// ChartPie 󰄫
    ChartPie,
    /// ChartPpf 󱎀
    ChartPpf,
    /// ChartSankey 󱇟
    ChartSankey,
    /// ChartSankeyVariant 󱇠
    ChartSankeyVariant,
    /// ChartScatterPlot 󰺒
    ChartScatterPlot,
    /// ChartScatterPlotHexbin 󰙭
    ChartScatterPlotHexbin,
    /// ChartTimeline 󰙮
    ChartTimeline,
    /// ChartTimelineVariant 󰺓
    ChartTimelineVariant,
    /// ChartTimelineVariantShimmer 󱖶
    ChartTimelineVariantShimmer,
    /// ChartTree 󰺔
    ChartTree,
    /// ChartWaterfall 󱤘
    ChartWaterfall,
    /// Chat 󰭹
    Chat,
    /// ChatAlert 󰭺
    ChatAlert,
    /// ChatAlertOutline 󱋉
    ChatAlertOutline,
    /// ChatMinus 󱐐
    ChatMinus,
    /// ChatMinusOutline 󱐓
    ChatMinusOutline,
    /// ChatOutline 󰻞
    ChatOutline,
    /// ChatPlus 󱐏
    ChatPlus,
    /// ChatPlusOutline 󱐒
    ChatPlusOutline,
    /// ChatProcessing 󰭻
    ChatProcessing,
    /// ChatProcessingOutline 󱋊
    ChatProcessingOutline,
    /// ChatQuestion 󱜸
    ChatQuestion,
    /// ChatQuestionOutline 󱜹
    ChatQuestionOutline,
    /// ChatRemove 󱐑
    ChatRemove,
    /// ChatRemoveOutline 󱐔
    ChatRemoveOutline,
    /// ChatSleep 󱋑
    ChatSleep,
    /// ChatSleepOutline 󱋒
    ChatSleepOutline,
    /// Check 
    Check,
    /// CheckAll 
    CheckAll,
    /// CheckAllOne 󰄭
    CheckAllOne,
    /// CheckBold 󰸞
    CheckBold,
    /// CheckCircle 
    CheckCircle,
    /// CheckCircleFill 
    CheckCircleFill,
    /// CheckCircleOne 󰗠
    CheckCircleOne,
    /// CheckCircleOutline 󰗡
    CheckCircleOutline,
    /// CheckDecagram 󰞑
    CheckDecagram,
    /// CheckDecagramOutline 󱝀
    CheckDecagramOutline,
    /// CheckEmpty 
    CheckEmpty,
    /// CheckMinus 
    CheckMinus,
    /// CheckNetwork 󰱓
    CheckNetwork,
    /// CheckNetworkOutline 󰱔
    CheckNetworkOutline,
    /// CheckOne 
    CheckOne,
    /// CheckOutline 󰡕
    CheckOutline,
    /// CheckSign 
    CheckSign,
    /// CheckThree 󰄬
    CheckThree,
    /// CheckTwo 
    CheckTwo,
    /// CheckUnderline 󰸟
    CheckUnderline,
    /// CheckUnderlineCircle 󰸠
    CheckUnderlineCircle,
    /// CheckUnderlineCircleOutline 󰸡
    CheckUnderlineCircleOutline,
    /// Checkbook 󰪝
    Checkbook,
    /// Checkbox 
    Checkbox,
    /// CheckboxBlank 󰄮
    CheckboxBlank,
    /// CheckboxBlankBadge 󱅶
    CheckboxBlankBadge,
    /// CheckboxBlankBadgeOutline 󰄗
    CheckboxBlankBadgeOutline,
    /// CheckboxBlankCircle 󰄯
    CheckboxBlankCircle,
    /// CheckboxBlankCircleOne 󰝥
    CheckboxBlankCircleOne,
    /// CheckboxBlankCircleOutline 󰄰
    CheckboxBlankCircleOutline,
    /// CheckboxBlankCircleOutlineOne 󰐽
    CheckboxBlankCircleOutlineOne,
    /// CheckboxBlankCircleOutlineTwo 󰝦
    CheckboxBlankCircleOutlineTwo,
    /// CheckboxBlankOff 󱋬
    CheckboxBlankOff,
    /// CheckboxBlankOffOutline 󱋭
    CheckboxBlankOffOutline,
    /// CheckboxBlankOutline 󰄱
    CheckboxBlankOutline,
    /// CheckboxIntermediate 󰡖
    CheckboxIntermediate,
    /// CheckboxMarked 󰄲
    CheckboxMarked,
    /// CheckboxMarkedCircle 󰄳
    CheckboxMarkedCircle,
    /// CheckboxMarkedCircleOutline 󰄴
    CheckboxMarkedCircleOutline,
    /// CheckboxMarkedCirclePlusOutline 󱤧
    CheckboxMarkedCirclePlusOutline,
    /// CheckboxMarkedOutline 󰄵
    CheckboxMarkedOutline,
    /// CheckboxMultipleBlank 󰄶
    CheckboxMultipleBlank,
    /// CheckboxMultipleBlankCircle 󰘻
    CheckboxMultipleBlankCircle,
    /// CheckboxMultipleBlankCircleOutline 󰘼
    CheckboxMultipleBlankCircleOutline,
    /// CheckboxMultipleBlankOutline 󰄷
    CheckboxMultipleBlankOutline,
    /// CheckboxMultipleMarked 󰄸
    CheckboxMultipleMarked,
    /// CheckboxMultipleMarkedCircle 󰘽
    CheckboxMultipleMarkedCircle,
    /// CheckboxMultipleMarkedCircleOutline 󰘾
    CheckboxMultipleMarkedCircleOutline,
    /// CheckboxMultipleMarkedOutline 󰄹
    CheckboxMultipleMarkedOutline,
    /// CheckboxMultipleOutline 󰱑
    CheckboxMultipleOutline,
    /// CheckboxOutline 󰱒
    CheckboxOutline,
    /// Checkerboard 󰄺
    Checkerboard,
    /// CheckerboardMinus 󱈂
    CheckerboardMinus,
    /// CheckerboardPlus 󱈁
    CheckerboardPlus,
    /// CheckerboardRemove 󱈃
    CheckerboardRemove,
    /// Checklist 
    Checklist,
    /// ChecklistO 
    ChecklistO,
    /// ChecklistOne 
    ChecklistOne,
    /// Cheese 󱊹
    Cheese,
    /// CheeseOff 󱏮
    CheeseOff,
    /// ChefHat 󰭼
    ChefHat,
    /// ChemicalWeapon 󰄻
    ChemicalWeapon,
    /// Cherry 
    Cherry,
    /// ChessBishop 
    ChessBishop,
    /// ChessBishopOne 󰡜
    ChessBishopOne,
    /// ChessHorse 
    ChessHorse,
    /// ChessKing 
    ChessKing,
    /// ChessKingOne 󰡗
    ChessKingOne,
    /// ChessKnight 󰡘
    ChessKnight,
    /// ChessPawn 
    ChessPawn,
    /// ChessPawnOne 󰡙
    ChessPawnOne,
    /// ChessQueen 
    ChessQueen,
    /// ChessQueenOne 󰡚
    ChessQueenOne,
    /// ChessRook 󰡛
    ChessRook,
    /// ChessTower 
    ChessTower,
    /// Chesse 
    Chesse,
    /// ChevronDoubleDown 󰄼
    ChevronDoubleDown,
    /// ChevronDoubleLeft 󰄽
    ChevronDoubleLeft,
    /// ChevronDoubleRight 󰄾
    ChevronDoubleRight,
    /// ChevronDoubleUp 󰄿
    ChevronDoubleUp,
    /// ChevronDown 
    ChevronDown,
    /// ChevronDownBox 󰧖
    ChevronDownBox,
    /// ChevronDownBoxOutline 󰧗
    ChevronDownBoxOutline,
    /// ChevronDownCircle 󰬦
    ChevronDownCircle,
    /// ChevronDownCircleOutline 󰬧
    ChevronDownCircleOutline,
    /// ChevronDownOne 
    ChevronDownOne,
    /// ChevronDownTwo 󰅀
    ChevronDownTwo,
    /// ChevronLeft 
    ChevronLeft,
    /// ChevronLeftBox 󰧘
    ChevronLeftBox,
    /// ChevronLeftBoxOutline 󰧙
    ChevronLeftBoxOutline,
    /// ChevronLeftCircle 󰬨
    ChevronLeftCircle,
    /// ChevronLeftCircleOutline 󰬩
    ChevronLeftCircleOutline,
    /// ChevronLeftOne 
    ChevronLeftOne,
    /// ChevronLeftTwo 󰅁
    ChevronLeftTwo,
    /// ChevronRight 
    ChevronRight,
    /// ChevronRightBox 󰧚
    ChevronRightBox,
    /// ChevronRightBoxOutline 󰧛
    ChevronRightBoxOutline,
    /// ChevronRightCircle 󰬪
    ChevronRightCircle,
    /// ChevronRightCircleOutline 󰬫
    ChevronRightCircleOutline,
    /// ChevronRightOne 
    ChevronRightOne,
    /// ChevronRightTwo 󰅂
    ChevronRightTwo,
    /// ChevronSignDown 
    ChevronSignDown,
    /// ChevronSignLeft 
    ChevronSignLeft,
    /// ChevronSignRight 
    ChevronSignRight,
    /// ChevronSignUp 
    ChevronSignUp,
    /// ChevronTripleDown 󰶹
    ChevronTripleDown,
    /// ChevronTripleLeft 󰶺
    ChevronTripleLeft,
    /// ChevronTripleRight 󰶻
    ChevronTripleRight,
    /// ChevronTripleUp 󰶼
    ChevronTripleUp,
    /// ChevronUp 
    ChevronUp,
    /// ChevronUpBox 󰧜
    ChevronUpBox,
    /// ChevronUpBoxOutline 󰧝
    ChevronUpBoxOutline,
    /// ChevronUpCircle 󰬬
    ChevronUpCircle,
    /// ChevronUpCircleOutline 󰬭
    ChevronUpCircleOutline,
    /// ChevronUpOne 
    ChevronUpOne,
    /// ChevronUpTwo 󰅃
    ChevronUpTwo,
    /// ChickenThigh 
    ChickenThigh,
    /// ChiliAlert 󱟪
    ChiliAlert,
    /// ChiliAlertOutline 󱟫
    ChiliAlertOutline,
    /// ChiliHot 󰞲
    ChiliHot,
    /// ChiliHotOutline 󱟬
    ChiliHotOutline,
    /// ChiliMedium 󰞳
    ChiliMedium,
    /// ChiliMediumOutline 󱟭
    ChiliMediumOutline,
    /// ChiliMild 󰞴
    ChiliMild,
    /// ChiliMildOutline 󱟮
    ChiliMildOutline,
    /// ChiliOff 󱑧
    ChiliOff,
    /// ChiliOffOutline 󱟯
    ChiliOffOutline,
    /// Chilli 
    Chilli,
    /// Chip 
    Chip,
    /// ChipOne 󰘚
    ChipOne,
    /// ChromeClose 
    ChromeClose,
    /// ChromeMaximize 
    ChromeMaximize,
    /// ChromeMinimize 
    ChromeMinimize,
    /// ChromeRestore 
    ChromeRestore,
    /// Church 󰅄
    Church,
    /// Cicling 
    Cicling,
    /// Cigar 󱆉
    Cigar,
    /// CigarOff 󱐛
    CigarOff,
    /// Cinnamon 
    Cinnamon,
    /// Circle 
    Circle,
    /// CircleArrowDown 
    CircleArrowDown,
    /// CircleArrowLeft 
    CircleArrowLeft,
    /// CircleArrowRight 
    CircleArrowRight,
    /// CircleArrowUp 
    CircleArrowUp,
    /// CircleBlank 
    CircleBlank,
    /// CircleBox 󱗜
    CircleBox,
    /// CircleBoxOutline 󱗝
    CircleBoxOutline,
    /// CircleDouble 󰺕
    CircleDouble,
    /// CircleEditOutline 󰣕
    CircleEditOutline,
    /// CircleExpand 󰺖
    CircleExpand,
    /// CircleFilled 
    CircleFilled,
    /// CircleHalf 󱎕
    CircleHalf,
    /// CircleHalfFull 󱎖
    CircleHalfFull,
    /// CircleLargeFilled 
    CircleLargeFilled,
    /// CircleLargeOutline 
    CircleLargeOutline,
    /// CircleMedium 󰧞
    CircleMedium,
    /// CircleMultiple 󰬸
    CircleMultiple,
    /// CircleMultipleOutline 󰚕
    CircleMultipleOutline,
    /// CircleOffOutline 󱃓
    CircleOffOutline,
    /// CircleOne 
    CircleOne,
    /// CircleOpacity 󱡓
    CircleOpacity,
    /// CircleOutline 
    CircleOutline,
    /// CircleSlash 
    CircleSlash,
    /// CircleSlashOne 
    CircleSlashOne,
    /// CircleSliceEight 󰪥
    CircleSliceEight,
    /// CircleSliceFive 󰪢
    CircleSliceFive,
    /// CircleSliceFour 󰪡
    CircleSliceFour,
    /// CircleSliceOne 󰪞
    CircleSliceOne,
    /// CircleSliceSeven 󰪤
    CircleSliceSeven,
    /// CircleSliceSix 󰪣
    CircleSliceSix,
    /// CircleSliceThree 󰪠
    CircleSliceThree,
    /// CircleSliceTwo 󰪟
    CircleSliceTwo,
    /// CircleSmall 󰧟
    CircleSmall,
    /// CircuitBoard 
    CircuitBoard,
    /// CircularSaw 󰸢
    CircularSaw,
    /// City 󰅆
    City,
    /// CityVariant 󰨶
    CityVariant,
    /// CityVariantOutline 󰨷
    CityVariantOutline,
    /// CleanCode 
    CleanCode,
    /// ClearAll 
    ClearAll,
    /// Clipboard 󰅇
    Clipboard,
    /// ClipboardAccount 󰅈
    ClipboardAccount,
    /// ClipboardAccountOutline 󰱕
    ClipboardAccountOutline,
    /// ClipboardAlert 󰅉
    ClipboardAlert,
    /// ClipboardAlertOutline 󰳷
    ClipboardAlertOutline,
    /// ClipboardArrowDown 󰅊
    ClipboardArrowDown,
    /// ClipboardArrowDownOutline 󰱖
    ClipboardArrowDownOutline,
    /// ClipboardArrowLeft 󰅋
    ClipboardArrowLeft,
    /// ClipboardArrowLeftOutline 󰳸
    ClipboardArrowLeftOutline,
    /// ClipboardArrowRight 󰳹
    ClipboardArrowRight,
    /// ClipboardArrowRightOutline 󰳺
    ClipboardArrowRightOutline,
    /// ClipboardArrowUp 󰱗
    ClipboardArrowUp,
    /// ClipboardArrowUpOutline 󰱘
    ClipboardArrowUpOutline,
    /// ClipboardCheck 󰅎
    ClipboardCheck,
    /// ClipboardCheckMultiple 󱉣
    ClipboardCheckMultiple,
    /// ClipboardCheckMultipleOutline 󱉤
    ClipboardCheckMultipleOutline,
    /// ClipboardCheckOutline 󰢨
    ClipboardCheckOutline,
    /// ClipboardClock 󱛢
    ClipboardClock,
    /// ClipboardClockOutline 󱛣
    ClipboardClockOutline,
    /// ClipboardEdit 󱓥
    ClipboardEdit,
    /// ClipboardEditOutline 󱓦
    ClipboardEditOutline,
    /// ClipboardFile 󱉥
    ClipboardFile,
    /// ClipboardFileOutline 󱉦
    ClipboardFileOutline,
    /// ClipboardFlow 󰛈
    ClipboardFlow,
    /// ClipboardFlowOutline 󱄗
    ClipboardFlowOutline,
    /// ClipboardList 󱃔
    ClipboardList,
    /// ClipboardListOutline 󱃕
    ClipboardListOutline,
    /// ClipboardMinus 󱘘
    ClipboardMinus,
    /// ClipboardMinusOutline 󱘙
    ClipboardMinusOutline,
    /// ClipboardMultiple 󱉧
    ClipboardMultiple,
    /// ClipboardMultipleOutline 󱉨
    ClipboardMultipleOutline,
    /// ClipboardOff 󱘚
    ClipboardOff,
    /// ClipboardOffOutline 󱘛
    ClipboardOffOutline,
    /// ClipboardOutline 󰅌
    ClipboardOutline,
    /// ClipboardPlay 󰱙
    ClipboardPlay,
    /// ClipboardPlayMultiple 󱉩
    ClipboardPlayMultiple,
    /// ClipboardPlayMultipleOutline 󱉪
    ClipboardPlayMultipleOutline,
    /// ClipboardPlayOutline 󰱚
    ClipboardPlayOutline,
    /// ClipboardPlus 󰝑
    ClipboardPlus,
    /// ClipboardPlusOutline 󱌟
    ClipboardPlusOutline,
    /// ClipboardPulse 󰡝
    ClipboardPulse,
    /// ClipboardPulseOutline 󰡞
    ClipboardPulseOutline,
    /// ClipboardRemove 󱘜
    ClipboardRemove,
    /// ClipboardRemoveOutline 󱘝
    ClipboardRemoveOutline,
    /// ClipboardSearch 󱘞
    ClipboardSearch,
    /// ClipboardSearchOutline 󱘟
    ClipboardSearchOutline,
    /// ClipboardText 󰅍
    ClipboardText,
    /// ClipboardTextClock 󱣹
    ClipboardTextClock,
    /// ClipboardTextClockOutline 󱣺
    ClipboardTextClockOutline,
    /// ClipboardTextMultiple 󱉫
    ClipboardTextMultiple,
    /// ClipboardTextMultipleOutline 󱉬
    ClipboardTextMultipleOutline,
    /// ClipboardTextOff 󱘠
    ClipboardTextOff,
    /// ClipboardTextOffOutline 󱘡
    ClipboardTextOffOutline,
    /// ClipboardTextOutline 󰨸
    ClipboardTextOutline,
    /// ClipboardTextPlay 󰱛
    ClipboardTextPlay,
    /// ClipboardTextPlayOutline 󰱜
    ClipboardTextPlayOutline,
    /// ClipboardTextSearch 󱘢
    ClipboardTextSearch,
    /// ClipboardTextSearchOutline 󱘣
    ClipboardTextSearchOutline,
    /// Clippy 
    Clippy,
    /// ClippyOne 󰅏
    ClippyOne,
    /// Clock 
    Clock,
    /// ClockAlert 󰥕
    ClockAlert,
    /// ClockAlertOutline 󰗎
    ClockAlertOutline,
    /// ClockCheck 󰾨
    ClockCheck,
    /// ClockCheckOutline 󰾩
    ClockCheckOutline,
    /// ClockDigital 󰺗
    ClockDigital,
    /// ClockEdit 󱦺
    ClockEdit,
    /// ClockEditOutline 󱦻
    ClockEditOutline,
    /// ClockEnd 󰅑
    ClockEnd,
    /// ClockFast 󰅒
    ClockFast,
    /// ClockFill 
    ClockFill,
    /// ClockIn 󰅓
    ClockIn,
    /// ClockMinus 󱡣
    ClockMinus,
    /// ClockMinusOutline 󱡤
    ClockMinusOutline,
    /// ClockOne 󰥔
    ClockOne,
    /// ClockOut 󰅔
    ClockOut,
    /// ClockOutline 󰅐
    ClockOutline,
    /// ClockPlus 󱡡
    ClockPlus,
    /// ClockPlusOutline 󱡢
    ClockPlusOutline,
    /// ClockRemove 󱡥
    ClockRemove,
    /// ClockRemoveOutline 󱡦
    ClockRemoveOutline,
    /// ClockStart 󰅕
    ClockStart,
    /// ClockTimeEight 󱑆
    ClockTimeEight,
    /// ClockTimeEightOutline 󱑒
    ClockTimeEightOutline,
    /// ClockTimeEleven 󱑉
    ClockTimeEleven,
    /// ClockTimeElevenOutline 󱑕
    ClockTimeElevenOutline,
    /// ClockTimeFive 󱑃
    ClockTimeFive,
    /// ClockTimeFiveOutline 󱑏
    ClockTimeFiveOutline,
    /// ClockTimeFour 󱑂
    ClockTimeFour,
    /// ClockTimeFourOutline 󱑎
    ClockTimeFourOutline,
    /// ClockTimeNine 󱑇
    ClockTimeNine,
    /// ClockTimeNineOutline 󱑓
    ClockTimeNineOutline,
    /// ClockTimeOne 󱐿
    ClockTimeOne,
    /// ClockTimeOneOutline 󱑋
    ClockTimeOneOutline,
    /// ClockTimeSeven 󱑅
    ClockTimeSeven,
    /// ClockTimeSevenOutline 󱑑
    ClockTimeSevenOutline,
    /// ClockTimeSix 󱑄
    ClockTimeSix,
    /// ClockTimeSixOutline 󱑐
    ClockTimeSixOutline,
    /// ClockTimeTen 󱑈
    ClockTimeTen,
    /// ClockTimeTenOutline 󱑔
    ClockTimeTenOutline,
    /// ClockTimeThree 󱑁
    ClockTimeThree,
    /// ClockTimeThreeOutline 󱑍
    ClockTimeThreeOutline,
    /// ClockTimeTwelve 󱑊
    ClockTimeTwelve,
    /// ClockTimeTwelveOutline 󱑖
    ClockTimeTwelveOutline,
    /// ClockTimeTwo 󱑀
    ClockTimeTwo,
    /// ClockTimeTwoOutline 󱑌
    ClockTimeTwoOutline,
    /// Close 
    Close,
    /// CloseAll 
    CloseAll,
    /// CloseBox 󰅗
    CloseBox,
    /// CloseBoxMultiple 󰱝
    CloseBoxMultiple,
    /// CloseBoxMultipleOutline 󰱞
    CloseBoxMultipleOutline,
    /// CloseBoxOutline 󰅘
    CloseBoxOutline,
    /// CloseCircle 󰅙
    CloseCircle,
    /// CloseCircleMultiple 󰘪
    CloseCircleMultiple,
    /// CloseCircleMultipleOutline 󰢃
    CloseCircleMultipleOutline,
    /// CloseCircleOutline 󰅚
    CloseCircleOutline,
    /// CloseNetwork 󰅛
    CloseNetwork,
    /// CloseNetworkOutline 󰱟
    CloseNetworkOutline,
    /// CloseOctagon 󰅜
    CloseOctagon,
    /// CloseOctagonOutline 󰅝
    CloseOctagonOutline,
    /// CloseOne 󰅖
    CloseOne,
    /// CloseOutline 󰛉
    CloseOutline,
    /// CloseThick 󱎘
    CloseThick,
    /// ClosedCaption 󰅞
    ClosedCaption,
    /// ClosedCaptionOutline 󰶽
    ClosedCaptionOutline,
    /// Cloud 
    Cloud,
    /// CloudAlert 󰧠
    CloudAlert,
    /// CloudBraces 󰞵
    CloudBraces,
    /// CloudCheck 󰅠
    CloudCheck,
    /// CloudCheckOutline 󱋌
    CloudCheckOutline,
    /// CloudCircle 󰅡
    CloudCircle,
    /// CloudDownload 
    CloudDownload,
    /// CloudDownloadOne 󰅢
    CloudDownloadOne,
    /// CloudDownloadOutline 󰭽
    CloudDownloadOutline,
    /// CloudFour 󰅟
    CloudFour,
    /// CloudLock 󱇱
    CloudLock,
    /// CloudLockOutline 󱇲
    CloudLockOutline,
    /// CloudOffOutline 󰅤
    CloudOffOutline,
    /// CloudOffline 
    CloudOffline,
    /// CloudOne 
    CloudOne,
    /// CloudOutline 󰅣
    CloudOutline,
    /// CloudPercent 󱨵
    CloudPercent,
    /// CloudPercentOutline 󱨶
    CloudPercentOutline,
    /// CloudPrint 󰅥
    CloudPrint,
    /// CloudPrintOutline 󰅦
    CloudPrintOutline,
    /// CloudQuestion 󰨹
    CloudQuestion,
    /// CloudRefresh 󰔪
    CloudRefresh,
    /// CloudSearch 󰥖
    CloudSearch,
    /// CloudSearchOutline 󰥗
    CloudSearchOutline,
    /// CloudSync 󰘿
    CloudSync,
    /// CloudSyncOutline 󱋖
    CloudSyncOutline,
    /// CloudTags 󰞶
    CloudTags,
    /// CloudThree 
    CloudThree,
    /// CloudTwo 
    CloudTwo,
    /// CloudUpload 
    CloudUpload,
    /// CloudUploadOne 󰅧
    CloudUploadOne,
    /// CloudUploadOutline 󰭾
    CloudUploadOutline,
    /// Clover 󰠖
    Clover,
    /// CoachLamp 󱀠
    CoachLamp,
    /// CoachLampVariant 󱨷
    CoachLampVariant,
    /// CoatRack 󱂞
    CoatRack,
    /// Cockroach 
    Cockroach,
    /// Code 
    Code,
    /// CodeArray 󰅨
    CodeArray,
    /// CodeBraces 󰅩
    CodeBraces,
    /// CodeBracesBox 󱃖
    CodeBracesBox,
    /// CodeBrackets 󰅪
    CodeBrackets,
    /// CodeEqual 󰅫
    CodeEqual,
    /// CodeFork 
    CodeFork,
    /// CodeGreaterThan 󰅬
    CodeGreaterThan,
    /// CodeGreaterThanOrEqual 󰅭
    CodeGreaterThanOrEqual,
    /// CodeJson 󰘦
    CodeJson,
    /// CodeLessThan 󰅮
    CodeLessThan,
    /// CodeLessThanOrEqual 󰅯
    CodeLessThanOrEqual,
    /// CodeNotEqual 󰅰
    CodeNotEqual,
    /// CodeNotEqualVariant 󰅱
    CodeNotEqualVariant,
    /// CodeOfConduct 
    CodeOfConduct,
    /// CodeOne 
    CodeOne,
    /// CodeParentheses 󰅲
    CodeParentheses,
    /// CodeParenthesesBox 󱃗
    CodeParenthesesBox,
    /// CodeReview 
    CodeReview,
    /// CodeSquare 
    CodeSquare,
    /// CodeString 󰅳
    CodeString,
    /// CodeTags 󰅴
    CodeTags,
    /// CodeTagsCheck 󰚔
    CodeTagsCheck,
    /// CodeTwo 
    CodeTwo,
    /// Codeberg 
    Codeberg,
    /// Codepen 󰅵
    Codepen,
    /// Codescan 
    Codescan,
    /// CodescanCheckmark 
    CodescanCheckmark,
    /// Codespaces 
    Codespaces,
    /// CoffeBeans 
    CoffeBeans,
    /// Coffee 
    Coffee,
    /// CoffeeMaker 󱂟
    CoffeeMaker,
    /// CoffeeMakerCheck 󱤱
    CoffeeMakerCheck,
    /// CoffeeMakerCheckOutline 󱤲
    CoffeeMakerCheckOutline,
    /// CoffeeMakerOutline 󱠛
    CoffeeMakerOutline,
    /// CoffeeOff 󰾪
    CoffeeOff,
    /// CoffeeOffOutline 󰾫
    CoffeeOffOutline,
    /// CoffeeOne 󰅶
    CoffeeOne,
    /// CoffeeOutline 󰛊
    CoffeeOutline,
    /// CoffeeToGo 󰅷
    CoffeeToGo,
    /// CoffeeToGoOutline 󱌎
    CoffeeToGoOutline,
    /// Coffin 󰭿
    Coffin,
    /// Cog 
    Cog,
    /// CogBox 󰒔
    CogBox,
    /// CogClockwise 󱇝
    CogClockwise,
    /// CogCounterclockwise 󱇞
    CogCounterclockwise,
    /// CogOff 󱏎
    CogOff,
    /// CogOffOutline 󱏏
    CogOffOutline,
    /// CogOne 󰒓
    CogOne,
    /// CogOutline 󰢻
    CogOutline,
    /// CogPause 󱤳
    CogPause,
    /// CogPauseOutline 󱤴
    CogPauseOutline,
    /// CogPlay 󱤵
    CogPlay,
    /// CogPlayOutline 󱤶
    CogPlayOutline,
    /// CogRefresh 󱑞
    CogRefresh,
    /// CogRefreshOutline 󱑟
    CogRefreshOutline,
    /// CogStop 󱤷
    CogStop,
    /// CogStopOutline 󱤸
    CogStopOutline,
    /// CogSync 󱑠
    CogSync,
    /// CogSyncOutline 󱑡
    CogSyncOutline,
    /// CogTransfer 󱁛
    CogTransfer,
    /// CogTransferOutline 󱁜
    CogTransferOutline,
    /// Cogs 
    Cogs,
    /// CogsOne 󰣖
    CogsOne,
    /// Coins 
    Coins,
    /// Collage 󰙀
    Collage,
    /// Collapse 
    Collapse,
    /// CollapseAll 
    CollapseAll,
    /// CollapseAllOne 󰪦
    CollapseAllOne,
    /// CollapseAllOutline 󰪧
    CollapseAllOutline,
    /// CollapseAlt 
    CollapseAlt,
    /// CollapseTop 
    CollapseTop,
    /// ColorHelper 󰅹
    ColorHelper,
    /// ColorMode 
    ColorMode,
    /// Columns 
    Columns,
    /// ColumnsOne 
    ColumnsOne,
    /// Comb 
    Comb,
    /// Combine 
    Combine,
    /// Comet 
    Comet,
    /// Comma 󰸣
    Comma,
    /// CommaBox 󰸫
    CommaBox,
    /// CommaBoxOutline 󰸤
    CommaBoxOutline,
    /// CommaCircle 󰸥
    CommaCircle,
    /// CommaCircleOutline 󰸦
    CommaCircleOutline,
    /// CommandPalette 
    CommandPalette,
    /// Comment 
    Comment,
    /// CommentAccount 󰅻
    CommentAccount,
    /// CommentAccountOutline 󰅼
    CommentAccountOutline,
    /// CommentAlert 󰅽
    CommentAlert,
    /// CommentAlertOutline 󰅾
    CommentAlertOutline,
    /// CommentAlt 
    CommentAlt,
    /// CommentArrowLeft 󰧡
    CommentArrowLeft,
    /// CommentArrowLeftOutline 󰧢
    CommentArrowLeftOutline,
    /// CommentArrowRight 󰧣
    CommentArrowRight,
    /// CommentArrowRightOutline 󰧤
    CommentArrowRightOutline,
    /// CommentBookmark 󱖮
    CommentBookmark,
    /// CommentBookmarkOutline 󱖯
    CommentBookmarkOutline,
    /// CommentCheck 󰅿
    CommentCheck,
    /// CommentCheckOutline 󰆀
    CommentCheckOutline,
    /// CommentDiscussion 
    CommentDiscussion,
    /// CommentDiscussionOne 
    CommentDiscussionOne,
    /// CommentEdit 󱆿
    CommentEdit,
    /// CommentEditOutline 󱋄
    CommentEditOutline,
    /// CommentEye 󰨺
    CommentEye,
    /// CommentEyeOutline 󰨻
    CommentEyeOutline,
    /// CommentFlash 󱖰
    CommentFlash,
    /// CommentFlashOutline 󱖱
    CommentFlashOutline,
    /// CommentMinus 󱗟
    CommentMinus,
    /// CommentMinusOutline 󱗠
    CommentMinusOutline,
    /// CommentMultiple 󰡟
    CommentMultiple,
    /// CommentMultipleOutline 󰆁
    CommentMultipleOutline,
    /// CommentOff 󱗡
    CommentOff,
    /// CommentOffOutline 󱗢
    CommentOffOutline,
    /// CommentOne 
    CommentOne,
    /// CommentOutline 󰆂
    CommentOutline,
    /// CommentPlus 󰧥
    CommentPlus,
    /// CommentPlusOutline 󰆃
    CommentPlusOutline,
    /// CommentProcessing 󰆄
    CommentProcessing,
    /// CommentProcessingOutline 󰆅
    CommentProcessingOutline,
    /// CommentQuestion 󰠗
    CommentQuestion,
    /// CommentQuestionOutline 󰆆
    CommentQuestionOutline,
    /// CommentQuote 󱀡
    CommentQuote,
    /// CommentQuoteOutline 󱀢
    CommentQuoteOutline,
    /// CommentRemove 󰗞
    CommentRemove,
    /// CommentRemoveOutline 󰆇
    CommentRemoveOutline,
    /// CommentSearch 󰨼
    CommentSearch,
    /// CommentSearchOutline 󰨽
    CommentSearchOutline,
    /// CommentText 󰆈
    CommentText,
    /// CommentTextMultiple 󰡠
    CommentTextMultiple,
    /// CommentTextMultipleOutline 󰡡
    CommentTextMultipleOutline,
    /// CommentTextOutline 󰆉
    CommentTextOutline,
    /// CommentThree 󰅺
    CommentThree,
    /// CommentTwo 
    CommentTwo,
    /// Comments 
    Comments,
    /// CommentsAlt 
    CommentsAlt,
    /// Commit 
    Commit,
    /// Compare 󰆊
    Compare,
    /// CompareHorizontal 󱒒
    CompareHorizontal,
    /// CompareRemove 󱢳
    CompareRemove,
    /// CompareVertical 󱒓
    CompareVertical,
    /// Compass 
    Compass,
    /// CompassActive 
    CompassActive,
    /// CompassDot 
    CompassDot,
    /// CompassOff 󰮀
    CompassOff,
    /// CompassOffOutline 󰮁
    CompassOffOutline,
    /// CompassOne 
    CompassOne,
    /// CompassOutline 󰆌
    CompassOutline,
    /// CompassRose 󱎂
    CompassRose,
    /// CompassTwo 󰆋
    CompassTwo,
    /// Compost 󱨸
    Compost,
    /// Cone 󱥌
    Cone,
    /// ConeOff 󱥍
    ConeOff,
    /// Connection 󱘖
    Connection,
    /// Console 󰆍
    Console,
    /// ConsoleLine 󰞷
    ConsoleLine,
    /// ConsoleNetwork 󰢩
    ConsoleNetwork,
    /// ConsoleNetworkOutline 󰱠
    ConsoleNetworkOutline,
    /// Consolidate 󱃘
    Consolidate,
    /// ContactlessPayment 󰵪
    ContactlessPayment,
    /// ContactlessPaymentCircle 󰌡
    ContactlessPaymentCircle,
    /// ContactlessPaymentCircleOutline 󰐈
    ContactlessPaymentCircleOutline,
    /// Contacts 󰛋
    Contacts,
    /// ContactsOutline 󰖸
    ContactsOutline,
    /// Contain 󰨾
    Contain,
    /// ContainEnd 󰨿
    ContainEnd,
    /// ContainStart 󰩀
    ContainStart,
    /// Container 
    Container,
    /// ContentCopy 󰆏
    ContentCopy,
    /// ContentCut 󰆐
    ContentCut,
    /// ContentDuplicate 󰆑
    ContentDuplicate,
    /// ContentPaste 󰆒
    ContentPaste,
    /// ContentSave 󰆓
    ContentSave,
    /// ContentSaveAlert 󰽂
    ContentSaveAlert,
    /// ContentSaveAlertOutline 󰽃
    ContentSaveAlertOutline,
    /// ContentSaveAll 󰆔
    ContentSaveAll,
    /// ContentSaveAllOutline 󰽄
    ContentSaveAllOutline,
    /// ContentSaveCheck 󱣪
    ContentSaveCheck,
    /// ContentSaveCheckOutline 󱣫
    ContentSaveCheckOutline,
    /// ContentSaveCog 󱑛
    ContentSaveCog,
    /// ContentSaveCogOutline 󱑜
    ContentSaveCogOutline,
    /// ContentSaveEdit 󰳻
    ContentSaveEdit,
    /// ContentSaveEditOutline 󰳼
    ContentSaveEditOutline,
    /// ContentSaveMove 󰸧
    ContentSaveMove,
    /// ContentSaveMoveOutline 󰸨
    ContentSaveMoveOutline,
    /// ContentSaveOff 󱙃
    ContentSaveOff,
    /// ContentSaveOffOutline 󱙄
    ContentSaveOffOutline,
    /// ContentSaveOutline 󰠘
    ContentSaveOutline,
    /// ContentSaveSettings 󰘛
    ContentSaveSettings,
    /// ContentSaveSettingsOutline 󰬮
    ContentSaveSettingsOutline,
    /// Contrast 󰆕
    Contrast,
    /// ContrastBox 󰆖
    ContrastBox,
    /// ContrastCircle 󰆗
    ContrastCircle,
    /// ControllerClassic 󰮂
    ControllerClassic,
    /// ControllerClassicOutline 󰮃
    ControllerClassicOutline,
    /// Cookie 󰆘
    Cookie,
    /// CookieAlert 󱛐
    CookieAlert,
    /// CookieAlertOutline 󱛑
    CookieAlertOutline,
    /// CookieCheck 󱛒
    CookieCheck,
    /// CookieCheckOutline 󱛓
    CookieCheckOutline,
    /// CookieClock 󱛤
    CookieClock,
    /// CookieClockOutline 󱛥
    CookieClockOutline,
    /// CookieCog 󱛔
    CookieCog,
    /// CookieCogOutline 󱛕
    CookieCogOutline,
    /// CookieEdit 󱛦
    CookieEdit,
    /// CookieEditOutline 󱛧
    CookieEditOutline,
    /// CookieLock 󱛨
    CookieLock,
    /// CookieLockOutline 󱛩
    CookieLockOutline,
    /// CookieMinus 󱛚
    CookieMinus,
    /// CookieMinusOutline 󱛛
    CookieMinusOutline,
    /// CookieOff 󱛪
    CookieOff,
    /// CookieOffOutline 󱛫
    CookieOffOutline,
    /// CookieOutline 󱛞
    CookieOutline,
    /// CookiePlus 󱛖
    CookiePlus,
    /// CookiePlusOutline 󱛗
    CookiePlusOutline,
    /// CookieRefresh 󱛬
    CookieRefresh,
    /// CookieRefreshOutline 󱛭
    CookieRefreshOutline,
    /// CookieRemove 󱛘
    CookieRemove,
    /// CookieRemoveOutline 󱛙
    CookieRemoveOutline,
    /// CookieSettings 󱛜
    CookieSettings,
    /// CookieSettingsOutline 󱛝
    CookieSettingsOutline,
    /// CoolantTemperature 󰏈
    CoolantTemperature,
    /// Copilot 
    Copilot,
    /// CopilotError 
    CopilotError,
    /// CopilotWarning 
    CopilotWarning,
    /// Copy 
    Copy,
    /// CopyOne 
    CopyOne,
    /// CopyTwo 
    CopyTwo,
    /// Copyleft 󱤹
    Copyleft,
    /// Copyright 󰗦
    Copyright,
    /// Cordova 󰥘
    Cordova,
    /// Coreos 
    Coreos,
    /// Corn 󰞸
    Corn,
    /// CornOff 󱏯
    CornOff,
    /// CosineWave 󱑹
    CosineWave,
    /// Counter 󰆙
    Counter,
    /// Countertop 󱠜
    Countertop,
    /// CountertopOutline 󱠝
    CountertopOutline,
    /// Cow 󰆚
    Cow,
    /// CowOff 󱣼
    CowOff,
    /// Cpu 
    Cpu,
    /// CpuSixfourBit 󰻠
    CpuSixfourBit,
    /// CpuThreetwoBit 󰻟
    CpuThreetwoBit,
    /// Cradle 󱦋
    Cradle,
    /// CradleOutline 󱦑
    CradleOutline,
    /// Crane 󰡢
    Crane,
    /// Creation 󰙴
    Creation,
    /// CreativeCommons 󰵫
    CreativeCommons,
    /// CreditCard 
    CreditCard,
    /// CreditCardCheck 󱏐
    CreditCardCheck,
    /// CreditCardCheckOutline 󱏑
    CreditCardCheckOutline,
    /// CreditCardChip 󱤏
    CreditCardChip,
    /// CreditCardChipOutline 󱤐
    CreditCardChipOutline,
    /// CreditCardClock 󰻡
    CreditCardClock,
    /// CreditCardClockOutline 󰻢
    CreditCardClockOutline,
    /// CreditCardEdit 󱟗
    CreditCardEdit,
    /// CreditCardEditOutline 󱟘
    CreditCardEditOutline,
    /// CreditCardFast 󱤑
    CreditCardFast,
    /// CreditCardFastOutline 󱤒
    CreditCardFastOutline,
    /// CreditCardLock 󱣧
    CreditCardLock,
    /// CreditCardLockOutline 󱣨
    CreditCardLockOutline,
    /// CreditCardMarker 󰚨
    CreditCardMarker,
    /// CreditCardMarkerOutline 󰶾
    CreditCardMarkerOutline,
    /// CreditCardMinus 󰾬
    CreditCardMinus,
    /// CreditCardMinusOutline 󰾭
    CreditCardMinusOutline,
    /// CreditCardMultiple 󰿰
    CreditCardMultiple,
    /// CreditCardMultipleOutline 󰆜
    CreditCardMultipleOutline,
    /// CreditCardOff 󰿱
    CreditCardOff,
    /// CreditCardOffOutline 󰗤
    CreditCardOffOutline,
    /// CreditCardOne 
    CreditCardOne,
    /// CreditCardOutline 󰆛
    CreditCardOutline,
    /// CreditCardPlus 󰿲
    CreditCardPlus,
    /// CreditCardPlusOutline 󰙶
    CreditCardPlusOutline,
    /// CreditCardRefresh 󱙅
    CreditCardRefresh,
    /// CreditCardRefreshOutline 󱙆
    CreditCardRefreshOutline,
    /// CreditCardRefund 󰿳
    CreditCardRefund,
    /// CreditCardRefundOutline 󰪨
    CreditCardRefundOutline,
    /// CreditCardRemove 󰾮
    CreditCardRemove,
    /// CreditCardRemoveOutline 󰾯
    CreditCardRemoveOutline,
    /// CreditCardScan 󰿴
    CreditCardScan,
    /// CreditCardScanOutline 󰆝
    CreditCardScanOutline,
    /// CreditCardSearch 󱙇
    CreditCardSearch,
    /// CreditCardSearchOutline 󱙈
    CreditCardSearchOutline,
    /// CreditCardSettings 󰿵
    CreditCardSettings,
    /// CreditCardSettingsOutline 󰣗
    CreditCardSettingsOutline,
    /// CreditCardSync 󱙉
    CreditCardSync,
    /// CreditCardSyncOutline 󱙊
    CreditCardSyncOutline,
    /// CreditCardTwo 󰿯
    CreditCardTwo,
    /// CreditCardWireless 󰠂
    CreditCardWireless,
    /// CreditCardWirelessOff 󰕺
    CreditCardWirelessOff,
    /// CreditCardWirelessOffOutline 󰕻
    CreditCardWirelessOffOutline,
    /// CreditCardWirelessOutline 󰵬
    CreditCardWirelessOutline,
    /// Cricket 󰵭
    Cricket,
    /// Crop 
    Crop,
    /// CropFree 󰆟
    CropFree,
    /// CropLandscape 󰆠
    CropLandscape,
    /// CropOne 󰆞
    CropOne,
    /// CropPortrait 󰆡
    CropPortrait,
    /// CropRotate 󰚖
    CropRotate,
    /// CropSquare 󰆢
    CropSquare,
    /// Cross 󰥓
    Cross,
    /// CrossBolnisi 󰳭
    CrossBolnisi,
    /// CrossCeltic 󰳵
    CrossCeltic,
    /// CrossOutline 󰳶
    CrossOutline,
    /// CrossReference 
    CrossReference,
    /// Crosshairs 󰆣
    Crosshairs,
    /// CrosshairsGps 󰆤
    CrosshairsGps,
    /// CrosshairsOff 󰽅
    CrosshairsOff,
    /// CrosshairsQuestion 󱄶
    CrosshairsQuestion,
    /// Crowd 󱥵
    Crowd,
    /// Crown 
    Crown,
    /// CrownCircle 󱟜
    CrownCircle,
    /// CrownCircleOutline 󱟝
    CrownCircleOutline,
    /// CrownOne 󰆥
    CrownOne,
    /// CrownOutline 󱇐
    CrownOutline,
    /// Cryengine 󰥙
    Cryengine,
    /// CrystalBall 󰬯
    CrystalBall,
    /// CrystalLinux 
    CrystalLinux,
    /// Cssthree 
    Cssthree,
    /// Cube 󰆦
    Cube,
    /// CubeOff 󱐜
    CubeOff,
    /// CubeOffOutline 󱐝
    CubeOffOutline,
    /// CubeOutline 󰆧
    CubeOutline,
    /// CubeScan 󰮄
    CubeScan,
    /// CubeSend 󰆨
    CubeSend,
    /// CubeUnfolded 󰆩
    CubeUnfolded,
    /// Cup 󰆪
    Cup,
    /// CupCoffe 
    CupCoffe,
    /// CupOff 󰗥
    CupOff,
    /// CupOffOutline 󱍽
    CupOffOutline,
    /// CupOutline 󱌏
    CupOutline,
    /// CupWater 󰆫
    CupWater,
    /// Cupboard 󰽆
    Cupboard,
    /// CupboardOutline 󰽇
    CupboardOutline,
    /// Cupcake 󰥚
    Cupcake,
    /// Curling 󰡣
    Curling,
    /// CurrencyBdt 󰡤
    CurrencyBdt,
    /// CurrencyBrl 󰮅
    CurrencyBrl,
    /// CurrencyBtc 󰆬
    CurrencyBtc,
    /// CurrencyCny 󰞺
    CurrencyCny,
    /// CurrencyEth 󰞻
    CurrencyEth,
    /// CurrencyEur 󰆭
    CurrencyEur,
    /// CurrencyEurOff 󱌕
    CurrencyEurOff,
    /// CurrencyFra 󱨹
    CurrencyFra,
    /// CurrencyGbp 󰆮
    CurrencyGbp,
    /// CurrencyIls 󰱡
    CurrencyIls,
    /// CurrencyInr 󰆯
    CurrencyInr,
    /// CurrencyJpy 󰞼
    CurrencyJpy,
    /// CurrencyKrw 󰞽
    CurrencyKrw,
    /// CurrencyKzt 󰡥
    CurrencyKzt,
    /// CurrencyMnt 󱔒
    CurrencyMnt,
    /// CurrencyNgn 󰆰
    CurrencyNgn,
    /// CurrencyPhp 󰧦
    CurrencyPhp,
    /// CurrencyRial 󰺜
    CurrencyRial,
    /// CurrencyRub 󰆱
    CurrencyRub,
    /// CurrencyRupee 󱥶
    CurrencyRupee,
    /// CurrencySign 󰞾
    CurrencySign,
    /// CurrencyTry 󰆲
    CurrencyTry,
    /// CurrencyTwd 󰞿
    CurrencyTwd,
    /// CurrencyUsd 󰇁
    CurrencyUsd,
    /// CurrencyUsdOff 󰙺
    CurrencyUsdOff,
    /// CurrentAc 󱒀
    CurrentAc,
    /// CurrentDc 󰥜
    CurrentDc,
    /// CursorDefault 󰇀
    CursorDefault,
    /// CursorDefaultClick 󰳽
    CursorDefaultClick,
    /// CursorDefaultClickOutline 󰳾
    CursorDefaultClickOutline,
    /// CursorDefaultGesture 󱄧
    CursorDefaultGesture,
    /// CursorDefaultGestureOutline 󱄨
    CursorDefaultGestureOutline,
    /// CursorDefaultOutline 󰆿
    CursorDefaultOutline,
    /// CursorMove 󰆾
    CursorMove,
    /// CursorPointer 󰆽
    CursorPointer,
    /// CursorText 󰗧
    CursorText,
    /// Curtains 󱡆
    Curtains,
    /// CurtainsClosed 󱡇
    CurtainsClosed,
    /// Cut 
    Cut,
    /// Cylinder 󱥎
    Cylinder,
    /// CylinderOff 󱥏
    CylinderOff,
    /// DanceBallroom 󱗻
    DanceBallroom,
    /// DancePole 󱕸
    DancePole,
    /// Dash 
    Dash,
    /// DashOne 
    DashOne,
    /// Dashboard 
    Dashboard,
    /// DashboardOne 
    DashboardOne,
    /// DataMatrix 󱔼
    DataMatrix,
    /// DataMatrixEdit 󱔽
    DataMatrixEdit,
    /// DataMatrixMinus 󱔾
    DataMatrixMinus,
    /// DataMatrixPlus 󱔿
    DataMatrixPlus,
    /// DataMatrixRemove 󱕀
    DataMatrixRemove,
    /// DataMatrixScan 󱕁
    DataMatrixScan,
    /// Database 
    Database,
    /// DatabaseAlert 󱘺
    DatabaseAlert,
    /// DatabaseAlertOutline 󱘤
    DatabaseAlertOutline,
    /// DatabaseArrowDown 󱘻
    DatabaseArrowDown,
    /// DatabaseArrowDownOutline 󱘥
    DatabaseArrowDownOutline,
    /// DatabaseArrowLeft 󱘼
    DatabaseArrowLeft,
    /// DatabaseArrowLeftOutline 󱘦
    DatabaseArrowLeftOutline,
    /// DatabaseArrowRight 󱘽
    DatabaseArrowRight,
    /// DatabaseArrowRightOutline 󱘧
    DatabaseArrowRightOutline,
    /// DatabaseArrowUp 󱘾
    DatabaseArrowUp,
    /// DatabaseArrowUpOutline 󱘨
    DatabaseArrowUpOutline,
    /// DatabaseCheck 󰪩
    DatabaseCheck,
    /// DatabaseCheckOutline 󱘩
    DatabaseCheckOutline,
    /// DatabaseClock 󱘿
    DatabaseClock,
    /// DatabaseClockOutline 󱘪
    DatabaseClockOutline,
    /// DatabaseCog 󱙋
    DatabaseCog,
    /// DatabaseCogOutline 󱙌
    DatabaseCogOutline,
    /// DatabaseEdit 󰮆
    DatabaseEdit,
    /// DatabaseEditOutline 󱘫
    DatabaseEditOutline,
    /// DatabaseExport 󰥞
    DatabaseExport,
    /// DatabaseExportOutline 󱘬
    DatabaseExportOutline,
    /// DatabaseEye 󱤟
    DatabaseEye,
    /// DatabaseEyeOff 󱤠
    DatabaseEyeOff,
    /// DatabaseEyeOffOutline 󱤡
    DatabaseEyeOffOutline,
    /// DatabaseEyeOutline 󱤢
    DatabaseEyeOutline,
    /// DatabaseImport 󰥝
    DatabaseImport,
    /// DatabaseImportOutline 󱘭
    DatabaseImportOutline,
    /// DatabaseLock 󰪪
    DatabaseLock,
    /// DatabaseLockOutline 󱘮
    DatabaseLockOutline,
    /// DatabaseMarker 󱋶
    DatabaseMarker,
    /// DatabaseMarkerOutline 󱘯
    DatabaseMarkerOutline,
    /// DatabaseMinus 󰆻
    DatabaseMinus,
    /// DatabaseMinusOutline 󱘰
    DatabaseMinusOutline,
    /// DatabaseOff 󱙀
    DatabaseOff,
    /// DatabaseOffOutline 󱘱
    DatabaseOffOutline,
    /// DatabaseOne 
    DatabaseOne,
    /// DatabaseOutline 󱘲
    DatabaseOutline,
    /// DatabasePlus 󰆺
    DatabasePlus,
    /// DatabasePlusOutline 󱘳
    DatabasePlusOutline,
    /// DatabaseRefresh 󰗂
    DatabaseRefresh,
    /// DatabaseRefreshOutline 󱘴
    DatabaseRefreshOutline,
    /// DatabaseRemove 󰴀
    DatabaseRemove,
    /// DatabaseRemoveOutline 󱘵
    DatabaseRemoveOutline,
    /// DatabaseSearch 󰡦
    DatabaseSearch,
    /// DatabaseSearchOutline 󱘶
    DatabaseSearchOutline,
    /// DatabaseSettings 󰴁
    DatabaseSettings,
    /// DatabaseSettingsOutline 󱘷
    DatabaseSettingsOutline,
    /// DatabaseSync 󰳿
    DatabaseSync,
    /// DatabaseSyncOutline 󱘸
    DatabaseSyncOutline,
    /// DatabaseTwo 󰆼
    DatabaseTwo,
    /// DeathStar 󰣘
    DeathStar,
    /// DeathStarVariant 󰣙
    DeathStarVariant,
    /// DeathlyHallows 󰮇
    DeathlyHallows,
    /// Debian 󰣚
    Debian,
    /// Debug 
    Debug,
    /// DebugAll 
    DebugAll,
    /// DebugAlt 
    DebugAlt,
    /// DebugAltSmall 
    DebugAltSmall,
    /// DebugBreakpointConditional 
    DebugBreakpointConditional,
    /// DebugBreakpointConditionalUnverified 
    DebugBreakpointConditionalUnverified,
    /// DebugBreakpointData 
    DebugBreakpointData,
    /// DebugBreakpointDataUnverified 
    DebugBreakpointDataUnverified,
    /// DebugBreakpointFunction 
    DebugBreakpointFunction,
    /// DebugBreakpointFunctionUnverified 
    DebugBreakpointFunctionUnverified,
    /// DebugBreakpointLog 
    DebugBreakpointLog,
    /// DebugBreakpointLogUnverified 
    DebugBreakpointLogUnverified,
    /// DebugBreakpointUnsupported 
    DebugBreakpointUnsupported,
    /// DebugConsole 
    DebugConsole,
    /// DebugContinue 
    DebugContinue,
    /// DebugContinueSmall 
    DebugContinueSmall,
    /// DebugCoverage 
    DebugCoverage,
    /// DebugDisconnect 
    DebugDisconnect,
    /// DebugLineByLine 
    DebugLineByLine,
    /// DebugPause 
    DebugPause,
    /// DebugRerun 
    DebugRerun,
    /// DebugRestart 
    DebugRestart,
    /// DebugRestartFrame 
    DebugRestartFrame,
    /// DebugReverseContinue 
    DebugReverseContinue,
    /// DebugStackframe 
    DebugStackframe,
    /// DebugStackframeActive 
    DebugStackframeActive,
    /// DebugStackframeDot 
    DebugStackframeDot,
    /// DebugStart 
    DebugStart,
    /// DebugStepBack 
    DebugStepBack,
    /// DebugStepInto 
    DebugStepInto,
    /// DebugStepIntoOne 󰆹
    DebugStepIntoOne,
    /// DebugStepOut 
    DebugStepOut,
    /// DebugStepOutOne 󰆸
    DebugStepOutOne,
    /// DebugStepOver 
    DebugStepOver,
    /// DebugStepOverOne 󰆷
    DebugStepOverOne,
    /// DebugStop 
    DebugStop,
    /// Decagram 󰝬
    Decagram,
    /// DecagramOutline 󰝭
    DecagramOutline,
    /// Decimal 󱂡
    Decimal,
    /// DecimalComma 󱂢
    DecimalComma,
    /// DecimalCommaDecrease 󱂣
    DecimalCommaDecrease,
    /// DecimalCommaIncrease 󱂤
    DecimalCommaIncrease,
    /// DecimalDecrease 󰆶
    DecimalDecrease,
    /// DecimalIncrease 󰆵
    DecimalIncrease,
    /// Deepin 
    Deepin,
    /// Delete 󰆴
    Delete,
    /// DeleteAlert 󱂥
    DeleteAlert,
    /// DeleteAlertOutline 󱂦
    DeleteAlertOutline,
    /// DeleteCircle 󰚃
    DeleteCircle,
    /// DeleteCircleOutline 󰮈
    DeleteCircleOutline,
    /// DeleteClock 󱕖
    DeleteClock,
    /// DeleteClockOutline 󱕗
    DeleteClockOutline,
    /// DeleteEmpty 󰛌
    DeleteEmpty,
    /// DeleteEmptyOutline 󰺝
    DeleteEmptyOutline,
    /// DeleteForever 󰗨
    DeleteForever,
    /// DeleteForeverOutline 󰮉
    DeleteForeverOutline,
    /// DeleteOff 󱂧
    DeleteOff,
    /// DeleteOffOutline 󱂨
    DeleteOffOutline,
    /// DeleteOutline 󰧧
    DeleteOutline,
    /// DeleteRestore 󰠙
    DeleteRestore,
    /// DeleteSweep 󰗩
    DeleteSweep,
    /// DeleteSweepOutline 󰱢
    DeleteSweepOutline,
    /// DeleteVariant 󰆳
    DeleteVariant,
    /// Delta 󰇂
    Delta,
    /// Dependabot 
    Dependabot,
    /// Desk 󱈹
    Desk,
    /// DeskLamp 󰥟
    DeskLamp,
    /// Deskphone 󰇃
    Deskphone,
    /// Desktop 
    Desktop,
    /// DesktopClassic 󰟀
    DesktopClassic,
    /// DesktopDownload 
    DesktopDownload,
    /// DesktopDownloadOne 
    DesktopDownloadOne,
    /// DesktopMac 󰇄
    DesktopMac,
    /// DesktopMacDashboard 󰧨
    DesktopMacDashboard,
    /// DesktopTower 󰇅
    DesktopTower,
    /// DesktopTowerMonitor 󰪫
    DesktopTowerMonitor,
    /// Details 󰇆
    Details,
    /// DevTo 󰵮
    DevTo,
    /// DeveloperBoard 󰚗
    DeveloperBoard,
    /// Deviantart 󰇇
    Deviantart,
    /// DeviceCamera 
    DeviceCamera,
    /// DeviceCameraOne 
    DeviceCameraOne,
    /// DeviceCameraVideo 
    DeviceCameraVideo,
    /// DeviceCameraVideoOne 
    DeviceCameraVideoOne,
    /// DeviceDesktop 
    DeviceDesktop,
    /// DeviceMobile 
    DeviceMobile,
    /// DeviceMobileOne 
    DeviceMobileOne,
    /// Devices 󰾰
    Devices,
    /// Devuan 
    Devuan,
    /// Dharmachakra 󰥋
    Dharmachakra,
    /// Diabetes 󱄦
    Diabetes,
    /// Dialpad 󰘜
    Dialpad,
    /// Diameter 󰱣
    Diameter,
    /// DiameterOutline 󰱤
    DiameterOutline,
    /// DiameterVariant 󰱥
    DiameterVariant,
    /// Diamond 
    Diamond,
    /// DiamondOne 󰮊
    DiamondOne,
    /// DiamondOutline 󰮋
    DiamondOutline,
    /// DiamondStone 󰇈
    DiamondStone,
    /// Dice 
    Dice,
    /// DiceDeight 󱅒
    DiceDeight,
    /// DiceDeightOutline 󰗬
    DiceDeightOutline,
    /// DiceDfour 󱅐
    DiceDfour,
    /// DiceDfourOutline 󰗫
    DiceDfourOutline,
    /// DiceDonetwo 󱅔
    DiceDonetwo,
    /// DiceDonetwoOutline 󰡧
    DiceDonetwoOutline,
    /// DiceDonezero 󱅓
    DiceDonezero,
    /// DiceDonezeroOutline 󰝯
    DiceDonezeroOutline,
    /// DiceDsix 󱅑
    DiceDsix,
    /// DiceDsixOutline 󰗭
    DiceDsixOutline,
    /// DiceDtwozero 󱅕
    DiceDtwozero,
    /// DiceDtwozeroOutline 󰗪
    DiceDtwozeroOutline,
    /// DiceFive 󰇎
    DiceFive,
    /// DiceFiveOutline 󱅎
    DiceFiveOutline,
    /// DiceFour 󰇍
    DiceFour,
    /// DiceFourOutline 󱅍
    DiceFourOutline,
    /// DiceMultiple 󰝮
    DiceMultiple,
    /// DiceMultipleOutline 󱅖
    DiceMultipleOutline,
    /// DiceOne 󰇊
    DiceOne,
    /// DiceOneOutline 󱅊
    DiceOneOutline,
    /// DiceSix 󰇏
    DiceSix,
    /// DiceSixOutline 󱅏
    DiceSixOutline,
    /// DiceThree 󰇌
    DiceThree,
    /// DiceThreeOutline 󱅌
    DiceThreeOutline,
    /// DiceTwo 󰇋
    DiceTwo,
    /// DiceTwoOutline 󱅋
    DiceTwoOutline,
    /// Diff 
    Diff,
    /// DiffAdded 
    DiffAdded,
    /// DiffAddedOne 
    DiffAddedOne,
    /// DiffIgnored 
    DiffIgnored,
    /// DiffIgnoredOne 
    DiffIgnoredOne,
    /// DiffModified 
    DiffModified,
    /// DiffModifiedOne 
    DiffModifiedOne,
    /// DiffOne 
    DiffOne,
    /// DiffRemoved 
    DiffRemoved,
    /// DiffRemovedOne 
    DiffRemovedOne,
    /// DiffRenamed 
    DiffRenamed,
    /// DiffRenamedOne 
    DiffRenamedOne,
    /// DigitalOcean 󱈷
    DigitalOcean,
    /// DipSwitch 󰟁
    DipSwitch,
    /// Directions 󰇐
    Directions,
    /// DirectionsFork 󰙁
    DirectionsFork,
    /// Disc 󰗮
    Disc,
    /// DiscAlert 󰇑
    DiscAlert,
    /// DiscPlayer 󰥠
    DiscPlayer,
    /// Discard 
    Discard,
    /// Disco 
    Disco,
    /// Discord 󰙯
    Discord,
    /// DiscussionClosed 
    DiscussionClosed,
    /// DiscussionDuplicate 
    DiscussionDuplicate,
    /// DiscussionOutdated 
    DiscussionOutdated,
    /// Dishwasher 󰪬
    Dishwasher,
    /// DishwasherAlert 󱆸
    DishwasherAlert,
    /// DishwasherOff 󱆹
    DishwasherOff,
    /// Disqus 󰇒
    Disqus,
    /// DistributeHorizontalCenter 󱇉
    DistributeHorizontalCenter,
    /// DistributeHorizontalLeft 󱇈
    DistributeHorizontalLeft,
    /// DistributeHorizontalRight 󱇊
    DistributeHorizontalRight,
    /// DistributeVerticalBottom 󱇋
    DistributeVerticalBottom,
    /// DistributeVerticalCenter 󱇌
    DistributeVerticalCenter,
    /// DistributeVerticalTop 󱇍
    DistributeVerticalTop,
    /// Diversify 󱡷
    Diversify,
    /// Diving 󱥷
    Diving,
    /// DivingFlippers 󰶿
    DivingFlippers,
    /// DivingHelmet 󰷀
    DivingHelmet,
    /// DivingScuba 󰷁
    DivingScuba,
    /// DivingScubaFlag 󰷂
    DivingScubaFlag,
    /// DivingScubaTank 󰷃
    DivingScubaTank,
    /// DivingScubaTankMultiple 󰷄
    DivingScubaTankMultiple,
    /// DivingSnorkel 󰷅
    DivingSnorkel,
    /// Division 󰇔
    Division,
    /// DivisionBox 󰇕
    DivisionBox,
    /// Dlna 󰩁
    Dlna,
    /// Dna 
    Dna,
    /// DnaOne 󰚄
    DnaOne,
    /// Dns 󰇖
    Dns,
    /// DnsOutline 󰮌
    DnsOutline,
    /// DockBottom 󱂩
    DockBottom,
    /// DockLeft 󱂪
    DockLeft,
    /// DockRight 󱂫
    DockRight,
    /// DockTop 󱔓
    DockTop,
    /// DockWindow 󱂬
    DockWindow,
    /// Docker 󰡨
    Docker,
    /// Doctor 󰩂
    Doctor,
    /// Dog 󰩃
    Dog,
    /// DogService 󰪭
    DogService,
    /// DogSide 󰩄
    DogSide,
    /// DogSideOff 󱛮
    DogSideOff,
    /// Dolby 󰚳
    Dolby,
    /// Dolly 󰺞
    Dolly,
    /// Dolphin 󱢴
    Dolphin,
    /// Domain 󰇗
    Domain,
    /// DomainOff 󰵯
    DomainOff,
    /// DomainPlus 󱂭
    DomainPlus,
    /// DomainRemove 󱂮
    DomainRemove,
    /// DomeLight 󱐞
    DomeLight,
    /// DominoMask 󱀣
    DominoMask,
    /// Donkey 󰟂
    Donkey,
    /// Donut 
    Donut,
    /// Door 󰠚
    Door,
    /// DoorClosed 󰠛
    DoorClosed,
    /// DoorClosedLock 󱂯
    DoorClosedLock,
    /// DoorOpen 󰠜
    DoorOpen,
    /// DoorSliding 󱠞
    DoorSliding,
    /// DoorSlidingLock 󱠟
    DoorSlidingLock,
    /// DoorSlidingOpen 󱠠
    DoorSlidingOpen,
    /// Doorbell 󱋦
    Doorbell,
    /// DoorbellVideo 󰡩
    DoorbellVideo,
    /// Dot 
    Dot,
    /// DotCircleAlt 
    DotCircleAlt,
    /// DotFill 
    DotFill,
    /// DotNet 󰪮
    DotNet,
    /// DotsCircle 󱥸
    DotsCircle,
    /// DotsGrid 󱗼
    DotsGrid,
    /// DotsHexagon 󱗿
    DotsHexagon,
    /// DotsHorizontal 󰇘
    DotsHorizontal,
    /// DotsHorizontalCircle 󰟃
    DotsHorizontalCircle,
    /// DotsHorizontalCircleOutline 󰮍
    DotsHorizontalCircleOutline,
    /// DotsSquare 󱗽
    DotsSquare,
    /// DotsTriangle 󱗾
    DotsTriangle,
    /// DotsVertical 󰇙
    DotsVertical,
    /// DotsVerticalCircle 󰟄
    DotsVerticalCircle,
    /// DotsVerticalCircleOutline 󰮎
    DotsVerticalCircleOutline,
    /// DoubleAngleDown 
    DoubleAngleDown,
    /// DoubleAngleLeft 
    DoubleAngleLeft,
    /// DoubleAngleRight 
    DoubleAngleRight,
    /// DoubleAngleUp 
    DoubleAngleUp,
    /// Download 
    Download,
    /// DownloadAlt 
    DownloadAlt,
    /// DownloadBox 󱑢
    DownloadBox,
    /// DownloadBoxOutline 󱑣
    DownloadBoxOutline,
    /// DownloadCircle 󱑤
    DownloadCircle,
    /// DownloadCircleOutline 󱑥
    DownloadCircleOutline,
    /// DownloadLock 󱌠
    DownloadLock,
    /// DownloadLockOutline 󱌡
    DownloadLockOutline,
    /// DownloadMultiple 󰧩
    DownloadMultiple,
    /// DownloadNetwork 󰛴
    DownloadNetwork,
    /// DownloadNetworkOutline 󰱦
    DownloadNetworkOutline,
    /// DownloadOff 󱂰
    DownloadOff,
    /// DownloadOffOutline 󱂱
    DownloadOffOutline,
    /// DownloadOne 
    DownloadOne,
    /// DownloadOutline 󰮏
    DownloadOutline,
    /// DownloadTwo 󰇚
    DownloadTwo,
    /// Drag 󰇛
    Drag,
    /// DragHorizontal 󰇜
    DragHorizontal,
    /// DragHorizontalVariant 󱋰
    DragHorizontalVariant,
    /// DragVariant 󰮐
    DragVariant,
    /// DragVertical 󰇝
    DragVertical,
    /// DragVerticalVariant 󱋱
    DragVerticalVariant,
    /// DramaMasks 󰴂
    DramaMasks,
    /// Draw 󰽉
    Draw,
    /// DrawPen 󱦹
    DrawPen,
    /// Drawing 󰇞
    Drawing,
    /// DrawingBox 󰇟
    DrawingBox,
    /// Dress 
    Dress,
    /// Dresser 󰽊
    Dresser,
    /// DresserOutline 󰽋
    DresserOutline,
    /// Dribble 
    Dribble,
    /// Drone 󰇢
    Drone,
    /// Drop 
    Drop,
    /// Dropbox 
    Dropbox,
    /// DropboxOne 󰇣
    DropboxOne,
    /// Drupal 󰇤
    Drupal,
    /// Duck 󰇥
    Duck,
    /// Dumbbell 󰇦
    Dumbbell,
    /// DumpTruck 󰱧
    DumpTruck,
    /// Duplicate 
    Duplicate,
    /// Dwm 
    Dwm,
    /// EarHearing 󰟅
    EarHearing,
    /// EarHearingLoop 󱫮
    EarHearingLoop,
    /// EarHearingOff 󰩅
    EarHearingOff,
    /// Earbuds 󱡏
    Earbuds,
    /// EarbudsOff 󱡐
    EarbudsOff,
    /// EarbudsOffOutline 󱡑
    EarbudsOffOutline,
    /// EarbudsOutline 󱡒
    EarbudsOutline,
    /// Earth 󰇧
    Earth,
    /// EarthArrowRight 󱌑
    EarthArrowRight,
    /// EarthBox 󰛍
    EarthBox,
    /// EarthBoxMinus 󱐇
    EarthBoxMinus,
    /// EarthBoxOff 󰛎
    EarthBoxOff,
    /// EarthBoxPlus 󱐆
    EarthBoxPlus,
    /// EarthBoxRemove 󱐈
    EarthBoxRemove,
    /// EarthMinus 󱐄
    EarthMinus,
    /// EarthOff 󰇨
    EarthOff,
    /// EarthPlus 󱐃
    EarthPlus,
    /// EarthRemove 󱐅
    EarthRemove,
    /// Edit 
    Edit,
    /// EditOne 
    EditOne,
    /// EditSign 
    EditSign,
    /// EditorLayout 
    EditorLayout,
    /// Egg 󰪯
    Egg,
    /// EggEaster 󰪰
    EggEaster,
    /// EggFried 󱡊
    EggFried,
    /// EggOff 󱏰
    EggOff,
    /// EggOffOutline 󱏱
    EggOffOutline,
    /// EggOutline 󱏲
    EggOutline,
    /// EiffelTower 󱕫
    EiffelTower,
    /// EightTrack 󰧪
    EightTrack,
    /// Eject 
    Eject,
    /// EjectOne 󰇪
    EjectOne,
    /// EjectOutline 󰮑
    EjectOutline,
    /// ElectricSwitch 󰺟
    ElectricSwitch,
    /// ElectricSwitchClosed 󱃙
    ElectricSwitchClosed,
    /// ElectronFramework 󱀤
    ElectronFramework,
    /// ElementaryOs 
    ElementaryOs,
    /// Elephant 󰟆
    Elephant,
    /// ElevationDecline 󰇫
    ElevationDecline,
    /// ElevationRise 󰇬
    ElevationRise,
    /// Elevator 󰇭
    Elevator,
    /// ElevatorDown 󱋂
    ElevatorDown,
    /// ElevatorPassenger 󱎁
    ElevatorPassenger,
    /// ElevatorPassengerOff 󱥹
    ElevatorPassengerOff,
    /// ElevatorPassengerOffOutline 󱥺
    ElevatorPassengerOffOutline,
    /// ElevatorPassengerOutline 󱥻
    ElevatorPassengerOutline,
    /// ElevatorUp 󱋁
    ElevatorUp,
    /// Ellipse 󰺠
    Ellipse,
    /// EllipseOutline 󰺡
    EllipseOutline,
    /// Ellipsis 
    Ellipsis,
    /// EllipsisHorizontal 
    EllipsisHorizontal,
    /// EllipsisOne 
    EllipsisOne,
    /// EllipsisVertical 
    EllipsisVertical,
    /// Ello 
    Ello,
    /// Email 󰇮
    Email,
    /// EmailAlert 󰛏
    EmailAlert,
    /// EmailAlertOutline 󰵂
    EmailAlertOutline,
    /// EmailBox 󰴃
    EmailBox,
    /// EmailCheck 󰪱
    EmailCheck,
    /// EmailCheckOutline 󰪲
    EmailCheckOutline,
    /// EmailEdit 󰻣
    EmailEdit,
    /// EmailEditOutline 󰻤
    EmailEditOutline,
    /// EmailFast 󱡯
    EmailFast,
    /// EmailFastOutline 󱡰
    EmailFastOutline,
    /// EmailLock 󰇱
    EmailLock,
    /// EmailMarkAsUnread 󰮒
    EmailMarkAsUnread,
    /// EmailMinus 󰻥
    EmailMinus,
    /// EmailMinusOutline 󰻦
    EmailMinusOutline,
    /// EmailMultiple 󰻧
    EmailMultiple,
    /// EmailMultipleOutline 󰻨
    EmailMultipleOutline,
    /// EmailNewsletter 󰾱
    EmailNewsletter,
    /// EmailOff 󱏣
    EmailOff,
    /// EmailOffOutline 󱏤
    EmailOffOutline,
    /// EmailOpen 󰇯
    EmailOpen,
    /// EmailOpenMultiple 󰻩
    EmailOpenMultiple,
    /// EmailOpenMultipleOutline 󰻪
    EmailOpenMultipleOutline,
    /// EmailOpenOutline 󰗯
    EmailOpenOutline,
    /// EmailOutline 󰇰
    EmailOutline,
    /// EmailPlus 󰧫
    EmailPlus,
    /// EmailPlusOutline 󰧬
    EmailPlusOutline,
    /// EmailReceive 󱃚
    EmailReceive,
    /// EmailReceiveOutline 󱃛
    EmailReceiveOutline,
    /// EmailRemove 󱙡
    EmailRemove,
    /// EmailRemoveOutline 󱙢
    EmailRemoveOutline,
    /// EmailSeal 󱥛
    EmailSeal,
    /// EmailSealOutline 󱥜
    EmailSealOutline,
    /// EmailSearch 󰥡
    EmailSearch,
    /// EmailSearchOutline 󰥢
    EmailSearchOutline,
    /// EmailSend 󱃜
    EmailSend,
    /// EmailSendOutline 󱃝
    EmailSendOutline,
    /// EmailSync 󱋇
    EmailSync,
    /// EmailSyncOutline 󱋈
    EmailSyncOutline,
    /// EmailVariant 󰗰
    EmailVariant,
    /// Ember 󰬰
    Ember,
    /// Emby 󰚴
    Emby,
    /// Emoticon 󰱨
    Emoticon,
    /// EmoticonAngry 󰱩
    EmoticonAngry,
    /// EmoticonAngryOutline 󰱪
    EmoticonAngryOutline,
    /// EmoticonConfused 󱃞
    EmoticonConfused,
    /// EmoticonConfusedOutline 󱃟
    EmoticonConfusedOutline,
    /// EmoticonCool 󰱫
    EmoticonCool,
    /// EmoticonCoolOutline 󰇳
    EmoticonCoolOutline,
    /// EmoticonCry 󰱬
    EmoticonCry,
    /// EmoticonCryOutline 󰱭
    EmoticonCryOutline,
    /// EmoticonDead 󰱮
    EmoticonDead,
    /// EmoticonDeadOutline 󰚛
    EmoticonDeadOutline,
    /// EmoticonDevil 󰱯
    EmoticonDevil,
    /// EmoticonDevilOutline 󰇴
    EmoticonDevilOutline,
    /// EmoticonExcited 󰱰
    EmoticonExcited,
    /// EmoticonExcitedOutline 󰚜
    EmoticonExcitedOutline,
    /// EmoticonFrown 󰽌
    EmoticonFrown,
    /// EmoticonFrownOutline 󰽍
    EmoticonFrownOutline,
    /// EmoticonHappy 󰱱
    EmoticonHappy,
    /// EmoticonHappyOutline 󰇵
    EmoticonHappyOutline,
    /// EmoticonKiss 󰱲
    EmoticonKiss,
    /// EmoticonKissOutline 󰱳
    EmoticonKissOutline,
    /// EmoticonLol 󱈔
    EmoticonLol,
    /// EmoticonLolOutline 󱈕
    EmoticonLolOutline,
    /// EmoticonNeutral 󰱴
    EmoticonNeutral,
    /// EmoticonNeutralOutline 󰇶
    EmoticonNeutralOutline,
    /// EmoticonOutline 󰇲
    EmoticonOutline,
    /// EmoticonPoop 󰇷
    EmoticonPoop,
    /// EmoticonPoopOutline 󰱵
    EmoticonPoopOutline,
    /// EmoticonSad 󰱶
    EmoticonSad,
    /// EmoticonSadOutline 󰇸
    EmoticonSadOutline,
    /// EmoticonSick 󱕼
    EmoticonSick,
    /// EmoticonSickOutline 󱕽
    EmoticonSickOutline,
    /// EmoticonTongue 󰇹
    EmoticonTongue,
    /// EmoticonTongueOutline 󰱷
    EmoticonTongueOutline,
    /// EmoticonWink 󰱸
    EmoticonWink,
    /// EmoticonWinkOutline 󰱹
    EmoticonWinkOutline,
    /// EmptyWindow 
    EmptyWindow,
    /// EndeavourOs 
    EndeavourOs,
    /// Engine 󰇺
    Engine,
    /// EngineOff 󰩆
    EngineOff,
    /// EngineOffOutline 󰩇
    EngineOffOutline,
    /// EngineOutline 󰇻
    EngineOutline,
    /// Enlightenment 
    Enlightenment,
    /// Envelope 
    Envelope,
    /// EnvelopeAlt 
    EnvelopeAlt,
    /// EnvelopeOpen 
    EnvelopeOpen,
    /// EnvelopeOpenO 
    EnvelopeOpenO,
    /// Epsilon 󱃠
    Epsilon,
    /// Equal 
    Equal,
    /// EqualBigger 
    EqualBigger,
    /// EqualBox 󰇽
    EqualBox,
    /// EqualOne 󰇼
    EqualOne,
    /// Equalizer 󰺢
    Equalizer,
    /// EqualizerOutline 󰺣
    EqualizerOutline,
    /// Eraser 󰇾
    Eraser,
    /// EraserVariant 󰙂
    EraserVariant,
    /// Error 
    Error,
    /// Escalator 󰇿
    Escalator,
    /// EscalatorBox 󱎙
    EscalatorBox,
    /// EscalatorDown 󱋀
    EscalatorDown,
    /// EscalatorUp 󱊿
    EscalatorUp,
    /// Eslint 󰱺
    Eslint,
    /// Et 󰪳
    Et,
    /// Ethereum 󰡪
    Ethereum,
    /// Ethernet 󰈀
    Ethernet,
    /// EthernetCable 󰈁
    EthernetCable,
    /// EthernetCableOff 󰈂
    EthernetCableOff,
    /// Eur 
    Eur,
    /// EvPlugCcsone 󱔙
    EvPlugCcsone,
    /// EvPlugCcstwo 󱔚
    EvPlugCcstwo,
    /// EvPlugChademo 󱔛
    EvPlugChademo,
    /// EvPlugTesla 󱔜
    EvPlugTesla,
    /// EvPlugTypeone 󱔝
    EvPlugTypeone,
    /// EvPlugTypetwo 󱔞
    EvPlugTypetwo,
    /// EvStation 󰗱
    EvStation,
    /// Evernote 󰈄
    Evernote,
    /// Excavator 󱀥
    Excavator,
    /// Exchange 
    Exchange,
    /// Exclamation 
    Exclamation,
    /// ExclamationOne 󰈅
    ExclamationOne,
    /// ExclamationSign 
    ExclamationSign,
    /// ExclamationThick 󱈸
    ExclamationThick,
    /// Exclude 
    Exclude,
    /// ExitRun 󰩈
    ExitRun,
    /// ExitToApp 󰈆
    ExitToApp,
    /// ExitToAppOne 󰗼
    ExitToAppOne,
    /// ExpandAll 
    ExpandAll,
    /// ExpandAllOne 󰪴
    ExpandAllOne,
    /// ExpandAllOutline 󰪵
    ExpandAllOutline,
    /// ExpandAlt 
    ExpandAlt,
    /// ExpansionCard 󰢮
    ExpansionCard,
    /// ExpansionCardVariant 󰾲
    ExpansionCardVariant,
    /// Exponent 󰥣
    Exponent,
    /// ExponentBox 󰥤
    ExponentBox,
    /// Export 
    Export,
    /// ExportOne 󰈇
    ExportOne,
    /// ExportVariant 󰮓
    ExportVariant,
    /// Extensions 
    Extensions,
    /// ExternalInterruption 
    ExternalInterruption,
    /// ExternalLink 
    ExternalLink,
    /// Eye 
    Eye,
    /// EyeArrowLeft 󱣽
    EyeArrowLeft,
    /// EyeArrowLeftOutline 󱣾
    EyeArrowLeftOutline,
    /// EyeArrowRight 󱣿
    EyeArrowRight,
    /// EyeArrowRightOutline 󱤀
    EyeArrowRightOutline,
    /// EyeCheck 󰴄
    EyeCheck,
    /// EyeCheckOutline 󰴅
    EyeCheckOutline,
    /// EyeCircle 󰮔
    EyeCircle,
    /// EyeCircleOutline 󰮕
    EyeCircleOutline,
    /// EyeClose 
    EyeClose,
    /// EyeClosed 
    EyeClosed,
    /// EyeClosedOne 
    EyeClosedOne,
    /// EyeMinus 󱀦
    EyeMinus,
    /// EyeMinusOutline 󱀧
    EyeMinusOutline,
    /// EyeOff 󰈉
    EyeOff,
    /// EyeOffOutline 󰛑
    EyeOffOutline,
    /// EyeOne 
    EyeOne,
    /// EyeOpen 
    EyeOpen,
    /// EyeOutline 󰛐
    EyeOutline,
    /// EyePlus 󰡫
    EyePlus,
    /// EyePlusOutline 󰡬
    EyePlusOutline,
    /// EyeRefresh 󱥼
    EyeRefresh,
    /// EyeRefreshOutline 󱥽
    EyeRefreshOutline,
    /// EyeRemove 󱗣
    EyeRemove,
    /// EyeRemoveOutline 󱗤
    EyeRemoveOutline,
    /// EyeSettings 󰡭
    EyeSettings,
    /// EyeSettingsOutline 󰡮
    EyeSettingsOutline,
    /// EyeTwo 󰈈
    EyeTwo,
    /// Eyedropper 󰈊
    Eyedropper,
    /// EyedropperMinus 󱏝
    EyedropperMinus,
    /// EyedropperOff 󱏟
    EyedropperOff,
    /// EyedropperPlus 󱏜
    EyedropperPlus,
    /// EyedropperRemove 󱏞
    EyedropperRemove,
    /// EyedropperVariant 󰈋
    EyedropperVariant,
    /// FDroid 
    FDroid,
    /// FaceAgent 󰵰
    FaceAgent,
    /// FaceMan 󰙃
    FaceMan,
    /// FaceManOutline 󰮖
    FaceManOutline,
    /// FaceManProfile 󰙄
    FaceManProfile,
    /// FaceManShimmer 󱗌
    FaceManShimmer,
    /// FaceManShimmerOutline 󱗍
    FaceManShimmerOutline,
    /// FaceMask 󱖆
    FaceMask,
    /// FaceMaskOutline 󱖇
    FaceMaskOutline,
    /// FaceRecognition 󰱻
    FaceRecognition,
    /// FaceWoman 󱁷
    FaceWoman,
    /// FaceWomanOutline 󱁸
    FaceWomanOutline,
    /// FaceWomanProfile 󱁶
    FaceWomanProfile,
    /// FaceWomanShimmer 󱗎
    FaceWomanShimmer,
    /// FaceWomanShimmerOutline 󱗏
    FaceWomanShimmerOutline,
    /// Facebook 
    Facebook,
    /// FacebookGaming 󰟝
    FacebookGaming,
    /// FacebookMessenger 󰈎
    FacebookMessenger,
    /// FacebookOne 󰈌
    FacebookOne,
    /// FacebookSign 
    FacebookSign,
    /// FacebookWorkplace 󰬱
    FacebookWorkplace,
    /// FacetimeVideo 
    FacetimeVideo,
    /// Factory 󰈏
    Factory,
    /// FamilyTree 󱘎
    FamilyTree,
    /// Fan 󰈐
    Fan,
    /// FanAlert 󱑬
    FanAlert,
    /// FanAuto 󱜝
    FanAuto,
    /// FanChevronDown 󱑭
    FanChevronDown,
    /// FanChevronUp 󱑮
    FanChevronUp,
    /// FanClock 󱨺
    FanClock,
    /// FanMinus 󱑰
    FanMinus,
    /// FanOff 󰠝
    FanOff,
    /// FanPlus 󱑯
    FanPlus,
    /// FanRemove 󱑱
    FanRemove,
    /// FanSpeedOne 󱑲
    FanSpeedOne,
    /// FanSpeedThree 󱑴
    FanSpeedThree,
    /// FanSpeedTwo 󱑳
    FanSpeedTwo,
    /// FastBackward 
    FastBackward,
    /// FastForward 󰈑
    FastForward,
    /// FastForwardFive 󱇸
    FastForwardFive,
    /// FastForwardOnefive 󱤺
    FastForwardOnefive,
    /// FastForwardOnezero 󰵱
    FastForwardOnezero,
    /// FastForwardOutline 󰛒
    FastForwardOutline,
    /// FastForwardSixzero 󱘋
    FastForwardSixzero,
    /// FastForwardThreezero 󰴆
    FastForwardThreezero,
    /// Fax 󰈒
    Fax,
    /// Feather 󰛓
    Feather,
    /// FeatureSearch 󰩉
    FeatureSearch,
    /// FeatureSearchOutline 󰩊
    FeatureSearchOutline,
    /// Fedora 󰣛
    Fedora,
    /// FedoraInverse 
    FedoraInverse,
    /// FeedDiscussion 
    FeedDiscussion,
    /// FeedForked 
    FeedForked,
    /// FeedHeart 
    FeedHeart,
    /// FeedMerged 
    FeedMerged,
    /// FeedPerson 
    FeedPerson,
    /// FeedRepo 
    FeedRepo,
    /// FeedRocket 
    FeedRocket,
    /// FeedStar 
    FeedStar,
    /// FeedTag 
    FeedTag,
    /// FeedTrophy 
    FeedTrophy,
    /// Feedback 
    Feedback,
    /// Feedly 
    Feedly,
    /// Female 
    Female,
    /// Fence 󱞚
    Fence,
    /// FenceElectric 󱟶
    FenceElectric,
    /// Fencing 󱓁
    Fencing,
    /// Ferris 
    Ferris,
    /// FerrisWheel 󰺤
    FerrisWheel,
    /// Ferry 󰈓
    Ferry,
    /// FighterJet 
    FighterJet,
    /// File 
    File,
    /// FileAccount 󰜻
    FileAccount,
    /// FileAccountOutline 󱀨
    FileAccountOutline,
    /// FileAdded 
    FileAdded,
    /// FileAlert 󰩋
    FileAlert,
    /// FileAlertOutline 󰩌
    FileAlertOutline,
    /// FileAlt 
    FileAlt,
    /// FileArrowLeftRight 󱪓
    FileArrowLeftRight,
    /// FileArrowLeftRightOutline 󱪔
    FileArrowLeftRightOutline,
    /// FileArrowUpDown 󱪕
    FileArrowUpDown,
    /// FileArrowUpDownOutline 󱪖
    FileArrowUpDownOutline,
    /// FileBadge 
    FileBadge,
    /// FileBinary 
    FileBinary,
    /// FileBinaryOne 
    FileBinaryOne,
    /// FileCabinet 󰪶
    FileCabinet,
    /// FileCad 󰻫
    FileCad,
    /// FileCadBox 󰻬
    FileCadBox,
    /// FileCancel 󰷆
    FileCancel,
    /// FileCancelOutline 󰷇
    FileCancelOutline,
    /// FileCertificate 󱆆
    FileCertificate,
    /// FileCertificateOutline 󱆇
    FileCertificateOutline,
    /// FileChart 󰈕
    FileChart,
    /// FileChartCheck 󱧆
    FileChartCheck,
    /// FileChartCheckOutline 󱧇
    FileChartCheckOutline,
    /// FileChartOutline 󱀩
    FileChartOutline,
    /// FileCheck 󰈖
    FileCheck,
    /// FileCheckOutline 󰸩
    FileCheckOutline,
    /// FileClock 󱋡
    FileClock,
    /// FileClockOutline 󱋢
    FileClockOutline,
    /// FileCloud 󰈗
    FileCloud,
    /// FileCloudOutline 󱀪
    FileCloudOutline,
    /// FileCode 
    FileCode,
    /// FileCodeOne 
    FileCodeOne,
    /// FileCodeOutline 󱀫
    FileCodeOutline,
    /// FileCodeThree 󰈮
    FileCodeThree,
    /// FileCodeTwo 
    FileCodeTwo,
    /// FileCog 󱁻
    FileCog,
    /// FileCogOutline 󱁼
    FileCogOutline,
    /// FileCompare 󰢪
    FileCompare,
    /// FileDelimited 󰈘
    FileDelimited,
    /// FileDelimitedOutline 󰺥
    FileDelimitedOutline,
    /// FileDiff 
    FileDiff,
    /// FileDirectory 
    FileDirectory,
    /// FileDirectoryFill 
    FileDirectoryFill,
    /// FileDirectoryOpenFill 
    FileDirectoryOpenFill,
    /// FileDocument 󰈙
    FileDocument,
    /// FileDocumentAlert 󱪗
    FileDocumentAlert,
    /// FileDocumentAlertOutline 󱪘
    FileDocumentAlertOutline,
    /// FileDocumentCheck 󱪙
    FileDocumentCheck,
    /// FileDocumentCheckOutline 󱪚
    FileDocumentCheckOutline,
    /// FileDocumentEdit 󰷈
    FileDocumentEdit,
    /// FileDocumentEditOutline 󰷉
    FileDocumentEditOutline,
    /// FileDocumentMinus 󱪛
    FileDocumentMinus,
    /// FileDocumentMinusOutline 󱪜
    FileDocumentMinusOutline,
    /// FileDocumentMultiple 󱔗
    FileDocumentMultiple,
    /// FileDocumentMultipleOutline 󱔘
    FileDocumentMultipleOutline,
    /// FileDocumentOutline 󰧮
    FileDocumentOutline,
    /// FileDocumentPlus 󱪝
    FileDocumentPlus,
    /// FileDocumentPlusOutline 󱪞
    FileDocumentPlusOutline,
    /// FileDocumentRemove 󱪟
    FileDocumentRemove,
    /// FileDocumentRemoveOutline 󱪠
    FileDocumentRemoveOutline,
    /// FileDownload 󰥥
    FileDownload,
    /// FileDownloadOutline 󰥦
    FileDownloadOutline,
    /// FileEdit 󱇧
    FileEdit,
    /// FileEditOutline 󱇨
    FileEditOutline,
    /// FileExcel 󰈛
    FileExcel,
    /// FileExcelBox 󰈜
    FileExcelBox,
    /// FileExcelBoxOutline 󱀬
    FileExcelBoxOutline,
    /// FileExcelOutline 󱀭
    FileExcelOutline,
    /// FileExport 
    FileExport,
    /// FileExportOne 󰈝
    FileExportOne,
    /// FileExportOutline 󱀮
    FileExportOutline,
    /// FileEye 󰷊
    FileEye,
    /// FileEyeOutline 󰷋
    FileEyeOutline,
    /// FileFind 󰈞
    FileFind,
    /// FileFindOutline 󰮗
    FileFindOutline,
    /// FileGifBox 󰵸
    FileGifBox,
    /// FileHidden 󰘓
    FileHidden,
    /// FileImage 󰈟
    FileImage,
    /// FileImageMarker 󱝲
    FileImageMarker,
    /// FileImageMarkerOutline 󱝳
    FileImageMarkerOutline,
    /// FileImageMinus 󱤻
    FileImageMinus,
    /// FileImageMinusOutline 󱤼
    FileImageMinusOutline,
    /// FileImageOutline 󰺰
    FileImageOutline,
    /// FileImagePlus 󱤽
    FileImagePlus,
    /// FileImagePlusOutline 󱤾
    FileImagePlusOutline,
    /// FileImageRemove 󱤿
    FileImageRemove,
    /// FileImageRemoveOutline 󱥀
    FileImageRemoveOutline,
    /// FileImport 
    FileImport,
    /// FileImportOne 󰈠
    FileImportOne,
    /// FileImportOutline 󱀯
    FileImportOutline,
    /// FileJpgBox 󰈥
    FileJpgBox,
    /// FileKey 󱆄
    FileKey,
    /// FileKeyOutline 󱆅
    FileKeyOutline,
    /// FileLink 󱅷
    FileLink,
    /// FileLinkOutline 󱅸
    FileLinkOutline,
    /// FileLock 󰈡
    FileLock,
    /// FileLockOpen 󱧈
    FileLockOpen,
    /// FileLockOpenOutline 󱧉
    FileLockOpenOutline,
    /// FileLockOutline 󱀰
    FileLockOutline,
    /// FileMarker 󱝴
    FileMarker,
    /// FileMarkerOutline 󱝵
    FileMarkerOutline,
    /// FileMedia 
    FileMedia,
    /// FileMediaOne 
    FileMediaOne,
    /// FileMinus 󱪡
    FileMinus,
    /// FileMinusOutline 󱪢
    FileMinusOutline,
    /// FileMove 󰪹
    FileMove,
    /// FileMoveOutline 󱀱
    FileMoveOutline,
    /// FileMoved 
    FileMoved,
    /// FileMultiple 󰈢
    FileMultiple,
    /// FileMultipleOutline 󱀲
    FileMultipleOutline,
    /// FileMusic 󰈣
    FileMusic,
    /// FileMusicOutline 󰸪
    FileMusicOutline,
    /// FileOne 
    FileOne,
    /// FileOutline 󰈤
    FileOutline,
    /// FilePdf 
    FilePdf,
    /// FilePdfBox 󰈦
    FilePdfBox,
    /// FilePercent 󰠞
    FilePercent,
    /// FilePercentOutline 󱀳
    FilePercentOutline,
    /// FilePhone 󱅹
    FilePhone,
    /// FilePhoneOutline 󱅺
    FilePhoneOutline,
    /// FilePlus 󰝒
    FilePlus,
    /// FilePlusOutline 󰻭
    FilePlusOutline,
    /// FilePngBox 󰸭
    FilePngBox,
    /// FilePowerpoint 󰈧
    FilePowerpoint,
    /// FilePowerpointBox 󰈨
    FilePowerpointBox,
    /// FilePowerpointBoxOutline 󱀴
    FilePowerpointBoxOutline,
    /// FilePowerpointOutline 󱀵
    FilePowerpointOutline,
    /// FilePresentationBox 󰈩
    FilePresentationBox,
    /// FileQuestion 󰡯
    FileQuestion,
    /// FileQuestionOutline 󱀶
    FileQuestionOutline,
    /// FileRefresh 󰤘
    FileRefresh,
    /// FileRefreshOutline 󰕁
    FileRefreshOutline,
    /// FileRemove 󰮘
    FileRemove,
    /// FileRemoveOutline 󱀷
    FileRemoveOutline,
    /// FileRemoved 
    FileRemoved,
    /// FileReplace 󰬲
    FileReplace,
    /// FileReplaceOutline 󰬳
    FileReplaceOutline,
    /// FileRestore 󰙰
    FileRestore,
    /// FileRestoreOutline 󱀸
    FileRestoreOutline,
    /// FileRotateLeft 󱨻
    FileRotateLeft,
    /// FileRotateLeftOutline 󱨼
    FileRotateLeftOutline,
    /// FileRotateRight 󱨽
    FileRotateRight,
    /// FileRotateRightOutline 󱨾
    FileRotateRightOutline,
    /// FileSearch 󰱼
    FileSearch,
    /// FileSearchOutline 󰱽
    FileSearchOutline,
    /// FileSend 󰈪
    FileSend,
    /// FileSendOutline 󱀹
    FileSendOutline,
    /// FileSettings 󱁹
    FileSettings,
    /// FileSettingsOutline 󱁺
    FileSettingsOutline,
    /// FileSign 󱧃
    FileSign,
    /// FileStar 󱀺
    FileStar,
    /// FileStarOutline 󱀻
    FileStarOutline,
    /// FileSubmodule 
    FileSubmodule,
    /// FileSubmoduleOne 
    FileSubmoduleOne,
    /// FileSwap 󰾴
    FileSwap,
    /// FileSwapOutline 󰾵
    FileSwapOutline,
    /// FileSymlinkDirectory 
    FileSymlinkDirectory,
    /// FileSymlinkDirectoryOne 
    FileSymlinkDirectoryOne,
    /// FileSymlinkFile 
    FileSymlinkFile,
    /// FileSymlinkFileOne 
    FileSymlinkFileOne,
    /// FileSync 󱈖
    FileSync,
    /// FileSyncOutline 󱈗
    FileSyncOutline,
    /// FileTable 󰱾
    FileTable,
    /// FileTableBox 󱃡
    FileTableBox,
    /// FileTableBoxMultiple 󱃢
    FileTableBoxMultiple,
    /// FileTableBoxMultipleOutline 󱃣
    FileTableBoxMultipleOutline,
    /// FileTableBoxOutline 󱃤
    FileTableBoxOutline,
    /// FileTableOutline 󰱿
    FileTableOutline,
    /// FileText 
    FileText,
    /// FileTextAlt 
    FileTextAlt,
    /// FileThree 󰈔
    FileThree,
    /// FileTree 󰙅
    FileTree,
    /// FileTreeOutline 󱏒
    FileTreeOutline,
    /// FileTwo 
    FileTwo,
    /// FileUndo 󰣜
    FileUndo,
    /// FileUndoOutline 󱀼
    FileUndoOutline,
    /// FileUpload 󰩍
    FileUpload,
    /// FileUploadOutline 󰩎
    FileUploadOutline,
    /// FileVideo 󰈫
    FileVideo,
    /// FileVideoOutline 󰸬
    FileVideoOutline,
    /// FileWord 󰈬
    FileWord,
    /// FileWordBox 󰈭
    FileWordBox,
    /// FileWordBoxOutline 󱀽
    FileWordBoxOutline,
    /// FileWordOutline 󱀾
    FileWordOutline,
    /// FileZip 
    FileZip,
    /// FileZipOne 
    FileZipOne,
    /// Files 
    Files,
    /// Film 
    Film,
    /// FilmOne 󰈯
    FilmOne,
    /// Filmstrip 󰈰
    Filmstrip,
    /// FilmstripBox 󰌲
    FilmstripBox,
    /// FilmstripBoxMultiple 󰴘
    FilmstripBoxMultiple,
    /// FilmstripOff 󰈱
    FilmstripOff,
    /// Filter 
    Filter,
    /// FilterCheck 󱣬
    FilterCheck,
    /// FilterCheckOutline 󱣭
    FilterCheckOutline,
    /// FilterCog 󱪣
    FilterCog,
    /// FilterCogOutline 󱪤
    FilterCogOutline,
    /// FilterFilled 
    FilterFilled,
    /// FilterMenu 󱃥
    FilterMenu,
    /// FilterMenuOutline 󱃦
    FilterMenuOutline,
    /// FilterMinus 󰻮
    FilterMinus,
    /// FilterMinusOutline 󰻯
    FilterMinusOutline,
    /// FilterMultiple 󱨿
    FilterMultiple,
    /// FilterMultipleOutline 󱩀
    FilterMultipleOutline,
    /// FilterOff 󱓯
    FilterOff,
    /// FilterOffOutline 󱓰
    FilterOffOutline,
    /// FilterOne 
    FilterOne,
    /// FilterOutline 󰈳
    FilterOutline,
    /// FilterPlus 󰻰
    FilterPlus,
    /// FilterPlusOutline 󰻱
    FilterPlusOutline,
    /// FilterRemove 󰈴
    FilterRemove,
    /// FilterRemoveOutline 󰈵
    FilterRemoveOutline,
    /// FilterSettings 󱪥
    FilterSettings,
    /// FilterSettingsOutline 󱪦
    FilterSettingsOutline,
    /// FilterThree 󰈲
    FilterThree,
    /// FilterTwo 
    FilterTwo,
    /// FilterVariant 󰈶
    FilterVariant,
    /// FilterVariantMinus 󱄒
    FilterVariantMinus,
    /// FilterVariantPlus 󱄓
    FilterVariantPlus,
    /// FilterVariantRemove 󱀿
    FilterVariantRemove,
    /// Finance 󰠟
    Finance,
    /// FindReplace 󰛔
    FindReplace,
    /// Fingerprint 
    Fingerprint,
    /// FingerprintOff 󰺱
    FingerprintOff,
    /// FingerprintOne 󰈷
    FingerprintOne,
    /// Fire 
    Fire,
    /// FireAlert 󱗗
    FireAlert,
    /// FireCircle 󱠇
    FireCircle,
    /// FireExtinguisher 󰻲
    FireExtinguisher,
    /// FireHydrant 󱄷
    FireHydrant,
    /// FireHydrantAlert 󱄸
    FireHydrantAlert,
    /// FireHydrantOff 󱄹
    FireHydrantOff,
    /// FireOff 󱜢
    FireOff,
    /// FireOne 󰈸
    FireOne,
    /// FireTruck 󰢫
    FireTruck,
    /// Firebase 󰥧
    Firebase,
    /// Firefox 󰈹
    Firefox,
    /// Fireplace 󰸮
    Fireplace,
    /// FireplaceOff 󰸯
    FireplaceOff,
    /// Firewire 󰖾
    Firewire,
    /// Firework 󰸰
    Firework,
    /// FireworkOff 󱜣
    FireworkOff,
    /// FiscalHost 
    FiscalHost,
    /// Fish 󰈺
    Fish,
    /// FishOff 󱏳
    FishOff,
    /// Fishbowl 󰻳
    Fishbowl,
    /// FishbowlOutline 󰻴
    FishbowlOutline,
    /// FitToPage 󰻵
    FitToPage,
    /// FitToPageOutline 󰻶
    FitToPageOutline,
    /// FitToScreen 󱣴
    FitToScreen,
    /// FitToScreenOutline 󱣵
    FitToScreenOutline,
    /// Fiveeighteight 
    Fiveeighteight,
    /// Fiveeightfive 
    Fiveeightfive,
    /// Fiveeightfour 
    Fiveeightfour,
    /// Fiveeightnine 
    Fiveeightnine,
    /// Fiveeightone 
    Fiveeightone,
    /// Fiveeightseven 
    Fiveeightseven,
    /// Fiveeightsix 
    Fiveeightsix,
    /// Fiveeightthree 
    Fiveeightthree,
    /// Fiveeighttwo 
    Fiveeighttwo,
    /// Fiveeightzero 
    Fiveeightzero,
    /// Fivefiveeight 
    Fivefiveeight,
    /// Fivefivefive 
    Fivefivefive,
    /// Fivefivefour 
    Fivefivefour,
    /// Fivefivenine 
    Fivefivenine,
    /// Fivefiveone 
    Fivefiveone,
    /// Fivefiveseven 
    Fivefiveseven,
    /// Fivefivesix 
    Fivefivesix,
    /// Fivefivethree 
    Fivefivethree,
    /// Fivefivetwo 
    Fivefivetwo,
    /// Fivefivezero 
    Fivefivezero,
    /// Fivefoureight 
    Fivefoureight,
    /// Fivefourfive 
    Fivefourfive,
    /// Fivefourfour 
    Fivefourfour,
    /// Fivefournine 
    Fivefournine,
    /// Fivefourone 
    Fivefourone,
    /// Fivefourseven 
    Fivefourseven,
    /// Fivefoursix 
    Fivefoursix,
    /// Fivefourthree 
    Fivefourthree,
    /// Fivefourtwo 
    Fivefourtwo,
    /// Fivefourzero 
    Fivefourzero,
    /// Fivenineeight 
    Fivenineeight,
    /// Fiveninefive 
    Fiveninefive,
    /// Fiveninefour 
    Fiveninefour,
    /// Fivenineone 
    Fivenineone,
    /// Fivenineseven 
    Fivenineseven,
    /// Fiveninesix 
    Fiveninesix,
    /// Fiveninethree 
    Fiveninethree,
    /// Fiveninetwo 
    Fiveninetwo,
    /// Fiveninezero 
    Fiveninezero,
    /// Fiveoneeight 
    Fiveoneeight,
    /// Fiveonefive 
    Fiveonefive,
    /// Fiveonefour 
    Fiveonefour,
    /// Fiveonenine 
    Fiveonenine,
    /// Fiveoneone 
    Fiveoneone,
    /// Fiveoneseven 
    Fiveoneseven,
    /// Fiveonesix 
    Fiveonesix,
    /// Fiveonethree 
    Fiveonethree,
    /// Fiveonetwo 
    Fiveonetwo,
    /// Fiveseveneight 
    Fiveseveneight,
    /// Fivesevenfive 
    Fivesevenfive,
    /// Fivesevenfour 
    Fivesevenfour,
    /// Fivesevennine 
    Fivesevennine,
    /// Fivesevenseven 
    Fivesevenseven,
    /// Fivesevensix 
    Fivesevensix,
    /// Fiveseventwo 
    Fiveseventwo,
    /// Fivesixeight 
    Fivesixeight,
    /// Fivesixfive 
    Fivesixfive,
    /// Fivesixfour 
    Fivesixfour,
    /// Fivesixnine 
    Fivesixnine,
    /// Fivesixone 
    Fivesixone,
    /// Fivesixseven 
    Fivesixseven,
    /// Fivesixsix 
    Fivesixsix,
    /// Fivesixthree 
    Fivesixthree,
    /// Fivesixtwo 
    Fivesixtwo,
    /// Fivesixzero 
    Fivesixzero,
    /// Fivethreeeight 
    Fivethreeeight,
    /// Fivethreefive 
    Fivethreefive,
    /// Fivethreefour 
    Fivethreefour,
    /// Fivethreenine 
    Fivethreenine,
    /// Fivethreeone 
    Fivethreeone,
    /// Fivethreeseven 
    Fivethreeseven,
    /// Fivethreesix 
    Fivethreesix,
    /// Fivethreethree 
    Fivethreethree,
    /// Fivethreetwo 
    Fivethreetwo,
    /// Fivethreezero 
    Fivethreezero,
    /// Fivetwoeight 
    Fivetwoeight,
    /// Fivetwofive 
    Fivetwofive,
    /// Fivetwofour 
    Fivetwofour,
    /// Fivetwonine 
    Fivetwonine,
    /// Fivetwoone 
    Fivetwoone,
    /// Fivetwoseven 
    Fivetwoseven,
    /// Fivetwosix 
    Fivetwosix,
    /// Fivetwothree 
    Fivetwothree,
    /// Fivetwotwo 
    Fivetwotwo,
    /// Fivetwozero 
    Fivetwozero,
    /// Fivezeroeight 
    Fivezeroeight,
    /// Fivezerofive 
    Fivezerofive,
    /// Fivezerofour 
    Fivezerofour,
    /// Fivezeronine 
    Fivezeronine,
    /// Fivezeroone 
    Fivezeroone,
    /// Fivezeroseven 
    Fivezeroseven,
    /// Fivezerosix 
    Fivezerosix,
    /// Fivezerothree 
    Fivezerothree,
    /// Fivezerotwo 
    Fivezerotwo,
    /// Fivezerozero 
    Fivezerozero,
    /// Flag 
    Flag,
    /// FlagAlt 
    FlagAlt,
    /// FlagCheckered 󰈼
    FlagCheckered,
    /// FlagMinus 󰮙
    FlagMinus,
    /// FlagMinusOutline 󱂲
    FlagMinusOutline,
    /// FlagOff 󱣮
    FlagOff,
    /// FlagOffOutline 󱣯
    FlagOffOutline,
    /// FlagOne 󰈻
    FlagOne,
    /// FlagOutline 󰈽
    FlagOutline,
    /// FlagPlus 󰮚
    FlagPlus,
    /// FlagPlusOutline 󱂳
    FlagPlusOutline,
    /// FlagRemove 󰮛
    FlagRemove,
    /// FlagRemoveOutline 󱂴
    FlagRemoveOutline,
    /// FlagTriangle 󰈿
    FlagTriangle,
    /// FlagVariant 󰉀
    FlagVariant,
    /// FlagVariantOutline 󰈾
    FlagVariantOutline,
    /// Flame 
    Flame,
    /// FlameOne 
    FlameOne,
    /// Flare 󰵲
    Flare,
    /// Flash 󰉁
    Flash,
    /// FlashAlert 󰻷
    FlashAlert,
    /// FlashAlertOutline 󰻸
    FlashAlertOutline,
    /// FlashAuto 󰉂
    FlashAuto,
    /// FlashOff 󰉃
    FlashOff,
    /// FlashOutline 󰛕
    FlashOutline,
    /// FlashRedEye 󰙻
    FlashRedEye,
    /// Flashlight 󰉄
    Flashlight,
    /// FlashlightOff 󰉅
    FlashlightOff,
    /// Flask 󰂓
    Flask,
    /// FlaskEmpty 󰂔
    FlaskEmpty,
    /// FlaskEmptyMinus 󱈺
    FlaskEmptyMinus,
    /// FlaskEmptyMinusOutline 󱈻
    FlaskEmptyMinusOutline,
    /// FlaskEmptyOff 󱏴
    FlaskEmptyOff,
    /// FlaskEmptyOffOutline 󱏵
    FlaskEmptyOffOutline,
    /// FlaskEmptyOutline 󰂕
    FlaskEmptyOutline,
    /// FlaskEmptyPlus 󱈼
    FlaskEmptyPlus,
    /// FlaskEmptyPlusOutline 󱈽
    FlaskEmptyPlusOutline,
    /// FlaskEmptyRemove 󱈾
    FlaskEmptyRemove,
    /// FlaskEmptyRemoveOutline 󱈿
    FlaskEmptyRemoveOutline,
    /// FlaskMinus 󱉀
    FlaskMinus,
    /// FlaskMinusOutline 󱉁
    FlaskMinusOutline,
    /// FlaskOff 󱏶
    FlaskOff,
    /// FlaskOffOutline 󱏷
    FlaskOffOutline,
    /// FlaskOutline 󰂖
    FlaskOutline,
    /// FlaskPlus 󱉂
    FlaskPlus,
    /// FlaskPlusOutline 󱉃
    FlaskPlusOutline,
    /// FlaskRemove 󱉄
    FlaskRemove,
    /// FlaskRemoveOutline 󱉅
    FlaskRemoveOutline,
    /// FlaskRoundBottom 󱉋
    FlaskRoundBottom,
    /// FlaskRoundBottomEmpty 󱉌
    FlaskRoundBottomEmpty,
    /// FlaskRoundBottomEmptyOutline 󱉍
    FlaskRoundBottomEmptyOutline,
    /// FlaskRoundBottomOutline 󱉎
    FlaskRoundBottomOutline,
    /// Flathub 
    Flathub,
    /// FleurDeLis 󱌃
    FleurDeLis,
    /// Flickr 
    Flickr,
    /// FlipHorizontal 󱃧
    FlipHorizontal,
    /// FlipToBack 󰉇
    FlipToBack,
    /// FlipToFront 󰉈
    FlipToFront,
    /// FlipVertical 󱃨
    FlipVertical,
    /// FloorLamp 󰣝
    FloorLamp,
    /// FloorLampDual 󱁀
    FloorLampDual,
    /// FloorLampDualOutline 󱟎
    FloorLampDualOutline,
    /// FloorLampOutline 󱟈
    FloorLampOutline,
    /// FloorLampTorchiere 󱝇
    FloorLampTorchiere,
    /// FloorLampTorchiereOutline 󱟖
    FloorLampTorchiereOutline,
    /// FloorLampTorchiereVariant 󱁁
    FloorLampTorchiereVariant,
    /// FloorLampTorchiereVariantOutline 󱟏
    FloorLampTorchiereVariantOutline,
    /// FloorPlan 󰠡
    FloorPlan,
    /// Floppy 
    Floppy,
    /// FloppyOne 󰉉
    FloppyOne,
    /// FloppyVariant 󰧯
    FloppyVariant,
    /// Flower 󰉊
    Flower,
    /// FlowerOutline 󰧰
    FlowerOutline,
    /// FlowerPollen 󱢅
    FlowerPollen,
    /// FlowerPollenOutline 󱢆
    FlowerPollenOutline,
    /// FlowerPoppy 󰴈
    FlowerPoppy,
    /// FlowerTulip 󰧱
    FlowerTulip,
    /// FlowerTulipOutline 󰧲
    FlowerTulipOutline,
    /// Fluxbox 
    Fluxbox,
    /// FocusAuto 󰽎
    FocusAuto,
    /// FocusField 󰽏
    FocusField,
    /// FocusFieldHorizontal 󰽐
    FocusFieldHorizontal,
    /// FocusFieldVertical 󰽑
    FocusFieldVertical,
    /// Fold 
    Fold,
    /// FoldDown 
    FoldDown,
    /// FoldDownOne 
    FoldDownOne,
    /// FoldOne 
    FoldOne,
    /// FoldUp 
    FoldUp,
    /// FoldUpOne 
    FoldUpOne,
    /// Folder 
    Folder,
    /// FolderAccount 󰉌
    FolderAccount,
    /// FolderAccountOutline 󰮜
    FolderAccountOutline,
    /// FolderActive 
    FolderActive,
    /// FolderAlert 󰷌
    FolderAlert,
    /// FolderAlertOutline 󰷍
    FolderAlertOutline,
    /// FolderArrowDown 󱧨
    FolderArrowDown,
    /// FolderArrowDownOutline 󱧩
    FolderArrowDownOutline,
    /// FolderArrowLeft 󱧪
    FolderArrowLeft,
    /// FolderArrowLeftOutline 󱧫
    FolderArrowLeftOutline,
    /// FolderArrowLeftRight 󱧬
    FolderArrowLeftRight,
    /// FolderArrowLeftRightOutline 󱧭
    FolderArrowLeftRightOutline,
    /// FolderArrowRight 󱧮
    FolderArrowRight,
    /// FolderArrowRightOutline 󱧯
    FolderArrowRightOutline,
    /// FolderArrowUp 󱧰
    FolderArrowUp,
    /// FolderArrowUpDown 󱧱
    FolderArrowUpDown,
    /// FolderArrowUpDownOutline 󱧲
    FolderArrowUpDownOutline,
    /// FolderArrowUpOutline 󱧳
    FolderArrowUpOutline,
    /// FolderCancel 󱧴
    FolderCancel,
    /// FolderCancelOutline 󱧵
    FolderCancelOutline,
    /// FolderCheck 󱥾
    FolderCheck,
    /// FolderCheckOutline 󱥿
    FolderCheckOutline,
    /// FolderClock 󰪺
    FolderClock,
    /// FolderClockOutline 󰪻
    FolderClockOutline,
    /// FolderClose 
    FolderClose,
    /// FolderCloseAlt 
    FolderCloseAlt,
    /// FolderCog 󱁿
    FolderCog,
    /// FolderCogOutline 󱂀
    FolderCogOutline,
    /// FolderDownload 󰉍
    FolderDownload,
    /// FolderDownloadOutline 󱃩
    FolderDownloadOutline,
    /// FolderEdit 󰣞
    FolderEdit,
    /// FolderEditOutline 󰷎
    FolderEditOutline,
    /// FolderEye 󱞊
    FolderEye,
    /// FolderEyeOutline 󱞋
    FolderEyeOutline,
    /// FolderFile 󱧶
    FolderFile,
    /// FolderFileOutline 󱧷
    FolderFileOutline,
    /// FolderGoogleDrive 󰉎
    FolderGoogleDrive,
    /// FolderHeart 󱃪
    FolderHeart,
    /// FolderHeartOutline 󱃫
    FolderHeartOutline,
    /// FolderHidden 󱞞
    FolderHidden,
    /// FolderHome 󱂵
    FolderHome,
    /// FolderHomeOutline 󱂶
    FolderHomeOutline,
    /// FolderImage 󰉏
    FolderImage,
    /// FolderInformation 󱂷
    FolderInformation,
    /// FolderInformationOutline 󱂸
    FolderInformationOutline,
    /// FolderKey 󰢬
    FolderKey,
    /// FolderKeyNetwork 󰢭
    FolderKeyNetwork,
    /// FolderKeyNetworkOutline 󰲀
    FolderKeyNetworkOutline,
    /// FolderKeyOutline 󱃬
    FolderKeyOutline,
    /// FolderLibrary 
    FolderLibrary,
    /// FolderLock 󰉐
    FolderLock,
    /// FolderLockOpen 󰉑
    FolderLockOpen,
    /// FolderLockOpenOutline 󱪧
    FolderLockOpenOutline,
    /// FolderLockOutline 󱪨
    FolderLockOutline,
    /// FolderMarker 󱉭
    FolderMarker,
    /// FolderMarkerOutline 󱉮
    FolderMarkerOutline,
    /// FolderMove 󰉒
    FolderMove,
    /// FolderMoveOutline 󱉆
    FolderMoveOutline,
    /// FolderMultiple 󰉓
    FolderMultiple,
    /// FolderMultipleImage 󰉔
    FolderMultipleImage,
    /// FolderMultipleOutline 󰉕
    FolderMultipleOutline,
    /// FolderMultiplePlus 󱑾
    FolderMultiplePlus,
    /// FolderMultiplePlusOutline 󱑿
    FolderMultiplePlusOutline,
    /// FolderMusic 󱍙
    FolderMusic,
    /// FolderMusicOutline 󱍚
    FolderMusicOutline,
    /// FolderNetwork 󰡰
    FolderNetwork,
    /// FolderNetworkOutline 󰲁
    FolderNetworkOutline,
    /// FolderOff 󱧸
    FolderOff,
    /// FolderOffOutline 󱧹
    FolderOffOutline,
    /// FolderOne 󰉋
    FolderOne,
    /// FolderOpen 󰝰
    FolderOpen,
    /// FolderOpenAlt 
    FolderOpenAlt,
    /// FolderOpenOutline 󰷏
    FolderOpenOutline,
    /// FolderOpened 
    FolderOpened,
    /// FolderOutline 󰉖
    FolderOutline,
    /// FolderPlay 󱧺
    FolderPlay,
    /// FolderPlayOutline 󱧻
    FolderPlayOutline,
    /// FolderPlus 󰉗
    FolderPlus,
    /// FolderPlusOutline 󰮝
    FolderPlusOutline,
    /// FolderPound 󰴉
    FolderPound,
    /// FolderPoundOutline 󰴊
    FolderPoundOutline,
    /// FolderQuestion 󱧊
    FolderQuestion,
    /// FolderQuestionOutline 󱧋
    FolderQuestionOutline,
    /// FolderRefresh 󰝉
    FolderRefresh,
    /// FolderRefreshOutline 󰕂
    FolderRefreshOutline,
    /// FolderRemove 󰉘
    FolderRemove,
    /// FolderRemoveOutline 󰮞
    FolderRemoveOutline,
    /// FolderSearch 󰥨
    FolderSearch,
    /// FolderSearchOutline 󰥩
    FolderSearchOutline,
    /// FolderSettings 󱁽
    FolderSettings,
    /// FolderSettingsOutline 󱁾
    FolderSettingsOutline,
    /// FolderStar 󰚝
    FolderStar,
    /// FolderStarMultiple 󱏓
    FolderStarMultiple,
    /// FolderStarMultipleOutline 󱏔
    FolderStarMultipleOutline,
    /// FolderStarOutline 󰮟
    FolderStarOutline,
    /// FolderSwap 󰾶
    FolderSwap,
    /// FolderSwapOutline 󰾷
    FolderSwapOutline,
    /// FolderSync 󰴋
    FolderSync,
    /// FolderSyncOutline 󰴌
    FolderSyncOutline,
    /// FolderTable 󱋣
    FolderTable,
    /// FolderTableOutline 󱋤
    FolderTableOutline,
    /// FolderText 󰲂
    FolderText,
    /// FolderTextOutline 󰲃
    FolderTextOutline,
    /// FolderUpload 󰉙
    FolderUpload,
    /// FolderUploadOutline 󱃭
    FolderUploadOutline,
    /// FolderWrench 󱧼
    FolderWrench,
    /// FolderWrenchOutline 󱧽
    FolderWrenchOutline,
    /// FolderZip 󰛫
    FolderZip,
    /// FolderZipOutline 󰞹
    FolderZipOutline,
    /// Foneab 
    Foneab,
    /// Foneafour 
    Foneafour,
    /// Foneaone 
    Foneaone,
    /// Fonefc 
    Fonefc,
    /// Fonefthree 
    Fonefthree,
    /// Fonesevenone 
    Fonesevenone,
    /// Font 
    Font,
    /// FontAwesome 󰀺
    FontAwesome,
    /// Food 
    Food,
    /// FoodApple 󰉛
    FoodApple,
    /// FoodAppleOutline 󰲄
    FoodAppleOutline,
    /// FoodCroissant 󰟈
    FoodCroissant,
    /// FoodDrumstick 󱐟
    FoodDrumstick,
    /// FoodDrumstickOff 󱑨
    FoodDrumstickOff,
    /// FoodDrumstickOffOutline 󱑩
    FoodDrumstickOffOutline,
    /// FoodDrumstickOutline 󱐠
    FoodDrumstickOutline,
    /// FoodForkDrink 󰗲
    FoodForkDrink,
    /// FoodHalal 󱕲
    FoodHalal,
    /// FoodHotDog 󱡋
    FoodHotDog,
    /// FoodKosher 󱕳
    FoodKosher,
    /// FoodOff 󰗳
    FoodOff,
    /// FoodOffOutline 󱤕
    FoodOffOutline,
    /// FoodOne 󰉚
    FoodOne,
    /// FoodOutline 󱤖
    FoodOutline,
    /// FoodSteak 󱑪
    FoodSteak,
    /// FoodSteakOff 󱑫
    FoodSteakOff,
    /// FoodTakeoutBox 󱠶
    FoodTakeoutBox,
    /// FoodTakeoutBoxOutline 󱠷
    FoodTakeoutBoxOutline,
    /// FoodTurkey 󱜜
    FoodTurkey,
    /// FoodVariant 󰉜
    FoodVariant,
    /// FoodVariantOff 󱏥
    FoodVariantOff,
    /// FootPrint 󰽒
    FootPrint,
    /// Football 󰉝
    Football,
    /// FootballAustralian 󰉞
    FootballAustralian,
    /// FootballHelmet 󰉟
    FootballHelmet,
    /// Footprint 
    Footprint,
    /// Forest 󱢗
    Forest,
    /// Forgejo 
    Forgejo,
    /// Forklift 󰟉
    Forklift,
    /// FormDropdown 󱐀
    FormDropdown,
    /// FormSelect 󱐁
    FormSelect,
    /// FormTextarea 󱂕
    FormTextarea,
    /// FormTextbox 󰘎
    FormTextbox,
    /// FormTextboxLock 󱍝
    FormTextboxLock,
    /// FormTextboxPassword 󰟵
    FormTextboxPassword,
    /// FormatAlignBottom 󰝓
    FormatAlignBottom,
    /// FormatAlignCenter 󰉠
    FormatAlignCenter,
    /// FormatAlignJustify 󰉡
    FormatAlignJustify,
    /// FormatAlignLeft 󰉢
    FormatAlignLeft,
    /// FormatAlignMiddle 󰝔
    FormatAlignMiddle,
    /// FormatAlignRight 󰉣
    FormatAlignRight,
    /// FormatAlignTop 󰝕
    FormatAlignTop,
    /// FormatAnnotationMinus 󰪼
    FormatAnnotationMinus,
    /// FormatAnnotationPlus 󰙆
    FormatAnnotationPlus,
    /// FormatBold 󰉤
    FormatBold,
    /// FormatClear 󰉥
    FormatClear,
    /// FormatColorFill 󰉦
    FormatColorFill,
    /// FormatColorHighlight 󰸱
    FormatColorHighlight,
    /// FormatColorMarkerCancel 󱌓
    FormatColorMarkerCancel,
    /// FormatColorText 󰚞
    FormatColorText,
    /// FormatColumns 󰣟
    FormatColumns,
    /// FormatFloatCenter 󰉧
    FormatFloatCenter,
    /// FormatFloatLeft 󰉨
    FormatFloatLeft,
    /// FormatFloatNone 󰉩
    FormatFloatNone,
    /// FormatFloatRight 󰉪
    FormatFloatRight,
    /// FormatFont 󰛖
    FormatFont,
    /// FormatFontSizeDecrease 󰧳
    FormatFontSizeDecrease,
    /// FormatFontSizeIncrease 󰧴
    FormatFontSizeIncrease,
    /// FormatHeaderDecrease 󰉱
    FormatHeaderDecrease,
    /// FormatHeaderEqual 󰉲
    FormatHeaderEqual,
    /// FormatHeaderFive 󰉯
    FormatHeaderFive,
    /// FormatHeaderFour 󰉮
    FormatHeaderFour,
    /// FormatHeaderIncrease 󰉳
    FormatHeaderIncrease,
    /// FormatHeaderOne 󰉫
    FormatHeaderOne,
    /// FormatHeaderPound 󰉴
    FormatHeaderPound,
    /// FormatHeaderSix 󰉰
    FormatHeaderSix,
    /// FormatHeaderThree 󰉭
    FormatHeaderThree,
    /// FormatHeaderTwo 󰉬
    FormatHeaderTwo,
    /// FormatHorizontalAlignCenter 󰘞
    FormatHorizontalAlignCenter,
    /// FormatHorizontalAlignLeft 󰘟
    FormatHorizontalAlignLeft,
    /// FormatHorizontalAlignRight 󰘠
    FormatHorizontalAlignRight,
    /// FormatIndentDecrease 󰉵
    FormatIndentDecrease,
    /// FormatIndentIncrease 󰉶
    FormatIndentIncrease,
    /// FormatItalic 󰉷
    FormatItalic,
    /// FormatLetterCase 󰬴
    FormatLetterCase,
    /// FormatLetterCaseLower 󰬵
    FormatLetterCaseLower,
    /// FormatLetterCaseUpper 󰬶
    FormatLetterCaseUpper,
    /// FormatLetterEndsWith 󰾸
    FormatLetterEndsWith,
    /// FormatLetterMatches 󰾹
    FormatLetterMatches,
    /// FormatLetterSpacing 󱥖
    FormatLetterSpacing,
    /// FormatLetterStartsWith 󰾺
    FormatLetterStartsWith,
    /// FormatLineSpacing 󰉸
    FormatLineSpacing,
    /// FormatLineStyle 󰗈
    FormatLineStyle,
    /// FormatLineWeight 󰗉
    FormatLineWeight,
    /// FormatListBulleted 󰉹
    FormatListBulleted,
    /// FormatListBulletedSquare 󰷐
    FormatListBulletedSquare,
    /// FormatListBulletedTriangle 󰺲
    FormatListBulletedTriangle,
    /// FormatListBulletedType 󰉺
    FormatListBulletedType,
    /// FormatListCheckbox 󰥪
    FormatListCheckbox,
    /// FormatListChecks 󰝖
    FormatListChecks,
    /// FormatListGroup 󱡠
    FormatListGroup,
    /// FormatListNumbered 󰉻
    FormatListNumbered,
    /// FormatListNumberedRtl 󰴍
    FormatListNumberedRtl,
    /// FormatListText 󱉯
    FormatListText,
    /// FormatOverline 󰺳
    FormatOverline,
    /// FormatPageBreak 󰛗
    FormatPageBreak,
    /// FormatPageSplit 󱤗
    FormatPageSplit,
    /// FormatPaint 󰉼
    FormatPaint,
    /// FormatParagraph 󰉽
    FormatParagraph,
    /// FormatPilcrow 󰛘
    FormatPilcrow,
    /// FormatQuoteClose 󰉾
    FormatQuoteClose,
    /// FormatQuoteCloseOutline 󱆨
    FormatQuoteCloseOutline,
    /// FormatQuoteOpen 󰝗
    FormatQuoteOpen,
    /// FormatQuoteOpenOutline 󱆧
    FormatQuoteOpenOutline,
    /// FormatRotateNinezero 󰚪
    FormatRotateNinezero,
    /// FormatSection 󰚟
    FormatSection,
    /// FormatSize 󰉿
    FormatSize,
    /// FormatStrikethrough 󰊀
    FormatStrikethrough,
    /// FormatStrikethroughVariant 󰊁
    FormatStrikethroughVariant,
    /// FormatSubscript 󰊂
    FormatSubscript,
    /// FormatSuperscript 󰊃
    FormatSuperscript,
    /// FormatText 󰊄
    FormatText,
    /// FormatTextRotationAngleDown 󰾻
    FormatTextRotationAngleDown,
    /// FormatTextRotationAngleUp 󰾼
    FormatTextRotationAngleUp,
    /// FormatTextRotationDown 󰵳
    FormatTextRotationDown,
    /// FormatTextRotationDownVertical 󰾽
    FormatTextRotationDownVertical,
    /// FormatTextRotationNone 󰵴
    FormatTextRotationNone,
    /// FormatTextRotationUp 󰾾
    FormatTextRotationUp,
    /// FormatTextRotationVertical 󰾿
    FormatTextRotationVertical,
    /// FormatTextVariant 󰸲
    FormatTextVariant,
    /// FormatTextVariantOutline 󱔏
    FormatTextVariantOutline,
    /// FormatTextWrappingClip 󰴎
    FormatTextWrappingClip,
    /// FormatTextWrappingOverflow 󰴏
    FormatTextWrappingOverflow,
    /// FormatTextWrappingWrap 󰴐
    FormatTextWrappingWrap,
    /// FormatTextbox 󰴑
    FormatTextbox,
    /// FormatTextdirectionLToR 󰊅
    FormatTextdirectionLToR,
    /// FormatTextdirectionRToL 󰊆
    FormatTextdirectionRToL,
    /// FormatTitle 󰗴
    FormatTitle,
    /// FormatUnderline 󰊇
    FormatUnderline,
    /// FormatUnderlineWavy 󱣩
    FormatUnderlineWavy,
    /// FormatVerticalAlignBottom 󰘡
    FormatVerticalAlignBottom,
    /// FormatVerticalAlignCenter 󰘢
    FormatVerticalAlignCenter,
    /// FormatVerticalAlignTop 󰘣
    FormatVerticalAlignTop,
    /// FormatWrapInline 󰊈
    FormatWrapInline,
    /// FormatWrapSquare 󰊉
    FormatWrapSquare,
    /// FormatWrapTight 󰊊
    FormatWrapTight,
    /// FormatWrapTopBottom 󰊋
    FormatWrapTopBottom,
    /// Forum 󰊌
    Forum,
    /// ForumMinus 󱪩
    ForumMinus,
    /// ForumMinusOutline 󱪪
    ForumMinusOutline,
    /// ForumOutline 󰠢
    ForumOutline,
    /// ForumPlus 󱪫
    ForumPlus,
    /// ForumPlusOutline 󱪬
    ForumPlusOutline,
    /// ForumRemove 󱪭
    ForumRemove,
    /// ForumRemoveOutline 󱪮
    ForumRemoveOutline,
    /// Forward 
    Forward,
    /// ForwardOne 󰊍
    ForwardOne,
    /// Forwardburger 󰵵
    Forwardburger,
    /// Fosdem 
    Fosdem,
    /// Fountain 󰥫
    Fountain,
    /// FountainPen 󰴒
    FountainPen,
    /// FountainPenTip 󰴓
    FountainPenTip,
    /// Foureighteight 
    Foureighteight,
    /// Foureightfive 
    Foureightfive,
    /// Foureightfour 
    Foureightfour,
    /// Foureightnine 
    Foureightnine,
    /// Foureightone 
    Foureightone,
    /// Foureightseven 
    Foureightseven,
    /// Foureightsix 
    Foureightsix,
    /// Foureightthree 
    Foureightthree,
    /// Foureighttwo 
    Foureighttwo,
    /// Foureightzero 
    Foureightzero,
    /// Fourfiveeight 
    Fourfiveeight,
    /// Fourfivefive 
    Fourfivefive,
    /// Fourfivefour 
    Fourfivefour,
    /// Fourfivenine 
    Fourfivenine,
    /// Fourfiveone 
    Fourfiveone,
    /// Fourfiveseven 
    Fourfiveseven,
    /// Fourfivesix 
    Fourfivesix,
    /// Fourfivethree 
    Fourfivethree,
    /// Fourfivetwo 
    Fourfivetwo,
    /// Fourfoureight 
    Fourfoureight,
    /// Fourfourfive 
    Fourfourfive,
    /// Fourfourfour 
    Fourfourfour,
    /// Fourfournine 
    Fourfournine,
    /// Fourfourseven 
    Fourfourseven,
    /// Fourfoursix 
    Fourfoursix,
    /// Fourfourthree 
    Fourfourthree,
    /// Fournineeight 
    Fournineeight,
    /// Fourninefour 
    Fourninefour,
    /// Fourninenine 
    Fourninenine,
    /// Fournineone 
    Fournineone,
    /// Fourninesix 
    Fourninesix,
    /// Fourninethree 
    Fourninethree,
    /// Fourninetwo 
    Fourninetwo,
    /// Fourninezero 
    Fourninezero,
    /// Fouroneeight 
    Fouroneeight,
    /// Fouronefive 
    Fouronefive,
    /// Fouronefour 
    Fouronefour,
    /// Fouronenine 
    Fouronenine,
    /// Fouroneone 
    Fouroneone,
    /// Fouroneseven 
    Fouroneseven,
    /// Fouronesix 
    Fouronesix,
    /// Fouronethree 
    Fouronethree,
    /// Fouronetwo 
    Fouronetwo,
    /// Fouronezero 
    Fouronezero,
    /// Fourseveneight 
    Fourseveneight,
    /// Foursevenfive 
    Foursevenfive,
    /// Foursevenfour 
    Foursevenfour,
    /// Foursevennine 
    Foursevennine,
    /// Foursevenone 
    Foursevenone,
    /// Foursevensix 
    Foursevensix,
    /// Fourseventhree 
    Fourseventhree,
    /// Fourseventwo 
    Fourseventwo,
    /// Foursevenzero 
    Foursevenzero,
    /// Foursixfour 
    Foursixfour,
    /// Foursixnine 
    Foursixnine,
    /// Foursixone 
    Foursixone,
    /// Foursixseven 
    Foursixseven,
    /// Foursixsix 
    Foursixsix,
    /// Foursixthree 
    Foursixthree,
    /// Foursixtwo 
    Foursixtwo,
    /// Foursixzero 
    Foursixzero,
    /// Foursquare 
    Foursquare,
    /// Fourthreeeight 
    Fourthreeeight,
    /// Fourthreefour 
    Fourthreefour,
    /// Fourthreenine 
    Fourthreenine,
    /// Fourthreeone 
    Fourthreeone,
    /// Fourthreethree 
    Fourthreethree,
    /// Fourthreetwo 
    Fourthreetwo,
    /// Fourthreezero 
    Fourthreezero,
    /// Fourtwoeight 
    Fourtwoeight,
    /// Fourtwofive 
    Fourtwofive,
    /// Fourtwofour 
    Fourtwofour,
    /// Fourtwonine 
    Fourtwonine,
    /// Fourtwoseven 
    Fourtwoseven,
    /// Fourtwosix 
    Fourtwosix,
    /// Fourtwothree 
    Fourtwothree,
    /// Fourtwotwo 
    Fourtwotwo,
    /// Fourzeroeight 
    Fourzeroeight,
    /// Fourzerofour 
    Fourzerofour,
    /// Fourzeronine 
    Fourzeronine,
    /// Fourzeroseven 
    Fourzeroseven,
    /// Fourzerosix 
    Fourzerosix,
    /// Fourzerothree 
    Fourzerothree,
    /// Fourzerotwo 
    Fourzerotwo,
    /// Fourzerozero 
    Fourzerozero,
    /// FractionOneHalf 󱦒
    FractionOneHalf,
    /// Freebsd 󰣠
    Freebsd,
    /// Freecad 
    Freecad,
    /// Freecodecamp 
    Freecodecamp,
    /// FreedesktopOrg 
    FreedesktopOrg,
    /// FrenchFries 󱥗
    FrenchFries,
    /// FrequentlyAskedQuestions 󰺴
    FrequentlyAskedQuestions,
    /// Fridge 󰊐
    Fridge,
    /// FridgeAlert 󱆱
    FridgeAlert,
    /// FridgeAlertOutline 󱆲
    FridgeAlertOutline,
    /// FridgeBottom 󰊒
    FridgeBottom,
    /// FridgeIndustrial 󱗮
    FridgeIndustrial,
    /// FridgeIndustrialAlert 󱗯
    FridgeIndustrialAlert,
    /// FridgeIndustrialAlertOutline 󱗰
    FridgeIndustrialAlertOutline,
    /// FridgeIndustrialOff 󱗱
    FridgeIndustrialOff,
    /// FridgeIndustrialOffOutline 󱗲
    FridgeIndustrialOffOutline,
    /// FridgeIndustrialOutline 󱗳
    FridgeIndustrialOutline,
    /// FridgeOff 󱆯
    FridgeOff,
    /// FridgeOffOutline 󱆰
    FridgeOffOutline,
    /// FridgeOutline 󰊏
    FridgeOutline,
    /// FridgeTop 󰊑
    FridgeTop,
    /// FridgeVariant 󱗴
    FridgeVariant,
    /// FridgeVariantAlert 󱗵
    FridgeVariantAlert,
    /// FridgeVariantAlertOutline 󱗶
    FridgeVariantAlertOutline,
    /// FridgeVariantOff 󱗷
    FridgeVariantOff,
    /// FridgeVariantOffOutline 󱗸
    FridgeVariantOffOutline,
    /// FridgeVariantOutline 󱗹
    FridgeVariantOutline,
    /// Frown 
    Frown,
    /// FruitCherries 󱁂
    FruitCherries,
    /// FruitCherriesOff 󱏸
    FruitCherriesOff,
    /// FruitCitrus 󱁃
    FruitCitrus,
    /// FruitCitrusOff 󱏹
    FruitCitrusOff,
    /// FruitGrapes 󱁄
    FruitGrapes,
    /// FruitGrapesOutline 󱁅
    FruitGrapesOutline,
    /// FruitPear 󱨎
    FruitPear,
    /// FruitPineapple 󱁆
    FruitPineapple,
    /// FruitWatermelon 󱁇
    FruitWatermelon,
    /// Ftwoonetwo 
    Ftwoonetwo,
    /// Ftwoonezero 
    Ftwoonezero,
    /// Ftwosevene 
    Ftwosevene,
    /// Ftwosixone 
    Ftwosixone,
    /// Ftwosixthree 
    Ftwosixthree,
    /// Ftwosixzero 
    Ftwosixzero,
    /// Fuel 󰟊
    Fuel,
    /// FuelCell 󱢵
    FuelCell,
    /// Fullscreen 
    Fullscreen,
    /// FullscreenExit 󰊔
    FullscreenExit,
    /// FullscreenOne 󰊓
    FullscreenOne,
    /// Function 󰊕
    Function,
    /// FunctionVariant 󰡱
    FunctionVariant,
    /// FuriganaHorizontal 󱂁
    FuriganaHorizontal,
    /// FuriganaVertical 󱂂
    FuriganaVertical,
    /// Fuse 󰲅
    Fuse,
    /// FuseAlert 󱐭
    FuseAlert,
    /// FuseBlade 󰲆
    FuseBlade,
    /// FuseOff 󱐬
    FuseOff,
    /// Fzerofe 
    Fzerofe,
    /// Galaxy 
    Galaxy,
    /// Galery 
    Galery,
    /// Gamepad 
    Gamepad,
    /// GamepadCircle 󰸳
    GamepadCircle,
    /// GamepadCircleDown 󰸴
    GamepadCircleDown,
    /// GamepadCircleLeft 󰸵
    GamepadCircleLeft,
    /// GamepadCircleOutline 󰸶
    GamepadCircleOutline,
    /// GamepadCircleRight 󰸷
    GamepadCircleRight,
    /// GamepadCircleUp 󰸸
    GamepadCircleUp,
    /// GamepadDown 󰸹
    GamepadDown,
    /// GamepadLeft 󰸺
    GamepadLeft,
    /// GamepadOne 󰊖
    GamepadOne,
    /// GamepadOutline 󱤙
    GamepadOutline,
    /// GamepadRight 󰸻
    GamepadRight,
    /// GamepadRound 󰸼
    GamepadRound,
    /// GamepadRoundDown 󰸽
    GamepadRoundDown,
    /// GamepadRoundLeft 󰸾
    GamepadRoundLeft,
    /// GamepadRoundOutline 󰸿
    GamepadRoundOutline,
    /// GamepadRoundRight 󰹀
    GamepadRoundRight,
    /// GamepadRoundUp 󰹁
    GamepadRoundUp,
    /// GamepadSquare 󰺵
    GamepadSquare,
    /// GamepadSquareOutline 󰺶
    GamepadSquareOutline,
    /// GamepadUp 󰹂
    GamepadUp,
    /// GamepadVariant 󰊗
    GamepadVariant,
    /// GamepadVariantOutline 󰺷
    GamepadVariantOutline,
    /// Gamma 󱃮
    Gamma,
    /// GantryCrane 󰷑
    GantryCrane,
    /// Garage 󰛙
    Garage,
    /// GarageAlert 󰡲
    GarageAlert,
    /// GarageAlertVariant 󱋕
    GarageAlertVariant,
    /// GarageLock 󱟻
    GarageLock,
    /// GarageOpen 󰛚
    GarageOpen,
    /// GarageOpenVariant 󱋔
    GarageOpenVariant,
    /// GarageVariant 󱋓
    GarageVariant,
    /// GarageVariantLock 󱟼
    GarageVariantLock,
    /// GarudaLinux 
    GarudaLinux,
    /// GasBurner 󱨛
    GasBurner,
    /// GasCylinder 󰙇
    GasCylinder,
    /// GasStation 󰊘
    GasStation,
    /// GasStationOff 󱐉
    GasStationOff,
    /// GasStationOffOutline 󱐊
    GasStationOffOutline,
    /// GasStationOutline 󰺸
    GasStationOutline,
    /// Gate 󰊙
    Gate,
    /// GateAlert 󱟸
    GateAlert,
    /// GateAnd 󰣡
    GateAnd,
    /// GateArrowLeft 󱟷
    GateArrowLeft,
    /// GateArrowRight 󱅩
    GateArrowRight,
    /// GateNand 󰣢
    GateNand,
    /// GateNor 󰣣
    GateNor,
    /// GateNot 󰣤
    GateNot,
    /// GateOpen 󱅪
    GateOpen,
    /// GateOr 󰣥
    GateOr,
    /// GateXnor 󰣦
    GateXnor,
    /// GateXor 󰣧
    GateXor,
    /// Gatsby 󰹃
    Gatsby,
    /// Gauge 󰊚
    Gauge,
    /// GaugeEmpty 󰡳
    GaugeEmpty,
    /// GaugeFull 󰡴
    GaugeFull,
    /// GaugeLow 󰡵
    GaugeLow,
    /// Gavel 󰊛
    Gavel,
    /// Gbp 
    Gbp,
    /// Gear 
    Gear,
    /// GearOne 
    GearOne,
    /// GenderFemale 󰊜
    GenderFemale,
    /// GenderMale 󰊝
    GenderMale,
    /// GenderMaleFemale 󰊞
    GenderMaleFemale,
    /// GenderMaleFemaleVariant 󱄿
    GenderMaleFemaleVariant,
    /// GenderNonBinary 󱅀
    GenderNonBinary,
    /// GenderTransgender 󰊟
    GenderTransgender,
    /// Gentoo 󰣨
    Gentoo,
    /// Gesture 󰟋
    Gesture,
    /// GestureDoubleTap 󰜼
    GestureDoubleTap,
    /// GesturePinch 󰪽
    GesturePinch,
    /// GestureSpread 󰪾
    GestureSpread,
    /// GestureSwipe 󰵶
    GestureSwipe,
    /// GestureSwipeDown 󰜽
    GestureSwipeDown,
    /// GestureSwipeHorizontal 󰪿
    GestureSwipeHorizontal,
    /// GestureSwipeLeft 󰜾
    GestureSwipeLeft,
    /// GestureSwipeRight 󰜿
    GestureSwipeRight,
    /// GestureSwipeUp 󰝀
    GestureSwipeUp,
    /// GestureSwipeVertical 󰫀
    GestureSwipeVertical,
    /// GestureTap 󰝁
    GestureTap,
    /// GestureTapBox 󱊩
    GestureTapBox,
    /// GestureTapButton 󱊨
    GestureTapButton,
    /// GestureTapHold 󰵷
    GestureTapHold,
    /// GestureTwoDoubleTap 󰝂
    GestureTwoDoubleTap,
    /// GestureTwoTap 󰝃
    GestureTwoTap,
    /// Ghost 󰊠
    Ghost,
    /// GhostOff 󰧵
    GhostOff,
    /// GhostOffOutline 󱙜
    GhostOffOutline,
    /// GhostOutline 󱙝
    GhostOutline,
    /// Gift 
    Gift,
    /// GiftCard 
    GiftCard,
    /// GiftOff 󱛯
    GiftOff,
    /// GiftOffOutline 󱛰
    GiftOffOutline,
    /// GiftOne 
    GiftOne,
    /// GiftOpen 󱛱
    GiftOpen,
    /// GiftOpenOutline 󱛲
    GiftOpenOutline,
    /// GiftOutline 󰊡
    GiftOutline,
    /// GiftThree 󰹄
    GiftThree,
    /// GiftTwo 
    GiftTwo,
    /// Gimp 
    Gimp,
    /// GistSecret 
    GistSecret,
    /// Git 󰊢
    Git,
    /// GitBranch 
    GitBranch,
    /// GitCommit 
    GitCommit,
    /// GitCommitOne 
    GitCommitOne,
    /// GitCompare 
    GitCompare,
    /// GitCompareOne 
    GitCompareOne,
    /// GitMerge 
    GitMerge,
    /// GitMergeOne 
    GitMergeOne,
    /// GitMergeQueue 
    GitMergeQueue,
    /// GitPullRequest 
    GitPullRequest,
    /// GitPullRequestClosed 
    GitPullRequestClosed,
    /// GitPullRequestClosedOne 
    GitPullRequestClosedOne,
    /// GitPullRequestCreate 
    GitPullRequestCreate,
    /// GitPullRequestDraft 
    GitPullRequestDraft,
    /// GitPullRequestDraftOne 
    GitPullRequestDraftOne,
    /// GitPullRequestOne 
    GitPullRequestOne,
    /// Gitea 
    Gitea,
    /// Github 
    Github,
    /// GithubAction 
    GithubAction,
    /// GithubAlt 
    GithubAlt,
    /// GithubInverted 
    GithubInverted,
    /// GithubOne 
    GithubOne,
    /// GithubSign 
    GithubSign,
    /// GithubTwo 󰊤
    GithubTwo,
    /// Gitlab 󰮠
    Gitlab,
    /// Gittip 
    Gittip,
    /// Glass 
    Glass,
    /// GlassCocktail 󰍖
    GlassCocktail,
    /// GlassCocktailOff 󱗦
    GlassCocktailOff,
    /// GlassFlute 󰊥
    GlassFlute,
    /// GlassFragile 󱡳
    GlassFragile,
    /// GlassMug 󰊦
    GlassMug,
    /// GlassMugOff 󱗧
    GlassMugOff,
    /// GlassMugVariant 󱄖
    GlassMugVariant,
    /// GlassMugVariantOff 󱗨
    GlassMugVariantOff,
    /// GlassOne 
    GlassOne,
    /// GlassPintOutline 󱌍
    GlassPintOutline,
    /// GlassStange 󰊧
    GlassStange,
    /// GlassTulip 󰊨
    GlassTulip,
    /// GlassWine 󰡶
    GlassWine,
    /// Glasses 󰊪
    Glasses,
    /// Globe 
    Globe,
    /// GlobeLight 󱋗
    GlobeLight,
    /// GlobeModel 󰣩
    GlobeModel,
    /// GlobeOne 
    GlobeOne,
    /// GlobeTwo 
    GlobeTwo,
    /// Gmail 󰊫
    Gmail,
    /// Gnome 󰊬
    Gnome,
    /// GnuGuix 
    GnuGuix,
    /// GoKart 󰵹
    GoKart,
    /// GoKartTrack 󰵺
    GoKartTrack,
    /// GoToFile 
    GoToFile,
    /// Goal 
    Goal,
    /// Gog 󰮡
    Gog,
    /// Gold 󱉏
    Gold,
    /// Golf 󰠣
    Golf,
    /// GolfCart 󱆤
    GolfCart,
    /// GolfTee 󱂃
    GolfTee,
    /// Gondola 󰚆
    Gondola,
    /// Goodreads 󰵻
    Goodreads,
    /// Google 󰊭
    Google,
    /// GoogleAds 󰲇
    GoogleAds,
    /// GoogleAnalytics 󰟌
    GoogleAnalytics,
    /// GoogleAssistant 󰟍
    GoogleAssistant,
    /// GoogleCardboard 󰊮
    GoogleCardboard,
    /// GoogleChrome 󰊯
    GoogleChrome,
    /// GoogleCircles 󰊰
    GoogleCircles,
    /// GoogleCirclesCommunities 󰊱
    GoogleCirclesCommunities,
    /// GoogleCirclesExtended 󰊲
    GoogleCirclesExtended,
    /// GoogleCirclesGroup 󰊳
    GoogleCirclesGroup,
    /// GoogleClassroom 󰋀
    GoogleClassroom,
    /// GoogleCloud 󱇶
    GoogleCloud,
    /// GoogleController 󰊴
    GoogleController,
    /// GoogleControllerOff 󰊵
    GoogleControllerOff,
    /// GoogleDownasaur 󱍢
    GoogleDownasaur,
    /// GoogleDrive 
    GoogleDrive,
    /// GoogleDriveOne 󰊶
    GoogleDriveOne,
    /// GoogleEarth 󰊷
    GoogleEarth,
    /// GoogleFit 󰥬
    GoogleFit,
    /// GoogleGlass 󰊸
    GoogleGlass,
    /// GoogleHangouts 󰋉
    GoogleHangouts,
    /// GoogleHome 󰠤
    GoogleHome,
    /// GoogleKeep 󰛜
    GoogleKeep,
    /// GoogleLens 󰧶
    GoogleLens,
    /// GoogleMaps 󰗵
    GoogleMaps,
    /// GoogleMyBusiness 󱁈
    GoogleMyBusiness,
    /// GoogleNearby 󰊹
    GoogleNearby,
    /// GooglePlay 
    GooglePlay,
    /// GooglePlayOne 󰊼
    GooglePlayOne,
    /// GooglePlus 󰊽
    GooglePlus,
    /// GooglePlusSign 
    GooglePlusSign,
    /// GooglePodcast 󰺹
    GooglePodcast,
    /// GoogleSpreadsheet 󰧷
    GoogleSpreadsheet,
    /// GoogleStreetView 󰲈
    GoogleStreetView,
    /// GoogleTranslate 󰊿
    GoogleTranslate,
    /// Gps 
    Gps,
    /// Grabber 
    Grabber,
    /// GrabberOne 
    GrabberOne,
    /// GradientHorizontal 󱝊
    GradientHorizontal,
    /// GradientVertical 󰚠
    GradientVertical,
    /// Grain 󰵼
    Grain,
    /// Graph 
    Graph,
    /// GraphLeft 
    GraphLeft,
    /// GraphLine 
    GraphLine,
    /// GraphOne 
    GraphOne,
    /// GraphOutline 󱁊
    GraphOutline,
    /// GraphScatter 
    GraphScatter,
    /// GraphTwo 󱁉
    GraphTwo,
    /// Graphql 󰡷
    Graphql,
    /// Grass 󱔐
    Grass,
    /// Grav 
    Grav,
    /// GraveStone 󰮢
    GraveStone,
    /// GreasePencil 󰙈
    GreasePencil,
    /// GreaterThan 󰥭
    GreaterThan,
    /// GreaterThanOrEqual 󰥮
    GreaterThanOrEqual,
    /// Greenhouse 󰀭
    Greenhouse,
    /// Grid 󰋁
    Grid,
    /// GridLarge 󰝘
    GridLarge,
    /// GridOff 󰋂
    GridOff,
    /// Grill 󰹅
    Grill,
    /// GrillOutline 󱆊
    GrillOutline,
    /// Gripper 
    Gripper,
    /// Group 
    Group,
    /// GroupByRefType 
    GroupByRefType,
    /// GroupOne 󰋃
    GroupOne,
    /// Gtk 
    Gtk,
    /// Guitar 
    Guitar,
    /// GuitarAcoustic 󰝱
    GuitarAcoustic,
    /// GuitarElectric 󰋄
    GuitarElectric,
    /// GuitarPick 󰋅
    GuitarPick,
    /// GuitarPickOutline 󰋆
    GuitarPickOutline,
    /// Gut 
    Gut,
    /// GuyFawkesMask 󰠥
    GuyFawkesMask,
    /// Gymnastics 󱩁
    Gymnastics,
    /// HSign 
    HSign,
    /// Hail 󰫁
    Hail,
    /// HairDryer 󱃯
    HairDryer,
    /// HairDryerOutline 󱃰
    HairDryerOutline,
    /// Halloween 󰮣
    Halloween,
    /// Halter 
    Halter,
    /// Hamburger 
    Hamburger,
    /// HamburgerCheck 󱝶
    HamburgerCheck,
    /// HamburgerMinus 󱝷
    HamburgerMinus,
    /// HamburgerOff 󱝸
    HamburgerOff,
    /// HamburgerOne 󰚅
    HamburgerOne,
    /// HamburgerPlus 󱝹
    HamburgerPlus,
    /// HamburgerRemove 󱝺
    HamburgerRemove,
    /// Hammer 󰣪
    Hammer,
    /// HammerScrewdriver 󱌢
    HammerScrewdriver,
    /// HammerSickle 󱢇
    HammerSickle,
    /// HammerWrench 󱌣
    HammerWrench,
    /// HandBackLeft 󰹆
    HandBackLeft,
    /// HandBackLeftOff 󱠰
    HandBackLeftOff,
    /// HandBackLeftOffOutline 󱠲
    HandBackLeftOffOutline,
    /// HandBackLeftOutline 󱠬
    HandBackLeftOutline,
    /// HandBackRight 󰹇
    HandBackRight,
    /// HandBackRightOff 󱠱
    HandBackRightOff,
    /// HandBackRightOffOutline 󱠳
    HandBackRightOffOutline,
    /// HandBackRightOutline 󱠭
    HandBackRightOutline,
    /// HandClap 󱥋
    HandClap,
    /// HandClapOff 󱩂
    HandClapOff,
    /// HandCoin 󱢏
    HandCoin,
    /// HandCoinOutline 󱢐
    HandCoinOutline,
    /// HandDown 
    HandDown,
    /// HandExtended 󱢶
    HandExtended,
    /// HandExtendedOutline 󱢷
    HandExtendedOutline,
    /// HandFrontLeft 󱠫
    HandFrontLeft,
    /// HandFrontLeftOutline 󱠮
    HandFrontLeftOutline,
    /// HandFrontRight 󰩏
    HandFrontRight,
    /// HandFrontRightOutline 󱠯
    HandFrontRightOutline,
    /// HandHeart 󱃱
    HandHeart,
    /// HandHeartOutline 󱕾
    HandHeartOutline,
    /// HandLeft 
    HandLeft,
    /// HandOkay 󰩐
    HandOkay,
    /// HandPeace 󰩑
    HandPeace,
    /// HandPeaceVariant 󰩒
    HandPeaceVariant,
    /// HandPointingDown 󰩓
    HandPointingDown,
    /// HandPointingLeft 󰩔
    HandPointingLeft,
    /// HandPointingRight 󰋇
    HandPointingRight,
    /// HandPointingUp 󰩕
    HandPointingUp,
    /// HandRight 
    HandRight,
    /// HandSaw 󰹈
    HandSaw,
    /// HandUp 
    HandUp,
    /// HandWash 󱕿
    HandWash,
    /// HandWashOutline 󱖀
    HandWashOutline,
    /// HandWater 󱎟
    HandWater,
    /// HandWave 󱠡
    HandWave,
    /// HandWaveOutline 󱠢
    HandWaveOutline,
    /// Handball 󰽓
    Handball,
    /// Handcuffs 󱄾
    Handcuffs,
    /// HandsPray 󰕹
    HandsPray,
    /// Handshake 󱈘
    Handshake,
    /// HandshakeOutline 󱖡
    HandshakeOutline,
    /// Hanger 󰋈
    Hanger,
    /// HardHat 󰥯
    HardHat,
    /// Harddisk 󰋊
    Harddisk,
    /// HarddiskPlus 󱁋
    HarddiskPlus,
    /// HarddiskRemove 󱁌
    HarddiskRemove,
    /// Hash 
    Hash,
    /// Hat 
    Hat,
    /// HatFedora 󰮤
    HatFedora,
    /// HazardLights 󰲉
    HazardLights,
    /// Hdd 
    Hdd,
    /// Hdr 󰵽
    Hdr,
    /// HdrOff 󰵾
    HdrOff,
    /// Head 󱍞
    Head,
    /// HeadAlert 󱌸
    HeadAlert,
    /// HeadAlertOutline 󱌹
    HeadAlertOutline,
    /// HeadCheck 󱌺
    HeadCheck,
    /// HeadCheckOutline 󱌻
    HeadCheckOutline,
    /// HeadCog 󱌼
    HeadCog,
    /// HeadCogOutline 󱌽
    HeadCogOutline,
    /// HeadDotsHorizontal 󱌾
    HeadDotsHorizontal,
    /// HeadDotsHorizontalOutline 󱌿
    HeadDotsHorizontalOutline,
    /// HeadFlash 󱍀
    HeadFlash,
    /// HeadFlashOutline 󱍁
    HeadFlashOutline,
    /// HeadHeart 󱍂
    HeadHeart,
    /// HeadHeartOutline 󱍃
    HeadHeartOutline,
    /// HeadLightbulb 󱍄
    HeadLightbulb,
    /// HeadLightbulbOutline 󱍅
    HeadLightbulbOutline,
    /// HeadMinus 󱍆
    HeadMinus,
    /// HeadMinusOutline 󱍇
    HeadMinusOutline,
    /// HeadOutline 󱍟
    HeadOutline,
    /// HeadPlus 󱍈
    HeadPlus,
    /// HeadPlusOutline 󱍉
    HeadPlusOutline,
    /// HeadQuestion 󱍊
    HeadQuestion,
    /// HeadQuestionOutline 󱍋
    HeadQuestionOutline,
    /// HeadRemove 󱍌
    HeadRemove,
    /// HeadRemoveOutline 󱍍
    HeadRemoveOutline,
    /// HeadSnowflake 󱍎
    HeadSnowflake,
    /// HeadSnowflakeOutline 󱍏
    HeadSnowflakeOutline,
    /// HeadSync 󱍐
    HeadSync,
    /// HeadSyncOutline 󱍑
    HeadSyncOutline,
    /// Heading 
    Heading,
    /// Headphones 
    Headphones,
    /// HeadphonesBluetooth 󰥰
    HeadphonesBluetooth,
    /// HeadphonesBox 󰋌
    HeadphonesBox,
    /// HeadphonesOff 󰟎
    HeadphonesOff,
    /// HeadphonesOne 󰋋
    HeadphonesOne,
    /// HeadphonesSettings 󰋍
    HeadphonesSettings,
    /// Headset 󰋎
    Headset,
    /// HeadsetDock 󰋏
    HeadsetDock,
    /// HeadsetOff 󰋐
    HeadsetOff,
    /// Heart ♥
    Heart,
    /// HeartBox 󰋒
    HeartBox,
    /// HeartBoxOutline 󰋓
    HeartBoxOutline,
    /// HeartBroken 󰋔
    HeartBroken,
    /// HeartBrokenOutline 󰴔
    HeartBrokenOutline,
    /// HeartCircle 󰥱
    HeartCircle,
    /// HeartCircleOutline 󰥲
    HeartCircleOutline,
    /// HeartCog 󱙣
    HeartCog,
    /// HeartCogOutline 󱙤
    HeartCogOutline,
    /// HeartEmpty 
    HeartEmpty,
    /// HeartFill 
    HeartFill,
    /// HeartFlash 󰻹
    HeartFlash,
    /// HeartFour 󰣐
    HeartFour,
    /// HeartHalf 󰛟
    HeartHalf,
    /// HeartHalfFull 󰛞
    HeartHalfFull,
    /// HeartHalfOutline 󰛠
    HeartHalfOutline,
    /// HeartMinus 󱐯
    HeartMinus,
    /// HeartMinusOutline 󱐲
    HeartMinusOutline,
    /// HeartMultiple 󰩖
    HeartMultiple,
    /// HeartMultipleOutline 󰩗
    HeartMultipleOutline,
    /// HeartOff 󰝙
    HeartOff,
    /// HeartOffOutline 󱐴
    HeartOffOutline,
    /// HeartOne 
    HeartOne,
    /// HeartOutline 󰋕
    HeartOutline,
    /// HeartOutlineOne 󱢠
    HeartOutlineOne,
    /// HeartPlus 󱐮
    HeartPlus,
    /// HeartPlusOutline 󱐱
    HeartPlusOutline,
    /// HeartPulse 󰗶
    HeartPulse,
    /// HeartRemove 󱐰
    HeartRemove,
    /// HeartRemoveOutline 󱐳
    HeartRemoveOutline,
    /// HeartSettings 󱙥
    HeartSettings,
    /// HeartSettingsOutline 󱙦
    HeartSettingsOutline,
    /// HeartThree 󰋑
    HeartThree,
    /// HeartTwo 
    HeartTwo,
    /// HeatPump 󱩃
    HeatPump,
    /// HeatPumpOutline 󱩄
    HeatPumpOutline,
    /// HeatWave 󱩅
    HeatWave,
    /// HeatingCoil 󱪯
    HeatingCoil,
    /// HeavyCircle ⭘
    HeavyCircle,
    /// Helicopter 󰫂
    Helicopter,
    /// Help 󰋖
    Help,
    /// HelpBox 󰞋
    HelpBox,
    /// HelpCircle 󰋗
    HelpCircle,
    /// HelpCircleOutline 󰘥
    HelpCircleOutline,
    /// HelpNetwork 󰛵
    HelpNetwork,
    /// HelpNetworkOutline 󰲊
    HelpNetworkOutline,
    /// HelpRhombus 󰮥
    HelpRhombus,
    /// HelpRhombusOutline 󰮦
    HelpRhombusOutline,
    /// Hexadecimal 󱊧
    Hexadecimal,
    /// Hexagon 
    Hexagon,
    /// HexagonMultiple 󰛡
    HexagonMultiple,
    /// HexagonMultipleOutline 󱃲
    HexagonMultipleOutline,
    /// HexagonOne 󰋘
    HexagonOne,
    /// HexagonOutline 󰋙
    HexagonOutline,
    /// HexagonSliceFive 󰫇
    HexagonSliceFive,
    /// HexagonSliceFour 󰫆
    HexagonSliceFour,
    /// HexagonSliceOne 󰫃
    HexagonSliceOne,
    /// HexagonSliceSix 󰫈
    HexagonSliceSix,
    /// HexagonSliceThree 󰫅
    HexagonSliceThree,
    /// HexagonSliceTwo 󰫄
    HexagonSliceTwo,
    /// Hexagram 󰫉
    Hexagram,
    /// HexagramOutline 󰫊
    HexagramOutline,
    /// HighDefinition 󰟏
    HighDefinition,
    /// HighDefinitionBox 󰡸
    HighDefinitionBox,
    /// HighHeel 
    HighHeel,
    /// Highway 󰗷
    Highway,
    /// Hiking 󰵿
    Hiking,
    /// History 
    History,
    /// HistoryOne 
    HistoryOne,
    /// HistoryTwo 󰋚
    HistoryTwo,
    /// HockeyPuck 󰡹
    HockeyPuck,
    /// HockeySticks 󰡺
    HockeySticks,
    /// Hololens 󰋛
    Hololens,
    /// Home 
    Home,
    /// HomeAccount 󰠦
    HomeAccount,
    /// HomeAlert 󰡻
    HomeAlert,
    /// HomeAlertOutline 󱗐
    HomeAlertOutline,
    /// HomeAnalytics 󰺺
    HomeAnalytics,
    /// HomeAssistant 󰟐
    HomeAssistant,
    /// HomeAutomation 󰟑
    HomeAutomation,
    /// HomeBattery 󱤁
    HomeBattery,
    /// HomeBatteryOutline 󱤂
    HomeBatteryOutline,
    /// HomeCircle 󰟒
    HomeCircle,
    /// HomeCircleOutline 󱁍
    HomeCircleOutline,
    /// HomeCity 󰴕
    HomeCity,
    /// HomeCityOutline 󰴖
    HomeCityOutline,
    /// HomeClock 󱨒
    HomeClock,
    /// HomeClockOutline 󱨓
    HomeClockOutline,
    /// HomeEdit 󱅙
    HomeEdit,
    /// HomeEditOutline 󱅚
    HomeEditOutline,
    /// HomeExportOutline 󰾛
    HomeExportOutline,
    /// HomeFill 
    HomeFill,
    /// HomeFlood 󰻺
    HomeFlood,
    /// HomeFloorA 󰶃
    HomeFloorA,
    /// HomeFloorB 󰶄
    HomeFloorB,
    /// HomeFloorG 󰶅
    HomeFloorG,
    /// HomeFloorL 󰶆
    HomeFloorL,
    /// HomeFloorNegativeOne 󰷓
    HomeFloorNegativeOne,
    /// HomeFloorOne 󰶀
    HomeFloorOne,
    /// HomeFloorThree 󰶂
    HomeFloorThree,
    /// HomeFloorTwo 󰶁
    HomeFloorTwo,
    /// HomeFloorZero 󰷒
    HomeFloorZero,
    /// HomeGroup 󰷔
    HomeGroup,
    /// HomeGroupMinus 󱧁
    HomeGroupMinus,
    /// HomeGroupPlus 󱧀
    HomeGroupPlus,
    /// HomeGroupRemove 󱧂
    HomeGroupRemove,
    /// HomeHeart 󰠧
    HomeHeart,
    /// HomeImportOutline 󰾜
    HomeImportOutline,
    /// HomeLightbulb 󱉑
    HomeLightbulb,
    /// HomeLightbulbOutline 󱉒
    HomeLightbulbOutline,
    /// HomeLightningBolt 󱤃
    HomeLightningBolt,
    /// HomeLightningBoltOutline 󱤄
    HomeLightningBoltOutline,
    /// HomeLock 󰣫
    HomeLock,
    /// HomeLockOpen 󰣬
    HomeLockOpen,
    /// HomeMapMarker 󰗸
    HomeMapMarker,
    /// HomeMinus 󰥴
    HomeMinus,
    /// HomeMinusOutline 󱏕
    HomeMinusOutline,
    /// HomeModern 󰋝
    HomeModern,
    /// HomeOff 󱩆
    HomeOff,
    /// HomeOffOutline 󱩇
    HomeOffOutline,
    /// HomeOne 
    HomeOne,
    /// HomeOutline 󰚡
    HomeOutline,
    /// HomePlus 󰥵
    HomePlus,
    /// HomePlusOutline 󱏖
    HomePlusOutline,
    /// HomeRemove 󱉇
    HomeRemove,
    /// HomeRemoveOutline 󱏗
    HomeRemoveOutline,
    /// HomeRoof 󱄫
    HomeRoof,
    /// HomeSearch 󱎰
    HomeSearch,
    /// HomeSearchOutline 󱎱
    HomeSearchOutline,
    /// HomeSwitch 󱞔
    HomeSwitch,
    /// HomeSwitchOutline 󱞕
    HomeSwitchOutline,
    /// HomeThermometer 󰽔
    HomeThermometer,
    /// HomeThermometerOutline 󰽕
    HomeThermometerOutline,
    /// HomeThree 󰋜
    HomeThree,
    /// HomeTwo 
    HomeTwo,
    /// HomeVariant 󰋞
    HomeVariant,
    /// HomeVariantOutline 󰮧
    HomeVariantOutline,
    /// Hook 󰛢
    Hook,
    /// HookOff 󰛣
    HookOff,
    /// HoopHouse 󰹖
    HoopHouse,
    /// Hops 󰋟
    Hops,
    /// HorizontalRotateClockwise 󱃳
    HorizontalRotateClockwise,
    /// HorizontalRotateCounterclockwise 󱃴
    HorizontalRotateCounterclockwise,
    /// HorizontalRule 
    HorizontalRule,
    /// HorizontalRuleOne 
    HorizontalRuleOne,
    /// Horse 󱖿
    Horse,
    /// HorseHuman 󱗀
    HorseHuman,
    /// HorseVariant 󱗁
    HorseVariant,
    /// HorseVariantFast 󱡮
    HorseVariantFast,
    /// Horseshoe 󰩘
    Horseshoe,
    /// Hospital 
    Hospital,
    /// HospitalBox 󰋠
    HospitalBox,
    /// HospitalBoxOutline 󰿷
    HospitalBoxOutline,
    /// HospitalBuilding 󰋡
    HospitalBuilding,
    /// HospitalMarker 󰋢
    HospitalMarker,
    /// HospitalOne 󰿶
    HospitalOne,
    /// HotTub 󰠨
    HotTub,
    /// Hotdog 
    Hotdog,
    /// Hourglass 
    Hourglass,
    /// HoursTwofour 󱑸
    HoursTwofour,
    /// Htmlfive 
    Htmlfive,
    /// Hubot 
    Hubot,
    /// HubotOne 
    HubotOne,
    /// Hubspot 󰴗
    Hubspot,
    /// Hulu 󰠩
    Hulu,
    /// Human 󰋦
    Human,
    /// HumanBabyChangingTable 󱎋
    HumanBabyChangingTable,
    /// HumanCane 󱖁
    HumanCane,
    /// HumanCapacityDecrease 󱖛
    HumanCapacityDecrease,
    /// HumanCapacityIncrease 󱖜
    HumanCapacityIncrease,
    /// HumanChild 󰋧
    HumanChild,
    /// HumanDolly 󱦀
    HumanDolly,
    /// HumanEdit 󱓨
    HumanEdit,
    /// HumanFemale 󰙉
    HumanFemale,
    /// HumanFemaleBoy 󰩙
    HumanFemaleBoy,
    /// HumanFemaleDance 󱗉
    HumanFemaleDance,
    /// HumanFemaleFemale 󰩚
    HumanFemaleFemale,
    /// HumanFemaleGirl 󰩛
    HumanFemaleGirl,
    /// HumanGreeting 󱟄
    HumanGreeting,
    /// HumanGreetingProximity 󱖝
    HumanGreetingProximity,
    /// HumanGreetingVariant 󰙊
    HumanGreetingVariant,
    /// HumanHandsdown 󰙋
    HumanHandsdown,
    /// HumanHandsup 󰙌
    HumanHandsup,
    /// HumanMale 󰙍
    HumanMale,
    /// HumanMaleBoard 󰢐
    HumanMaleBoard,
    /// HumanMaleBoardPoll 󰡆
    HumanMaleBoardPoll,
    /// HumanMaleBoy 󰩜
    HumanMaleBoy,
    /// HumanMaleChild 󱎌
    HumanMaleChild,
    /// HumanMaleFemale 󰋨
    HumanMaleFemale,
    /// HumanMaleFemaleChild 󱠣
    HumanMaleFemaleChild,
    /// HumanMaleGirl 󰩝
    HumanMaleGirl,
    /// HumanMaleHeight 󰻻
    HumanMaleHeight,
    /// HumanMaleHeightVariant 󰻼
    HumanMaleHeightVariant,
    /// HumanMaleMale 󰩞
    HumanMaleMale,
    /// HumanNonBinary 󱡈
    HumanNonBinary,
    /// HumanPregnant 󰗏
    HumanPregnant,
    /// HumanQueue 󱕱
    HumanQueue,
    /// HumanScooter 󱇩
    HumanScooter,
    /// HumanWheelchair 󱎍
    HumanWheelchair,
    /// HumanWhiteCane 󱦁
    HumanWhiteCane,
    /// HumbleBundle 󰝄
    HumbleBundle,
    /// Hvac 󱍒
    Hvac,
    /// HvacOff 󱖞
    HvacOff,
    /// HydraulicOilLevel 󱌤
    HydraulicOilLevel,
    /// HydraulicOilTemperature 󱌥
    HydraulicOilTemperature,
    /// HydroPower 󱋥
    HydroPower,
    /// HydrogenStation 󱢔
    HydrogenStation,
    /// HyperbolaGnuLinuxLibre 
    HyperbolaGnuLinuxLibre,
    /// Hyprland 
    Hyprland,
    /// ICustomAsm 
    ICustomAsm,
    /// ICustomC 
    ICustomC,
    /// ICustomCommonLisp 
    ICustomCommonLisp,
    /// ICustomCpp 
    ICustomCpp,
    /// ICustomCrystal 
    ICustomCrystal,
    /// ICustomDefault 
    ICustomDefault,
    /// ICustomElectron 
    ICustomElectron,
    /// ICustomElixir 
    ICustomElixir,
    /// ICustomElm 
    ICustomElm,
    /// ICustomEmacs 
    ICustomEmacs,
    /// ICustomFennel 
    ICustomFennel,
    /// ICustomFolder 
    ICustomFolder,
    /// ICustomFolderConfig 
    ICustomFolderConfig,
    /// ICustomFolderGit 
    ICustomFolderGit,
    /// ICustomFolderGithub 
    ICustomFolderGithub,
    /// ICustomFolderNpm 
    ICustomFolderNpm,
    /// ICustomFolderOct 
    ICustomFolderOct,
    /// ICustomFolderOpen 
    ICustomFolderOpen,
    /// ICustomGo 
    ICustomGo,
    /// ICustomHome 
    ICustomHome,
    /// ICustomKotlin 
    ICustomKotlin,
    /// ICustomMsdos 
    ICustomMsdos,
    /// ICustomNeovim 
    ICustomNeovim,
    /// ICustomOrgmode 
    ICustomOrgmode,
    /// ICustomPlayArrow 
    ICustomPlayArrow,
    /// ICustomPurescript 
    ICustomPurescript,
    /// ICustomScheme 
    ICustomScheme,
    /// ICustomToml 
    ICustomToml,
    /// ICustomVLang 
    ICustomVLang,
    /// ICustomVim 
    ICustomVim,
    /// ICustomWindows 
    ICustomWindows,
    /// IIndentLine 
    IIndentLine,
    /// ISetiApple 
    ISetiApple,
    /// ISetiArgdown 
    ISetiArgdown,
    /// ISetiAsm 
    ISetiAsm,
    /// ISetiAudio 
    ISetiAudio,
    /// ISetiBabel 
    ISetiBabel,
    /// ISetiBazel 
    ISetiBazel,
    /// ISetiBicep 
    ISetiBicep,
    /// ISetiBower 
    ISetiBower,
    /// ISetiBsl 
    ISetiBsl,
    /// ISetiC 
    ISetiC,
    /// ISetiCSharp 
    ISetiCSharp,
    /// ISetiCake 
    ISetiCake,
    /// ISetiCakePhp 
    ISetiCakePhp,
    /// ISetiCheckbox 
    ISetiCheckbox,
    /// ISetiCheckboxUnchecked 
    ISetiCheckboxUnchecked,
    /// ISetiClock 
    ISetiClock,
    /// ISetiClojure 
    ISetiClojure,
    /// ISetiCodeClimate 
    ISetiCodeClimate,
    /// ISetiCodeSearch 
    ISetiCodeSearch,
    /// ISetiCoffee 
    ISetiCoffee,
    /// ISetiColdfusion 
    ISetiColdfusion,
    /// ISetiConfig 
    ISetiConfig,
    /// ISetiCpp 
    ISetiCpp,
    /// ISetiCrystalEmbedded 
    ISetiCrystalEmbedded,
    /// ISetiCss 
    ISetiCss,
    /// ISetiCsv 
    ISetiCsv,
    /// ISetiCu 
    ISetiCu,
    /// ISetiD 
    ISetiD,
    /// ISetiDart 
    ISetiDart,
    /// ISetiDb 
    ISetiDb,
    /// ISetiDefault 
    ISetiDefault,
    /// ISetiDeprecationCop 
    ISetiDeprecationCop,
    /// ISetiDocker 
    ISetiDocker,
    /// ISetiEditorconfig 
    ISetiEditorconfig,
    /// ISetiEjs 
    ISetiEjs,
    /// ISetiElixirScript 
    ISetiElixirScript,
    /// ISetiError 
    ISetiError,
    /// ISetiEslint 
    ISetiEslint,
    /// ISetiEthereum 
    ISetiEthereum,
    /// ISetiFSharp 
    ISetiFSharp,
    /// ISetiFavicon 
    ISetiFavicon,
    /// ISetiFirebase 
    ISetiFirebase,
    /// ISetiFirefox 
    ISetiFirefox,
    /// ISetiFolder 
    ISetiFolder,
    /// ISetiFont 
    ISetiFont,
    /// ISetiGit 
    ISetiGit,
    /// ISetiGithub 
    ISetiGithub,
    /// ISetiGitlab 
    ISetiGitlab,
    /// ISetiGo 
    ISetiGo,
    /// ISetiGodot 
    ISetiGodot,
    /// ISetiGotwo 
    ISetiGotwo,
    /// ISetiGradle 
    ISetiGradle,
    /// ISetiGrails 
    ISetiGrails,
    /// ISetiGraphql 
    ISetiGraphql,
    /// ISetiGrunt 
    ISetiGrunt,
    /// ISetiGulp 
    ISetiGulp,
    /// ISetiHacklang 
    ISetiHacklang,
    /// ISetiHaml 
    ISetiHaml,
    /// ISetiHappenings 
    ISetiHappenings,
    /// ISetiHaskell 
    ISetiHaskell,
    /// ISetiHaxe 
    ISetiHaxe,
    /// ISetiHeroku 
    ISetiHeroku,
    /// ISetiHex 
    ISetiHex,
    /// ISetiHtml 
    ISetiHtml,
    /// ISetiIgnored 
    ISetiIgnored,
    /// ISetiIllustrator 
    ISetiIllustrator,
    /// ISetiImage 
    ISetiImage,
    /// ISetiInfo 
    ISetiInfo,
    /// ISetiIonic 
    ISetiIonic,
    /// ISetiJade 
    ISetiJade,
    /// ISetiJava 
    ISetiJava,
    /// ISetiJavascript 
    ISetiJavascript,
    /// ISetiJenkins 
    ISetiJenkins,
    /// ISetiJinja 
    ISetiJinja,
    /// ISetiJson 
    ISetiJson,
    /// ISetiJulia 
    ISetiJulia,
    /// ISetiKarma 
    ISetiKarma,
    /// ISetiLicense 
    ISetiLicense,
    /// ISetiLiquid 
    ISetiLiquid,
    /// ISetiLivescript 
    ISetiLivescript,
    /// ISetiLock 
    ISetiLock,
    /// ISetiLua 
    ISetiLua,
    /// ISetiMakefile 
    ISetiMakefile,
    /// ISetiMarkdown 
    ISetiMarkdown,
    /// ISetiMaven 
    ISetiMaven,
    /// ISetiMdo 
    ISetiMdo,
    /// ISetiMustache 
    ISetiMustache,
    /// ISetiNewFile 
    ISetiNewFile,
    /// ISetiNim 
    ISetiNim,
    /// ISetiNotebook 
    ISetiNotebook,
    /// ISetiNpm 
    ISetiNpm,
    /// ISetiNunjucks 
    ISetiNunjucks,
    /// ISetiOcaml 
    ISetiOcaml,
    /// ISetiOdata 
    ISetiOdata,
    /// ISetiPddl 
    ISetiPddl,
    /// ISetiPdf 
    ISetiPdf,
    /// ISetiPerl 
    ISetiPerl,
    /// ISetiPhotoshop 
    ISetiPhotoshop,
    /// ISetiPhp 
    ISetiPhp,
    /// ISetiPipeline 
    ISetiPipeline,
    /// ISetiPlan 
    ISetiPlan,
    /// ISetiPlatformio 
    ISetiPlatformio,
    /// ISetiPowershell 
    ISetiPowershell,
    /// ISetiPrisma 
    ISetiPrisma,
    /// ISetiProject 
    ISetiProject,
    /// ISetiProlog 
    ISetiProlog,
    /// ISetiPug 
    ISetiPug,
    /// ISetiPuppet 
    ISetiPuppet,
    /// ISetiPython 
    ISetiPython,
    /// ISetiR 
    ISetiR,
    /// ISetiRails 
    ISetiRails,
    /// ISetiReact 
    ISetiReact,
    /// ISetiReasonml 
    ISetiReasonml,
    /// ISetiRescript 
    ISetiRescript,
    /// ISetiRollup 
    ISetiRollup,
    /// ISetiRuby 
    ISetiRuby,
    /// ISetiRust 
    ISetiRust,
    /// ISetiSalesforce 
    ISetiSalesforce,
    /// ISetiSass 
    ISetiSass,
    /// ISetiSbt 
    ISetiSbt,
    /// ISetiScala 
    ISetiScala,
    /// ISetiSearch 
    ISetiSearch,
    /// ISetiSettings 
    ISetiSettings,
    /// ISetiShell 
    ISetiShell,
    /// ISetiSlim 
    ISetiSlim,
    /// ISetiSmarty 
    ISetiSmarty,
    /// ISetiSpring 
    ISetiSpring,
    /// ISetiStylelint 
    ISetiStylelint,
    /// ISetiStylus 
    ISetiStylus,
    /// ISetiSublime 
    ISetiSublime,
    /// ISetiSvelte 
    ISetiSvelte,
    /// ISetiSvg 
    ISetiSvg,
    /// ISetiSwift 
    ISetiSwift,
    /// ISetiTerraform 
    ISetiTerraform,
    /// ISetiTex 
    ISetiTex,
    /// ISetiTodo 
    ISetiTodo,
    /// ISetiTsconfig 
    ISetiTsconfig,
    /// ISetiTwig 
    ISetiTwig,
    /// ISetiTypescript 
    ISetiTypescript,
    /// ISetiVala 
    ISetiVala,
    /// ISetiVideo 
    ISetiVideo,
    /// ISetiVue 
    ISetiVue,
    /// ISetiWasm 
    ISetiWasm,
    /// ISetiWat 
    ISetiWat,
    /// ISetiWebpack 
    ISetiWebpack,
    /// ISetiWgt 
    ISetiWgt,
    /// ISetiWord 
    ISetiWord,
    /// ISetiXls 
    ISetiXls,
    /// ISetiXml 
    ISetiXml,
    /// ISetiYarn 
    ISetiYarn,
    /// ISetiYml 
    ISetiYml,
    /// ISetiZig 
    ISetiZig,
    /// ISetiZip 
    ISetiZip,
    /// IceCream 
    IceCream,
    /// IceCreamOff 󰹒
    IceCreamOff,
    /// IceCreamOne 󰠪
    IceCreamOne,
    /// IcePop 󰻽
    IcePop,
    /// IdBadge 
    IdBadge,
    /// IdCard 
    IdCard,
    /// IdCardOne 󰿀
    IdCardOne,
    /// Identifier 󰻾
    Identifier,
    /// IdeogramCjk 󱌱
    IdeogramCjk,
    /// IdeogramCjkVariant 󱌲
    IdeogramCjkVariant,
    /// Illumos 
    Illumos,
    /// Image 
    Image,
    /// ImageAlbum 󰋪
    ImageAlbum,
    /// ImageArea 󰋫
    ImageArea,
    /// ImageAreaClose 󰋬
    ImageAreaClose,
    /// ImageAutoAdjust 󰿁
    ImageAutoAdjust,
    /// ImageBroken 󰋭
    ImageBroken,
    /// ImageBrokenVariant 󰋮
    ImageBrokenVariant,
    /// ImageEdit 󱇣
    ImageEdit,
    /// ImageEditOutline 󱇤
    ImageEditOutline,
    /// ImageFilterBlackWhite 󰋰
    ImageFilterBlackWhite,
    /// ImageFilterCenterFocus 󰋱
    ImageFilterCenterFocus,
    /// ImageFilterCenterFocusStrong 󰻿
    ImageFilterCenterFocusStrong,
    /// ImageFilterCenterFocusStrongOutline 󰼀
    ImageFilterCenterFocusStrongOutline,
    /// ImageFilterCenterFocusWeak 󰋲
    ImageFilterCenterFocusWeak,
    /// ImageFilterDrama 󰋳
    ImageFilterDrama,
    /// ImageFilterFrames 󰋴
    ImageFilterFrames,
    /// ImageFilterHdr 󰋵
    ImageFilterHdr,
    /// ImageFilterHdrOne 󰔉
    ImageFilterHdrOne,
    /// ImageFilterNone 󰋶
    ImageFilterNone,
    /// ImageFilterTiltShift 󰋷
    ImageFilterTiltShift,
    /// ImageFilterVintage 󰋸
    ImageFilterVintage,
    /// ImageFrame 󰹉
    ImageFrame,
    /// ImageLock 󱪰
    ImageLock,
    /// ImageLockOutline 󱪱
    ImageLockOutline,
    /// ImageMarker 󱝻
    ImageMarker,
    /// ImageMarkerOutline 󱝼
    ImageMarkerOutline,
    /// ImageMinus 󱐙
    ImageMinus,
    /// ImageMove 󰧸
    ImageMove,
    /// ImageMultiple 󰋹
    ImageMultiple,
    /// ImageMultipleOutline 󰋯
    ImageMultipleOutline,
    /// ImageOff 󰠫
    ImageOff,
    /// ImageOffOutline 󱇑
    ImageOffOutline,
    /// ImageOne 󰋩
    ImageOne,
    /// ImageOutline 󰥶
    ImageOutline,
    /// ImagePlus 󰡼
    ImagePlus,
    /// ImageRefresh 󱧾
    ImageRefresh,
    /// ImageRefreshOutline 󱧿
    ImageRefreshOutline,
    /// ImageRemove 󱐘
    ImageRemove,
    /// ImageSearch 󰥷
    ImageSearch,
    /// ImageSearchOutline 󰥸
    ImageSearchOutline,
    /// ImageSizeSelectActual 󰲍
    ImageSizeSelectActual,
    /// ImageSizeSelectLarge 󰲎
    ImageSizeSelectLarge,
    /// ImageSizeSelectSmall 󰲏
    ImageSizeSelectSmall,
    /// ImageSync 󱨀
    ImageSync,
    /// ImageSyncOutline 󱨁
    ImageSyncOutline,
    /// ImageText 󱘍
    ImageText,
    /// Imdb 
    Imdb,
    /// Import 󰋺
    Import,
    /// Inbox 
    Inbox,
    /// InboxArrowDown 󰋻
    InboxArrowDown,
    /// InboxArrowDownOutline 󱉰
    InboxArrowDownOutline,
    /// InboxArrowUp 󰏑
    InboxArrowUp,
    /// InboxArrowUpOutline 󱉱
    InboxArrowUpOutline,
    /// InboxFull 󱉲
    InboxFull,
    /// InboxFullOutline 󱉳
    InboxFullOutline,
    /// InboxMultiple 󰢰
    InboxMultiple,
    /// InboxMultipleOutline 󰮨
    InboxMultipleOutline,
    /// InboxOne 
    InboxOne,
    /// InboxOutline 󱉴
    InboxOutline,
    /// InboxRemove 󱖟
    InboxRemove,
    /// InboxRemoveOutline 󱖠
    InboxRemoveOutline,
    /// InboxThree 󰚇
    InboxThree,
    /// InboxTwo 
    InboxTwo,
    /// Incognito 󰗹
    Incognito,
    /// IncognitoCircle 󱐡
    IncognitoCircle,
    /// IncognitoCircleOff 󱐢
    IncognitoCircleOff,
    /// IncognitoOff 󰁵
    IncognitoOff,
    /// IndentLeft 
    IndentLeft,
    /// IndentRight 
    IndentRight,
    /// Induction 󱡌
    Induction,
    /// Infinity 
    Infinity,
    /// InfinityOne 
    InfinityOne,
    /// InfinityTwo 󰛤
    InfinityTwo,
    /// Info 
    Info,
    /// InfoOne 
    InfoOne,
    /// InfoSign 
    InfoSign,
    /// Information 󰋼
    Information,
    /// InformationOff 󱞌
    InformationOff,
    /// InformationOffOutline 󱞍
    InformationOffOutline,
    /// InformationOutline 󰋽
    InformationOutline,
    /// InformationVariant 󰙎
    InformationVariant,
    /// Injection 
    Injection,
    /// Inkscape 
    Inkscape,
    /// Inr 
    Inr,
    /// Inspect 
    Inspect,
    /// Instagram 
    Instagram,
    /// InstagramOne 󰋾
    InstagramOne,
    /// InstrumentTriangle 󱁎
    InstrumentTriangle,
    /// IntegratedCircuitChip 󱤓
    IntegratedCircuitChip,
    /// InternalInterruption 
    InternalInterruption,
    /// InvertColors 󰌁
    InvertColors,
    /// InvertColorsOff 󰹊
    InvertColorsOff,
    /// Iobroker 󱋨
    Iobroker,
    /// Ip 󰩟
    Ip,
    /// IpNetwork 󰩠
    IpNetwork,
    /// IpNetworkOutline 󰲐
    IpNetworkOutline,
    /// IpOutline 󱦂
    IpOutline,
    /// Ipod 󰲑
    Ipod,
    /// Iron 󱠤
    Iron,
    /// IronBoard 󱠸
    IronBoard,
    /// IronOutline 󱠥
    IronOutline,
    /// Island 󱁏
    Island,
    /// Isle 
    Isle,
    /// IssueClosed 
    IssueClosed,
    /// IssueDraft 
    IssueDraft,
    /// IssueDraftOne 
    IssueDraftOne,
    /// IssueOpened 
    IssueOpened,
    /// IssueReopened 
    IssueReopened,
    /// IssueReopenedOne 
    IssueReopenedOne,
    /// IssueTrackedBy 
    IssueTrackedBy,
    /// IssueTracks 
    IssueTracks,
    /// Issues 
    Issues,
    /// Italic 
    Italic,
    /// ItalicOne 
    ItalicOne,
    /// ItalicTwo 
    ItalicTwo,
    /// Iterations 
    Iterations,
    /// Ithree 
    Ithree,
    /// IvBag 󱂹
    IvBag,
    /// Jabber 󰷕
    Jabber,
    /// Java 
    Java,
    /// Jeepney 󰌂
    Jeepney,
    /// Jellyfish 󰼁
    Jellyfish,
    /// JellyfishOutline 󰼂
    JellyfishOutline,
    /// Jersey 
    Jersey,
    /// Jira 󰌃
    Jira,
    /// Jpy 
    Jpy,
    /// Jquery 󰡽
    Jquery,
    /// Jsfiddle 󰌄
    Jsfiddle,
    /// Json 
    Json,
    /// JumpRope 󱋿
    JumpRope,
    /// Jwm 
    Jwm,
    /// Kabaddi 󰶇
    Kabaddi,
    /// KaliLinux 
    KaliLinux,
    /// Kangaroo 󱕘
    Kangaroo,
    /// Karate 󰠬
    Karate,
    /// Kayaking 󰢯
    Kayaking,
    /// KdeNeon 
    KdeNeon,
    /// KdePlasma 
    KdePlasma,
    /// Kdenlive 
    Kdenlive,
    /// KebabHorizontal 
    KebabHorizontal,
    /// KebabVertical 
    KebabVertical,
    /// Keg 󰌅
    Keg,
    /// Kettle 󰗺
    Kettle,
    /// KettleAlert 󱌗
    KettleAlert,
    /// KettleAlertOutline 󱌘
    KettleAlertOutline,
    /// KettleOff 󱌛
    KettleOff,
    /// KettleOffOutline 󱌜
    KettleOffOutline,
    /// KettleOutline 󰽖
    KettleOutline,
    /// KettlePourOver 󱜼
    KettlePourOver,
    /// KettleSteam 󱌙
    KettleSteam,
    /// KettleSteamOutline 󱌚
    KettleSteamOutline,
    /// Kettlebell 󱌀
    Kettlebell,
    /// Key 
    Key,
    /// KeyAlert 󱦃
    KeyAlert,
    /// KeyAlertOutline 󱦄
    KeyAlertOutline,
    /// KeyArrowRight 󱌒
    KeyArrowRight,
    /// KeyAsterisk 
    KeyAsterisk,
    /// KeyChain 󱕴
    KeyChain,
    /// KeyChainVariant 󱕵
    KeyChainVariant,
    /// KeyChange 󰌇
    KeyChange,
    /// KeyLink 󱆟
    KeyLink,
    /// KeyMinus 󰌈
    KeyMinus,
    /// KeyOne 
    KeyOne,
    /// KeyOutline 󰷖
    KeyOutline,
    /// KeyPlus 󰌉
    KeyPlus,
    /// KeyRemove 󰌊
    KeyRemove,
    /// KeyStar 󱆞
    KeyStar,
    /// KeyThree 󰌆
    KeyThree,
    /// KeyTwo 
    KeyTwo,
    /// KeyVariant 󰌋
    KeyVariant,
    /// KeyWireless 󰿂
    KeyWireless,
    /// Keyboard 
    Keyboard,
    /// KeyboardBackspace 󰌍
    KeyboardBackspace,
    /// KeyboardCaps 󰌎
    KeyboardCaps,
    /// KeyboardClose 󰌏
    KeyboardClose,
    /// KeyboardEsc 󱊷
    KeyboardEsc,
    /// KeyboardFeight 󱊲
    KeyboardFeight,
    /// KeyboardFfive 󱊯
    KeyboardFfive,
    /// KeyboardFfour 󱊮
    KeyboardFfour,
    /// KeyboardFnine 󱊳
    KeyboardFnine,
    /// KeyboardFone 󱊫
    KeyboardFone,
    /// KeyboardFoneone 󱊵
    KeyboardFoneone,
    /// KeyboardFonetwo 󱊶
    KeyboardFonetwo,
    /// KeyboardFonezero 󱊴
    KeyboardFonezero,
    /// KeyboardFseven 󱊱
    KeyboardFseven,
    /// KeyboardFsix 󱊰
    KeyboardFsix,
    /// KeyboardFthree 󱊭
    KeyboardFthree,
    /// KeyboardFtwo 󱊬
    KeyboardFtwo,
    /// KeyboardOff 󰌐
    KeyboardOff,
    /// KeyboardOffOutline 󰹋
    KeyboardOffOutline,
    /// KeyboardOne 󰌌
    KeyboardOne,
    /// KeyboardOutline 󰥻
    KeyboardOutline,
    /// KeyboardReturn 󰌑
    KeyboardReturn,
    /// KeyboardSettings 󰧹
    KeyboardSettings,
    /// KeyboardSettingsOutline 󰧺
    KeyboardSettingsOutline,
    /// KeyboardSpace 󱁐
    KeyboardSpace,
    /// KeyboardTab 󰌒
    KeyboardTab,
    /// KeyboardTabReverse 󰌥
    KeyboardTabReverse,
    /// KeyboardVariant 󰌓
    KeyboardVariant,
    /// Khanda 󱃽
    Khanda,
    /// Kicad 
    Kicad,
    /// Kickstarter 󰝅
    Kickstarter,
    /// Kite 󱦅
    Kite,
    /// KiteOutline 󱦆
    KiteOutline,
    /// Kitesurfing 󱝄
    Kitesurfing,
    /// Klingon 󱍛
    Klingon,
    /// Knife 󰧻
    Knife,
    /// KnifeMilitary 󰧼
    KnifeMilitary,
    /// Koala 󱜿
    Koala,
    /// Kodi 󰌔
    Kodi,
    /// Krita 
    Krita,
    /// Krw 
    Krw,
    /// Kubernetes 󱃾
    Kubernetes,
    /// Kubuntu 
    Kubuntu,
    /// KubuntuInverse 
    KubuntuInverse,
    /// Label 󰌕
    Label,
    /// LabelMultiple 󱍵
    LabelMultiple,
    /// LabelMultipleOutline 󱍶
    LabelMultipleOutline,
    /// LabelOff 󰫋
    LabelOff,
    /// LabelOffOutline 󰫌
    LabelOffOutline,
    /// LabelOutline 󰌖
    LabelOutline,
    /// LabelPercent 󱋪
    LabelPercent,
    /// LabelPercentOutline 󱋫
    LabelPercentOutline,
    /// LabelVariant 󰫍
    LabelVariant,
    /// LabelVariantOutline 󰫎
    LabelVariantOutline,
    /// Ladder 󱖢
    Ladder,
    /// Ladybug 󰠭
    Ladybug,
    /// Lambda 󰘧
    Lambda,
    /// Lamp 󰚵
    Lamp,
    /// LampOutline 󱟐
    LampOutline,
    /// Lamps 󱕶
    Lamps,
    /// LampsOutline 󱟑
    LampsOutline,
    /// Lan 󰌗
    Lan,
    /// LanCheck 󱊪
    LanCheck,
    /// LanConnect 󰌘
    LanConnect,
    /// LanDisconnect 󰌙
    LanDisconnect,
    /// LanPending 󰌚
    LanPending,
    /// LandFields 󱪲
    LandFields,
    /// LandPlots 󱪳
    LandPlots,
    /// LandPlotsCircle 󱪴
    LandPlotsCircle,
    /// LandPlotsCircleVariant 󱪵
    LandPlotsCircleVariant,
    /// LandRowsHorizontal 󱪶
    LandRowsHorizontal,
    /// LandRowsVertical 󱪷
    LandRowsVertical,
    /// Landslide 󱩈
    Landslide,
    /// LandslideOutline 󱩉
    LandslideOutline,
    /// LanguageC 󰙱
    LanguageC,
    /// LanguageCpp 󰙲
    LanguageCpp,
    /// LanguageCsharp 󰌛
    LanguageCsharp,
    /// LanguageCssthree 󰌜
    LanguageCssthree,
    /// LanguageFortran 󱈚
    LanguageFortran,
    /// LanguageGo 󰟓
    LanguageGo,
    /// LanguageHaskell 󰲒
    LanguageHaskell,
    /// LanguageHtmlfive 󰌝
    LanguageHtmlfive,
    /// LanguageJava 󰬷
    LanguageJava,
    /// LanguageJavascript 󰌞
    LanguageJavascript,
    /// LanguageKotlin 󱈙
    LanguageKotlin,
    /// LanguageLua 󰢱
    LanguageLua,
    /// LanguageMarkdown 󰍔
    LanguageMarkdown,
    /// LanguageMarkdownOutline 󰽛
    LanguageMarkdownOutline,
    /// LanguagePhp 󰌟
    LanguagePhp,
    /// LanguagePython 󰌠
    LanguagePython,
    /// LanguageR 󰟔
    LanguageR,
    /// LanguageRuby 󰴭
    LanguageRuby,
    /// LanguageRubyOnRails 󰫏
    LanguageRubyOnRails,
    /// LanguageRust 󱘗
    LanguageRust,
    /// LanguageSwift 󰛥
    LanguageSwift,
    /// LanguageTypescript 󰛦
    LanguageTypescript,
    /// LanguageXaml 󰙳
    LanguageXaml,
    /// Laptop 
    Laptop,
    /// LaptopAccount 󱩊
    LaptopAccount,
    /// LaptopOff 󰛧
    LaptopOff,
    /// LaptopOne 󰌢
    LaptopOne,
    /// Laravel 󰫐
    Laravel,
    /// LaserPointer 󱒄
    LaserPointer,
    /// Lasso 󰼃
    Lasso,
    /// Lastpass 󰑆
    Lastpass,
    /// Latitude 󰽗
    Latitude,
    /// Launch 󰌧
    Launch,
    /// LavaLamp 󰟕
    LavaLamp,
    /// Law 
    Law,
    /// LawOne 
    LawOne,
    /// Layers 
    Layers,
    /// LayersActive 
    LayersActive,
    /// LayersDot 
    LayersDot,
    /// LayersEdit 󱢒
    LayersEdit,
    /// LayersMinus 󰹌
    LayersMinus,
    /// LayersOff 󰌩
    LayersOff,
    /// LayersOffOutline 󰧽
    LayersOffOutline,
    /// LayersOne 
    LayersOne,
    /// LayersOutline 󰧾
    LayersOutline,
    /// LayersPlus 󰹍
    LayersPlus,
    /// LayersRemove 󰹎
    LayersRemove,
    /// LayersSearch 󱈆
    LayersSearch,
    /// LayersSearchOutline 󱈇
    LayersSearchOutline,
    /// LayersTriple 󰽘
    LayersTriple,
    /// LayersTripleOutline 󰽙
    LayersTripleOutline,
    /// LayersTwo 󰌨
    LayersTwo,
    /// Layout 
    Layout,
    /// LeadPencil 󰙏
    LeadPencil,
    /// Leaf 
    Leaf,
    /// LeafCircle 󱤅
    LeafCircle,
    /// LeafCircleOutline 󱤆
    LeafCircleOutline,
    /// LeafMaple 󰲓
    LeafMaple,
    /// LeafMapleOff 󱋚
    LeafMapleOff,
    /// LeafOff 󱋙
    LeafOff,
    /// LeafOne 󰌪
    LeafOne,
    /// Leak 󰷗
    Leak,
    /// LeakOff 󰷘
    LeakOff,
    /// Lecturn 󱫰
    Lecturn,
    /// LedOff 󰌫
    LedOff,
    /// LedOn 󰌬
    LedOn,
    /// LedOutline 󰌭
    LedOutline,
    /// LedStrip 󰟖
    LedStrip,
    /// LedStripVariant 󱁑
    LedStripVariant,
    /// LedStripVariantOff 󱩋
    LedStripVariantOff,
    /// LedVariantOff 󰌮
    LedVariantOff,
    /// LedVariantOn 󰌯
    LedVariantOn,
    /// LedVariantOutline 󰌰
    LedVariantOutline,
    /// Leek 󱅽
    Leek,
    /// Legal 
    Legal,
    /// Lemon 
    Lemon,
    /// LessThan 󰥼
    LessThan,
    /// LessThanOrEqual 󰥽
    LessThanOrEqual,
    /// LevelDown 
    LevelDown,
    /// LevelUp 
    LevelUp,
    /// Library 
    Library,
    /// LibraryOne 󰌱
    LibraryOne,
    /// LibraryOutline 󱨢
    LibraryOutline,
    /// LibraryShelves 󰮩
    LibraryShelves,
    /// License 󰿃
    License,
    /// Lifebuoy 󰡾
    Lifebuoy,
    /// LightBulb 
    LightBulb,
    /// LightFloodDown 󱦇
    LightFloodDown,
    /// LightFloodUp 󱦈
    LightFloodUp,
    /// LightRecessed 󱞛
    LightRecessed,
    /// LightSwitch 󰥾
    LightSwitch,
    /// LightSwitchOff 󱨤
    LightSwitchOff,
    /// Lightbulb 
    Lightbulb,
    /// LightbulbAlert 󱧡
    LightbulbAlert,
    /// LightbulbAlertOutline 󱧢
    LightbulbAlertOutline,
    /// LightbulbAuto 󱠀
    LightbulbAuto,
    /// LightbulbAutoOutline 󱠁
    LightbulbAutoOutline,
    /// LightbulbAutofix 
    LightbulbAutofix,
    /// LightbulbCfl 󱈈
    LightbulbCfl,
    /// LightbulbCflOff 󱈉
    LightbulbCflOff,
    /// LightbulbCflSpiral 󱉵
    LightbulbCflSpiral,
    /// LightbulbCflSpiralOff 󱋃
    LightbulbCflSpiralOff,
    /// LightbulbFluorescentTube 󱠄
    LightbulbFluorescentTube,
    /// LightbulbFluorescentTubeOutline 󱠅
    LightbulbFluorescentTubeOutline,
    /// LightbulbGroup 󱉓
    LightbulbGroup,
    /// LightbulbGroupOff 󱋍
    LightbulbGroupOff,
    /// LightbulbGroupOffOutline 󱋎
    LightbulbGroupOffOutline,
    /// LightbulbGroupOutline 󱉔
    LightbulbGroupOutline,
    /// LightbulbMultiple 󱉕
    LightbulbMultiple,
    /// LightbulbMultipleOff 󱋏
    LightbulbMultipleOff,
    /// LightbulbMultipleOffOutline 󱋐
    LightbulbMultipleOffOutline,
    /// LightbulbMultipleOutline 󱉖
    LightbulbMultipleOutline,
    /// LightbulbNight 󱩌
    LightbulbNight,
    /// LightbulbNightOutline 󱩍
    LightbulbNightOutline,
    /// LightbulbOff 󰹏
    LightbulbOff,
    /// LightbulbOffOutline 󰹐
    LightbulbOffOutline,
    /// LightbulbOn 󰛨
    LightbulbOn,
    /// LightbulbOnEightzero 󱩕
    LightbulbOnEightzero,
    /// LightbulbOnFivezero 󱩒
    LightbulbOnFivezero,
    /// LightbulbOnFourzero 󱩑
    LightbulbOnFourzero,
    /// LightbulbOnNinezero 󱩖
    LightbulbOnNinezero,
    /// LightbulbOnOnezero 󱩎
    LightbulbOnOnezero,
    /// LightbulbOnOutline 󰛩
    LightbulbOnOutline,
    /// LightbulbOnSevenzero 󱩔
    LightbulbOnSevenzero,
    /// LightbulbOnSixzero 󱩓
    LightbulbOnSixzero,
    /// LightbulbOnThreezero 󱩐
    LightbulbOnThreezero,
    /// LightbulbOnTwozero 󱩏
    LightbulbOnTwozero,
    /// LightbulbOne 󰌵
    LightbulbOne,
    /// LightbulbOutline 󰌶
    LightbulbOutline,
    /// LightbulbQuestion 󱧣
    LightbulbQuestion,
    /// LightbulbQuestionOutline 󱧤
    LightbulbQuestionOutline,
    /// LightbulbSpot 󱟴
    LightbulbSpot,
    /// LightbulbSpotOff 󱟵
    LightbulbSpotOff,
    /// LightbulbVariant 󱠂
    LightbulbVariant,
    /// LightbulbVariantOutline 󱠃
    LightbulbVariantOutline,
    /// Lighthouse 󰧿
    Lighthouse,
    /// LighthouseOn 󰨀
    LighthouseOn,
    /// LightningBolt 󱐋
    LightningBolt,
    /// LightningBoltCircle 󰠠
    LightningBoltCircle,
    /// LightningBoltOutline 󱐌
    LightningBoltOutline,
    /// LineScan 󰘤
    LineScan,
    /// Lingerie 󱑶
    Lingerie,
    /// Link 
    Link,
    /// LinkBox 󰴚
    LinkBox,
    /// LinkBoxOutline 󰴛
    LinkBoxOutline,
    /// LinkBoxVariant 󰴜
    LinkBoxVariant,
    /// LinkBoxVariantOutline 󰴝
    LinkBoxVariantOutline,
    /// LinkExternal 
    LinkExternal,
    /// LinkExternalOne 
    LinkExternalOne,
    /// LinkLock 󱂺
    LinkLock,
    /// LinkOff 󰌸
    LinkOff,
    /// LinkOne 
    LinkOne,
    /// LinkPlus 󰲔
    LinkPlus,
    /// LinkThree 󰌷
    LinkThree,
    /// LinkTwo 
    LinkTwo,
    /// LinkVariant 󰌹
    LinkVariant,
    /// LinkVariantMinus 󱃿
    LinkVariantMinus,
    /// LinkVariantOff 󰌺
    LinkVariantOff,
    /// LinkVariantPlus 󱄀
    LinkVariantPlus,
    /// LinkVariantRemove 󱄁
    LinkVariantRemove,
    /// Linkedin 
    Linkedin,
    /// LinkedinOne 󰌻
    LinkedinOne,
    /// LinkedinSign 
    LinkedinSign,
    /// Linux 
    Linux,
    /// LinuxMint 󰣭
    LinuxMint,
    /// LinuxMintInverse 
    LinuxMintInverse,
    /// LinuxOne 󰌽
    LinuxOne,
    /// Lips 
    Lips,
    /// Lipstick 
    Lipstick,
    /// LipstickOne 󱎵
    LipstickOne,
    /// LiquidSpot 󱠦
    LiquidSpot,
    /// Liquor 󱤞
    Liquor,
    /// List 
    List,
    /// ListAlt 
    ListAlt,
    /// ListFilter 
    ListFilter,
    /// ListFlat 
    ListFlat,
    /// ListOrdered 
    ListOrdered,
    /// ListOrderedOne 
    ListOrderedOne,
    /// ListSelection 
    ListSelection,
    /// ListStatus 󱖫
    ListStatus,
    /// ListTree 
    ListTree,
    /// ListUnordered 
    ListUnordered,
    /// ListUnorderedOne 
    ListUnorderedOne,
    /// Litecoin 󰩡
    Litecoin,
    /// LiveShare 
    LiveShare,
    /// Liver 
    Liver,
    /// Loading 
    Loading,
    /// LoadingOne 󰝲
    LoadingOne,
    /// LocOs 
    LocOs,
    /// Location 
    Location,
    /// LocationArrow 
    LocationArrow,
    /// LocationEnter 󰿄
    LocationEnter,
    /// LocationExit 󰿅
    LocationExit,
    /// LocationOne 
    LocationOne,
    /// Lock 
    Lock,
    /// LockAlert 󰣮
    LockAlert,
    /// LockAlertOutline 󱗑
    LockAlertOutline,
    /// LockCheck 󱎚
    LockCheck,
    /// LockCheckOutline 󱚨
    LockCheckOutline,
    /// LockClock 󰥿
    LockClock,
    /// LockMinus 󱚩
    LockMinus,
    /// LockMinusOutline 󱚪
    LockMinusOutline,
    /// LockOff 󱙱
    LockOff,
    /// LockOffOutline 󱙲
    LockOffOutline,
    /// LockOne 
    LockOne,
    /// LockOpen 󰌿
    LockOpen,
    /// LockOpenAlert 󱎛
    LockOpenAlert,
    /// LockOpenAlertOutline 󱗒
    LockOpenAlertOutline,
    /// LockOpenCheck 󱎜
    LockOpenCheck,
    /// LockOpenCheckOutline 󱚫
    LockOpenCheckOutline,
    /// LockOpenMinus 󱚬
    LockOpenMinus,
    /// LockOpenMinusOutline 󱚭
    LockOpenMinusOutline,
    /// LockOpenOutline 󰍀
    LockOpenOutline,
    /// LockOpenPlus 󱚮
    LockOpenPlus,
    /// LockOpenPlusOutline 󱚯
    LockOpenPlusOutline,
    /// LockOpenRemove 󱚰
    LockOpenRemove,
    /// LockOpenRemoveOutline 󱚱
    LockOpenRemoveOutline,
    /// LockOpenVariant 󰿆
    LockOpenVariant,
    /// LockOpenVariantOutline 󰿇
    LockOpenVariantOutline,
    /// LockOutline 󰍁
    LockOutline,
    /// LockPattern 󰛪
    LockPattern,
    /// LockPlus 󰗻
    LockPlus,
    /// LockPlusOutline 󱚲
    LockPlusOutline,
    /// LockQuestion 󰣯
    LockQuestion,
    /// LockRemove 󱚳
    LockRemove,
    /// LockRemoveOutline 󱚴
    LockRemoveOutline,
    /// LockReset 󰝳
    LockReset,
    /// LockSmall 
    LockSmall,
    /// LockSmart 󰢲
    LockSmart,
    /// LockThree 󰌾
    LockThree,
    /// LockTwo 
    LockTwo,
    /// Locker 󰟗
    Locker,
    /// LockerMultiple 󰟘
    LockerMultiple,
    /// Log 
    Log,
    /// Login 󰍂
    Login,
    /// LogoGist 
    LogoGist,
    /// LogoGithub 
    LogoGithub,
    /// Logout 󰍃
    Logout,
    /// LogoutVariant 󰗽
    LogoutVariant,
    /// Lollipop 
    Lollipop,
    /// LongArrowDown 
    LongArrowDown,
    /// LongArrowLeft 
    LongArrowLeft,
    /// LongArrowRight 
    LongArrowRight,
    /// LongArrowUp 
    LongArrowUp,
    /// LongPause 
    LongPause,
    /// Longitude 󰽚
    Longitude,
    /// Looks 󰍄
    Looks,
    /// Lotion 󱖂
    Lotion,
    /// LotionOutline 󱖃
    LotionOutline,
    /// LotionPlus 󱖄
    LotionPlus,
    /// LotionPlusOutline 󱖅
    LotionPlusOutline,
    /// Loupe 󰍅
    Loupe,
    /// LoyaltyCard 
    LoyaltyCard,
    /// Lumx 󰍆
    Lumx,
    /// Lung 
    Lung,
    /// Lungs 󱂄
    Lungs,
    /// Lxde 
    Lxde,
    /// LxleLinux 
    LxleLinux,
    /// Lxqt 
    Lxqt,
    /// Mace 󱡃
    Mace,
    /// MagazinePistol 󰌤
    MagazinePistol,
    /// MagazineRifle 󰌣
    MagazineRifle,
    /// Mageia 
    Mageia,
    /// Magic 
    Magic,
    /// MagicStaff 󱡄
    MagicStaff,
    /// Magnet 
    Magnet,
    /// MagnetOn 󰍈
    MagnetOn,
    /// MagnetOne 
    MagnetOne,
    /// MagnetTwo 󰍇
    MagnetTwo,
    /// Magnify 󰍉
    Magnify,
    /// MagnifyClose 󰦀
    MagnifyClose,
    /// MagnifyExpand 󱡴
    MagnifyExpand,
    /// MagnifyMinus 󰍊
    MagnifyMinus,
    /// MagnifyMinusCursor 󰩢
    MagnifyMinusCursor,
    /// MagnifyMinusOutline 󰛬
    MagnifyMinusOutline,
    /// MagnifyPlus 󰍋
    MagnifyPlus,
    /// MagnifyPlusCursor 󰩣
    MagnifyPlusCursor,
    /// MagnifyPlusOutline 󰛭
    MagnifyPlusOutline,
    /// MagnifyRemoveCursor 󱈌
    MagnifyRemoveCursor,
    /// MagnifyRemoveOutline 󱈍
    MagnifyRemoveOutline,
    /// MagnifyScan 󱉶
    MagnifyScan,
    /// Mail 
    Mail,
    /// MailOne 
    MailOne,
    /// MailRead 
    MailRead,
    /// MailTwo 󰺻
    MailTwo,
    /// Mailbox 󰛮
    Mailbox,
    /// MailboxOpen 󰶈
    MailboxOpen,
    /// MailboxOpenOutline 󰶉
    MailboxOpenOutline,
    /// MailboxOpenUp 󰶊
    MailboxOpenUp,
    /// MailboxOpenUpOutline 󰶋
    MailboxOpenUpOutline,
    /// MailboxOutline 󰶌
    MailboxOutline,
    /// MailboxUp 󰶍
    MailboxUp,
    /// MailboxUpOutline 󰶎
    MailboxUpOutline,
    /// MakeupBrushes 
    MakeupBrushes,
    /// Male 
    Male,
    /// Mandriva 
    Mandriva,
    /// Manjaro 󱘊
    Manjaro,
    /// Map 󰍍
    Map,
    /// MapCheck 󰺼
    MapCheck,
    /// MapCheckOutline 󰺽
    MapCheckOutline,
    /// MapClock 󰴞
    MapClock,
    /// MapClockOutline 󰴟
    MapClockOutline,
    /// MapLegend 󰨁
    MapLegend,
    /// MapMarker 󰍎
    MapMarker,
    /// MapMarkerAccount 󱣣
    MapMarkerAccount,
    /// MapMarkerAccountOutline 󱣤
    MapMarkerAccountOutline,
    /// MapMarkerAlert 󰼅
    MapMarkerAlert,
    /// MapMarkerAlertOutline 󰼆
    MapMarkerAlertOutline,
    /// MapMarkerCheck 󰲕
    MapMarkerCheck,
    /// MapMarkerCheckOutline 󱋻
    MapMarkerCheckOutline,
    /// MapMarkerCircle 󰍏
    MapMarkerCircle,
    /// MapMarkerDistance 󰣰
    MapMarkerDistance,
    /// MapMarkerDown 󱄂
    MapMarkerDown,
    /// MapMarkerLeft 󱋛
    MapMarkerLeft,
    /// MapMarkerLeftOutline 󱋝
    MapMarkerLeftOutline,
    /// MapMarkerMinus 󰙐
    MapMarkerMinus,
    /// MapMarkerMinusOutline 󱋹
    MapMarkerMinusOutline,
    /// MapMarkerMultiple 󰍐
    MapMarkerMultiple,
    /// MapMarkerMultipleOutline 󱉷
    MapMarkerMultipleOutline,
    /// MapMarkerOff 󰍑
    MapMarkerOff,
    /// MapMarkerOffOutline 󱋽
    MapMarkerOffOutline,
    /// MapMarkerOutline 󰟙
    MapMarkerOutline,
    /// MapMarkerPath 󰴠
    MapMarkerPath,
    /// MapMarkerPlus 󰙑
    MapMarkerPlus,
    /// MapMarkerPlusOutline 󱋸
    MapMarkerPlusOutline,
    /// MapMarkerQuestion 󰼇
    MapMarkerQuestion,
    /// MapMarkerQuestionOutline 󰼈
    MapMarkerQuestionOutline,
    /// MapMarkerRadius 󰍒
    MapMarkerRadius,
    /// MapMarkerRadiusOutline 󱋼
    MapMarkerRadiusOutline,
    /// MapMarkerRemove 󰼉
    MapMarkerRemove,
    /// MapMarkerRemoveOutline 󱋺
    MapMarkerRemoveOutline,
    /// MapMarkerRemoveVariant 󰼊
    MapMarkerRemoveVariant,
    /// MapMarkerRight 󱋜
    MapMarkerRight,
    /// MapMarkerRightOutline 󱋞
    MapMarkerRightOutline,
    /// MapMarkerStar 󱘈
    MapMarkerStar,
    /// MapMarkerStarOutline 󱘉
    MapMarkerStarOutline,
    /// MapMarkerUp 󱄃
    MapMarkerUp,
    /// MapMinus 󰦁
    MapMinus,
    /// MapOutline 󰦂
    MapOutline,
    /// MapPlus 󰦃
    MapPlus,
    /// MapSearch 󰦄
    MapSearch,
    /// MapSearchOutline 󰦅
    MapSearchOutline,
    /// Mapbox 󰮪
    Mapbox,
    /// Margin 󰍓
    Margin,
    /// MarkGithub 
    MarkGithub,
    /// Markdown 
    Markdown,
    /// MarkdownOne 
    MarkdownOne,
    /// Marker 󰙒
    Marker,
    /// MarkerCancel 󰷙
    MarkerCancel,
    /// MarkerCheck 󰍕
    MarkerCheck,
    /// Mastodon 󰫑
    Mastodon,
    /// Mate 
    Mate,
    /// MaterialDesign 󰦆
    MaterialDesign,
    /// MaterialUi 󰍗
    MaterialUi,
    /// MathCompass 󰍘
    MathCompass,
    /// MathCos 󰲖
    MathCos,
    /// MathIntegral 󰿈
    MathIntegral,
    /// MathIntegralBox 󰿉
    MathIntegralBox,
    /// MathLog 󱂅
    MathLog,
    /// MathNorm 󰿊
    MathNorm,
    /// MathNormBox 󰿋
    MathNormBox,
    /// MathSin 󰲗
    MathSin,
    /// MathTan 󰲘
    MathTan,
    /// Matrix 󰘨
    Matrix,
    /// Maxcdn 
    Maxcdn,
    /// Maximize 
    Maximize,
    /// Meat 
    Meat,
    /// Medal 󰦇
    Medal,
    /// MedalOutline 󱌦
    MedalOutline,
    /// MedicalBag 󰛯
    MedicalBag,
    /// MedicalCottonSwab 󱪸
    MedicalCottonSwab,
    /// Medicine 
    Medicine,
    /// Meditation 󱅻
    Meditation,
    /// Medkit 
    Medkit,
    /// Megaphone 
    Megaphone,
    /// MegaphoneOne 
    MegaphoneOne,
    /// Meh 
    Meh,
    /// Memory 󰍛
    Memory,
    /// Menorah 󱟔
    Menorah,
    /// MenorahFire 󱟕
    MenorahFire,
    /// Mention 
    Mention,
    /// MentionOne 
    MentionOne,
    /// Menu 
    Menu,
    /// MenuDown 󰍝
    MenuDown,
    /// MenuDownOutline 󰚶
    MenuDownOutline,
    /// MenuLeft 󰍞
    MenuLeft,
    /// MenuLeftOutline 󰨂
    MenuLeftOutline,
    /// MenuOne 󰍜
    MenuOne,
    /// MenuOpen 󰮫
    MenuOpen,
    /// MenuRight 󰍟
    MenuRight,
    /// MenuRightOutline 󰨃
    MenuRightOutline,
    /// MenuSwap 󰩤
    MenuSwap,
    /// MenuSwapOutline 󰩥
    MenuSwapOutline,
    /// MenuUp 󰍠
    MenuUp,
    /// MenuUpOutline 󰚷
    MenuUpOutline,
    /// Merge 
    Merge,
    /// MergeOne 󰽜
    MergeOne,
    /// Message 󰍡
    Message,
    /// MessageAlert 󰍢
    MessageAlert,
    /// MessageAlertOutline 󰨄
    MessageAlertOutline,
    /// MessageArrowLeft 󱋲
    MessageArrowLeft,
    /// MessageArrowLeftOutline 󱋳
    MessageArrowLeftOutline,
    /// MessageArrowRight 󱋴
    MessageArrowRight,
    /// MessageArrowRightOutline 󱋵
    MessageArrowRightOutline,
    /// MessageBadge 󱥁
    MessageBadge,
    /// MessageBadgeOutline 󱥂
    MessageBadgeOutline,
    /// MessageBookmark 󱖬
    MessageBookmark,
    /// MessageBookmarkOutline 󱖭
    MessageBookmarkOutline,
    /// MessageBulleted 󰚢
    MessageBulleted,
    /// MessageBulletedOff 󰚣
    MessageBulletedOff,
    /// MessageCog 󰛱
    MessageCog,
    /// MessageCogOutline 󱅲
    MessageCogOutline,
    /// MessageDraw 󰍣
    MessageDraw,
    /// MessageFast 󱧌
    MessageFast,
    /// MessageFastOutline 󱧍
    MessageFastOutline,
    /// MessageFlash 󱖩
    MessageFlash,
    /// MessageFlashOutline 󱖪
    MessageFlashOutline,
    /// MessageImage 󰍤
    MessageImage,
    /// MessageImageOutline 󱅬
    MessageImageOutline,
    /// MessageLock 󰿌
    MessageLock,
    /// MessageLockOutline 󱅭
    MessageLockOutline,
    /// MessageMinus 󱅮
    MessageMinus,
    /// MessageMinusOutline 󱅯
    MessageMinusOutline,
    /// MessageOff 󱙍
    MessageOff,
    /// MessageOffOutline 󱙎
    MessageOffOutline,
    /// MessageOutline 󰍥
    MessageOutline,
    /// MessagePlus 󰙓
    MessagePlus,
    /// MessagePlusOutline 󱂻
    MessagePlusOutline,
    /// MessageProcessing 󰍦
    MessageProcessing,
    /// MessageProcessingOutline 󱅰
    MessageProcessingOutline,
    /// MessageQuestion 󱜺
    MessageQuestion,
    /// MessageQuestionOutline 󱜻
    MessageQuestionOutline,
    /// MessageReply 󰍧
    MessageReply,
    /// MessageReplyOutline 󱜽
    MessageReplyOutline,
    /// MessageReplyText 󰍨
    MessageReplyText,
    /// MessageReplyTextOutline 󱜾
    MessageReplyTextOutline,
    /// MessageSettings 󰛰
    MessageSettings,
    /// MessageSettingsOutline 󱅱
    MessageSettingsOutline,
    /// MessageStar 󰚚
    MessageStar,
    /// MessageStarOutline 󱉐
    MessageStarOutline,
    /// MessageText 󰍩
    MessageText,
    /// MessageTextClock 󱅳
    MessageTextClock,
    /// MessageTextClockOutline 󱅴
    MessageTextClockOutline,
    /// MessageTextFast 󱧎
    MessageTextFast,
    /// MessageTextFastOutline 󱧏
    MessageTextFastOutline,
    /// MessageTextLock 󰿍
    MessageTextLock,
    /// MessageTextLockOutline 󱅵
    MessageTextLockOutline,
    /// MessageTextOutline 󰍪
    MessageTextOutline,
    /// MessageVideo 󰍫
    MessageVideo,
    /// Meteor 󰘩
    Meteor,
    /// Meter 
    Meter,
    /// MeterElectric 󱩗
    MeterElectric,
    /// MeterElectricOutline 󱩘
    MeterElectricOutline,
    /// MeterGas 󱩙
    MeterGas,
    /// MeterGasOutline 󱩚
    MeterGasOutline,
    /// Metronome 󰟚
    Metronome,
    /// MetronomeTick 󰟛
    MetronomeTick,
    /// MicroSd 󰟜
    MicroSd,
    /// Microphone 
    Microphone,
    /// MicrophoneMinus 󰢳
    MicrophoneMinus,
    /// MicrophoneOff 󰍭
    MicrophoneOff,
    /// MicrophoneOne 󰍬
    MicrophoneOne,
    /// MicrophoneOutline 󰍮
    MicrophoneOutline,
    /// MicrophonePlus 󰢴
    MicrophonePlus,
    /// MicrophoneQuestion 󱦉
    MicrophoneQuestion,
    /// MicrophoneQuestionOutline 󱦊
    MicrophoneQuestionOutline,
    /// MicrophoneSettings 󰍯
    MicrophoneSettings,
    /// MicrophoneVariant 󰍰
    MicrophoneVariant,
    /// MicrophoneVariantOff 󰍱
    MicrophoneVariantOff,
    /// Microscope 
    Microscope,
    /// MicroscopeOne 󰙔
    MicroscopeOne,
    /// Microsoft 󰍲
    Microsoft,
    /// MicrosoftAccess 󱎎
    MicrosoftAccess,
    /// MicrosoftAzure 󰠅
    MicrosoftAzure,
    /// MicrosoftAzureDevops 󰿕
    MicrosoftAzureDevops,
    /// MicrosoftBing 󰂤
    MicrosoftBing,
    /// MicrosoftDynamicsThreesixfive 󰦈
    MicrosoftDynamicsThreesixfive,
    /// MicrosoftEdge 󰇩
    MicrosoftEdge,
    /// MicrosoftExcel 󱎏
    MicrosoftExcel,
    /// MicrosoftInternetExplorer 󰌀
    MicrosoftInternetExplorer,
    /// MicrosoftOffice 󰏆
    MicrosoftOffice,
    /// MicrosoftOnedrive 󰏊
    MicrosoftOnedrive,
    /// MicrosoftOnenote 󰝇
    MicrosoftOnenote,
    /// MicrosoftOutlook 󰴢
    MicrosoftOutlook,
    /// MicrosoftPowerpoint 󱎐
    MicrosoftPowerpoint,
    /// MicrosoftSharepoint 󱎑
    MicrosoftSharepoint,
    /// MicrosoftTeams 󰊻
    MicrosoftTeams,
    /// MicrosoftVisualStudio 󰘐
    MicrosoftVisualStudio,
    /// MicrosoftVisualStudioCode 󰨞
    MicrosoftVisualStudioCode,
    /// MicrosoftWindows 󰖳
    MicrosoftWindows,
    /// MicrosoftWindowsClassic 󰨡
    MicrosoftWindowsClassic,
    /// MicrosoftWord 󱎒
    MicrosoftWord,
    /// MicrosoftXbox 󰖹
    MicrosoftXbox,
    /// MicrosoftXboxController 󰖺
    MicrosoftXboxController,
    /// MicrosoftXboxControllerBatteryAlert 󰝋
    MicrosoftXboxControllerBatteryAlert,
    /// MicrosoftXboxControllerBatteryCharging 󰨢
    MicrosoftXboxControllerBatteryCharging,
    /// MicrosoftXboxControllerBatteryEmpty 󰝌
    MicrosoftXboxControllerBatteryEmpty,
    /// MicrosoftXboxControllerBatteryFull 󰝍
    MicrosoftXboxControllerBatteryFull,
    /// MicrosoftXboxControllerBatteryLow 󰝎
    MicrosoftXboxControllerBatteryLow,
    /// MicrosoftXboxControllerBatteryMedium 󰝏
    MicrosoftXboxControllerBatteryMedium,
    /// MicrosoftXboxControllerBatteryUnknown 󰝐
    MicrosoftXboxControllerBatteryUnknown,
    /// MicrosoftXboxControllerMenu 󰹯
    MicrosoftXboxControllerMenu,
    /// MicrosoftXboxControllerOff 󰖻
    MicrosoftXboxControllerOff,
    /// MicrosoftXboxControllerView 󰹰
    MicrosoftXboxControllerView,
    /// Microwave 󰲙
    Microwave,
    /// MicrowaveOff 󱐣
    MicrowaveOff,
    /// Middleware 󰽝
    Middleware,
    /// MiddlewareOutline 󰽞
    MiddlewareOutline,
    /// Midi 󰣱
    Midi,
    /// MidiPort 󰣲
    MidiPort,
    /// Milestone 
    Milestone,
    /// MilestoneOne 
    MilestoneOne,
    /// MilkBottle 
    MilkBottle,
    /// Mine 󰷚
    Mine,
    /// Minecraft 󰍳
    Minecraft,
    /// MiniSd 󰨅
    MiniSd,
    /// Minidisc 󰨆
    Minidisc,
    /// Minimize 
    Minimize,
    /// Minus 
    Minus,
    /// MinusBox 󰍵
    MinusBox,
    /// MinusBoxMultiple 󱅁
    MinusBoxMultiple,
    /// MinusBoxMultipleOutline 󱅂
    MinusBoxMultipleOutline,
    /// MinusBoxOutline 󰛲
    MinusBoxOutline,
    /// MinusCircle 󰍶
    MinusCircle,
    /// MinusCircleMultiple 󰍚
    MinusCircleMultiple,
    /// MinusCircleMultipleOutline 󰫓
    MinusCircleMultipleOutline,
    /// MinusCircleOff 󱑙
    MinusCircleOff,
    /// MinusCircleOffOutline 󱑚
    MinusCircleOffOutline,
    /// MinusCircleOutline 󰍷
    MinusCircleOutline,
    /// MinusNetwork 󰍸
    MinusNetwork,
    /// MinusNetworkOutline 󰲚
    MinusNetworkOutline,
    /// MinusOne 󰍴
    MinusOne,
    /// MinusSign 
    MinusSign,
    /// MinusSignAlt 
    MinusSignAlt,
    /// MinusThick 󱘹
    MinusThick,
    /// Mirror 
    Mirror,
    /// MirrorOne 
    MirrorOne,
    /// MirrorRectangle 󱞟
    MirrorRectangle,
    /// MirrorTwo 󱇽
    MirrorTwo,
    /// MirrorVariant 󱞠
    MirrorVariant,
    /// MixedMartialArts 󰶏
    MixedMartialArts,
    /// MixedReality 󰡿
    MixedReality,
    /// MobilePhone 
    MobilePhone,
    /// Molecule 
    Molecule,
    /// MoleculeCo 󱋾
    MoleculeCo,
    /// MoleculeCotwo 󰟤
    MoleculeCotwo,
    /// MoleculeOne 󰮬
    MoleculeOne,
    /// Money 
    Money,
    /// Monitor 󰍹
    Monitor,
    /// MonitorAccount 󱩛
    MonitorAccount,
    /// MonitorArrowDown 󱧐
    MonitorArrowDown,
    /// MonitorArrowDownVariant 󱧑
    MonitorArrowDownVariant,
    /// MonitorCellphone 󰦉
    MonitorCellphone,
    /// MonitorCellphoneStar 󰦊
    MonitorCellphoneStar,
    /// MonitorDashboard 󰨇
    MonitorDashboard,
    /// MonitorEdit 󱋆
    MonitorEdit,
    /// MonitorEye 󱎴
    MonitorEye,
    /// MonitorLock 󰷛
    MonitorLock,
    /// MonitorMultiple 󰍺
    MonitorMultiple,
    /// MonitorOff 󰶐
    MonitorOff,
    /// MonitorScreenshot 󰹑
    MonitorScreenshot,
    /// MonitorShare 󱒃
    MonitorShare,
    /// MonitorShimmer 󱄄
    MonitorShimmer,
    /// MonitorSmall 󱡶
    MonitorSmall,
    /// MonitorSpeaker 󰽟
    MonitorSpeaker,
    /// MonitorSpeakerOff 󰽠
    MonitorSpeakerOff,
    /// MonitorStar 󰷜
    MonitorStar,
    /// Moon 
    Moon,
    /// MoonCloud 
    MoonCloud,
    /// MoonFirstQuarter 󰽡
    MoonFirstQuarter,
    /// MoonFull 󰽢
    MoonFull,
    /// MoonLastQuarter 󰽣
    MoonLastQuarter,
    /// MoonNew 󰽤
    MoonNew,
    /// MoonWaningCrescent 󰽥
    MoonWaningCrescent,
    /// MoonWaningGibbous 󰽦
    MoonWaningGibbous,
    /// MoonWaxingCrescent 󰽧
    MoonWaxingCrescent,
    /// MoonWaxingGibbous 󰽨
    MoonWaxingGibbous,
    /// Moped 󱂆
    Moped,
    /// MopedElectric 󱖷
    MopedElectric,
    /// MopedElectricOutline 󱖸
    MopedElectricOutline,
    /// MopedOutline 󱖹
    MopedOutline,
    /// More 󰍻
    More,
    /// MortarBoard 
    MortarBoard,
    /// MortarBoardOne 
    MortarBoardOne,
    /// MortarPestle 󱝈
    MortarPestle,
    /// MortarPestlePlus 󰏱
    MortarPestlePlus,
    /// Mosque 󱠧
    Mosque,
    /// MotherHeart 󱌔
    MotherHeart,
    /// MotherNurse 󰴡
    MotherNurse,
    /// Motion 󱖲
    Motion,
    /// MotionOutline 󱖳
    MotionOutline,
    /// MotionPause 󱖐
    MotionPause,
    /// MotionPauseOutline 󱖒
    MotionPauseOutline,
    /// MotionPlay 󱖏
    MotionPlay,
    /// MotionPlayOutline 󱖑
    MotionPlayOutline,
    /// MotionSensor 󰶑
    MotionSensor,
    /// MotionSensorOff 󱐵
    MotionSensorOff,
    /// Motorbike 󰍼
    Motorbike,
    /// MotorbikeElectric 󱖺
    MotorbikeElectric,
    /// Mountains 
    Mountains,
    /// Mouse 󰍽
    Mouse,
    /// MouseBluetooth 󰦋
    MouseBluetooth,
    /// MouseMoveDown 󱕐
    MouseMoveDown,
    /// MouseMoveUp 󱕑
    MouseMoveUp,
    /// MouseMoveVertical 󱕒
    MouseMoveVertical,
    /// MouseOff 󰍾
    MouseOff,
    /// MouseVariant 󰍿
    MouseVariant,
    /// MouseVariantOff 󰎀
    MouseVariantOff,
    /// Move 
    Move,
    /// MoveOne 
    MoveOne,
    /// MoveResize 󰙕
    MoveResize,
    /// MoveResizeVariant 󰙖
    MoveResizeVariant,
    /// MoveToBottom 
    MoveToBottom,
    /// MoveToEnd 
    MoveToEnd,
    /// MoveToStart 
    MoveToStart,
    /// MoveToTop 
    MoveToTop,
    /// Movie 󰎁
    Movie,
    /// MovieCheck 󱛳
    MovieCheck,
    /// MovieCheckOutline 󱛴
    MovieCheckOutline,
    /// MovieCog 󱛵
    MovieCog,
    /// MovieCogOutline 󱛶
    MovieCogOutline,
    /// MovieEdit 󱄢
    MovieEdit,
    /// MovieEditOutline 󱄣
    MovieEditOutline,
    /// MovieFilter 󱄤
    MovieFilter,
    /// MovieFilterOutline 󱄥
    MovieFilterOutline,
    /// MovieMinus 󱛷
    MovieMinus,
    /// MovieMinusOutline 󱛸
    MovieMinusOutline,
    /// MovieOff 󱛹
    MovieOff,
    /// MovieOffOutline 󱛺
    MovieOffOutline,
    /// MovieOpen 󰿎
    MovieOpen,
    /// MovieOpenCheck 󱛻
    MovieOpenCheck,
    /// MovieOpenCheckOutline 󱛼
    MovieOpenCheckOutline,
    /// MovieOpenCog 󱛽
    MovieOpenCog,
    /// MovieOpenCogOutline 󱛾
    MovieOpenCogOutline,
    /// MovieOpenEdit 󱛿
    MovieOpenEdit,
    /// MovieOpenEditOutline 󱜀
    MovieOpenEditOutline,
    /// MovieOpenMinus 󱜁
    MovieOpenMinus,
    /// MovieOpenMinusOutline 󱜂
    MovieOpenMinusOutline,
    /// MovieOpenOff 󱜃
    MovieOpenOff,
    /// MovieOpenOffOutline 󱜄
    MovieOpenOffOutline,
    /// MovieOpenOutline 󰿏
    MovieOpenOutline,
    /// MovieOpenPlay 󱜅
    MovieOpenPlay,
    /// MovieOpenPlayOutline 󱜆
    MovieOpenPlayOutline,
    /// MovieOpenPlus 󱜇
    MovieOpenPlus,
    /// MovieOpenPlusOutline 󱜈
    MovieOpenPlusOutline,
    /// MovieOpenRemove 󱜉
    MovieOpenRemove,
    /// MovieOpenRemoveOutline 󱜊
    MovieOpenRemoveOutline,
    /// MovieOpenSettings 󱜋
    MovieOpenSettings,
    /// MovieOpenSettingsOutline 󱜌
    MovieOpenSettingsOutline,
    /// MovieOpenStar 󱜍
    MovieOpenStar,
    /// MovieOpenStarOutline 󱜎
    MovieOpenStarOutline,
    /// MovieOutline 󰷝
    MovieOutline,
    /// MoviePlay 󱜏
    MoviePlay,
    /// MoviePlayOutline 󱜐
    MoviePlayOutline,
    /// MoviePlus 󱜑
    MoviePlus,
    /// MoviePlusOutline 󱜒
    MoviePlusOutline,
    /// MovieRemove 󱜓
    MovieRemove,
    /// MovieRemoveOutline 󱜔
    MovieRemoveOutline,
    /// MovieRoll 󰟞
    MovieRoll,
    /// MovieSearch 󱇒
    MovieSearch,
    /// MovieSearchOutline 󱇓
    MovieSearchOutline,
    /// MovieSettings 󱜕
    MovieSettings,
    /// MovieSettingsOutline 󱜖
    MovieSettingsOutline,
    /// MovieStar 󱜗
    MovieStar,
    /// MovieStarOutline 󱜘
    MovieStarOutline,
    /// Mower 󱙯
    Mower,
    /// MowerBag 󱙰
    MowerBag,
    /// Mpv 
    Mpv,
    /// Muffin 󰦌
    Muffin,
    /// MultiSelect 
    MultiSelect,
    /// Multicast 󱢓
    Multicast,
    /// MultipleWindows 
    MultipleWindows,
    /// Multiplication 󰎂
    Multiplication,
    /// MultiplicationBox 󰎃
    MultiplicationBox,
    /// Mushroom 
    Mushroom,
    /// MushroomOff 󱏺
    MushroomOff,
    /// MushroomOffOutline 󱏻
    MushroomOffOutline,
    /// MushroomOne 󰟟
    MushroomOne,
    /// MushroomOutline 󰟠
    MushroomOutline,
    /// Music 
    Music,
    /// MusicAccidentalDoubleFlat 󰽩
    MusicAccidentalDoubleFlat,
    /// MusicAccidentalDoubleSharp 󰽪
    MusicAccidentalDoubleSharp,
    /// MusicAccidentalFlat 󰽫
    MusicAccidentalFlat,
    /// MusicAccidentalNatural 󰽬
    MusicAccidentalNatural,
    /// MusicAccidentalSharp 󰽭
    MusicAccidentalSharp,
    /// MusicBox 󰎄
    MusicBox,
    /// MusicBoxMultiple 󰌳
    MusicBoxMultiple,
    /// MusicBoxMultipleOutline 󰼄
    MusicBoxMultipleOutline,
    /// MusicBoxOutline 󰎅
    MusicBoxOutline,
    /// MusicCircle 󰎆
    MusicCircle,
    /// MusicCircleOutline 󰫔
    MusicCircleOutline,
    /// MusicClefAlto 󰽮
    MusicClefAlto,
    /// MusicClefBass 󰽯
    MusicClefBass,
    /// MusicClefTreble 󰽰
    MusicClefTreble,
    /// MusicNote 󰎇
    MusicNote,
    /// MusicNoteBluetooth 󰗾
    MusicNoteBluetooth,
    /// MusicNoteBluetoothOff 󰗿
    MusicNoteBluetoothOff,
    /// MusicNoteEighthDotted 󰽱
    MusicNoteEighthDotted,
    /// MusicNoteHalf 󰎉
    MusicNoteHalf,
    /// MusicNoteHalfDotted 󰽲
    MusicNoteHalfDotted,
    /// MusicNoteOff 󰎊
    MusicNoteOff,
    /// MusicNoteOffOutline 󰽳
    MusicNoteOffOutline,
    /// MusicNoteOne 󰎈
    MusicNoteOne,
    /// MusicNoteOutline 󰽴
    MusicNoteOutline,
    /// MusicNotePlus 󰷞
    MusicNotePlus,
    /// MusicNoteQuarter 󰎋
    MusicNoteQuarter,
    /// MusicNoteQuarterDotted 󰽵
    MusicNoteQuarterDotted,
    /// MusicNoteSixteenth 󰎌
    MusicNoteSixteenth,
    /// MusicNoteSixteenthDotted 󰽶
    MusicNoteSixteenthDotted,
    /// MusicNoteWhole 󰎍
    MusicNoteWhole,
    /// MusicNoteWholeDotted 󰽷
    MusicNoteWholeDotted,
    /// MusicOff 󰝛
    MusicOff,
    /// MusicOne 󰝚
    MusicOne,
    /// MusicRestEighth 󰽸
    MusicRestEighth,
    /// MusicRestHalf 󰽹
    MusicRestHalf,
    /// MusicRestQuarter 󰽺
    MusicRestQuarter,
    /// MusicRestSixteenth 󰽻
    MusicRestSixteenth,
    /// MusicRestWhole 󰽼
    MusicRestWhole,
    /// Mustache 
    Mustache,
    /// MustacheOne 󱗞
    MustacheOne,
    /// Mute 
    Mute,
    /// MuteOne 
    MuteOne,
    /// MxLinux 
    MxLinux,
    /// Mysql 
    Mysql,
    /// Nail 󰷟
    Nail,
    /// Nas 󰣳
    Nas,
    /// Nativescript 󰢀
    Nativescript,
    /// Nature 󰎎
    Nature,
    /// NaturePeople 󰎏
    NaturePeople,
    /// Navigation 󰎐
    Navigation,
    /// NavigationOutline 󱘇
    NavigationOutline,
    /// NavigationVariantOutline 󱣱
    NavigationVariantOutline,
    /// NearMe 󰗍
    NearMe,
    /// NearMeOne 󱣰
    NearMeOne,
    /// Necklace 󰼋
    Necklace,
    /// Needle 󰎑
    Needle,
    /// NeedleOff 󱧒
    NeedleOff,
    /// Neovim 
    Neovim,
    /// Netflix 󰝆
    Netflix,
    /// Network 󰛳
    Network,
    /// NetworkOff 󰲛
    NetworkOff,
    /// NetworkOffOutline 󰲜
    NetworkOffOutline,
    /// NetworkOutline 󰲝
    NetworkOutline,
    /// NetworkPos 󱫋
    NetworkPos,
    /// NetworkStrengthFour 󰣺
    NetworkStrengthFour,
    /// NetworkStrengthFourAlert 󰣻
    NetworkStrengthFourAlert,
    /// NetworkStrengthFourCog 󱤚
    NetworkStrengthFourCog,
    /// NetworkStrengthOff 󰣼
    NetworkStrengthOff,
    /// NetworkStrengthOffOutline 󰣽
    NetworkStrengthOffOutline,
    /// NetworkStrengthOne 󰣴
    NetworkStrengthOne,
    /// NetworkStrengthOneAlert 󰣵
    NetworkStrengthOneAlert,
    /// NetworkStrengthOutline 󰣾
    NetworkStrengthOutline,
    /// NetworkStrengthThree 󰣸
    NetworkStrengthThree,
    /// NetworkStrengthThreeAlert 󰣹
    NetworkStrengthThreeAlert,
    /// NetworkStrengthTwo 󰣶
    NetworkStrengthTwo,
    /// NetworkStrengthTwoAlert 󰣷
    NetworkStrengthTwoAlert,
    /// NewBox 󰎔
    NewBox,
    /// NewFile 
    NewFile,
    /// NewFolder 
    NewFolder,
    /// Newline 
    Newline,
    /// Newspaper 󰎕
    Newspaper,
    /// NewspaperCheck 󱥃
    NewspaperCheck,
    /// NewspaperMinus 󰼌
    NewspaperMinus,
    /// NewspaperPlus 󰼍
    NewspaperPlus,
    /// NewspaperRemove 󱥄
    NewspaperRemove,
    /// NewspaperVariant 󱀁
    NewspaperVariant,
    /// NewspaperVariantMultiple 󱀂
    NewspaperVariantMultiple,
    /// NewspaperVariantMultipleOutline 󱀃
    NewspaperVariantMultipleOutline,
    /// NewspaperVariantOutline 󱀄
    NewspaperVariantOutline,
    /// Nfc 󰎖
    Nfc,
    /// NfcSearchVariant 󰹓
    NfcSearchVariant,
    /// NfcTap 󰎗
    NfcTap,
    /// NfcVariant 󰎘
    NfcVariant,
    /// NfcVariantOff 󰹔
    NfcVariantOff,
    /// Ninja 󰝴
    Ninja,
    /// Nintendo 
    Nintendo,
    /// NintendoGameBoy 󱎓
    NintendoGameBoy,
    /// NintendoSwitch 󰟡
    NintendoSwitch,
    /// NintendoWii 󰖫
    NintendoWii,
    /// NintendoWiiu 󰜭
    NintendoWiiu,
    /// Nix 󱄅
    Nix,
    /// Nixos 
    Nixos,
    /// NoEntry 
    NoEntry,
    /// NoNewline 
    NoNewline,
    /// Nodejs 󰎙
    Nodejs,
    /// Nonmarkingreturn
    Nonmarkingreturn,
    /// Noodles 󱅾
    Noodles,
    /// NorthStar 
    NorthStar,
    /// NotEqual 󰦍
    NotEqual,
    /// NotEqualVariant 󰦎
    NotEqualVariant,
    /// Note 
    Note,
    /// NoteAlert 󱝽
    NoteAlert,
    /// NoteAlertOutline 󱝾
    NoteAlertOutline,
    /// NoteCheck 󱝿
    NoteCheck,
    /// NoteCheckOutline 󱞀
    NoteCheckOutline,
    /// NoteEdit 󱞁
    NoteEdit,
    /// NoteEditOutline 󱞂
    NoteEditOutline,
    /// NoteMinus 󱙏
    NoteMinus,
    /// NoteMinusOutline 󱙐
    NoteMinusOutline,
    /// NoteMultiple 󰚸
    NoteMultiple,
    /// NoteMultipleOutline 󰚹
    NoteMultipleOutline,
    /// NoteOff 󱞃
    NoteOff,
    /// NoteOffOutline 󱞄
    NoteOffOutline,
    /// NoteOne 
    NoteOne,
    /// NoteOutline 󰎛
    NoteOutline,
    /// NotePlus 󰎜
    NotePlus,
    /// NotePlusOutline 󰎝
    NotePlusOutline,
    /// NoteRemove 󱙑
    NoteRemove,
    /// NoteRemoveOutline 󱙒
    NoteRemoveOutline,
    /// NoteSearch 󱙓
    NoteSearch,
    /// NoteSearchOutline 󱙔
    NoteSearchOutline,
    /// NoteText 󰎞
    NoteText,
    /// NoteTextOutline 󱇗
    NoteTextOutline,
    /// NoteTwo 󰎚
    NoteTwo,
    /// Notebook 
    Notebook,
    /// NotebookCheck 󱓵
    NotebookCheck,
    /// NotebookCheckOutline 󱓶
    NotebookCheckOutline,
    /// NotebookEdit 󱓧
    NotebookEdit,
    /// NotebookEditOutline 󱓩
    NotebookEditOutline,
    /// NotebookHeart 󱨋
    NotebookHeart,
    /// NotebookHeartOutline 󱨌
    NotebookHeartOutline,
    /// NotebookMinus 󱘐
    NotebookMinus,
    /// NotebookMinusOutline 󱘑
    NotebookMinusOutline,
    /// NotebookMultiple 󰹕
    NotebookMultiple,
    /// NotebookOne 󰠮
    NotebookOne,
    /// NotebookOutline 󰺿
    NotebookOutline,
    /// NotebookPlus 󱘒
    NotebookPlus,
    /// NotebookPlusOutline 󱘓
    NotebookPlusOutline,
    /// NotebookRemove 󱘔
    NotebookRemove,
    /// NotebookRemoveOutline 󱘕
    NotebookRemoveOutline,
    /// NotebookTemplate 
    NotebookTemplate,
    /// NotificationClearAll 󰎟
    NotificationClearAll,
    /// Npm 󰛷
    Npm,
    /// Nuke 󰚤
    Nuke,
    /// Null 󰟢
    Null,
    /// Number 
    Number,
    /// Numeric 󰎠
    Numeric,
    /// NumericEight 󰭁
    NumericEight,
    /// NumericEightBox 󰎹
    NumericEightBox,
    /// NumericEightBoxMultiple 󰼖
    NumericEightBoxMultiple,
    /// NumericEightBoxMultipleOutline 󰎺
    NumericEightBoxMultipleOutline,
    /// NumericEightBoxOutline 󰎻
    NumericEightBoxOutline,
    /// NumericEightCircle 󰲮
    NumericEightCircle,
    /// NumericEightCircleOutline 󰲯
    NumericEightCircleOutline,
    /// NumericFive 󰬾
    NumericFive,
    /// NumericFiveBox 󰎱
    NumericFiveBox,
    /// NumericFiveBoxMultiple 󰼓
    NumericFiveBoxMultiple,
    /// NumericFiveBoxMultipleOutline 󰎯
    NumericFiveBoxMultipleOutline,
    /// NumericFiveBoxOutline 󰎰
    NumericFiveBoxOutline,
    /// NumericFiveCircle 󰲨
    NumericFiveCircle,
    /// NumericFiveCircleOutline 󰲩
    NumericFiveCircleOutline,
    /// NumericFour 󰬽
    NumericFour,
    /// NumericFourBox 󰎭
    NumericFourBox,
    /// NumericFourBoxMultiple 󰼒
    NumericFourBoxMultiple,
    /// NumericFourBoxMultipleOutline 󰎲
    NumericFourBoxMultipleOutline,
    /// NumericFourBoxOutline 󰎮
    NumericFourBoxOutline,
    /// NumericFourCircle 󰲦
    NumericFourCircle,
    /// NumericFourCircleOutline 󰲧
    NumericFourCircleOutline,
    /// NumericNegativeOne 󱁒
    NumericNegativeOne,
    /// NumericNine 󰭂
    NumericNine,
    /// NumericNineBox 󰎼
    NumericNineBox,
    /// NumericNineBoxMultiple 󰼗
    NumericNineBoxMultiple,
    /// NumericNineBoxMultipleOutline 󰎽
    NumericNineBoxMultipleOutline,
    /// NumericNineBoxOutline 󰎾
    NumericNineBoxOutline,
    /// NumericNineCircle 󰲰
    NumericNineCircle,
    /// NumericNineCircleOutline 󰲱
    NumericNineCircleOutline,
    /// NumericNinePlus 󰿮
    NumericNinePlus,
    /// NumericNinePlusBox 󰎿
    NumericNinePlusBox,
    /// NumericNinePlusBoxMultiple 󰼘
    NumericNinePlusBoxMultiple,
    /// NumericNinePlusBoxMultipleOutline 󰏀
    NumericNinePlusBoxMultipleOutline,
    /// NumericNinePlusBoxOutline 󰏁
    NumericNinePlusBoxOutline,
    /// NumericNinePlusCircle 󰲲
    NumericNinePlusCircle,
    /// NumericNinePlusCircleOutline 󰲳
    NumericNinePlusCircleOutline,
    /// NumericOff 󱧓
    NumericOff,
    /// NumericOne 󰬺
    NumericOne,
    /// NumericOneBox 󰎤
    NumericOneBox,
    /// NumericOneBoxMultiple 󰼏
    NumericOneBoxMultiple,
    /// NumericOneBoxMultipleOutline 󰎥
    NumericOneBoxMultipleOutline,
    /// NumericOneBoxOutline 󰎦
    NumericOneBoxOutline,
    /// NumericOneCircle 󰲠
    NumericOneCircle,
    /// NumericOneCircleOutline 󰲡
    NumericOneCircleOutline,
    /// NumericOnezero 󰿩
    NumericOnezero,
    /// NumericOnezeroBox 󰽽
    NumericOnezeroBox,
    /// NumericOnezeroBoxMultiple 󰿪
    NumericOnezeroBoxMultiple,
    /// NumericOnezeroBoxMultipleOutline 󰿫
    NumericOnezeroBoxMultipleOutline,
    /// NumericOnezeroBoxOutline 󰽾
    NumericOnezeroBoxOutline,
    /// NumericOnezeroCircle 󰿬
    NumericOnezeroCircle,
    /// NumericOnezeroCircleOutline 󰿭
    NumericOnezeroCircleOutline,
    /// NumericPositiveOne 󱗋
    NumericPositiveOne,
    /// NumericSeven 󰭀
    NumericSeven,
    /// NumericSevenBox 󰎶
    NumericSevenBox,
    /// NumericSevenBoxMultiple 󰼕
    NumericSevenBoxMultiple,
    /// NumericSevenBoxMultipleOutline 󰎷
    NumericSevenBoxMultipleOutline,
    /// NumericSevenBoxOutline 󰎸
    NumericSevenBoxOutline,
    /// NumericSevenCircle 󰲬
    NumericSevenCircle,
    /// NumericSevenCircleOutline 󰲭
    NumericSevenCircleOutline,
    /// NumericSix 󰬿
    NumericSix,
    /// NumericSixBox 󰎳
    NumericSixBox,
    /// NumericSixBoxMultiple 󰼔
    NumericSixBoxMultiple,
    /// NumericSixBoxMultipleOutline 󰎴
    NumericSixBoxMultipleOutline,
    /// NumericSixBoxOutline 󰎵
    NumericSixBoxOutline,
    /// NumericSixCircle 󰲪
    NumericSixCircle,
    /// NumericSixCircleOutline 󰲫
    NumericSixCircleOutline,
    /// NumericThree 󰬼
    NumericThree,
    /// NumericThreeBox 󰎪
    NumericThreeBox,
    /// NumericThreeBoxMultiple 󰼑
    NumericThreeBoxMultiple,
    /// NumericThreeBoxMultipleOutline 󰎫
    NumericThreeBoxMultipleOutline,
    /// NumericThreeBoxOutline 󰎬
    NumericThreeBoxOutline,
    /// NumericThreeCircle 󰲤
    NumericThreeCircle,
    /// NumericThreeCircleOutline 󰲥
    NumericThreeCircleOutline,
    /// NumericTwo 󰬻
    NumericTwo,
    /// NumericTwoBox 󰎧
    NumericTwoBox,
    /// NumericTwoBoxMultiple 󰼐
    NumericTwoBoxMultiple,
    /// NumericTwoBoxMultipleOutline 󰎨
    NumericTwoBoxMultipleOutline,
    /// NumericTwoBoxOutline 󰎩
    NumericTwoBoxOutline,
    /// NumericTwoCircle 󰲢
    NumericTwoCircle,
    /// NumericTwoCircleOutline 󰲣
    NumericTwoCircleOutline,
    /// NumericZeroBox 󰎡
    NumericZeroBox,
    /// NumericZeroBoxMultiple 󰼎
    NumericZeroBoxMultiple,
    /// NumericZeroBoxMultipleOutline 󰎢
    NumericZeroBoxMultipleOutline,
    /// NumericZeroBoxOutline 󰎣
    NumericZeroBoxOutline,
    /// Nut 󰛸
    Nut,
    /// Nutrition 󰏂
    Nutrition,
    /// Nuxt 󱄆
    Nuxt,
    /// Oar 󰙼
    Oar,
    /// Ocarina 󰷠
    Ocarina,
    /// Oci 󱋩
    Oci,
    /// Ocr 󱄺
    Ocr,
    /// Octagon 󰏃
    Octagon,
    /// OctagonOutline 󰏄
    OctagonOutline,
    /// Octagram 󰛹
    Octagram,
    /// OctagramOutline 󰝵
    OctagramOutline,
    /// Octahedron 󱥐
    Octahedron,
    /// OctahedronOff 󱥑
    OctahedronOff,
    /// Octoface 
    Octoface,
    /// Octoprint 
    Octoprint,
    /// Odnoklassniki 󰏅
    Odnoklassniki,
    /// Off 
    Off,
    /// Offer 󱈛
    Offer,
    /// OfficeBuilding 󰦑
    OfficeBuilding,
    /// OfficeBuildingCog 󱥉
    OfficeBuildingCog,
    /// OfficeBuildingCogOutline 󱥊
    OfficeBuildingCogOutline,
    /// OfficeBuildingMarker 󱔠
    OfficeBuildingMarker,
    /// OfficeBuildingMarkerOutline 󱔡
    OfficeBuildingMarkerOutline,
    /// OfficeBuildingOutline 󱔟
    OfficeBuildingOutline,
    /// Oil 󰏇
    Oil,
    /// OilLamp 󰼙
    OilLamp,
    /// OilLevel 󱁓
    OilLevel,
    /// OilTemperature 󰿸
    OilTemperature,
    /// Ok 
    Ok,
    /// OkCircle 
    OkCircle,
    /// OkSign 
    OkSign,
    /// Ol 
    Ol,
    /// Om 󰥳
    Om,
    /// Omega 󰏉
    Omega,
    /// OneUp 󰮭
    OneUp,
    /// Onepassword 󰢁
    Onepassword,
    /// Opacity 󰗌
    Opacity,
    /// OpenInApp 󰏋
    OpenInApp,
    /// OpenInNew 󰏌
    OpenInNew,
    /// OpenPreview 
    OpenPreview,
    /// OpenSourceInitiative 󰮮
    OpenSourceInitiative,
    /// Openbsd 
    Openbsd,
    /// Openid 󰏍
    Openid,
    /// Openscad 
    Openscad,
    /// Opensuse 
    Opensuse,
    /// Opera 󰏎
    Opera,
    /// Orange 
    Orange,
    /// Orbit 󰀘
    Orbit,
    /// OrbitVariant 󱗛
    OrbitVariant,
    /// OrderAlphabeticalAscending 󰈍
    OrderAlphabeticalAscending,
    /// OrderAlphabeticalDescending 󰴇
    OrderAlphabeticalDescending,
    /// OrderBoolAscending 󰊾
    OrderBoolAscending,
    /// OrderBoolAscendingVariant 󰦏
    OrderBoolAscendingVariant,
    /// OrderBoolDescending 󱎄
    OrderBoolDescending,
    /// OrderBoolDescendingVariant 󰦐
    OrderBoolDescendingVariant,
    /// OrderNumericAscending 󰕅
    OrderNumericAscending,
    /// OrderNumericDescending 󰕆
    OrderNumericDescending,
    /// Organization 
    Organization,
    /// OrganizationOne 
    OrganizationOne,
    /// Origin 󰭃
    Origin,
    /// Ornament 󰏏
    Ornament,
    /// OrnamentVariant 󰏐
    OrnamentVariant,
    /// Osh 
    Osh,
    /// Oshwa 
    Oshwa,
    /// Osi 
    Osi,
    /// OutdoorLamp 󱁔
    OutdoorLamp,
    /// Output 
    Output,
    /// Overscan 󱀅
    Overscan,
    /// Owl 󰏒
    Owl,
    /// PacMan 󰮯
    PacMan,
    /// Package 
    Package,
    /// PackageDependencies 
    PackageDependencies,
    /// PackageDependents 
    PackageDependents,
    /// PackageDown 󰏔
    PackageDown,
    /// PackageOne 
    PackageOne,
    /// PackageTwo 󰏓
    PackageTwo,
    /// PackageUp 󰏕
    PackageUp,
    /// PackageVariant 󰏖
    PackageVariant,
    /// PackageVariantClosed 󰏗
    PackageVariantClosed,
    /// PackageVariantClosedMinus 󱧔
    PackageVariantClosedMinus,
    /// PackageVariantClosedPlus 󱧕
    PackageVariantClosedPlus,
    /// PackageVariantClosedRemove 󱧖
    PackageVariantClosedRemove,
    /// PackageVariantMinus 󱧗
    PackageVariantMinus,
    /// PackageVariantPlus 󱧘
    PackageVariantPlus,
    /// PackageVariantRemove 󱧙
    PackageVariantRemove,
    /// PageFirst 󰘀
    PageFirst,
    /// PageLast 󰘁
    PageLast,
    /// PageLayoutBody 󰛺
    PageLayoutBody,
    /// PageLayoutFooter 󰛻
    PageLayoutFooter,
    /// PageLayoutHeader 󰛼
    PageLayoutHeader,
    /// PageLayoutHeaderFooter 󰽿
    PageLayoutHeaderFooter,
    /// PageLayoutSidebarLeft 󰛽
    PageLayoutSidebarLeft,
    /// PageLayoutSidebarRight 󰛾
    PageLayoutSidebarRight,
    /// PageNext 󰮰
    PageNext,
    /// PageNextOutline 󰮱
    PageNextOutline,
    /// PagePrevious 󰮲
    PagePrevious,
    /// PagePreviousOutline 󰮳
    PagePreviousOutline,
    /// Pail 󱐗
    Pail,
    /// PailMinus 󱐷
    PailMinus,
    /// PailMinusOutline 󱐼
    PailMinusOutline,
    /// PailOff 󱐹
    PailOff,
    /// PailOffOutline 󱐾
    PailOffOutline,
    /// PailOutline 󱐺
    PailOutline,
    /// PailPlus 󱐶
    PailPlus,
    /// PailPlusOutline 󱐻
    PailPlusOutline,
    /// PailRemove 󱐸
    PailRemove,
    /// PailRemoveOutline 󱐽
    PailRemoveOutline,
    /// Paintbrush 
    Paintbrush,
    /// Paintcan 
    Paintcan,
    /// PairProgramming 
    PairProgramming,
    /// Palette 󰏘
    Palette,
    /// PaletteAdvanced 󰏙
    PaletteAdvanced,
    /// PaletteColor 
    PaletteColor,
    /// PaletteOutline 󰸌
    PaletteOutline,
    /// PaletteSwatch 󰢵
    PaletteSwatch,
    /// PaletteSwatchOutline 󱍜
    PaletteSwatchOutline,
    /// PaletteSwatchVariant 󱥚
    PaletteSwatchVariant,
    /// PalmTree 󱁕
    PalmTree,
    /// Pan 󰮴
    Pan,
    /// PanBottomLeft 󰮵
    PanBottomLeft,
    /// PanBottomRight 󰮶
    PanBottomRight,
    /// PanDown 󰮷
    PanDown,
    /// PanHorizontal 󰮸
    PanHorizontal,
    /// PanLeft 󰮹
    PanLeft,
    /// PanRight 󰮺
    PanRight,
    /// PanTopLeft 󰮻
    PanTopLeft,
    /// PanTopRight 󰮼
    PanTopRight,
    /// PanUp 󰮽
    PanUp,
    /// PanVertical 󰮾
    PanVertical,
    /// Panda 󰏚
    Panda,
    /// Pandora 󰏛
    Pandora,
    /// Panorama 󰏜
    Panorama,
    /// PanoramaFisheye 󰏝
    PanoramaFisheye,
    /// PanoramaHorizontal 󱤨
    PanoramaHorizontal,
    /// PanoramaHorizontalOutline 󰏞
    PanoramaHorizontalOutline,
    /// PanoramaOutline 󱦌
    PanoramaOutline,
    /// PanoramaSphere 󱦍
    PanoramaSphere,
    /// PanoramaSphereOutline 󱦎
    PanoramaSphereOutline,
    /// PanoramaVariant 󱦏
    PanoramaVariant,
    /// PanoramaVariantOutline 󱦐
    PanoramaVariantOutline,
    /// PanoramaVertical 󱤩
    PanoramaVertical,
    /// PanoramaVerticalOutline 󰏟
    PanoramaVerticalOutline,
    /// PanoramaWideAngle 󱥟
    PanoramaWideAngle,
    /// PanoramaWideAngleOutline 󰏠
    PanoramaWideAngleOutline,
    /// PaperAirplane 
    PaperAirplane,
    /// PaperClip 
    PaperClip,
    /// PaperCutVertical 󰏡
    PaperCutVertical,
    /// PaperRoll 󱅗
    PaperRoll,
    /// PaperRollOutline 󱅘
    PaperRollOutline,
    /// Paperclip 
    Paperclip,
    /// PaperclipCheck 󱫆
    PaperclipCheck,
    /// PaperclipLock 󱧚
    PaperclipLock,
    /// PaperclipMinus 󱫇
    PaperclipMinus,
    /// PaperclipOff 󱫈
    PaperclipOff,
    /// PaperclipOne 󰏢
    PaperclipOne,
    /// PaperclipPlus 󱫉
    PaperclipPlus,
    /// PaperclipRemove 󱫊
    PaperclipRemove,
    /// ParabolaGnuLinuxLibre 
    ParabolaGnuLinuxLibre,
    /// Parachute 󰲴
    Parachute,
    /// ParachuteOutline 󰲵
    ParachuteOutline,
    /// Paragliding 󱝅
    Paragliding,
    /// Parking 󰏣
    Parking,
    /// ParrotOs 
    ParrotOs,
    /// PartyPopper 󱁖
    PartyPopper,
    /// Pass 
    Pass,
    /// PassFilled 
    PassFilled,
    /// PasskeyFill 
    PasskeyFill,
    /// Passport 󰟣
    Passport,
    /// PassportBiometric 󰷡
    PassportBiometric,
    /// Pasta 󱅠
    Pasta,
    /// Paste 
    Paste,
    /// PasteOne 
    PasteOne,
    /// PatioHeater 󰾀
    PatioHeater,
    /// Patreon 󰢂
    Patreon,
    /// Pause 
    Pause,
    /// PauseCircle 󰏥
    PauseCircle,
    /// PauseCircleOutline 󰏦
    PauseCircleOutline,
    /// PauseOctagon 󰏧
    PauseOctagon,
    /// PauseOctagonOutline 󰏨
    PauseOctagonOutline,
    /// PauseOne 󰏤
    PauseOne,
    /// Paw 󰏩
    Paw,
    /// PawOff 󰙗
    PawOff,
    /// PawOffOutline 󱙶
    PawOffOutline,
    /// PawOutline 󱙵
    PawOutline,
    /// Peace 󰢄
    Peace,
    /// Peach 
    Peach,
    /// Peanut 󰿼
    Peanut,
    /// PeanutOff 󰿽
    PeanutOff,
    /// PeanutOffOutline 󰿿
    PeanutOffOutline,
    /// PeanutOutline 󰿾
    PeanutOutline,
    /// Pear 
    Pear,
    /// Pen 󰏪
    Pen,
    /// PenLock 󰷢
    PenLock,
    /// PenMinus 󰷣
    PenMinus,
    /// PenOff 󰷤
    PenOff,
    /// PenPlus 󰷥
    PenPlus,
    /// PenRemove 󰷦
    PenRemove,
    /// Pencil 
    Pencil,
    /// PencilBox 󰏬
    PencilBox,
    /// PencilBoxMultiple 󱅄
    PencilBoxMultiple,
    /// PencilBoxMultipleOutline 󱅅
    PencilBoxMultipleOutline,
    /// PencilBoxOutline 󰏭
    PencilBoxOutline,
    /// PencilCircle 󰛿
    PencilCircle,
    /// PencilCircleOutline 󰝶
    PencilCircleOutline,
    /// PencilLock 󰏮
    PencilLock,
    /// PencilLockOutline 󰷧
    PencilLockOutline,
    /// PencilMinus 󰷨
    PencilMinus,
    /// PencilMinusOutline 󰷩
    PencilMinusOutline,
    /// PencilOff 󰏯
    PencilOff,
    /// PencilOffOutline 󰷪
    PencilOffOutline,
    /// PencilOne 
    PencilOne,
    /// PencilOutline 󰲶
    PencilOutline,
    /// PencilPlus 󰷫
    PencilPlus,
    /// PencilPlusOutline 󰷬
    PencilPlusOutline,
    /// PencilRemove 󰷭
    PencilRemove,
    /// PencilRemoveOutline 󰷮
    PencilRemoveOutline,
    /// PencilRuler 󱍓
    PencilRuler,
    /// PencilTwo 󰏫
    PencilTwo,
    /// Penguin 󰻀
    Penguin,
    /// Pentagon 󰜁
    Pentagon,
    /// PentagonOutline 󰜀
    PentagonOutline,
    /// Pentagram 󱙧
    Pentagram,
    /// People 
    People,
    /// Percent 󰏰
    Percent,
    /// PercentBox 󱨂
    PercentBox,
    /// PercentBoxOutline 󱨃
    PercentBoxOutline,
    /// PercentCircle 󱨄
    PercentCircle,
    /// PercentCircleOutline 󱨅
    PercentCircleOutline,
    /// PercentOutline 󱉸
    PercentOutline,
    /// PeriodicTable 󰢶
    PeriodicTable,
    /// Person 
    Person,
    /// PersonAdd 
    PersonAdd,
    /// PersonAddOne 
    PersonAddOne,
    /// PersonFill 
    PersonFill,
    /// PersonOne 
    PersonOne,
    /// PerspectiveLess 󰴣
    PerspectiveLess,
    /// PerspectiveMore 󰴤
    PerspectiveMore,
    /// Ph 󱟅
    Ph,
    /// Phone 
    Phone,
    /// PhoneAlert 󰼚
    PhoneAlert,
    /// PhoneAlertOutline 󱆎
    PhoneAlertOutline,
    /// PhoneBluetooth 󰏳
    PhoneBluetooth,
    /// PhoneBluetoothOutline 󱆏
    PhoneBluetoothOutline,
    /// PhoneCancel 󱂼
    PhoneCancel,
    /// PhoneCancelOutline 󱆐
    PhoneCancelOutline,
    /// PhoneCheck 󱆩
    PhoneCheck,
    /// PhoneCheckOutline 󱆪
    PhoneCheckOutline,
    /// PhoneClassic 󰘂
    PhoneClassic,
    /// PhoneClassicOff 󱉹
    PhoneClassicOff,
    /// PhoneClock 󱧛
    PhoneClock,
    /// PhoneDial 󱕙
    PhoneDial,
    /// PhoneDialOutline 󱕚
    PhoneDialOutline,
    /// PhoneForward 󰏴
    PhoneForward,
    /// PhoneForwardOutline 󱆑
    PhoneForwardOutline,
    /// PhoneHangup 󰏵
    PhoneHangup,
    /// PhoneHangupOutline 󱆒
    PhoneHangupOutline,
    /// PhoneInTalk 󰏶
    PhoneInTalk,
    /// PhoneInTalkOutline 󱆂
    PhoneInTalkOutline,
    /// PhoneIncoming 󰏷
    PhoneIncoming,
    /// PhoneIncomingOutline 󱆓
    PhoneIncomingOutline,
    /// PhoneLock 󰏸
    PhoneLock,
    /// PhoneLockOutline 󱆔
    PhoneLockOutline,
    /// PhoneLog 󰏹
    PhoneLog,
    /// PhoneLogOutline 󱆕
    PhoneLogOutline,
    /// PhoneMessage 󱆖
    PhoneMessage,
    /// PhoneMessageOutline 󱆗
    PhoneMessageOutline,
    /// PhoneMinus 󰙘
    PhoneMinus,
    /// PhoneMinusOutline 󱆘
    PhoneMinusOutline,
    /// PhoneMissed 󰏺
    PhoneMissed,
    /// PhoneMissedOutline 󱆥
    PhoneMissedOutline,
    /// PhoneOff 󰷯
    PhoneOff,
    /// PhoneOffOutline 󱆦
    PhoneOffOutline,
    /// PhoneOne 󰏲
    PhoneOne,
    /// PhoneOutgoing 󰏻
    PhoneOutgoing,
    /// PhoneOutgoingOutline 󱆙
    PhoneOutgoingOutline,
    /// PhoneOutline 󰷰
    PhoneOutline,
    /// PhonePaused 󰏼
    PhonePaused,
    /// PhonePausedOutline 󱆚
    PhonePausedOutline,
    /// PhonePlus 󰙙
    PhonePlus,
    /// PhonePlusOutline 󱆛
    PhonePlusOutline,
    /// PhoneRefresh 󱦓
    PhoneRefresh,
    /// PhoneRefreshOutline 󱦔
    PhoneRefreshOutline,
    /// PhoneRemove 󱔯
    PhoneRemove,
    /// PhoneRemoveOutline 󱔰
    PhoneRemoveOutline,
    /// PhoneReturn 󰠯
    PhoneReturn,
    /// PhoneReturnOutline 󱆜
    PhoneReturnOutline,
    /// PhoneRing 󱆫
    PhoneRing,
    /// PhoneRingOutline 󱆬
    PhoneRingOutline,
    /// PhoneRotateLandscape 󰢅
    PhoneRotateLandscape,
    /// PhoneRotatePortrait 󰢆
    PhoneRotatePortrait,
    /// PhoneSettings 󰏽
    PhoneSettings,
    /// PhoneSettingsOutline 󱆝
    PhoneSettingsOutline,
    /// PhoneSign 
    PhoneSign,
    /// PhoneSync 󱦕
    PhoneSync,
    /// PhoneSyncOutline 󱦖
    PhoneSyncOutline,
    /// PhoneVoip 󰏾
    PhoneVoip,
    /// Pi 
    Pi,
    /// PiBox 󰐀
    PiBox,
    /// PiHole 󰷱
    PiHole,
    /// PiOne 󰏿
    PiOne,
    /// Piano 󰙽
    Piano,
    /// PianoOff 󰚘
    PianoOff,
    /// Pickaxe 󰢷
    Pickaxe,
    /// Picture 
    Picture,
    /// PictureInPictureBottomRight 󰹗
    PictureInPictureBottomRight,
    /// PictureInPictureBottomRightOutline 󰹘
    PictureInPictureBottomRightOutline,
    /// PictureInPictureTopRight 󰹙
    PictureInPictureTopRight,
    /// PictureInPictureTopRightOutline 󰹚
    PictureInPictureTopRightOutline,
    /// PieChart 
    PieChart,
    /// Pier 󰢇
    Pier,
    /// PierCrane 󰢈
    PierCrane,
    /// Pig 󰐁
    Pig,
    /// PigVariant 󱀆
    PigVariant,
    /// PigVariantOutline 󱙸
    PigVariantOutline,
    /// PiggyBank 󱀇
    PiggyBank,
    /// PiggyBankOutline 󱙹
    PiggyBankOutline,
    /// Pill 󰐂
    Pill,
    /// PillOff 󱩜
    PillOff,
    /// Pillar 󰜂
    Pillar,
    /// Pin 
    Pin,
    /// PinOff 󰐄
    PinOff,
    /// PinOffOutline 󰤰
    PinOffOutline,
    /// PinOne 
    PinOne,
    /// PinOutline 󰤱
    PinOutline,
    /// PinTwo 󰐃
    PinTwo,
    /// PineTree 󰐅
    PineTree,
    /// PineTreeBox 󰐆
    PineTreeBox,
    /// PineTreeFire 󱐚
    PineTreeFire,
    /// Pinned 
    Pinned,
    /// PinnedDirty 
    PinnedDirty,
    /// Pinterest 
    Pinterest,
    /// PinterestOne 󰐇
    PinterestOne,
    /// PinterestSign 
    PinterestSign,
    /// Pinwheel 󰫕
    Pinwheel,
    /// PinwheelOutline 󰫖
    PinwheelOutline,
    /// Pipe 󰟥
    Pipe,
    /// PipeDisconnected 󰟦
    PipeDisconnected,
    /// PipeLeak 󰢉
    PipeLeak,
    /// PipeValve 󱡍
    PipeValve,
    /// PipeWrench 󱍔
    PipeWrench,
    /// Pirate 󰨈
    Pirate,
    /// Pistol 󰜃
    Pistol,
    /// Piston 󰢊
    Piston,
    /// Pitchfork 󱕓
    Pitchfork,
    /// Pizza 
    Pizza,
    /// PizzaOne 󰐉
    PizzaOne,
    /// Plane 
    Plane,
    /// Planet 
    Planet,
    /// Plant 
    Plant,
    /// Play 
    Play,
    /// PlayBox 󱉺
    PlayBox,
    /// PlayBoxLock 󱨖
    PlayBoxLock,
    /// PlayBoxLockOpen 󱨗
    PlayBoxLockOpen,
    /// PlayBoxLockOpenOutline 󱨘
    PlayBoxLockOpenOutline,
    /// PlayBoxLockOutline 󱨙
    PlayBoxLockOutline,
    /// PlayBoxMultiple 󰴙
    PlayBoxMultiple,
    /// PlayBoxMultipleOutline 󱏦
    PlayBoxMultipleOutline,
    /// PlayBoxOutline 󰐋
    PlayBoxOutline,
    /// PlayCircle 
    PlayCircle,
    /// PlayCircleOne 󰐌
    PlayCircleOne,
    /// PlayCircleOutline 󰐍
    PlayCircleOutline,
    /// PlayNetwork 󰢋
    PlayNetwork,
    /// PlayNetworkOutline 󰲷
    PlayNetworkOutline,
    /// PlayOne 
    PlayOne,
    /// PlayOutline 󰼛
    PlayOutline,
    /// PlayPause 󰐎
    PlayPause,
    /// PlayProtectedContent 󰐏
    PlayProtectedContent,
    /// PlaySign 
    PlaySign,
    /// PlaySpeed 󰣿
    PlaySpeed,
    /// PlayThree 󰐊
    PlayThree,
    /// PlayTwo 
    PlayTwo,
    /// PlaylistCheck 󰗇
    PlaylistCheck,
    /// PlaylistEdit 󰤀
    PlaylistEdit,
    /// PlaylistMinus 󰐐
    PlaylistMinus,
    /// PlaylistMusic 󰲸
    PlaylistMusic,
    /// PlaylistMusicOutline 󰲹
    PlaylistMusicOutline,
    /// PlaylistPlay 󰐑
    PlaylistPlay,
    /// PlaylistPlus 󰐒
    PlaylistPlus,
    /// PlaylistRemove 󰐓
    PlaylistRemove,
    /// PlaylistStar 󰷲
    PlaylistStar,
    /// Playstation 
    Playstation,
    /// Plex 󰚺
    Plex,
    /// Pliers 󱦤
    Pliers,
    /// Plug 
    Plug,
    /// PlugOne 
    PlugOne,
    /// Plus 
    Plus,
    /// PlusBox 󰐖
    PlusBox,
    /// PlusBoxMultiple 󰌴
    PlusBoxMultiple,
    /// PlusBoxMultipleOutline 󱅃
    PlusBoxMultipleOutline,
    /// PlusBoxOutline 󰜄
    PlusBoxOutline,
    /// PlusCircle 
    PlusCircle,
    /// PlusCircleMultiple 󰍌
    PlusCircleMultiple,
    /// PlusCircleMultipleOutline 󰐘
    PlusCircleMultipleOutline,
    /// PlusCircleOne 󰐗
    PlusCircleOne,
    /// PlusCircleOutline 󰐙
    PlusCircleOutline,
    /// PlusLock 󱩝
    PlusLock,
    /// PlusLockOpen 󱩞
    PlusLockOpen,
    /// PlusMinus 󰦒
    PlusMinus,
    /// PlusMinusBox 󰦓
    PlusMinusBox,
    /// PlusMinusVariant 󱓉
    PlusMinusVariant,
    /// PlusNetwork 󰐚
    PlusNetwork,
    /// PlusNetworkOutline 󰲺
    PlusNetworkOutline,
    /// PlusOne 
    PlusOne,
    /// PlusOutline 󰜅
    PlusOutline,
    /// PlusSign 
    PlusSign,
    /// PlusSquareO 
    PlusSquareO,
    /// PlusThick 󱇬
    PlusThick,
    /// PlusTwo 󰐕
    PlusTwo,
    /// Podcast 󰦔
    Podcast,
    /// Podium 󰴥
    Podium,
    /// PodiumBronze 󰴦
    PodiumBronze,
    /// PodiumGold 󰴧
    PodiumGold,
    /// PodiumSilver 󰴨
    PodiumSilver,
    /// PointOfSale 󰶒
    PointOfSale,
    /// Poison 
    Poison,
    /// Pokeball 󰐝
    Pokeball,
    /// PokemonGo 󰨉
    PokemonGo,
    /// PokerChip 󰠰
    PokerChip,
    /// Polaroid 󰐞
    Polaroid,
    /// PoliceBadge 󱅧
    PoliceBadge,
    /// PoliceBadgeOutline 󱅨
    PoliceBadgeOutline,
    /// PoliceStation 󱠹
    PoliceStation,
    /// Poll 󰐟
    Poll,
    /// Polo 󱓃
    Polo,
    /// Polymer 󰐡
    Polymer,
    /// PomodoroDone 
    PomodoroDone,
    /// PomodoroEstimated 
    PomodoroEstimated,
    /// PomodoroSquashed 
    PomodoroSquashed,
    /// PomodoroTicking 
    PomodoroTicking,
    /// Pool 󰘆
    Pool,
    /// PoolThermometer 󱩟
    PoolThermometer,
    /// PopOs 
    PopOs,
    /// Popcorn 
    Popcorn,
    /// PopcornOne 󰐢
    PopcornOne,
    /// Popsicle 
    Popsicle,
    /// Post 󱀈
    Post,
    /// PostLamp 󱩠
    PostLamp,
    /// PostOutline 󱀉
    PostOutline,
    /// PostageStamp 󰲻
    PostageStamp,
    /// Pot 󰋥
    Pot,
    /// PotMix 󰙛
    PotMix,
    /// PotMixOutline 󰙷
    PotMixOutline,
    /// PotOutline 󰋿
    PotOutline,
    /// PotSteam 󰙚
    PotSteam,
    /// PotSteamOutline 󰌦
    PotSteamOutline,
    /// Pound 󰐣
    Pound,
    /// PoundBox 󰐤
    PoundBox,
    /// PoundBoxOutline 󱅿
    PoundBoxOutline,
    /// Power 󰐥
    Power,
    /// PowerCycle 󰤁
    PowerCycle,
    /// PowerOff 󰤂
    PowerOff,
    /// PowerOn 󰤃
    PowerOn,
    /// PowerOnOffSymbol ⏼
    PowerOnOffSymbol,
    /// PowerOnSymbol ⏽
    PowerOnSymbol,
    /// PowerPlug 󰚥
    PowerPlug,
    /// PowerPlugOff 󰚦
    PowerPlugOff,
    /// PowerPlugOffOutline 󱐤
    PowerPlugOffOutline,
    /// PowerPlugOutline 󱐥
    PowerPlugOutline,
    /// PowerSettings 󰐦
    PowerSettings,
    /// PowerSleep 󰤄
    PowerSleep,
    /// PowerSleepSymbol ⏾
    PowerSleepSymbol,
    /// PowerSocket 󰐧
    PowerSocket,
    /// PowerSocketAu 󰤅
    PowerSocketAu,
    /// PowerSocketCh 󰾳
    PowerSocketCh,
    /// PowerSocketDe 󱄇
    PowerSocketDe,
    /// PowerSocketEu 󰟧
    PowerSocketEu,
    /// PowerSocketFr 󱄈
    PowerSocketFr,
    /// PowerSocketIt 󱓿
    PowerSocketIt,
    /// PowerSocketJp 󱄉
    PowerSocketJp,
    /// PowerSocketUk 󰟨
    PowerSocketUk,
    /// PowerSocketUs 󰟩
    PowerSocketUs,
    /// PowerStandby 󰤆
    PowerStandby,
    /// PowerSymbol ⏻
    PowerSymbol,
    /// Powershell 󰨊
    Powershell,
    /// Prescription 󰜆
    Prescription,
    /// Presentation 󰐨
    Presentation,
    /// PresentationPlay 󰐩
    PresentationPlay,
    /// PreserveCase 
    PreserveCase,
    /// Pretzel 󱕢
    Pretzel,
    /// Preview 
    Preview,
    /// PrimitiveSquare 
    PrimitiveSquare,
    /// Print 
    Print,
    /// Printer 󰐪
    Printer,
    /// PrinterAlert 󰐬
    PrinterAlert,
    /// PrinterCheck 󱅆
    PrinterCheck,
    /// PrinterEye 󱑘
    PrinterEye,
    /// PrinterOff 󰹝
    PrinterOff,
    /// PrinterOffOutline 󱞅
    PrinterOffOutline,
    /// PrinterOutline 󱞆
    PrinterOutline,
    /// PrinterPos 󱁗
    PrinterPos,
    /// PrinterSearch 󱑗
    PrinterSearch,
    /// PrinterSettings 󰜇
    PrinterSettings,
    /// PrinterThreed 󰐫
    PrinterThreed,
    /// PrinterThreedNozzle 󰹛
    PrinterThreedNozzle,
    /// PrinterThreedNozzleAlert 󱇀
    PrinterThreedNozzleAlert,
    /// PrinterThreedNozzleAlertOutline 󱇁
    PrinterThreedNozzleAlertOutline,
    /// PrinterThreedNozzleHeat 󱢸
    PrinterThreedNozzleHeat,
    /// PrinterThreedNozzleHeatOutline 󱢹
    PrinterThreedNozzleHeatOutline,
    /// PrinterThreedNozzleOutline 󰹜
    PrinterThreedNozzleOutline,
    /// PrinterWireless 󰨋
    PrinterWireless,
    /// PriorityHigh 󰘃
    PriorityHigh,
    /// PriorityLow 󰘄
    PriorityLow,
    /// ProfessionalHexagon 󰐭
    ProfessionalHexagon,
    /// ProgressAlert 󰲼
    ProgressAlert,
    /// ProgressCheck 󰦕
    ProgressCheck,
    /// ProgressClock 󰦖
    ProgressClock,
    /// ProgressClose 󱄊
    ProgressClose,
    /// ProgressDownload 󰦗
    ProgressDownload,
    /// ProgressPencil 󱞇
    ProgressPencil,
    /// ProgressQuestion 󱔢
    ProgressQuestion,
    /// ProgressStar 󱞈
    ProgressStar,
    /// ProgressUpload 󰦘
    ProgressUpload,
    /// ProgressWrench 󰲽
    ProgressWrench,
    /// Project 
    Project,
    /// ProjectOne 
    ProjectOne,
    /// ProjectRoadmap 
    ProjectRoadmap,
    /// ProjectSymlink 
    ProjectSymlink,
    /// ProjectTemplate 
    ProjectTemplate,
    /// Projector 󰐮
    Projector,
    /// ProjectorOff 󱨣
    ProjectorOff,
    /// ProjectorScreen 󰐯
    ProjectorScreen,
    /// ProjectorScreenOff 󱠍
    ProjectorScreenOff,
    /// ProjectorScreenOffOutline 󱠎
    ProjectorScreenOffOutline,
    /// ProjectorScreenOutline 󱜤
    ProjectorScreenOutline,
    /// ProjectorScreenVariant 󱠏
    ProjectorScreenVariant,
    /// ProjectorScreenVariantOff 󱠐
    ProjectorScreenVariantOff,
    /// ProjectorScreenVariantOffOutline 󱠑
    ProjectorScreenVariantOffOutline,
    /// ProjectorScreenVariantOutline 󱠒
    ProjectorScreenVariantOutline,
    /// PropaneTank 󱍗
    PropaneTank,
    /// PropaneTankOutline 󱍘
    PropaneTankOutline,
    /// Protocol 󰿘
    Protocol,
    /// PrusaSlicer 
    PrusaSlicer,
    /// Publish 󰚧
    Publish,
    /// PublishOff 󱥅
    PublishOff,
    /// Pulse 
    Pulse,
    /// PulseOne 
    PulseOne,
    /// PulseThree 󰐰
    PulseThree,
    /// PulseTwo 
    PulseTwo,
    /// Pump 󱐂
    Pump,
    /// Pumpkin 󰮿
    Pumpkin,
    /// PuppyLinux 
    PuppyLinux,
    /// Purse 󰼜
    Purse,
    /// PurseOutline 󰼝
    PurseOutline,
    /// Pushpin 
    Pushpin,
    /// Puzzle 󰐱
    Puzzle,
    /// PuzzleCheck 󱐦
    PuzzleCheck,
    /// PuzzleCheckOutline 󱐧
    PuzzleCheckOutline,
    /// PuzzleEdit 󱓓
    PuzzleEdit,
    /// PuzzleEditOutline 󱓙
    PuzzleEditOutline,
    /// PuzzleHeart 󱓔
    PuzzleHeart,
    /// PuzzleHeartOutline 󱓚
    PuzzleHeartOutline,
    /// PuzzleMinus 󱓑
    PuzzleMinus,
    /// PuzzleMinusOutline 󱓗
    PuzzleMinusOutline,
    /// PuzzleOutline 󰩦
    PuzzleOutline,
    /// PuzzlePiece 
    PuzzlePiece,
    /// PuzzlePlus 󱓐
    PuzzlePlus,
    /// PuzzlePlusOutline 󱓖
    PuzzlePlusOutline,
    /// PuzzleRemove 󱓒
    PuzzleRemove,
    /// PuzzleRemoveOutline 󱓘
    PuzzleRemoveOutline,
    /// PuzzleStar 󱓕
    PuzzleStar,
    /// PuzzleStarOutline 󱓛
    PuzzleStarOutline,
    /// Pyramid 󱥒
    Pyramid,
    /// PyramidOff 󱥓
    PyramidOff,
    /// Python 
    Python,
    /// Qi 󰦙
    Qi,
    /// Qqchat 󰘅
    Qqchat,
    /// Qrcode 
    Qrcode,
    /// QrcodeEdit 󰢸
    QrcodeEdit,
    /// QrcodeMinus 󱆌
    QrcodeMinus,
    /// QrcodeOne 󰐲
    QrcodeOne,
    /// QrcodePlus 󱆋
    QrcodePlus,
    /// QrcodeRemove 󱆍
    QrcodeRemove,
    /// QrcodeScan 󰐳
    QrcodeScan,
    /// Qtile 
    Qtile,
    /// Quadcopter 󰐴
    Quadcopter,
    /// QualityHigh 󰐵
    QualityHigh,
    /// QualityLow 󰨌
    QualityLow,
    /// QualityMedium 󰨍
    QualityMedium,
    /// Qubesos 
    Qubesos,
    /// Question 
    Question,
    /// QuestionOne 
    QuestionOne,
    /// QuestionSign 
    QuestionSign,
    /// QuestionTwo 
    QuestionTwo,
    /// Quora 󰴩
    Quora,
    /// QuoraCircle 
    QuoraCircle,
    /// QuoraSquare 
    QuoraSquare,
    /// Quote 
    Quote,
    /// QuoteLeft 
    QuoteLeft,
    /// QuoteOne 
    QuoteOne,
    /// QuoteRight 
    QuoteRight,
    /// Rabbit 󰤇
    Rabbit,
    /// RabbitVariant 󱩡
    RabbitVariant,
    /// RabbitVariantOutline 󱩢
    RabbitVariantOutline,
    /// RacingHelmet 󰶓
    RacingHelmet,
    /// Racquetball 󰶔
    Racquetball,
    /// Radar 󰐷
    Radar,
    /// Radiator 󰐸
    Radiator,
    /// RadiatorDisabled 󰫗
    RadiatorDisabled,
    /// RadiatorOff 󰫘
    RadiatorOff,
    /// Radio 󰐹
    Radio,
    /// RadioAm 󰲾
    RadioAm,
    /// RadioFm 󰲿
    RadioFm,
    /// RadioHandheld 󰐺
    RadioHandheld,
    /// RadioOff 󱈜
    RadioOff,
    /// RadioTower 
    RadioTower,
    /// RadioTowerOne 󰐻
    RadioTowerOne,
    /// Radioactive 
    Radioactive,
    /// RadioactiveCircle 󱡝
    RadioactiveCircle,
    /// RadioactiveCircleOutline 󱡞
    RadioactiveCircleOutline,
    /// RadioactiveOff 󰻁
    RadioactiveOff,
    /// RadioactiveOne 󰐼
    RadioactiveOne,
    /// RadioboxMarked 󰐾
    RadioboxMarked,
    /// RadiologyBox 󱓅
    RadiologyBox,
    /// RadiologyBoxOutline 󱓆
    RadiologyBoxOutline,
    /// Radius 󰳀
    Radius,
    /// RadiusOutline 󰳁
    RadiusOutline,
    /// RailroadLight 󰼞
    RailroadLight,
    /// Raining 
    Raining,
    /// Rake 󱕄
    Rake,
    /// Random 
    Random,
    /// RaspberryPi 󰐿
    RaspberryPi,
    /// Raw 󱨏
    Raw,
    /// RawOff 󱨐
    RawOff,
    /// RayEnd 󰑀
    RayEnd,
    /// RayEndArrow 󰑁
    RayEndArrow,
    /// RayStart 󰑂
    RayStart,
    /// RayStartArrow 󰑃
    RayStartArrow,
    /// RayStartEnd 󰑄
    RayStartEnd,
    /// RayStartVertexEnd 󱗘
    RayStartVertexEnd,
    /// RayVertex 󰑅
    RayVertex,
    /// RazorDoubleEdge 󱦗
    RazorDoubleEdge,
    /// RazorSingleEdge 󱦘
    RazorSingleEdge,
    /// React 󰜈
    React,
    /// Reactions 
    Reactions,
    /// Read 
    Read,
    /// ReadOne 󰑇
    ReadOne,
    /// RealHeart 
    RealHeart,
    /// Receipt 󰑉
    Receipt,
    /// ReceiptOutline 󱧜
    ReceiptOutline,
    /// ReceiptTextCheck 󱩣
    ReceiptTextCheck,
    /// ReceiptTextCheckOutline 󱩤
    ReceiptTextCheckOutline,
    /// ReceiptTextMinus 󱩥
    ReceiptTextMinus,
    /// ReceiptTextMinusOutline 󱩦
    ReceiptTextMinusOutline,
    /// ReceiptTextPlus 󱩧
    ReceiptTextPlus,
    /// ReceiptTextPlusOutline 󱩨
    ReceiptTextPlusOutline,
    /// ReceiptTextRemove 󱩩
    ReceiptTextRemove,
    /// ReceiptTextRemoveOutline 󱩪
    ReceiptTextRemoveOutline,
    /// Record 
    Record,
    /// RecordCircle 󰻂
    RecordCircle,
    /// RecordCircleOutline 󰻃
    RecordCircleOutline,
    /// RecordKeys 
    RecordKeys,
    /// RecordOne 󰑊
    RecordOne,
    /// RecordPlayer 󰦚
    RecordPlayer,
    /// RecordRec 󰑋
    RecordRec,
    /// Rectangle 󰹞
    Rectangle,
    /// RectangleOutline 󰹟
    RectangleOutline,
    /// Recycle 󰑌
    Recycle,
    /// RecycleVariant 󱎝
    RecycleVariant,
    /// RedHat 
    RedHat,
    /// Reddit 󰑍
    Reddit,
    /// Redhat 󱄛
    Redhat,
    /// Redo 
    Redo,
    /// RedoOne 󰑎
    RedoOne,
    /// RedoVariant 󰑏
    RedoVariant,
    /// References 
    References,
    /// ReflectHorizontal 󰨎
    ReflectHorizontal,
    /// ReflectVertical 󰨏
    ReflectVertical,
    /// Refresh 
    Refresh,
    /// RefreshAuto 󱣲
    RefreshAuto,
    /// RefreshCircle 󱍷
    RefreshCircle,
    /// RefreshOne 
    RefreshOne,
    /// RefreshTwo 󰑐
    RefreshTwo,
    /// Refrigerator 
    Refrigerator,
    /// Regex 
    Regex,
    /// RegexOne 󰑑
    RegexOne,
    /// RegisteredTrademark 󰩧
    RegisteredTrademark,
    /// Reiterate 󱖈
    Reiterate,
    /// RelFilePath 
    RelFilePath,
    /// RelationManyToMany 󱒖
    RelationManyToMany,
    /// RelationManyToOne 󱒗
    RelationManyToOne,
    /// RelationManyToOneOrMany 󱒘
    RelationManyToOneOrMany,
    /// RelationManyToOnlyOne 󱒙
    RelationManyToOnlyOne,
    /// RelationManyToZeroOrMany 󱒚
    RelationManyToZeroOrMany,
    /// RelationManyToZeroOrOne 󱒛
    RelationManyToZeroOrOne,
    /// RelationOneOrManyToMany 󱒜
    RelationOneOrManyToMany,
    /// RelationOneOrManyToOne 󱒝
    RelationOneOrManyToOne,
    /// RelationOneOrManyToOneOrMany 󱒞
    RelationOneOrManyToOneOrMany,
    /// RelationOneOrManyToOnlyOne 󱒟
    RelationOneOrManyToOnlyOne,
    /// RelationOneOrManyToZeroOrMany 󱒠
    RelationOneOrManyToZeroOrMany,
    /// RelationOneOrManyToZeroOrOne 󱒡
    RelationOneOrManyToZeroOrOne,
    /// RelationOneToMany 󱒢
    RelationOneToMany,
    /// RelationOneToOne 󱒣
    RelationOneToOne,
    /// RelationOneToOneOrMany 󱒤
    RelationOneToOneOrMany,
    /// RelationOneToOnlyOne 󱒥
    RelationOneToOnlyOne,
    /// RelationOneToZeroOrMany 󱒦
    RelationOneToZeroOrMany,
    /// RelationOneToZeroOrOne 󱒧
    RelationOneToZeroOrOne,
    /// RelationOnlyOneToMany 󱒨
    RelationOnlyOneToMany,
    /// RelationOnlyOneToOne 󱒩
    RelationOnlyOneToOne,
    /// RelationOnlyOneToOneOrMany 󱒪
    RelationOnlyOneToOneOrMany,
    /// RelationOnlyOneToOnlyOne 󱒫
    RelationOnlyOneToOnlyOne,
    /// RelationOnlyOneToZeroOrMany 󱒬
    RelationOnlyOneToZeroOrMany,
    /// RelationOnlyOneToZeroOrOne 󱒭
    RelationOnlyOneToZeroOrOne,
    /// RelationZeroOrManyToMany 󱒮
    RelationZeroOrManyToMany,
    /// RelationZeroOrManyToOne 󱒯
    RelationZeroOrManyToOne,
    /// RelationZeroOrManyToOneOrMany 󱒰
    RelationZeroOrManyToOneOrMany,
    /// RelationZeroOrManyToOnlyOne 󱒱
    RelationZeroOrManyToOnlyOne,
    /// RelationZeroOrManyToZeroOrMany 󱒲
    RelationZeroOrManyToZeroOrMany,
    /// RelationZeroOrManyToZeroOrOne 󱒳
    RelationZeroOrManyToZeroOrOne,
    /// RelationZeroOrOneToMany 󱒴
    RelationZeroOrOneToMany,
    /// RelationZeroOrOneToOne 󱒵
    RelationZeroOrOneToOne,
    /// RelationZeroOrOneToOneOrMany 󱒶
    RelationZeroOrOneToOneOrMany,
    /// RelationZeroOrOneToOnlyOne 󱒷
    RelationZeroOrOneToOnlyOne,
    /// RelationZeroOrOneToZeroOrMany 󱒸
    RelationZeroOrOneToZeroOrMany,
    /// RelationZeroOrOneToZeroOrOne 󱒹
    RelationZeroOrOneToZeroOrOne,
    /// RelativeScale 󰑒
    RelativeScale,
    /// Reload 󰑓
    Reload,
    /// ReloadAlert 󱄋
    ReloadAlert,
    /// Reminder 󰢌
    Reminder,
    /// Remote 
    Remote,
    /// RemoteDesktop 󰢹
    RemoteDesktop,
    /// RemoteExplorer 
    RemoteExplorer,
    /// RemoteOff 󰻄
    RemoteOff,
    /// RemoteOne 󰑔
    RemoteOne,
    /// RemoteTv 󰻅
    RemoteTv,
    /// RemoteTvOff 󰻆
    RemoteTvOff,
    /// Remove 
    Remove,
    /// RemoveCircle 
    RemoveCircle,
    /// RemoveOne 
    RemoveOne,
    /// RemoveSign 
    RemoveSign,
    /// RenameBox 󰑕
    RenameBox,
    /// Renren 
    Renren,
    /// Reorder 
    Reorder,
    /// ReorderHorizontal 󰚈
    ReorderHorizontal,
    /// ReorderVertical 󰚉
    ReorderVertical,
    /// Repeat 
    Repeat,
    /// RepeatOff 󰑗
    RepeatOff,
    /// RepeatOnce 󰑘
    RepeatOnce,
    /// RepeatOne 󰑖
    RepeatOne,
    /// RepeatVariant 󰕇
    RepeatVariant,
    /// Replace 
    Replace,
    /// ReplaceAll 
    ReplaceAll,
    /// Replay 󰑙
    Replay,
    /// Reply 
    Reply,
    /// ReplyAll 󰑛
    ReplyAll,
    /// ReplyAllOutline 󰼟
    ReplyAllOutline,
    /// ReplyCircle 󱆮
    ReplyCircle,
    /// ReplyOne 
    ReplyOne,
    /// ReplyOutline 󰼠
    ReplyOutline,
    /// ReplyThree 󰑚
    ReplyThree,
    /// ReplyTwo 
    ReplyTwo,
    /// Repo 
    Repo,
    /// RepoClone 
    RepoClone,
    /// RepoCloneOne 
    RepoCloneOne,
    /// RepoDeleted 
    RepoDeleted,
    /// RepoForcePush 
    RepoForcePush,
    /// RepoForked 
    RepoForked,
    /// RepoForkedOne 
    RepoForkedOne,
    /// RepoLocked 
    RepoLocked,
    /// RepoOne 
    RepoOne,
    /// RepoPull 
    RepoPull,
    /// RepoPullOne 
    RepoPullOne,
    /// RepoPush 
    RepoPush,
    /// RepoPushOne 
    RepoPushOne,
    /// RepoTemplate 
    RepoTemplate,
    /// Report 
    Report,
    /// ReportOne 
    ReportOne,
    /// Reprap 
    Reprap,
    /// Reproduction 󰑜
    Reproduction,
    /// RequestChanges 
    RequestChanges,
    /// Resistor 󰭄
    Resistor,
    /// ResistorNodes 󰭅
    ResistorNodes,
    /// Resize 󰩨
    Resize,
    /// ResizeBottomRight 󰑝
    ResizeBottomRight,
    /// ResizeFull 
    ResizeFull,
    /// ResizeHorizontal 
    ResizeHorizontal,
    /// ResizeSmall 
    ResizeSmall,
    /// ResizeVertical 
    ResizeVertical,
    /// Responsive 󰑞
    Responsive,
    /// Restart 󰜉
    Restart,
    /// RestartAlert 󱄌
    RestartAlert,
    /// RestartOff 󰶕
    RestartOff,
    /// Restore 
    Restore,
    /// RestoreAlert 󱄍
    RestoreAlert,
    /// RestoreOne 󰦛
    RestoreOne,
    /// Retweet 
    Retweet,
    /// Rewind 󰑟
    Rewind,
    /// RewindFive 󱇹
    RewindFive,
    /// RewindOnefive 󱥆
    RewindOnefive,
    /// RewindOnezero 󰴪
    RewindOnezero,
    /// RewindOutline 󰜊
    RewindOutline,
    /// RewindSixzero 󱘌
    RewindSixzero,
    /// RewindThreezero 󰶖
    RewindThreezero,
    /// Rhombus 󰜋
    Rhombus,
    /// RhombusMedium 󰨐
    RhombusMedium,
    /// RhombusMediumOutline 󱓜
    RhombusMediumOutline,
    /// RhombusOutline 󰜌
    RhombusOutline,
    /// RhombusSplit 󰨑
    RhombusSplit,
    /// RhombusSplitOutline 󱓝
    RhombusSplitOutline,
    /// Ribbon 󰑠
    Ribbon,
    /// Rice 󰟪
    Rice,
    /// Rickshaw 󱖻
    Rickshaw,
    /// RickshawElectric 󱖼
    RickshawElectric,
    /// Ring 
    Ring,
    /// RingOne 󰟫
    RingOne,
    /// RiscV 
    RiscV,
    /// Rivet 󰹠
    Rivet,
    /// Road 
    Road,
    /// RoadOne 󰑡
    RoadOne,
    /// RoadVariant 󰑢
    RoadVariant,
    /// Robber 󱁘
    Robber,
    /// Robot 󰚩
    Robot,
    /// RobotAngry 󱚝
    RobotAngry,
    /// RobotAngryOutline 󱚞
    RobotAngryOutline,
    /// RobotConfused 󱚟
    RobotConfused,
    /// RobotConfusedOutline 󱚠
    RobotConfusedOutline,
    /// RobotDead 󱚡
    RobotDead,
    /// RobotDeadOutline 󱚢
    RobotDeadOutline,
    /// RobotExcited 󱚣
    RobotExcited,
    /// RobotExcitedOutline 󱚤
    RobotExcitedOutline,
    /// RobotHappy 󱜙
    RobotHappy,
    /// RobotHappyOutline 󱜚
    RobotHappyOutline,
    /// RobotIndustrial 󰭆
    RobotIndustrial,
    /// RobotIndustrialOutline 󱨚
    RobotIndustrialOutline,
    /// RobotLove 󱚥
    RobotLove,
    /// RobotLoveOutline 󱚦
    RobotLoveOutline,
    /// RobotMower 󱇷
    RobotMower,
    /// RobotMowerOutline 󱇳
    RobotMowerOutline,
    /// RobotOff 󱚧
    RobotOff,
    /// RobotOffOutline 󱙻
    RobotOffOutline,
    /// RobotOutline 󱙺
    RobotOutline,
    /// RobotVacuum 󰜍
    RobotVacuum,
    /// RobotVacuumVariant 󰤈
    RobotVacuumVariant,
    /// Rocket 
    Rocket,
    /// RocketLaunch 󱓞
    RocketLaunch,
    /// RocketLaunchOutline 󱓟
    RocketLaunchOutline,
    /// RocketOne 
    RocketOne,
    /// RocketOutline 󱎯
    RocketOutline,
    /// RocketThree 󰑣
    RocketThree,
    /// RocketTwo 
    RocketTwo,
    /// RockyLinux 
    RockyLinux,
    /// Rodent 󱌧
    Rodent,
    /// RollerShade 󱩫
    RollerShade,
    /// RollerShadeClosed 󱩬
    RollerShadeClosed,
    /// RollerSkate 󰴫
    RollerSkate,
    /// RollerSkateOff 󰅅
    RollerSkateOff,
    /// Rollerblade 󰴬
    Rollerblade,
    /// RollerbladeOff 󰀮
    RollerbladeOff,
    /// Rollupjs 󰯀
    Rollupjs,
    /// Rolodex 󱪹
    Rolodex,
    /// RolodexOutline 󱪺
    RolodexOutline,
    /// RomanNumeralEight 󱂏
    RomanNumeralEight,
    /// RomanNumeralFour 󱂋
    RomanNumeralFour,
    /// RomanNumeralNine 󱂐
    RomanNumeralNine,
    /// RomanNumeralSeven 󱂎
    RomanNumeralSeven,
    /// RomanNumeralSix 󱂍
    RomanNumeralSix,
    /// RomanNumeralThree 󱂊
    RomanNumeralThree,
    /// RomanNumeralTwo 󱂉
    RomanNumeralTwo,
    /// RoomService 󰢍
    RoomService,
    /// RoomServiceOutline 󰶗
    RoomServiceOutline,
    /// RootFolder 
    RootFolder,
    /// RootFolderOpened 
    RootFolderOpened,
    /// RotateLeft 󰑥
    RotateLeft,
    /// RotateLeftVariant 󰑦
    RotateLeftVariant,
    /// RotateOrbit 󰶘
    RotateOrbit,
    /// RotateRight 󰑧
    RotateRight,
    /// RotateRightVariant 󰑨
    RotateRightVariant,
    /// RotateThreed 󰻇
    RotateThreed,
    /// RotateThreedVariant 󰑤
    RotateThreedVariant,
    /// RotateThreesixzero 󱦙
    RotateThreesixzero,
    /// RoundedCorner 󰘇
    RoundedCorner,
    /// Router 󱇢
    Router,
    /// RouterNetwork 󱂇
    RouterNetwork,
    /// RouterWireless 󰑩
    RouterWireless,
    /// RouterWirelessOff 󱖣
    RouterWirelessOff,
    /// RouterWirelessSettings 󰩩
    RouterWirelessSettings,
    /// Routes 󰑪
    Routes,
    /// RoutesClock 󱁙
    RoutesClock,
    /// Rowing 󰘈
    Rowing,
    /// Rows 
    Rows,
    /// Rss 
    Rss,
    /// RssBox 󰑬
    RssBox,
    /// RssOff 󰼡
    RssOff,
    /// RssOne 
    RssOne,
    /// RssThree 󰑫
    RssThree,
    /// RssTwo 
    RssTwo,
    /// Rub 
    Rub,
    /// Ruby 
    Ruby,
    /// RubyO 
    RubyO,
    /// RubyOne 
    RubyOne,
    /// RubyTwo 
    RubyTwo,
    /// Rug 󱑵
    Rug,
    /// Rugby 󰶙
    Rugby,
    /// Ruler 
    Ruler,
    /// RulerOne 󰑭
    RulerOne,
    /// RulerSquare 󰳂
    RulerSquare,
    /// RulerSquareCompass 󰺾
    RulerSquareCompass,
    /// Run 󰜎
    Run,
    /// RunAbove 
    RunAbove,
    /// RunAll 
    RunAll,
    /// RunBelow 
    RunBelow,
    /// RunErrors 
    RunErrors,
    /// RunFast 󰑮
    RunFast,
    /// RvTruck 󱇔
    RvTruck,
    /// Sabayon 
    Sabayon,
    /// Sack 󰴮
    Sack,
    /// SackPercent 󰴯
    SackPercent,
    /// Safe 󰩪
    Safe,
    /// SafeSquare 󱉼
    SafeSquare,
    /// SafeSquareOutline 󱉽
    SafeSquareOutline,
    /// SafetyGoggles 󰴰
    SafetyGoggles,
    /// SailBoat 󰻈
    SailBoat,
    /// SailBoatSink 󱫯
    SailBoatSink,
    /// Sale 󰑯
    Sale,
    /// SaleOutline 󱨆
    SaleOutline,
    /// Salesforce 󰢎
    Salesforce,
    /// Sass 󰟬
    Sass,
    /// Satellite 󰑰
    Satellite,
    /// SatelliteUplink 󰤉
    SatelliteUplink,
    /// SatelliteVariant 󰑱
    SatelliteVariant,
    /// Sausage 󰢺
    Sausage,
    /// SausageOff 󱞉
    SausageOff,
    /// Save 
    Save,
    /// SaveAll 
    SaveAll,
    /// SaveAs 
    SaveAs,
    /// SaveOne 
    SaveOne,
    /// SawBlade 󰹡
    SawBlade,
    /// SawtoothWave 󱑺
    SawtoothWave,
    /// Saxophone 󰘉
    Saxophone,
    /// Scale 󰑲
    Scale,
    /// ScaleBalance 󰗑
    ScaleBalance,
    /// ScaleBathroom 󰑳
    ScaleBathroom,
    /// ScaleOff 󱁚
    ScaleOff,
    /// ScaleUnbalanced 󱦸
    ScaleUnbalanced,
    /// ScanHelper 󱏘
    ScanHelper,
    /// Scanner 󰚫
    Scanner,
    /// ScannerOff 󰤊
    ScannerOff,
    /// ScatterPlot 󰻉
    ScatterPlot,
    /// ScatterPlotOutline 󰻊
    ScatterPlotOutline,
    /// Scent 󱥘
    Scent,
    /// ScentOff 󱥙
    ScentOff,
    /// School 󰑴
    School,
    /// SchoolOutline 󱆀
    SchoolOutline,
    /// ScissorsCutting 󰩫
    ScissorsCutting,
    /// Scooter 󱖽
    Scooter,
    /// ScooterElectric 󱖾
    ScooterElectric,
    /// Scoreboard 󱉾
    Scoreboard,
    /// ScoreboardOutline 󱉿
    ScoreboardOutline,
    /// ScreenFull 
    ScreenFull,
    /// ScreenFullOne 
    ScreenFullOne,
    /// ScreenNormal 
    ScreenNormal,
    /// ScreenNormalOne 
    ScreenNormalOne,
    /// ScreenRotation 󰑵
    ScreenRotation,
    /// ScreenRotationLock 󰑸
    ScreenRotationLock,
    /// Screenshot 
    Screenshot,
    /// ScrewFlatTop 󰷳
    ScrewFlatTop,
    /// ScrewLag 󰷴
    ScrewLag,
    /// ScrewMachineFlatTop 󰷵
    ScrewMachineFlatTop,
    /// ScrewMachineRoundTop 󰷶
    ScrewMachineRoundTop,
    /// ScrewRoundTop 󰷷
    ScrewRoundTop,
    /// Screwdriver 󰑶
    Screwdriver,
    /// Script 󰯁
    Script,
    /// ScriptOutline 󰑷
    ScriptOutline,
    /// ScriptText 󰯂
    ScriptText,
    /// ScriptTextKey 󱜥
    ScriptTextKey,
    /// ScriptTextKeyOutline 󱜦
    ScriptTextKeyOutline,
    /// ScriptTextOutline 󰯃
    ScriptTextOutline,
    /// ScriptTextPlay 󱜧
    ScriptTextPlay,
    /// ScriptTextPlayOutline 󱜨
    ScriptTextPlayOutline,
    /// Sd 󰑹
    Sd,
    /// Seal 󰑺
    Seal,
    /// SealVariant 󰿙
    SealVariant,
    /// Search 
    Search,
    /// SearchOne 
    SearchOne,
    /// SearchStop 
    SearchStop,
    /// SearchTwo 
    SearchTwo,
    /// SearchWeb 󰜏
    SearchWeb,
    /// Seat 󰳃
    Seat,
    /// SeatFlat 󰑻
    SeatFlat,
    /// SeatFlatAngled 󰑼
    SeatFlatAngled,
    /// SeatIndividualSuite 󰑽
    SeatIndividualSuite,
    /// SeatLegroomExtra 󰑾
    SeatLegroomExtra,
    /// SeatLegroomNormal 󰑿
    SeatLegroomNormal,
    /// SeatLegroomReduced 󰒀
    SeatLegroomReduced,
    /// SeatOutline 󰳄
    SeatOutline,
    /// SeatPassenger 󱉉
    SeatPassenger,
    /// SeatReclineExtra 󰒁
    SeatReclineExtra,
    /// SeatReclineNormal 󰒂
    SeatReclineNormal,
    /// Seatbelt 󰳅
    Seatbelt,
    /// Security 󰒃
    Security,
    /// SecurityNetwork 󰒄
    SecurityNetwork,
    /// Seed 󰹢
    Seed,
    /// SeedOff 󱏽
    SeedOff,
    /// SeedOffOutline 󱏾
    SeedOffOutline,
    /// SeedOutline 󰹣
    SeedOutline,
    /// SeedPlus 󱩭
    SeedPlus,
    /// SeedPlusOutline 󱩮
    SeedPlusOutline,
    /// Seesaw 󱖤
    Seesaw,
    /// Segment 󰻋
    Segment,
    /// Select 󰒅
    Select,
    /// SelectAll 󰒆
    SelectAll,
    /// SelectColor 󰴱
    SelectColor,
    /// SelectCompare 󰫙
    SelectCompare,
    /// SelectDrag 󰩬
    SelectDrag,
    /// SelectGroup 󰾂
    SelectGroup,
    /// SelectInverse 󰒇
    SelectInverse,
    /// SelectMarker 󱊀
    SelectMarker,
    /// SelectMultiple 󱊁
    SelectMultiple,
    /// SelectMultipleMarker 󱊂
    SelectMultipleMarker,
    /// SelectOff 󰒈
    SelectOff,
    /// SelectPlace 󰿚
    SelectPlace,
    /// SelectRemove 󱟁
    SelectRemove,
    /// SelectSearch 󱈄
    SelectSearch,
    /// Selection 󰒉
    Selection,
    /// SelectionDrag 󰩭
    SelectionDrag,
    /// SelectionEllipse 󰴲
    SelectionEllipse,
    /// SelectionEllipseArrowInside 󰼢
    SelectionEllipseArrowInside,
    /// SelectionEllipseRemove 󱟂
    SelectionEllipseRemove,
    /// SelectionMarker 󱊃
    SelectionMarker,
    /// SelectionMultiple 󱊅
    SelectionMultiple,
    /// SelectionMultipleMarker 󱊄
    SelectionMultipleMarker,
    /// SelectionOff 󰝷
    SelectionOff,
    /// SelectionRemove 󱟃
    SelectionRemove,
    /// SelectionSearch 󱈅
    SelectionSearch,
    /// SemanticWeb 󱌖
    SemanticWeb,
    /// Send 󰒊
    Send,
    /// SendCheck 󱅡
    SendCheck,
    /// SendCheckOutline 󱅢
    SendCheckOutline,
    /// SendCircle 󰷸
    SendCircle,
    /// SendCircleOutline 󰷹
    SendCircleOutline,
    /// SendClock 󱅣
    SendClock,
    /// SendClockOutline 󱅤
    SendClockOutline,
    /// SendLock 󰟭
    SendLock,
    /// SendLockOutline 󱅦
    SendLockOutline,
    /// SendOutline 󱅥
    SendOutline,
    /// SerialPort 󰙜
    SerialPort,
    /// Server 
    Server,
    /// ServerEnvironment 
    ServerEnvironment,
    /// ServerMinus 󰒌
    ServerMinus,
    /// ServerNetwork 󰒍
    ServerNetwork,
    /// ServerNetworkOff 󰒎
    ServerNetworkOff,
    /// ServerOff 󰒏
    ServerOff,
    /// ServerOne 
    ServerOne,
    /// ServerPlus 󰒐
    ServerPlus,
    /// ServerProcess 
    ServerProcess,
    /// ServerRemove 󰒑
    ServerRemove,
    /// ServerSecurity 󰒒
    ServerSecurity,
    /// ServerTwo 󰒋
    ServerTwo,
    /// SetAll 󰝸
    SetAll,
    /// SetCenter 󰝹
    SetCenter,
    /// SetCenterRight 󰝺
    SetCenterRight,
    /// SetLeft 󰝻
    SetLeft,
    /// SetLeftCenter 󰝼
    SetLeftCenter,
    /// SetLeftRight 󰝽
    SetLeftRight,
    /// SetMerge 󱓠
    SetMerge,
    /// SetNone 󰝾
    SetNone,
    /// SetRight 󰝿
    SetRight,
    /// SetSplit 󱓡
    SetSplit,
    /// SetSquare 󱑝
    SetSquare,
    /// SetTopBox 󰦟
    SetTopBox,
    /// Settings 
    Settings,
    /// SettingsGear 
    SettingsGear,
    /// SettingsHelper 󰩮
    SettingsHelper,
    /// Shaker 󱄎
    Shaker,
    /// ShakerOutline 󱄏
    ShakerOutline,
    /// Shape 󰠱
    Shape,
    /// ShapeCirclePlus 󰙝
    ShapeCirclePlus,
    /// ShapeOutline 󰠲
    ShapeOutline,
    /// ShapeOvalPlus 󱇺
    ShapeOvalPlus,
    /// ShapePlus 󰒕
    ShapePlus,
    /// ShapePolygonPlus 󰙞
    ShapePolygonPlus,
    /// ShapeRectanglePlus 󰙟
    ShapeRectanglePlus,
    /// ShapeSquarePlus 󰙠
    ShapeSquarePlus,
    /// ShapeSquareRoundedPlus 󱓺
    ShapeSquareRoundedPlus,
    /// Share 
    Share,
    /// ShareAll 󱇴
    ShareAll,
    /// ShareAllOutline 󱇵
    ShareAllOutline,
    /// ShareAlt 
    ShareAlt,
    /// ShareAndroid 
    ShareAndroid,
    /// ShareCircle 󱆭
    ShareCircle,
    /// ShareOff 󰼣
    ShareOff,
    /// ShareOffOutline 󰼤
    ShareOffOutline,
    /// ShareOne 
    ShareOne,
    /// ShareOutline 󰤲
    ShareOutline,
    /// ShareSign 
    ShareSign,
    /// ShareTwo 󰒖
    ShareTwo,
    /// ShareVariant 󰒗
    ShareVariant,
    /// ShareVariantOutline 󱔔
    ShareVariantOutline,
    /// Shark 󱢺
    Shark,
    /// SharkFin 󱙳
    SharkFin,
    /// SharkFinOutline 󱙴
    SharkFinOutline,
    /// SharkOff 󱢻
    SharkOff,
    /// Sheep 󰳆
    Sheep,
    /// Shield 
    Shield,
    /// ShieldAccount 󰢏
    ShieldAccount,
    /// ShieldAccountOutline 󰨒
    ShieldAccountOutline,
    /// ShieldAccountVariant 󱖧
    ShieldAccountVariant,
    /// ShieldAccountVariantOutline 󱖨
    ShieldAccountVariantOutline,
    /// ShieldAirplane 󰚻
    ShieldAirplane,
    /// ShieldAirplaneOutline 󰳇
    ShieldAirplaneOutline,
    /// ShieldAlert 󰻌
    ShieldAlert,
    /// ShieldAlertOutline 󰻍
    ShieldAlertOutline,
    /// ShieldBug 󱏚
    ShieldBug,
    /// ShieldBugOutline 󱏛
    ShieldBugOutline,
    /// ShieldCar 󰾃
    ShieldCar,
    /// ShieldCheck 
    ShieldCheck,
    /// ShieldCheckOne 󰕥
    ShieldCheckOne,
    /// ShieldCheckOutline 󰳈
    ShieldCheckOutline,
    /// ShieldCross 󰳉
    ShieldCross,
    /// ShieldCrossOutline 󰳊
    ShieldCrossOutline,
    /// ShieldCrown 󱢼
    ShieldCrown,
    /// ShieldCrownOutline 󱢽
    ShieldCrownOutline,
    /// ShieldEdit 󱆠
    ShieldEdit,
    /// ShieldEditOutline 󱆡
    ShieldEditOutline,
    /// ShieldHalf 󱍠
    ShieldHalf,
    /// ShieldHalfFull 󰞀
    ShieldHalfFull,
    /// ShieldHome 󰚊
    ShieldHome,
    /// ShieldHomeOutline 󰳋
    ShieldHomeOutline,
    /// ShieldKey 󰯄
    ShieldKey,
    /// ShieldKeyOutline 󰯅
    ShieldKeyOutline,
    /// ShieldLinkVariant 󰴳
    ShieldLinkVariant,
    /// ShieldLinkVariantOutline 󰴴
    ShieldLinkVariantOutline,
    /// ShieldLock 
    ShieldLock,
    /// ShieldLockOne 󰦝
    ShieldLockOne,
    /// ShieldLockOpen 󱦚
    ShieldLockOpen,
    /// ShieldLockOpenOutline 󱦛
    ShieldLockOpenOutline,
    /// ShieldLockOutline 󰳌
    ShieldLockOutline,
    /// ShieldMoon 󱠨
    ShieldMoon,
    /// ShieldMoonOutline 󱠩
    ShieldMoonOutline,
    /// ShieldOff 󰦞
    ShieldOff,
    /// ShieldOffOutline 󰦜
    ShieldOffOutline,
    /// ShieldOne 
    ShieldOne,
    /// ShieldOutline 󰒙
    ShieldOutline,
    /// ShieldPlus 󰫚
    ShieldPlus,
    /// ShieldPlusOutline 󰫛
    ShieldPlusOutline,
    /// ShieldRefresh 󰂪
    ShieldRefresh,
    /// ShieldRefreshOutline 󰇠
    ShieldRefreshOutline,
    /// ShieldRemove 󰫜
    ShieldRemove,
    /// ShieldRemoveOutline 󰫝
    ShieldRemoveOutline,
    /// ShieldSearch 󰶚
    ShieldSearch,
    /// ShieldSlash 
    ShieldSlash,
    /// ShieldStar 󱄻
    ShieldStar,
    /// ShieldStarOutline 󱄼
    ShieldStarOutline,
    /// ShieldSun 󱁝
    ShieldSun,
    /// ShieldSunOutline 󱁞
    ShieldSunOutline,
    /// ShieldSword 󱢾
    ShieldSword,
    /// ShieldSwordOutline 󱢿
    ShieldSwordOutline,
    /// ShieldSync 󱆢
    ShieldSync,
    /// ShieldSyncOutline 󱆣
    ShieldSyncOutline,
    /// ShieldThree 󰒘
    ShieldThree,
    /// ShieldTwo 
    ShieldTwo,
    /// ShieldX 
    ShieldX,
    /// Shimmer 󱕅
    Shimmer,
    /// ShipWheel 󰠳
    ShipWheel,
    /// ShippingPallet 󱡎
    ShippingPallet,
    /// Shirt 
    Shirt,
    /// ShoeBallet 󱗊
    ShoeBallet,
    /// ShoeCleat 󱗇
    ShoeCleat,
    /// ShoeFormal 󰭇
    ShoeFormal,
    /// ShoeHeel 󰭈
    ShoeHeel,
    /// ShoePrint 󰷺
    ShoePrint,
    /// ShoeSneaker 󱗈
    ShoeSneaker,
    /// Shopping 󰒚
    Shopping,
    /// ShoppingCart 
    ShoppingCart,
    /// ShoppingMusic 󰒛
    ShoppingMusic,
    /// ShoppingOutline 󱇕
    ShoppingOutline,
    /// ShoppingSearch 󰾄
    ShoppingSearch,
    /// ShoppingSearchOutline 󱩯
    ShoppingSearchOutline,
    /// Shore 󱓹
    Shore,
    /// ShortPause 
    ShortPause,
    /// Shovel 󰜐
    Shovel,
    /// ShovelOff 󰜑
    ShovelOff,
    /// Shower 󰦠
    Shower,
    /// ShowerHead 󰦡
    ShowerHead,
    /// Shredder 󰒜
    Shredder,
    /// Shuffle 󰒝
    Shuffle,
    /// ShuffleDisabled 󰒞
    ShuffleDisabled,
    /// ShuffleVariant 󰒟
    ShuffleVariant,
    /// Shuriken 󱍿
    Shuriken,
    /// Sickle 󱣀
    Sickle,
    /// SidebarCollapse 
    SidebarCollapse,
    /// SidebarExpand 
    SidebarExpand,
    /// Sigma 󰒠
    Sigma,
    /// SigmaLower 󰘫
    SigmaLower,
    /// SignBlank 
    SignBlank,
    /// SignCaution 󰒡
    SignCaution,
    /// SignDirection 󰞁
    SignDirection,
    /// SignDirectionMinus 󱀀
    SignDirectionMinus,
    /// SignDirectionPlus 󰿜
    SignDirectionPlus,
    /// SignDirectionRemove 󰿝
    SignDirectionRemove,
    /// SignIn 
    SignIn,
    /// SignInOne 
    SignInOne,
    /// SignOut 
    SignOut,
    /// SignOutOne 
    SignOutOne,
    /// SignPole 󱓸
    SignPole,
    /// SignRealEstate 󱄘
    SignRealEstate,
    /// SignText 󰞂
    SignText,
    /// Signal 
    Signal,
    /// SignalCellularOne 󰢼
    SignalCellularOne,
    /// SignalCellularOutline 󰢿
    SignalCellularOutline,
    /// SignalCellularThree 󰢾
    SignalCellularThree,
    /// SignalCellularTwo 󰢽
    SignalCellularTwo,
    /// SignalDistanceVariant 󰹤
    SignalDistanceVariant,
    /// SignalFiveg 󰩯
    SignalFiveg,
    /// SignalFourg 󰜔
    SignalFourg,
    /// SignalHspa 󰜕
    SignalHspa,
    /// SignalHspaPlus 󰜖
    SignalHspaPlus,
    /// SignalOff 󰞃
    SignalOff,
    /// SignalOne 󰒢
    SignalOne,
    /// SignalThreeg 󰜓
    SignalThreeg,
    /// SignalTwog 󰜒
    SignalTwog,
    /// SignalVariant 󰘊
    SignalVariant,
    /// Signature 󰷻
    Signature,
    /// SignatureFreehand 󰷼
    SignatureFreehand,
    /// SignatureImage 󰷽
    SignatureImage,
    /// SignatureText 󰷾
    SignatureText,
    /// Signin 
    Signin,
    /// Signout 
    Signout,
    /// Silo 󰭉
    Silo,
    /// Silverware 󰒣
    Silverware,
    /// SilverwareClean 󰿞
    SilverwareClean,
    /// SilverwareFork 󰒤
    SilverwareFork,
    /// SilverwareForkKnife 󰩰
    SilverwareForkKnife,
    /// SilverwareSpoon 󰒥
    SilverwareSpoon,
    /// SilverwareVariant 󰒦
    SilverwareVariant,
    /// Sim 󰒧
    Sim,
    /// SimAlert 󰒨
    SimAlert,
    /// SimAlertOutline 󱗓
    SimAlertOutline,
    /// SimOff 󰒩
    SimOff,
    /// SimOffOutline 󱗔
    SimOffOutline,
    /// SimOutline 󱗕
    SimOutline,
    /// SimpleIcons 󱌝
    SimpleIcons,
    /// SinaWeibo 󰫟
    SinaWeibo,
    /// SineWave 󰥛
    SineWave,
    /// SingleSelect 
    SingleSelect,
    /// Sitemap 
    Sitemap,
    /// SitemapOne 󰒪
    SitemapOne,
    /// SitemapOutline 󱦜
    SitemapOutline,
    /// Sixoneeight 
    Sixoneeight,
    /// Sixonefive 
    Sixonefive,
    /// Sixonefour 
    Sixonefour,
    /// Sixonenine 
    Sixonenine,
    /// Sixoneone 
    Sixoneone,
    /// Sixoneseven 
    Sixoneseven,
    /// Sixonesix 
    Sixonesix,
    /// Sixonethree 
    Sixonethree,
    /// Sixonetwo 
    Sixonetwo,
    /// Sixonezero 
    Sixonezero,
    /// Sixtwoeight 
    Sixtwoeight,
    /// Sixtwofive 
    Sixtwofive,
    /// Sixtwofour 
    Sixtwofour,
    /// Sixtwonine 
    Sixtwonine,
    /// Sixtwoone 
    Sixtwoone,
    /// Sixtwoseven 
    Sixtwoseven,
    /// Sixtwosix 
    Sixtwosix,
    /// Sixtwothree 
    Sixtwothree,
    /// Sixtwotwo 
    Sixtwotwo,
    /// Sixtwozero 
    Sixtwozero,
    /// Sixzeroeight 
    Sixzeroeight,
    /// Sixzerofour 
    Sixzerofour,
    /// Sixzeronine 
    Sixzeronine,
    /// Sixzeroseven 
    Sixzeroseven,
    /// Sixzerothree 
    Sixzerothree,
    /// Sixzerotwo 
    Sixzerotwo,
    /// SizeM 󱎥
    SizeM,
    /// SizeS 󱎤
    SizeS,
    /// SizeXl 󱎧
    SizeXl,
    /// SizeXs 󱎣
    SizeXs,
    /// SizeXxl 󱎨
    SizeXxl,
    /// SizeXxs 󱎢
    SizeXxs,
    /// SizeXxxl 󱎩
    SizeXxxl,
    /// Skate 󰴵
    Skate,
    /// SkateOff 󰚙
    SkateOff,
    /// Skateboard 󱓂
    Skateboard,
    /// Skateboarding 󰔁
    Skateboarding,
    /// SkewLess 󰴶
    SkewLess,
    /// SkewMore 󰴷
    SkewMore,
    /// Ski 󱌄
    Ski,
    /// SkiCrossCountry 󱌅
    SkiCrossCountry,
    /// SkiWater 󱌆
    SkiWater,
    /// Skip 
    Skip,
    /// SkipBackward 󰒫
    SkipBackward,
    /// SkipBackwardOutline 󰼥
    SkipBackwardOutline,
    /// SkipFill 
    SkipFill,
    /// SkipForward 󰒬
    SkipForward,
    /// SkipForwardOutline 󰼦
    SkipForwardOutline,
    /// SkipNext 󰒭
    SkipNext,
    /// SkipNextCircle 󰙡
    SkipNextCircle,
    /// SkipNextCircleOutline 󰙢
    SkipNextCircleOutline,
    /// SkipNextOutline 󰼧
    SkipNextOutline,
    /// SkipPrevious 󰒮
    SkipPrevious,
    /// SkipPreviousCircle 󰙣
    SkipPreviousCircle,
    /// SkipPreviousCircleOutline 󰙤
    SkipPreviousCircleOutline,
    /// SkipPreviousOutline 󰼨
    SkipPreviousOutline,
    /// Skull 󰚌
    Skull,
    /// SkullCrossbones 󰯆
    SkullCrossbones,
    /// SkullCrossbonesOutline 󰯇
    SkullCrossbonesOutline,
    /// SkullOutline 󰯈
    SkullOutline,
    /// SkullScan 󱓇
    SkullScan,
    /// SkullScanOutline 󱓈
    SkullScanOutline,
    /// Skype 
    Skype,
    /// SkypeBusiness 󰒰
    SkypeBusiness,
    /// SkypeOne 󰒯
    SkypeOne,
    /// Slack 󰒱
    Slack,
    /// Slackware 
    Slackware,
    /// SlackwareInverse 
    SlackwareInverse,
    /// Slash 
    Slash,
    /// SlashForward 󰿟
    SlashForward,
    /// SlashForwardBox 󰿠
    SlashForwardBox,
    /// Sledding 󰐛
    Sledding,
    /// Sleep 󰒲
    Sleep,
    /// SleepOff 󰒳
    SleepOff,
    /// Slide 󱖥
    Slide,
    /// Sliders 
    Sliders,
    /// SlopeDownhill 󰷿
    SlopeDownhill,
    /// SlopeUphill 󰸀
    SlopeUphill,
    /// SlotMachine 󱄔
    SlotMachine,
    /// SlotMachineOutline 󱄕
    SlotMachineOutline,
    /// Smaller 
    Smaller,
    /// SmartCard 󱂽
    SmartCard,
    /// SmartCardOff 󱣷
    SmartCardOff,
    /// SmartCardOffOutline 󱣸
    SmartCardOffOutline,
    /// SmartCardOutline 󱂾
    SmartCardOutline,
    /// SmartCardReader 󱂿
    SmartCardReader,
    /// SmartCardReaderOutline 󱃀
    SmartCardReaderOutline,
    /// Smile 
    Smile,
    /// Smiley 
    Smiley,
    /// SmileyOne 
    SmileyOne,
    /// Smog 󰩱
    Smog,
    /// Smoke 󱞙
    Smoke,
    /// SmokeDetector 󰎒
    SmokeDetector,
    /// SmokeDetectorAlert 󱤮
    SmokeDetectorAlert,
    /// SmokeDetectorAlertOutline 󱤯
    SmokeDetectorAlertOutline,
    /// SmokeDetectorOff 󱠉
    SmokeDetectorOff,
    /// SmokeDetectorOffOutline 󱠊
    SmokeDetectorOffOutline,
    /// SmokeDetectorOutline 󱠈
    SmokeDetectorOutline,
    /// SmokeDetectorVariant 󱠋
    SmokeDetectorVariant,
    /// SmokeDetectorVariantAlert 󱤰
    SmokeDetectorVariantAlert,
    /// SmokeDetectorVariantOff 󱠌
    SmokeDetectorVariantOff,
    /// Smoking 󰒴
    Smoking,
    /// SmokingOff 󰒵
    SmokingOff,
    /// SmokingPipe 󱐍
    SmokingPipe,
    /// SmokingPipeOff 󱐨
    SmokingPipeOff,
    /// Snail 󱙷
    Snail,
    /// Snake 󱔎
    Snake,
    /// Snapchat 󰒶
    Snapchat,
    /// Snappy 
    Snappy,
    /// Snowboard 󱌇
    Snowboard,
    /// Snowflake 󰜗
    Snowflake,
    /// SnowflakeAlert 󰼩
    SnowflakeAlert,
    /// SnowflakeCheck 󱩰
    SnowflakeCheck,
    /// SnowflakeMelt 󱋋
    SnowflakeMelt,
    /// SnowflakeOff 󱓣
    SnowflakeOff,
    /// SnowflakeThermometer 󱩱
    SnowflakeThermometer,
    /// SnowflakeVariant 󰼪
    SnowflakeVariant,
    /// Snowing 
    Snowing,
    /// Snowman 󰒷
    Snowman,
    /// Snowmobile 󰛝
    Snowmobile,
    /// Snowshoeing 󱩲
    Snowshoeing,
    /// Soccer 󰒸
    Soccer,
    /// SoccerField 󰠴
    SoccerField,
    /// SocialDistanceSixFeet 󱕺
    SocialDistanceSixFeet,
    /// SocialDistanceTwoMeters 󱕹
    SocialDistanceTwoMeters,
    /// Soda 
    Soda,
    /// Sofa 
    Sofa,
    /// SofaOne 󰒹
    SofaOne,
    /// SofaOutline 󱕭
    SofaOutline,
    /// SofaSingle 󱕮
    SofaSingle,
    /// SofaSingleOutline 󱕯
    SofaSingleOutline,
    /// SolarPanel 󰶛
    SolarPanel,
    /// SolarPanelLarge 󰶜
    SolarPanelLarge,
    /// SolarPower 󰩲
    SolarPower,
    /// SolarPowerVariant 󱩳
    SolarPowerVariant,
    /// SolarPowerVariantOutline 󱩴
    SolarPowerVariantOutline,
    /// SolderingIron 󱂒
    SolderingIron,
    /// Solid 󰚍
    Solid,
    /// Solus 
    Solus,
    /// SonyPlaystation 󰐔
    SonyPlaystation,
    /// Sort 
    Sort,
    /// SortAlphabeticalAscending 󰖽
    SortAlphabeticalAscending,
    /// SortAlphabeticalAscendingVariant 󱅈
    SortAlphabeticalAscendingVariant,
    /// SortAlphabeticalDescending 󰖿
    SortAlphabeticalDescending,
    /// SortAlphabeticalDescendingVariant 󱅉
    SortAlphabeticalDescendingVariant,
    /// SortAlphabeticalVariant 󰒻
    SortAlphabeticalVariant,
    /// SortAsc 
    SortAsc,
    /// SortAscending 󰒼
    SortAscending,
    /// SortBoolAscending 󱎅
    SortBoolAscending,
    /// SortBoolAscendingVariant 󱎆
    SortBoolAscendingVariant,
    /// SortBoolDescending 󱎇
    SortBoolDescending,
    /// SortBoolDescendingVariant 󱎈
    SortBoolDescendingVariant,
    /// SortByAlphabet 
    SortByAlphabet,
    /// SortByAttributes 
    SortByAttributes,
    /// SortByAttributesAlt 
    SortByAttributesAlt,
    /// SortByOrder 
    SortByOrder,
    /// SortByOrderAlt 
    SortByOrderAlt,
    /// SortCalendarAscending 󱕇
    SortCalendarAscending,
    /// SortCalendarDescending 󱕈
    SortCalendarDescending,
    /// SortClockAscending 󱕉
    SortClockAscending,
    /// SortClockAscendingOutline 󱕊
    SortClockAscendingOutline,
    /// SortClockDescending 󱕋
    SortClockDescending,
    /// SortClockDescendingOutline 󱕌
    SortClockDescendingOutline,
    /// SortDesc 
    SortDesc,
    /// SortDescending 󰒽
    SortDescending,
    /// SortDown 
    SortDown,
    /// SortNumericAscending 󱎉
    SortNumericAscending,
    /// SortNumericAscendingVariant 󰤍
    SortNumericAscendingVariant,
    /// SortNumericDescending 󱎊
    SortNumericDescending,
    /// SortNumericDescendingVariant 󰫒
    SortNumericDescendingVariant,
    /// SortNumericVariant 󰒾
    SortNumericVariant,
    /// SortOne 󰒺
    SortOne,
    /// SortPrecedence 
    SortPrecedence,
    /// SortReverseVariant 󰌼
    SortReverseVariant,
    /// SortUp 
    SortUp,
    /// SortVariant 󰒿
    SortVariant,
    /// SortVariantLock 󰳍
    SortVariantLock,
    /// SortVariantLockOpen 󰳎
    SortVariantLockOpen,
    /// SortVariantOff 󱪻
    SortVariantOff,
    /// SortVariantRemove 󱅇
    SortVariantRemove,
    /// Soundbar 󱟛
    Soundbar,
    /// Soundcloud 󰓀
    Soundcloud,
    /// Soup 
    Soup,
    /// SourceBranch 󰘬
    SourceBranch,
    /// SourceBranchCheck 󱓏
    SourceBranchCheck,
    /// SourceBranchMinus 󱓋
    SourceBranchMinus,
    /// SourceBranchPlus 󱓊
    SourceBranchPlus,
    /// SourceBranchRefresh 󱓍
    SourceBranchRefresh,
    /// SourceBranchRemove 󱓌
    SourceBranchRemove,
    /// SourceBranchSync 󱓎
    SourceBranchSync,
    /// SourceCommit 󰜘
    SourceCommit,
    /// SourceCommitEnd 󰜙
    SourceCommitEnd,
    /// SourceCommitEndLocal 󰜚
    SourceCommitEndLocal,
    /// SourceCommitLocal 󰜛
    SourceCommitLocal,
    /// SourceCommitNextLocal 󰜜
    SourceCommitNextLocal,
    /// SourceCommitStart 󰜝
    SourceCommitStart,
    /// SourceCommitStartNextLocal 󰜞
    SourceCommitStartNextLocal,
    /// SourceControl 
    SourceControl,
    /// SourceFork 󰓁
    SourceFork,
    /// SourceMerge 󰘭
    SourceMerge,
    /// SourcePull 󰓂
    SourcePull,
    /// SourceRepository 󰳏
    SourceRepository,
    /// SourceRepositoryMultiple 󰳐
    SourceRepositoryMultiple,
    /// SoySauce 󰟮
    SoySauce,
    /// SoySauceOff 󱏼
    SoySauceOff,
    /// Spa 󰳑
    Spa,
    /// SpaOutline 󰳒
    SpaOutline,
    /// SpaceInvaders 󰯉
    SpaceInvaders,
    /// SpaceStation 󱎃
    SpaceStation,
    /// Spade 󰹥
    Spade,
    /// SparkleFill 
    SparkleFill,
    /// Speaker 󰓃
    Speaker,
    /// SpeakerBluetooth 󰦢
    SpeakerBluetooth,
    /// SpeakerMultiple 󰴸
    SpeakerMultiple,
    /// SpeakerOff 󰓄
    SpeakerOff,
    /// SpeakerWireless 󰜟
    SpeakerWireless,
    /// Spear 󱡅
    Spear,
    /// Speedometer 󰓅
    Speedometer,
    /// SpeedometerMedium 󰾅
    SpeedometerMedium,
    /// SpeedometerSlow 󰾆
    SpeedometerSlow,
    /// Spellcheck 󰓆
    Spellcheck,
    /// Spermatozoon 
    Spermatozoon,
    /// Sphere 󱥔
    Sphere,
    /// SphereOff 󱥕
    SphereOff,
    /// Spider 󱇪
    Spider,
    /// SpiderThread 󱇫
    SpiderThread,
    /// SpiderWeb 󰯊
    SpiderWeb,
    /// SpinDouble 
    SpinDouble,
    /// Spinner 
    Spinner,
    /// SpiritLevel 󱓱
    SpiritLevel,
    /// SplitHorizontal 
    SplitHorizontal,
    /// SplitVertical 
    SplitVertical,
    /// SponsorTiers 
    SponsorTiers,
    /// SpoonSugar 󱐩
    SpoonSugar,
    /// Spotify 󰓇
    Spotify,
    /// Spotlight 󰓈
    Spotlight,
    /// SpotlightBeam 󰓉
    SpotlightBeam,
    /// Spray 󰙥
    Spray,
    /// SprayBottle 󰫠
    SprayBottle,
    /// Sprinkler 󱁟
    Sprinkler,
    /// SprinklerFire 󱦝
    SprinklerFire,
    /// SprinklerVariant 󱁠
    SprinklerVariant,
    /// Sprout 󰹦
    Sprout,
    /// SproutOutline 󰹧
    SproutOutline,
    /// Square 
    Square,
    /// SquareCircle 󱔀
    SquareCircle,
    /// SquareEditOutline 󰤌
    SquareEditOutline,
    /// SquareFill 
    SquareFill,
    /// SquareMedium 󰨓
    SquareMedium,
    /// SquareMediumOutline 󰨔
    SquareMediumOutline,
    /// SquareOff 󱋮
    SquareOff,
    /// SquareOffOutline 󱋯
    SquareOffOutline,
    /// SquareOne 󰝤
    SquareOne,
    /// SquareOpacity 󱡔
    SquareOpacity,
    /// SquareOutline 󰝣
    SquareOutline,
    /// SquareRoot 󰞄
    SquareRoot,
    /// SquareRootBox 󰦣
    SquareRootBox,
    /// SquareRounded 󱓻
    SquareRounded,
    /// SquareRoundedBadge 󱨇
    SquareRoundedBadge,
    /// SquareRoundedBadgeOutline 󱨈
    SquareRoundedBadgeOutline,
    /// SquareRoundedOutline 󱓼
    SquareRoundedOutline,
    /// SquareSmall 󰨕
    SquareSmall,
    /// SquareWave 󱑻
    SquareWave,
    /// Squeegee 󰫡
    Squeegee,
    /// Squirrel 
    Squirrel,
    /// SquirrelOne 
    SquirrelOne,
    /// Ssh 󰣀
    Ssh,
    /// Stack 
    Stack,
    /// StackExchange 󰘋
    StackExchange,
    /// StackOverflow 󰓌
    StackOverflow,
    /// Stackexchange 
    Stackexchange,
    /// Stackpath 󰍙
    Stackpath,
    /// Stadium 󰿹
    Stadium,
    /// StadiumVariant 󰜠
    StadiumVariant,
    /// Stairs 󰓍
    Stairs,
    /// StairsBox 󱎞
    StairsBox,
    /// StairsDown 󱊾
    StairsDown,
    /// StairsUp 󱊽
    StairsUp,
    /// Stamper 󰴹
    Stamper,
    /// StandardDefinition 󰟯
    StandardDefinition,
    /// Star 
    Star,
    /// StarBox 󰩳
    StarBox,
    /// StarBoxMultiple 󱊆
    StarBoxMultiple,
    /// StarBoxMultipleOutline 󱊇
    StarBoxMultipleOutline,
    /// StarBoxOutline 󰩴
    StarBoxOutline,
    /// StarCheck 󱕦
    StarCheck,
    /// StarCheckOutline 󱕪
    StarCheckOutline,
    /// StarCircle 󰓏
    StarCircle,
    /// StarCircleOutline 󰦤
    StarCircleOutline,
    /// StarCog 󱙨
    StarCog,
    /// StarCogOutline 󱙩
    StarCogOutline,
    /// StarCrescent 󰥹
    StarCrescent,
    /// StarDavid 󰥺
    StarDavid,
    /// StarEmpty 
    StarEmpty,
    /// StarFace 󰦥
    StarFace,
    /// StarFill 
    StarFill,
    /// StarFourPoints 󰫢
    StarFourPoints,
    /// StarFourPointsOutline 󰫣
    StarFourPointsOutline,
    /// StarFull 
    StarFull,
    /// StarHalf 
    StarHalf,
    /// StarHalfEmpty 
    StarHalfEmpty,
    /// StarHalfFull 󰓐
    StarHalfFull,
    /// StarHalfOne 󰉆
    StarHalfOne,
    /// StarMinus 󱕤
    StarMinus,
    /// StarMinusOutline 󱕨
    StarMinusOutline,
    /// StarOff 󰓑
    StarOff,
    /// StarOffOutline 󱕛
    StarOffOutline,
    /// StarOne 
    StarOne,
    /// StarOutline 󰓒
    StarOutline,
    /// StarPlus 󱕣
    StarPlus,
    /// StarPlusOutline 󱕧
    StarPlusOutline,
    /// StarRemove 󱕥
    StarRemove,
    /// StarRemoveOutline 󱕩
    StarRemoveOutline,
    /// StarSettings 󱙪
    StarSettings,
    /// StarSettingsOutline 󱙫
    StarSettingsOutline,
    /// StarShooting 󱝁
    StarShooting,
    /// StarShootingOutline 󱝂
    StarShootingOutline,
    /// StarThreePoints 󰫤
    StarThreePoints,
    /// StarThreePointsOutline 󰫥
    StarThreePointsOutline,
    /// StarTwo 󰓎
    StarTwo,
    /// StateMachine 󱇯
    StateMachine,
    /// Steam 󰓓
    Steam,
    /// Steering 󰓔
    Steering,
    /// SteeringOff 󰤎
    SteeringOff,
    /// StepBackward 󰓕
    StepBackward,
    /// StepBackwardTwo 󰓖
    StepBackwardTwo,
    /// StepForward 󰓗
    StepForward,
    /// StepForwardTwo 󰓘
    StepForwardTwo,
    /// Stethoscope 
    Stethoscope,
    /// StethoscopeOne 󰓙
    StethoscopeOne,
    /// Sticker 󱍤
    Sticker,
    /// StickerAlert 󱍥
    StickerAlert,
    /// StickerAlertOutline 󱍦
    StickerAlertOutline,
    /// StickerCheck 󱍧
    StickerCheck,
    /// StickerCheckOutline 󱍨
    StickerCheckOutline,
    /// StickerCircleOutline 󰗐
    StickerCircleOutline,
    /// StickerEmoji 󰞅
    StickerEmoji,
    /// StickerMinus 󱍩
    StickerMinus,
    /// StickerMinusOutline 󱍪
    StickerMinusOutline,
    /// StickerOutline 󱍫
    StickerOutline,
    /// StickerPlus 󱍬
    StickerPlus,
    /// StickerPlusOutline 󱍭
    StickerPlusOutline,
    /// StickerRemove 󱍮
    StickerRemove,
    /// StickerRemoveOutline 󱍯
    StickerRemoveOutline,
    /// StickerText 󱞎
    StickerText,
    /// StickerTextOutline 󱞏
    StickerTextOutline,
    /// Stocking 󰓚
    Stocking,
    /// Stomach 
    Stomach,
    /// StomachOne 󱂓
    StomachOne,
    /// Stool 󱥝
    Stool,
    /// StoolOutline 󱥞
    StoolOutline,
    /// Stop 
    Stop,
    /// StopCircle 
    StopCircle,
    /// StopCircleOne 󰙦
    StopCircleOne,
    /// StopCircleOutline 󰙧
    StopCircleOutline,
    /// StopOne 
    StopOne,
    /// StopTwo 󰓛
    StopTwo,
    /// Stopwatch 
    Stopwatch,
    /// StorageTank 󱩵
    StorageTank,
    /// StorageTankOutline 󱩶
    StorageTankOutline,
    /// Store 󰓜
    Store,
    /// StoreAlert 󱣁
    StoreAlert,
    /// StoreAlertOutline 󱣂
    StoreAlertOutline,
    /// StoreCheck 󱣃
    StoreCheck,
    /// StoreCheckOutline 󱣄
    StoreCheckOutline,
    /// StoreClock 󱣅
    StoreClock,
    /// StoreClockOutline 󱣆
    StoreClockOutline,
    /// StoreCog 󱣇
    StoreCog,
    /// StoreCogOutline 󱣈
    StoreCogOutline,
    /// StoreEdit 󱣉
    StoreEdit,
    /// StoreEditOutline 󱣊
    StoreEditOutline,
    /// StoreMarker 󱣋
    StoreMarker,
    /// StoreMarkerOutline 󱣌
    StoreMarkerOutline,
    /// StoreMinus 󱙞
    StoreMinus,
    /// StoreMinusOutline 󱣍
    StoreMinusOutline,
    /// StoreOff 󱣎
    StoreOff,
    /// StoreOffOutline 󱣏
    StoreOffOutline,
    /// StoreOutline 󱍡
    StoreOutline,
    /// StorePlus 󱙟
    StorePlus,
    /// StorePlusOutline 󱣐
    StorePlusOutline,
    /// StoreRemove 󱙠
    StoreRemove,
    /// StoreRemoveOutline 󱣑
    StoreRemoveOutline,
    /// StoreSearch 󱣒
    StoreSearch,
    /// StoreSearchOutline 󱣓
    StoreSearchOutline,
    /// StoreSettings 󱣔
    StoreSettings,
    /// StoreSettingsOutline 󱣕
    StoreSettingsOutline,
    /// StoreTwofourHour 󰓝
    StoreTwofourHour,
    /// Storefront 󰟇
    Storefront,
    /// StorefrontOutline 󱃁
    StorefrontOutline,
    /// Storm 
    Storm,
    /// Stove 󰓞
    Stove,
    /// Strategy 󱇖
    Strategy,
    /// StretchToPage 󰼫
    StretchToPage,
    /// StretchToPageOutline 󰼬
    StretchToPageOutline,
    /// Strikethrough 
    Strikethrough,
    /// StrikethroughOne 
    StrikethroughOne,
    /// StringLights 󱊺
    StringLights,
    /// StringLightsOff 󱊻
    StringLightsOff,
    /// SubdirectoryArrowLeft 󰘌
    SubdirectoryArrowLeft,
    /// SubdirectoryArrowRight 󰘍
    SubdirectoryArrowRight,
    /// Submarine 󱕬
    Submarine,
    /// Subscript 
    Subscript,
    /// Subtitles 󰨖
    Subtitles,
    /// SubtitlesOutline 󰨗
    SubtitlesOutline,
    /// Subway 󰚬
    Subway,
    /// SubwayAlertVariant 󰶝
    SubwayAlertVariant,
    /// SubwayVariant 󰓟
    SubwayVariant,
    /// Suitcase 
    Suitcase,
    /// Summit 󰞆
    Summit,
    /// Sun 
    Sun,
    /// SunClock 󱩷
    SunClock,
    /// SunClockOutline 󱩸
    SunClockOutline,
    /// SunCloud 
    SunCloud,
    /// SunCompass 󱦥
    SunCompass,
    /// SunOne 
    SunOne,
    /// SunSnowflake 󱞖
    SunSnowflake,
    /// SunSnowflakeVariant 󱩹
    SunSnowflakeVariant,
    /// SunThermometer 󱣖
    SunThermometer,
    /// SunThermometerOutline 󱣗
    SunThermometerOutline,
    /// SunWireless 󱟾
    SunWireless,
    /// SunWirelessOutline 󱟿
    SunWirelessOutline,
    /// Sunglasses 󰓠
    Sunglasses,
    /// Superscript 
    Superscript,
    /// Surfing 󱝆
    Surfing,
    /// SurroundSound 󰗅
    SurroundSound,
    /// SurroundSoundFiveOne 󰟲
    SurroundSoundFiveOne,
    /// SurroundSoundFiveOneTwo 󱜪
    SurroundSoundFiveOneTwo,
    /// SurroundSoundSevenOne 󰟳
    SurroundSoundSevenOne,
    /// SurroundSoundThreeOne 󰟱
    SurroundSoundThreeOne,
    /// SurroundSoundTwoOne 󱜩
    SurroundSoundTwoOne,
    /// SurroundSoundTwoZero 󰟰
    SurroundSoundTwoZero,
    /// Sushi 
    Sushi,
    /// Svg 󰜡
    Svg,
    /// SwapHorizontal 󰓡
    SwapHorizontal,
    /// SwapHorizontalBold 󰯍
    SwapHorizontalBold,
    /// SwapHorizontalCircle 󰿡
    SwapHorizontalCircle,
    /// SwapHorizontalCircleOutline 󰿢
    SwapHorizontalCircleOutline,
    /// SwapHorizontalVariant 󰣁
    SwapHorizontalVariant,
    /// SwapVertical 󰓢
    SwapVertical,
    /// SwapVerticalBold 󰯎
    SwapVerticalBold,
    /// SwapVerticalCircle 󰿣
    SwapVerticalCircle,
    /// SwapVerticalCircleOutline 󰿤
    SwapVerticalCircleOutline,
    /// SwapVerticalVariant 󰣂
    SwapVerticalVariant,
    /// Sway 
    Sway,
    /// Swim 󰓣
    Swim,
    /// Switch 󰓤
    Switch,
    /// Sword 󰓥
    Sword,
    /// SwordCross 󰞇
    SwordCross,
    /// SyllabaryHangul 󱌳
    SyllabaryHangul,
    /// SyllabaryHiragana 󱌴
    SyllabaryHiragana,
    /// SyllabaryKatakana 󱌵
    SyllabaryKatakana,
    /// SyllabaryKatakanaHalfwidth 󱌶
    SyllabaryKatakanaHalfwidth,
    /// Symbol 󱔁
    Symbol,
    /// SymbolArray 
    SymbolArray,
    /// SymbolBoolean 
    SymbolBoolean,
    /// SymbolClass 
    SymbolClass,
    /// SymbolColor 
    SymbolColor,
    /// SymbolConstant 
    SymbolConstant,
    /// SymbolEnum 
    SymbolEnum,
    /// SymbolEnumMember 
    SymbolEnumMember,
    /// SymbolEvent 
    SymbolEvent,
    /// SymbolField 
    SymbolField,
    /// SymbolFile 
    SymbolFile,
    /// SymbolInterface 
    SymbolInterface,
    /// SymbolKey 
    SymbolKey,
    /// SymbolKeyword 
    SymbolKeyword,
    /// SymbolMethod 
    SymbolMethod,
    /// SymbolMisc 
    SymbolMisc,
    /// SymbolNamespace 
    SymbolNamespace,
    /// SymbolNumeric 
    SymbolNumeric,
    /// SymbolOperator 
    SymbolOperator,
    /// SymbolParameter 
    SymbolParameter,
    /// SymbolProperty 
    SymbolProperty,
    /// SymbolRuler 
    SymbolRuler,
    /// SymbolSnippet 
    SymbolSnippet,
    /// SymbolString 
    SymbolString,
    /// SymbolStructure 
    SymbolStructure,
    /// SymbolVariable 
    SymbolVariable,
    /// Symfony 󰫦
    Symfony,
    /// Sync 
    Sync,
    /// SyncAlert 󰓧
    SyncAlert,
    /// SyncCircle 󱍸
    SyncCircle,
    /// SyncIgnored 
    SyncIgnored,
    /// SyncOff 󰓨
    SyncOff,
    /// SyncOne 
    SyncOne,
    /// SyncTwo 󰓦
    SyncTwo,
    /// Tab 
    Tab,
    /// TabExternal 
    TabExternal,
    /// TabMinus 󰭋
    TabMinus,
    /// TabOne 󰓩
    TabOne,
    /// TabPlus 󰝜
    TabPlus,
    /// TabRemove 󰭌
    TabRemove,
    /// TabSearch 󱦞
    TabSearch,
    /// TabUnselected 󰓪
    TabUnselected,
    /// Table 
    Table,
    /// TableAccount 󱎹
    TableAccount,
    /// TableAlert 󱎺
    TableAlert,
    /// TableArrowDown 󱎻
    TableArrowDown,
    /// TableArrowLeft 󱎼
    TableArrowLeft,
    /// TableArrowRight 󱎽
    TableArrowRight,
    /// TableArrowUp 󱎾
    TableArrowUp,
    /// TableBorder 󰨘
    TableBorder,
    /// TableCancel 󱎿
    TableCancel,
    /// TableChair 󱁡
    TableChair,
    /// TableCheck 󱏀
    TableCheck,
    /// TableClock 󱏁
    TableClock,
    /// TableCog 󱏂
    TableCog,
    /// TableColumn 󰠵
    TableColumn,
    /// TableColumnPlusAfter 󰓬
    TableColumnPlusAfter,
    /// TableColumnPlusBefore 󰓭
    TableColumnPlusBefore,
    /// TableColumnRemove 󰓮
    TableColumnRemove,
    /// TableColumnWidth 󰓯
    TableColumnWidth,
    /// TableEdit 󰓰
    TableEdit,
    /// TableEye 󱂔
    TableEye,
    /// TableEyeOff 󱏃
    TableEyeOff,
    /// TableFurniture 󰖼
    TableFurniture,
    /// TableHeadersEye 󱈝
    TableHeadersEye,
    /// TableHeadersEyeOff 󱈞
    TableHeadersEyeOff,
    /// TableHeart 󱏄
    TableHeart,
    /// TableKey 󱏅
    TableKey,
    /// TableLarge 󰓱
    TableLarge,
    /// TableLargePlus 󰾇
    TableLargePlus,
    /// TableLargeRemove 󰾈
    TableLargeRemove,
    /// TableLock 󱏆
    TableLock,
    /// TableMergeCells 󰦦
    TableMergeCells,
    /// TableMinus 󱏇
    TableMinus,
    /// TableMultiple 󱏈
    TableMultiple,
    /// TableNetwork 󱏉
    TableNetwork,
    /// TableOfContents 󰠶
    TableOfContents,
    /// TableOff 󱏊
    TableOff,
    /// TableOne 
    TableOne,
    /// TablePicnic 󱝃
    TablePicnic,
    /// TablePivot 󱠼
    TablePivot,
    /// TablePlus 󰩵
    TablePlus,
    /// TableRefresh 󱎠
    TableRefresh,
    /// TableRemove 󰩶
    TableRemove,
    /// TableRow 󰠷
    TableRow,
    /// TableRowHeight 󰓲
    TableRowHeight,
    /// TableRowPlusAfter 󰓳
    TableRowPlusAfter,
    /// TableRowPlusBefore 󰓴
    TableRowPlusBefore,
    /// TableRowRemove 󰓵
    TableRowRemove,
    /// TableSearch 󰤏
    TableSearch,
    /// TableSettings 󰠸
    TableSettings,
    /// TableSplitCell 󱐪
    TableSplitCell,
    /// TableStar 󱏋
    TableStar,
    /// TableSync 󱎡
    TableSync,
    /// TableTennis 󰹨
    TableTennis,
    /// TableThree 󰓫
    TableThree,
    /// TableTwo 
    TableTwo,
    /// Tablet 
    Tablet,
    /// TabletAndroid 󰓷
    TabletAndroid,
    /// TabletCellphone 󰦧
    TabletCellphone,
    /// TabletDashboard 󰻎
    TabletDashboard,
    /// TabletOne 󰓶
    TabletOne,
    /// Taco 󰝢
    Taco,
    /// Tacos 
    Tacos,
    /// Tag 
    Tag,
    /// TagArrowDown 󱜫
    TagArrowDown,
    /// TagArrowDownOutline 󱜬
    TagArrowDownOutline,
    /// TagArrowLeft 󱜭
    TagArrowLeft,
    /// TagArrowLeftOutline 󱜮
    TagArrowLeftOutline,
    /// TagArrowRight 󱜯
    TagArrowRight,
    /// TagArrowRightOutline 󱜰
    TagArrowRightOutline,
    /// TagArrowUp 󱜱
    TagArrowUp,
    /// TagArrowUpOutline 󱜲
    TagArrowUpOutline,
    /// TagCheck 󱩺
    TagCheck,
    /// TagCheckOutline 󱩻
    TagCheckOutline,
    /// TagFaces 󰓺
    TagFaces,
    /// TagHeart 󰚋
    TagHeart,
    /// TagHeartOutline 󰯏
    TagHeartOutline,
    /// TagMinus 󰤐
    TagMinus,
    /// TagMinusOutline 󱈟
    TagMinusOutline,
    /// TagMultiple 󰓻
    TagMultiple,
    /// TagMultipleOutline 󱋷
    TagMultipleOutline,
    /// TagOff 󱈠
    TagOff,
    /// TagOffOutline 󱈡
    TagOffOutline,
    /// TagOne 
    TagOne,
    /// TagOutline 󰓼
    TagOutline,
    /// TagPlus 󰜢
    TagPlus,
    /// TagPlusOutline 󱈢
    TagPlusOutline,
    /// TagRemove 󰜣
    TagRemove,
    /// TagRemoveOutline 󱈣
    TagRemoveOutline,
    /// TagSearch 󱤇
    TagSearch,
    /// TagSearchOutline 󱤈
    TagSearchOutline,
    /// TagText 󱈤
    TagText,
    /// TagTextOutline 󰓽
    TagTextOutline,
    /// TagThree 󰓹
    TagThree,
    /// TagTwo 
    TagTwo,
    /// Tags 
    Tags,
    /// Tails 
    Tails,
    /// Tailwind 󱏿
    Tailwind,
    /// TallyMarkFive 󱫀
    TallyMarkFive,
    /// TallyMarkFour 󱪿
    TallyMarkFour,
    /// TallyMarkOne 󱪼
    TallyMarkOne,
    /// TallyMarkThree 󱪾
    TallyMarkThree,
    /// TallyMarkTwo 󱪽
    TallyMarkTwo,
    /// Tangram 󰓸
    Tangram,
    /// Tank 󰴺
    Tank,
    /// TankerTruck 󰿥
    TankerTruck,
    /// TapeDrive 󱛟
    TapeDrive,
    /// TapeMeasure 󰭍
    TapeMeasure,
    /// Target 󰓾
    Target,
    /// TargetAccount 󰯐
    TargetAccount,
    /// TargetVariant 󰩷
    TargetVariant,
    /// Tasklist 
    Tasklist,
    /// TasklistOne 
    TasklistOne,
    /// Tasks 
    Tasks,
    /// Taxi 󰓿
    Taxi,
    /// Tea 󰶞
    Tea,
    /// TeaOutline 󰶟
    TeaOutline,
    /// Teamviewer 󰔀
    Teamviewer,
    /// TeddyBear 󱣻
    TeddyBear,
    /// Telegram 
    Telegram,
    /// TelegramCircle 
    TelegramCircle,
    /// Telescope 
    Telescope,
    /// TelescopeFill 
    TelescopeFill,
    /// TelescopeOne 
    TelescopeOne,
    /// TelescopeThree 󰭎
    TelescopeThree,
    /// TelescopeTwo 
    TelescopeTwo,
    /// Television 󰔂
    Television,
    /// TelevisionAmbientLight 󱍖
    TelevisionAmbientLight,
    /// TelevisionBox 󰠹
    TelevisionBox,
    /// TelevisionClassic 󰟴
    TelevisionClassic,
    /// TelevisionClassicOff 󰠺
    TelevisionClassicOff,
    /// TelevisionGuide 󰔃
    TelevisionGuide,
    /// TelevisionOff 󰠻
    TelevisionOff,
    /// TelevisionPause 󰾉
    TelevisionPause,
    /// TelevisionPlay 󰻏
    TelevisionPlay,
    /// TelevisionShimmer 󱄐
    TelevisionShimmer,
    /// TelevisionStop 󰾊
    TelevisionStop,
    /// TemperatureCelsius 󰔄
    TemperatureCelsius,
    /// TemperatureFahrenheit 󰔅
    TemperatureFahrenheit,
    /// TemperatureKelvin 󰔆
    TemperatureKelvin,
    /// Tennis 󰶠
    Tennis,
    /// TennisBall 󰔇
    TennisBall,
    /// Tent 󰔈
    Tent,
    /// Terminal 
    Terminal,
    /// TerminalBash 
    TerminalBash,
    /// TerminalCmd 
    TerminalCmd,
    /// TerminalDebian 
    TerminalDebian,
    /// TerminalLinux 
    TerminalLinux,
    /// TerminalOne 
    TerminalOne,
    /// TerminalPowershell 
    TerminalPowershell,
    /// TerminalTmux 
    TerminalTmux,
    /// TerminalTwo 
    TerminalTwo,
    /// TerminalUbuntu 
    TerminalUbuntu,
    /// Terraform 󱁢
    Terraform,
    /// TestTube 󰙨
    TestTube,
    /// TestTubeEmpty 󰤑
    TestTubeEmpty,
    /// TestTubeOff 󰤒
    TestTubeOff,
    /// Text 󰦨
    Text,
    /// TextAccount 󱕰
    TextAccount,
    /// TextBox 󰈚
    TextBox,
    /// TextBoxCheck 󰺦
    TextBoxCheck,
    /// TextBoxCheckOutline 󰺧
    TextBoxCheckOutline,
    /// TextBoxEdit 󱩼
    TextBoxEdit,
    /// TextBoxEditOutline 󱩽
    TextBoxEditOutline,
    /// TextBoxMinus 󰺨
    TextBoxMinus,
    /// TextBoxMinusOutline 󰺩
    TextBoxMinusOutline,
    /// TextBoxMultiple 󰪷
    TextBoxMultiple,
    /// TextBoxMultipleOutline 󰪸
    TextBoxMultipleOutline,
    /// TextBoxOutline 󰧭
    TextBoxOutline,
    /// TextBoxPlus 󰺪
    TextBoxPlus,
    /// TextBoxPlusOutline 󰺫
    TextBoxPlusOutline,
    /// TextBoxRemove 󰺬
    TextBoxRemove,
    /// TextBoxRemoveOutline 󰺭
    TextBoxRemoveOutline,
    /// TextBoxSearch 󰺮
    TextBoxSearch,
    /// TextBoxSearchOutline 󰺯
    TextBoxSearchOutline,
    /// TextHeight 
    TextHeight,
    /// TextLong 󰦪
    TextLong,
    /// TextRecognition 󱄽
    TextRecognition,
    /// TextSearch 󱎸
    TextSearch,
    /// TextSearchVariant 󱩾
    TextSearchVariant,
    /// TextShadow 󰙩
    TextShadow,
    /// TextShort 󰦩
    TextShort,
    /// TextSize 
    TextSize,
    /// TextToSpeech 󰔊
    TextToSpeech,
    /// TextToSpeechOff 󰔋
    TextToSpeechOff,
    /// TextWidth 
    TextWidth,
    /// Texture 󰔌
    Texture,
    /// TextureBox 󰿦
    TextureBox,
    /// Th 
    Th,
    /// ThLarge 
    ThLarge,
    /// ThList 
    ThList,
    /// Theater 󰔍
    Theater,
    /// ThemeLightDark 󰔎
    ThemeLightDark,
    /// Thermometer 
    Thermometer,
    /// ThermometerAlert 󰸁
    ThermometerAlert,
    /// ThermometerBluetooth 󱢕
    ThermometerBluetooth,
    /// ThermometerCheck 󱩿
    ThermometerCheck,
    /// ThermometerChevronDown 󰸂
    ThermometerChevronDown,
    /// ThermometerChevronUp 󰸃
    ThermometerChevronUp,
    /// ThermometerHigh 
    ThermometerHigh,
    /// ThermometerHighOne 󱃂
    ThermometerHighOne,
    /// ThermometerLines 󰔐
    ThermometerLines,
    /// ThermometerLow 
    ThermometerLow,
    /// ThermometerLowOne 󱃃
    ThermometerLowOne,
    /// ThermometerMinus 󰸄
    ThermometerMinus,
    /// ThermometerOff 󱔱
    ThermometerOff,
    /// ThermometerOne 󰔏
    ThermometerOne,
    /// ThermometerPlus 󰸅
    ThermometerPlus,
    /// ThermometerWater 󱪀
    ThermometerWater,
    /// Thermostat 󰎓
    Thermostat,
    /// ThermostatBox 󰢑
    ThermostatBox,
    /// ThinClose 
    ThinClose,
    /// ThoughtBubble 󰟶
    ThoughtBubble,
    /// ThoughtBubbleOutline 󰟷
    ThoughtBubbleOutline,
    /// ThreeBars 
    ThreeBars,
    /// ThreeBarsOne 
    ThreeBarsOne,
    /// Threeeighteight 
    Threeeighteight,
    /// Threeeightfive 
    Threeeightfive,
    /// Threeeightfour 
    Threeeightfour,
    /// Threeeightnine 
    Threeeightnine,
    /// Threeeightseven 
    Threeeightseven,
    /// Threeeightsix 
    Threeeightsix,
    /// Threeeightthree 
    Threeeightthree,
    /// Threeeighttwo 
    Threeeighttwo,
    /// Threeeightzero 
    Threeeightzero,
    /// Threenineeight 
    Threenineeight,
    /// Threeninefive 
    Threeninefive,
    /// Threeninenine 
    Threeninenine,
    /// Threenineseven 
    Threenineseven,
    /// Threeninesix 
    Threeninesix,
    /// Threeninethree 
    Threeninethree,
    /// Threeninetwo 
    Threeninetwo,
    /// Threeoneseven 
    Threeoneseven,
    /// Threeonetwo 
    Threeonetwo,
    /// Threeseveneight 
    Threeseveneight,
    /// Threesevenfour 
    Threesevenfour,
    /// Threesevensix 
    Threesevensix,
    /// Threeseventwo 
    Threeseventwo,
    /// Threesixsix 
    Threesixsix,
    /// Threethreefive 
    Threethreefive,
    /// Threethreefour 
    Threethreefour,
    /// Threetwonine 
    Threetwonine,
    /// Threezerothree 
    Threezerothree,
    /// ThumbDown 󰔑
    ThumbDown,
    /// ThumbDownOutline 󰔒
    ThumbDownOutline,
    /// ThumbUp 󰔓
    ThumbUp,
    /// ThumbUpOutline 󰔔
    ThumbUpOutline,
    /// ThumbsDownAlt 
    ThumbsDownAlt,
    /// ThumbsUpAlt 
    ThumbsUpAlt,
    /// ThumbsUpDown 󰔕
    ThumbsUpDown,
    /// ThumbsUpDownOutline 󱤔
    ThumbsUpDownOutline,
    /// Thumbsdown 
    Thumbsdown,
    /// ThumbsdownOne 
    ThumbsdownOne,
    /// Thumbsup 
    Thumbsup,
    /// ThumbsupOne 
    ThumbsupOne,
    /// Thunderbird 
    Thunderbird,
    /// Ticket 
    Ticket,
    /// TicketAccount 󰔗
    TicketAccount,
    /// TicketConfirmation 󰔘
    TicketConfirmation,
    /// TicketConfirmationOutline 󱎪
    TicketConfirmationOutline,
    /// TicketOne 󰔖
    TicketOne,
    /// TicketOutline 󰤓
    TicketOutline,
    /// TicketPercent 󰜤
    TicketPercent,
    /// TicketPercentOutline 󱐫
    TicketPercentOutline,
    /// Tie 󰔙
    Tie,
    /// Tilde 󰜥
    Tilde,
    /// TildeOff 󱣳
    TildeOff,
    /// Time 
    Time,
    /// Timelapse 󰔚
    Timelapse,
    /// Timeline 󰯑
    Timeline,
    /// TimelineAlert 󰾕
    TimelineAlert,
    /// TimelineAlertOutline 󰾘
    TimelineAlertOutline,
    /// TimelineCheck 󱔲
    TimelineCheck,
    /// TimelineCheckOutline 󱔳
    TimelineCheckOutline,
    /// TimelineClock 󱇻
    TimelineClock,
    /// TimelineClockOutline 󱇼
    TimelineClockOutline,
    /// TimelineHelp 󰾙
    TimelineHelp,
    /// TimelineHelpOutline 󰾚
    TimelineHelpOutline,
    /// TimelineMinus 󱔴
    TimelineMinus,
    /// TimelineMinusOutline 󱔵
    TimelineMinusOutline,
    /// TimelineOutline 󰯒
    TimelineOutline,
    /// TimelinePlus 󰾖
    TimelinePlus,
    /// TimelinePlusOutline 󰾗
    TimelinePlusOutline,
    /// TimelineRemove 󱔶
    TimelineRemove,
    /// TimelineRemoveOutline 󱔷
    TimelineRemoveOutline,
    /// TimelineText 󰯓
    TimelineText,
    /// TimelineTextOutline 󰯔
    TimelineTextOutline,
    /// Timer 󱎫
    Timer,
    /// TimerAlert 󱫌
    TimerAlert,
    /// TimerAlertOutline 󱫍
    TimerAlertOutline,
    /// TimerCancel 󱫎
    TimerCancel,
    /// TimerCancelOutline 󱫏
    TimerCancelOutline,
    /// TimerCheck 󱫐
    TimerCheck,
    /// TimerCheckOutline 󱫑
    TimerCheckOutline,
    /// TimerCog 󱤥
    TimerCog,
    /// TimerCogOutline 󱤦
    TimerCogOutline,
    /// TimerEdit 󱫒
    TimerEdit,
    /// TimerEditOutline 󱫓
    TimerEditOutline,
    /// TimerLock 󱫔
    TimerLock,
    /// TimerLockOpen 󱫕
    TimerLockOpen,
    /// TimerLockOpenOutline 󱫖
    TimerLockOpenOutline,
    /// TimerLockOutline 󱫗
    TimerLockOutline,
    /// TimerMarker 󱫘
    TimerMarker,
    /// TimerMarkerOutline 󱫙
    TimerMarkerOutline,
    /// TimerMinus 󱫚
    TimerMinus,
    /// TimerMinusOutline 󱫛
    TimerMinusOutline,
    /// TimerMusic 󱫜
    TimerMusic,
    /// TimerMusicOutline 󱫝
    TimerMusicOutline,
    /// TimerOff 󱎬
    TimerOff,
    /// TimerOffOutline 󰔞
    TimerOffOutline,
    /// TimerOnezero 󰔜
    TimerOnezero,
    /// TimerOutline 󰔛
    TimerOutline,
    /// TimerPause 󱫞
    TimerPause,
    /// TimerPauseOutline 󱫟
    TimerPauseOutline,
    /// TimerPlay 󱫠
    TimerPlay,
    /// TimerPlayOutline 󱫡
    TimerPlayOutline,
    /// TimerPlus 󱫢
    TimerPlus,
    /// TimerPlusOutline 󱫣
    TimerPlusOutline,
    /// TimerRefresh 󱫤
    TimerRefresh,
    /// TimerRefreshOutline 󱫥
    TimerRefreshOutline,
    /// TimerRemove 󱫦
    TimerRemove,
    /// TimerRemoveOutline 󱫧
    TimerRemoveOutline,
    /// TimerSand 󰔟
    TimerSand,
    /// TimerSandComplete 󱦟
    TimerSandComplete,
    /// TimerSandEmpty 󰚭
    TimerSandEmpty,
    /// TimerSandFull 󰞌
    TimerSandFull,
    /// TimerSandPaused 󱦠
    TimerSandPaused,
    /// TimerSettings 󱤣
    TimerSettings,
    /// TimerSettingsOutline 󱤤
    TimerSettingsOutline,
    /// TimerStar 󱫨
    TimerStar,
    /// TimerStarOutline 󱫩
    TimerStarOutline,
    /// TimerStop 󱫪
    TimerStop,
    /// TimerStopOutline 󱫫
    TimerStopOutline,
    /// TimerSync 󱫬
    TimerSync,
    /// TimerSyncOutline 󱫭
    TimerSyncOutline,
    /// TimerThree 󰔝
    TimerThree,
    /// Timetable 󰔠
    Timetable,
    /// Tint 
    Tint,
    /// Tire 󱢖
    Tire,
    /// Toaster 󱁣
    Toaster,
    /// ToasterOff 󱆷
    ToasterOff,
    /// ToasterOven 󰳓
    ToasterOven,
    /// ToggleSwitch 󰔡
    ToggleSwitch,
    /// ToggleSwitchOff 󰔢
    ToggleSwitchOff,
    /// ToggleSwitchOffOutline 󰨙
    ToggleSwitchOffOutline,
    /// ToggleSwitchOutline 󰨚
    ToggleSwitchOutline,
    /// ToggleSwitchVariant 󱨥
    ToggleSwitchVariant,
    /// ToggleSwitchVariantOff 󱨦
    ToggleSwitchVariantOff,
    /// Toilet 
    Toilet,
    /// ToiletOne 󰦫
    ToiletOne,
    /// Toolbox 󰦬
    Toolbox,
    /// ToolboxOutline 󰦭
    ToolboxOutline,
    /// Tools 
    Tools,
    /// ToolsOne 
    ToolsOne,
    /// ToolsThree 󱁤
    ToolsThree,
    /// ToolsTwo 
    ToolsTwo,
    /// Tooltip 󰔣
    Tooltip,
    /// TooltipAccount 󰀌
    TooltipAccount,
    /// TooltipCellphone 󱠻
    TooltipCellphone,
    /// TooltipCheck 󱕜
    TooltipCheck,
    /// TooltipCheckOutline 󱕝
    TooltipCheckOutline,
    /// TooltipEdit 󰔤
    TooltipEdit,
    /// TooltipEditOutline 󱋅
    TooltipEditOutline,
    /// TooltipImage 󰔥
    TooltipImage,
    /// TooltipImageOutline 󰯕
    TooltipImageOutline,
    /// TooltipMinus 󱕞
    TooltipMinus,
    /// TooltipMinusOutline 󱕟
    TooltipMinusOutline,
    /// TooltipOutline 󰔦
    TooltipOutline,
    /// TooltipPlus 󰯖
    TooltipPlus,
    /// TooltipPlusOutline 󰔧
    TooltipPlusOutline,
    /// TooltipRemove 󱕠
    TooltipRemove,
    /// TooltipRemoveOutline 󱕡
    TooltipRemoveOutline,
    /// TooltipText 󰔨
    TooltipText,
    /// TooltipTextOutline 󰯗
    TooltipTextOutline,
    /// Tooth 
    Tooth,
    /// ToothOne 󰣃
    ToothOne,
    /// ToothOutline 󰔩
    ToothOutline,
    /// Toothbrush 󱄩
    Toothbrush,
    /// ToothbrushElectric 󱄬
    ToothbrushElectric,
    /// ToothbrushPaste 󱄪
    ToothbrushPaste,
    /// TorBrowser 
    TorBrowser,
    /// Torch 󱘆
    Torch,
    /// Tortoise 󰴻
    Tortoise,
    /// Toslink 󱊸
    Toslink,
    /// Tournament 󰦮
    Tournament,
    /// TowTruck 󰠼
    TowTruck,
    /// TowerBeach 󰚁
    TowerBeach,
    /// TowerFire 󰚂
    TowerFire,
    /// TownHall 󱡵
    TownHall,
    /// ToyBrick 󱊈
    ToyBrick,
    /// ToyBrickMarker 󱊉
    ToyBrickMarker,
    /// ToyBrickMarkerOutline 󱊊
    ToyBrickMarkerOutline,
    /// ToyBrickMinus 󱊋
    ToyBrickMinus,
    /// ToyBrickMinusOutline 󱊌
    ToyBrickMinusOutline,
    /// ToyBrickOutline 󱊍
    ToyBrickOutline,
    /// ToyBrickPlus 󱊎
    ToyBrickPlus,
    /// ToyBrickPlusOutline 󱊏
    ToyBrickPlusOutline,
    /// ToyBrickRemove 󱊐
    ToyBrickRemove,
    /// ToyBrickRemoveOutline 󱊑
    ToyBrickRemoveOutline,
    /// ToyBrickSearch 󱊒
    ToyBrickSearch,
    /// ToyBrickSearchOutline 󱊓
    ToyBrickSearchOutline,
    /// TrackLight 󰤔
    TrackLight,
    /// Trackpad 󰟸
    Trackpad,
    /// TrackpadLock 󰤳
    TrackpadLock,
    /// Tractor 󰢒
    Tractor,
    /// TractorVariant 󱓄
    TractorVariant,
    /// Trademark 󰩸
    Trademark,
    /// TrafficCone 󱍼
    TrafficCone,
    /// TrafficLight 󰔫
    TrafficLight,
    /// TrafficLightOutline 󱠪
    TrafficLightOutline,
    /// Train 󰔬
    Train,
    /// TrainCar 󰯘
    TrainCar,
    /// TrainCarPassenger 󱜳
    TrainCarPassenger,
    /// TrainCarPassengerDoor 󱜴
    TrainCarPassengerDoor,
    /// TrainCarPassengerDoorOpen 󱜵
    TrainCarPassengerDoorOpen,
    /// TrainCarPassengerVariant 󱜶
    TrainCarPassengerVariant,
    /// TrainVariant 󰣄
    TrainVariant,
    /// Tram 󰔭
    Tram,
    /// TramSide 󰿧
    TramSide,
    /// Transcribe 󰔮
    Transcribe,
    /// TranscribeClose 󰔯
    TranscribeClose,
    /// Transfer 󱁥
    Transfer,
    /// TransferDown 󰶡
    TransferDown,
    /// TransferLeft 󰶢
    TransferLeft,
    /// TransferRight 󰔰
    TransferRight,
    /// TransferUp 󰶣
    TransferUp,
    /// TransitConnection 󰴼
    TransitConnection,
    /// TransitConnectionHorizontal 󱕆
    TransitConnectionHorizontal,
    /// TransitConnectionVariant 󰴽
    TransitConnectionVariant,
    /// TransitDetour 󰾋
    TransitDetour,
    /// TransitSkip 󱔕
    TransitSkip,
    /// TransitTransfer 󰚮
    TransitTransfer,
    /// Transition 󰤕
    Transition,
    /// TransitionMasked 󰤖
    TransitionMasked,
    /// Translate 󰗊
    Translate,
    /// TranslateOff 󰸆
    TranslateOff,
    /// TransmissionTower 󰴾
    TransmissionTower,
    /// TransmissionTowerExport 󱤬
    TransmissionTowerExport,
    /// TransmissionTowerImport 󱤭
    TransmissionTowerImport,
    /// TransmissionTowerOff 󱧝
    TransmissionTowerOff,
    /// Trash 
    Trash,
    /// TrashCan 󰩹
    TrashCan,
    /// TrashCanOutline 󰩺
    TrashCanOutline,
    /// TrashOne 
    TrashOne,
    /// TrashTwo 
    TrashTwo,
    /// Tray 󱊔
    Tray,
    /// TrayAlert 󱊕
    TrayAlert,
    /// TrayArrowDown 󰄠
    TrayArrowDown,
    /// TrayArrowUp 󰄝
    TrayArrowUp,
    /// TrayFull 󱊖
    TrayFull,
    /// TrayMinus 󱊗
    TrayMinus,
    /// TrayPlus 󱊘
    TrayPlus,
    /// TrayRemove 󱊙
    TrayRemove,
    /// TreasureChest 󰜦
    TreasureChest,
    /// Tree 
    Tree,
    /// TreeOne 󰔱
    TreeOne,
    /// TreeOutline 󰹩
    TreeOutline,
    /// Trello 
    Trello,
    /// TrelloOne 󰔲
    TrelloOne,
    /// TrendingDown 󰔳
    TrendingDown,
    /// TrendingNeutral 󰔴
    TrendingNeutral,
    /// TrendingUp 󰔵
    TrendingUp,
    /// Triangle 󰔶
    Triangle,
    /// TriangleDown 
    TriangleDown,
    /// TriangleDownOne 
    TriangleDownOne,
    /// TriangleLeft 
    TriangleLeft,
    /// TriangleLeftOne 
    TriangleLeftOne,
    /// TriangleOutline 󰔷
    TriangleOutline,
    /// TriangleRight 
    TriangleRight,
    /// TriangleRightOne 
    TriangleRightOne,
    /// TriangleRuler 
    TriangleRuler,
    /// TriangleSmallDown 󱨉
    TriangleSmallDown,
    /// TriangleSmallUp 󱨊
    TriangleSmallUp,
    /// TriangleUp 
    TriangleUp,
    /// TriangleUpOne 
    TriangleUpOne,
    /// TriangleWave 󱑼
    TriangleWave,
    /// Triforce 󰯙
    Triforce,
    /// TrisquelGnuLinux 
    TrisquelGnuLinux,
    /// Trophy 
    Trophy,
    /// TrophyAward 󰔹
    TrophyAward,
    /// TrophyBroken 󰶤
    TrophyBroken,
    /// TrophyOne 
    TrophyOne,
    /// TrophyOutline 󰔺
    TrophyOutline,
    /// TrophyTwo 󰔸
    TrophyTwo,
    /// TrophyVariant 󰔻
    TrophyVariant,
    /// TrophyVariantOutline 󰔼
    TrophyVariantOutline,
    /// Truck 
    Truck,
    /// TruckAlert 󱧞
    TruckAlert,
    /// TruckAlertOutline 󱧟
    TruckAlertOutline,
    /// TruckCargoContainer 󱣘
    TruckCargoContainer,
    /// TruckCheck 󰳔
    TruckCheck,
    /// TruckCheckOutline 󱊚
    TruckCheckOutline,
    /// TruckDelivery 󰔾
    TruckDelivery,
    /// TruckDeliveryOutline 󱊛
    TruckDeliveryOutline,
    /// TruckFast 󰞈
    TruckFast,
    /// TruckFastOutline 󱊜
    TruckFastOutline,
    /// TruckFlatbed 󱢑
    TruckFlatbed,
    /// TruckMinus 󱦮
    TruckMinus,
    /// TruckMinusOutline 󱦽
    TruckMinusOutline,
    /// TruckOne 󰔽
    TruckOne,
    /// TruckOutline 󱊝
    TruckOutline,
    /// TruckPlus 󱦭
    TruckPlus,
    /// TruckPlusOutline 󱦼
    TruckPlusOutline,
    /// TruckRemove 󱦯
    TruckRemove,
    /// TruckRemoveOutline 󱦾
    TruckRemoveOutline,
    /// TruckSnowflake 󱦦
    TruckSnowflake,
    /// TruckTrailer 󰜧
    TruckTrailer,
    /// Trumpet 󱂖
    Trumpet,
    /// TshirtCrew 󰩻
    TshirtCrew,
    /// TshirtCrewOutline 󰔿
    TshirtCrewOutline,
    /// TshirtV 󰩼
    TshirtV,
    /// TshirtVOutline 󰕀
    TshirtVOutline,
    /// Tsunami 󱪁
    Tsunami,
    /// TumbleDryer 󰤗
    TumbleDryer,
    /// TumbleDryerAlert 󱆺
    TumbleDryerAlert,
    /// TumbleDryerOff 󱆻
    TumbleDryerOff,
    /// Tumblr 
    Tumblr,
    /// TumblrSign 
    TumblrSign,
    /// Tune 󰘮
    Tune,
    /// TuneVariant 󱕂
    TuneVariant,
    /// TuneVertical 󰙪
    TuneVertical,
    /// TuneVerticalVariant 󱕃
    TuneVerticalVariant,
    /// Tunnel 󱠽
    Tunnel,
    /// TunnelOutline 󱠾
    TunnelOutline,
    /// Turbine 󱪂
    Turbine,
    /// Turkey 󱜛
    Turkey,
    /// Turnstile 󰳕
    Turnstile,
    /// TurnstileOutline 󰳖
    TurnstileOutline,
    /// Turtle 󰳗
    Turtle,
    /// Tux 
    Tux,
    /// Twitch 󰕃
    Twitch,
    /// Twitter 
    Twitter,
    /// TwitterOne 
    TwitterOne,
    /// TwitterSign 
    TwitterSign,
    /// TwitterTwo 󰕄
    TwitterTwo,
    /// TwoFactorAuthentication 󰦯
    TwoFactorAuthentication,
    /// Twoeightthree 
    Twoeightthree,
    /// Twosevennine 
    Twosevennine,
    /// TypeHierarchy 
    TypeHierarchy,
    /// TypeHierarchySub 
    TypeHierarchySub,
    /// TypeHierarchySuper 
    TypeHierarchySuper,
    /// Typewriter 󰼭
    Typewriter,
    /// Typography 
    Typography,
    /// Ubisoft 󰯚
    Ubisoft,
    /// Ubuntu 󰕈
    Ubuntu,
    /// UbuntuInverse 
    UbuntuInverse,
    /// Ufo 󱃄
    Ufo,
    /// UfoOutline 󱃅
    UfoOutline,
    /// Ul 
    Ul,
    /// UltraHighDefinition 󰟹
    UltraHighDefinition,
    /// Umbraco 󰕉
    Umbraco,
    /// Umbrella 
    Umbrella,
    /// UmbrellaBeach 󱢊
    UmbrellaBeach,
    /// UmbrellaBeachOutline 󱢋
    UmbrellaBeachOutline,
    /// UmbrellaClosed 󰦰
    UmbrellaClosed,
    /// UmbrellaClosedOutline 󱏢
    UmbrellaClosedOutline,
    /// UmbrellaClosedVariant 󱏡
    UmbrellaClosedVariant,
    /// UmbrellaOne 
    UmbrellaOne,
    /// UmbrellaOutline 󰕋
    UmbrellaOutline,
    /// UmbrellaTwo 󰕊
    UmbrellaTwo,
    /// Underline 
    Underline,
    /// Undo 
    Undo,
    /// UndoOne 󰕌
    UndoOne,
    /// UndoVariant 󰕍
    UndoVariant,
    /// Unfold 
    Unfold,
    /// UnfoldLessHorizontal 󰕎
    UnfoldLessHorizontal,
    /// UnfoldLessVertical 󰝠
    UnfoldLessVertical,
    /// UnfoldMoreHorizontal 󰕏
    UnfoldMoreHorizontal,
    /// UnfoldMoreVertical 󰝡
    UnfoldMoreVertical,
    /// UnfoldOne 
    UnfoldOne,
    /// Ungroup 󰕐
    Ungroup,
    /// UngroupByRefType 
    UngroupByRefType,
    /// Unicode 󰻐
    Unicode,
    /// Unicorn 󱗂
    Unicorn,
    /// UnicornVariant 󱗃
    UnicornVariant,
    /// Unicycle 󱗥
    Unicycle,
    /// Unity 󰚯
    Unity,
    /// Unlink 
    Unlink,
    /// UnlinkOne 
    UnlinkOne,
    /// Unlock 
    Unlock,
    /// UnlockAlt 
    UnlockAlt,
    /// UnlockOne 
    UnlockOne,
    /// UnlockTwo 
    UnlockTwo,
    /// Unmute 
    Unmute,
    /// UnmuteOne 
    UnmuteOne,
    /// Unread 
    Unread,
    /// Unreal 󰦱
    Unreal,
    /// Unverified 
    Unverified,
    /// UnverifiedOne 
    UnverifiedOne,
    /// Update 󰚰
    Update,
    /// Upload 
    Upload,
    /// UploadAlt 
    UploadAlt,
    /// UploadLock 󱍳
    UploadLock,
    /// UploadLockOutline 󱍴
    UploadLockOutline,
    /// UploadMultiple 󰠽
    UploadMultiple,
    /// UploadNetwork 󰛶
    UploadNetwork,
    /// UploadNetworkOutline 󰳘
    UploadNetworkOutline,
    /// UploadOff 󱃆
    UploadOff,
    /// UploadOffOutline 󱃇
    UploadOffOutline,
    /// UploadOne 
    UploadOne,
    /// UploadOutline 󰸇
    UploadOutline,
    /// UploadTwo 󰕒
    UploadTwo,
    /// Usb 󰕓
    Usb,
    /// UsbFlashDrive 󱊞
    UsbFlashDrive,
    /// UsbFlashDriveOutline 󱊟
    UsbFlashDriveOutline,
    /// UsbPort 󱇰
    UsbPort,
    /// Usd 
    Usd,
    /// User 
    User,
    /// UserMd 
    UserMd,
    /// Uterus 
    Uterus,
    /// Vacuum 󱦡
    Vacuum,
    /// VacuumOutline 󱦢
    VacuumOutline,
    /// Valve 󱁦
    Valve,
    /// ValveClosed 󱁧
    ValveClosed,
    /// ValveOpen 󱁨
    ValveOpen,
    /// VanPassenger 󰟺
    VanPassenger,
    /// VanUtility 󰟻
    VanUtility,
    /// VanillaOs 
    VanillaOs,
    /// Vanish 󰟼
    Vanish,
    /// VanishQuarter 󱕔
    VanishQuarter,
    /// VanityLight 󱇡
    VanityLight,
    /// Variable 󰫧
    Variable,
    /// VariableBox 󱄑
    VariableBox,
    /// VariableGroup 
    VariableGroup,
    /// VectorArrangeAbove 󰕔
    VectorArrangeAbove,
    /// VectorArrangeBelow 󰕕
    VectorArrangeBelow,
    /// VectorBezier 󰫨
    VectorBezier,
    /// VectorCircle 󰕖
    VectorCircle,
    /// VectorCircleVariant 󰕗
    VectorCircleVariant,
    /// VectorCombine 󰕘
    VectorCombine,
    /// VectorCurve 󰕙
    VectorCurve,
    /// VectorDifference 󰕚
    VectorDifference,
    /// VectorDifferenceAb 󰕛
    VectorDifferenceAb,
    /// VectorDifferenceBa 󰕜
    VectorDifferenceBa,
    /// VectorEllipse 󰢓
    VectorEllipse,
    /// VectorIntersection 󰕝
    VectorIntersection,
    /// VectorLine 󰕞
    VectorLine,
    /// VectorLink 󰿨
    VectorLink,
    /// VectorPoint 󰕟
    VectorPoint,
    /// VectorPolygon 󰕠
    VectorPolygon,
    /// VectorPolygonVariant 󱡖
    VectorPolygonVariant,
    /// VectorPolyline 󰕡
    VectorPolyline,
    /// VectorPolylineEdit 󱈥
    VectorPolylineEdit,
    /// VectorPolylineMinus 󱈦
    VectorPolylineMinus,
    /// VectorPolylinePlus 󱈧
    VectorPolylinePlus,
    /// VectorPolylineRemove 󱈨
    VectorPolylineRemove,
    /// VectorRadius 󰝊
    VectorRadius,
    /// VectorRectangle 󰗆
    VectorRectangle,
    /// VectorSelection 󰕢
    VectorSelection,
    /// VectorSquare 󰀁
    VectorSquare,
    /// VectorSquareClose 󱡗
    VectorSquareClose,
    /// VectorSquareEdit 󱣙
    VectorSquareEdit,
    /// VectorSquareMinus 󱣚
    VectorSquareMinus,
    /// VectorSquareOpen 󱡘
    VectorSquareOpen,
    /// VectorSquarePlus 󱣛
    VectorSquarePlus,
    /// VectorSquareRemove 󱣜
    VectorSquareRemove,
    /// VectorTriangle 󰕣
    VectorTriangle,
    /// VectorUnion 󰕤
    VectorUnion,
    /// Venus 
    Venus,
    /// Verified 
    Verified,
    /// VerifiedFilled 
    VerifiedFilled,
    /// VerifiedOne 
    VerifiedOne,
    /// Versions 
    Versions,
    /// VersionsOne 
    VersionsOne,
    /// Vhs 󰨛
    Vhs,
    /// Vibrate 󰕦
    Vibrate,
    /// VibrateOff 󰳙
    VibrateOff,
    /// Video 
    Video,
    /// VideoAccount 󰤙
    VideoAccount,
    /// VideoBox 󰃽
    VideoBox,
    /// VideoBoxOff 󰃾
    VideoBoxOff,
    /// VideoCheck 󱁩
    VideoCheck,
    /// VideoCheckOutline 󱁪
    VideoCheckOutline,
    /// VideoFourkBox 󰠾
    VideoFourkBox,
    /// VideoHighDefinition 󱔮
    VideoHighDefinition,
    /// VideoImage 󰤚
    VideoImage,
    /// VideoInputAntenna 󰠿
    VideoInputAntenna,
    /// VideoInputComponent 󰡀
    VideoInputComponent,
    /// VideoInputHdmi 󰡁
    VideoInputHdmi,
    /// VideoInputScart 󰾌
    VideoInputScart,
    /// VideoInputSvideo 󰡂
    VideoInputSvideo,
    /// VideoMarker 󱦩
    VideoMarker,
    /// VideoMarkerOutline 󱦪
    VideoMarkerOutline,
    /// VideoMinus 󰦲
    VideoMinus,
    /// VideoMinusOutline 󰊺
    VideoMinusOutline,
    /// VideoOff 󰕨
    VideoOff,
    /// VideoOffOutline 󰯛
    VideoOffOutline,
    /// VideoOne 󰕧
    VideoOne,
    /// VideoOutline 󰯜
    VideoOutline,
    /// VideoPlus 󰦳
    VideoPlus,
    /// VideoPlusOutline 󰇓
    VideoPlusOutline,
    /// VideoStabilization 󰤛
    VideoStabilization,
    /// VideoSwitch 󰕩
    VideoSwitch,
    /// VideoSwitchOutline 󰞐
    VideoSwitchOutline,
    /// VideoThreed 󰟽
    VideoThreed,
    /// VideoThreedOff 󱏙
    VideoThreedOff,
    /// VideoThreedVariant 󰻑
    VideoThreedVariant,
    /// VideoTwod 󱨜
    VideoTwod,
    /// VideoVintage 󰨜
    VideoVintage,
    /// VideoWireless 󰻒
    VideoWireless,
    /// VideoWirelessOutline 󰻓
    VideoWirelessOutline,
    /// ViewAgenda 󰕪
    ViewAgenda,
    /// ViewAgendaOutline 󱇘
    ViewAgendaOutline,
    /// ViewArray 󰕫
    ViewArray,
    /// ViewArrayOutline 󱒅
    ViewArrayOutline,
    /// ViewCarousel 󰕬
    ViewCarousel,
    /// ViewCarouselOutline 󱒆
    ViewCarouselOutline,
    /// ViewColumn 󰕭
    ViewColumn,
    /// ViewColumnOutline 󱒇
    ViewColumnOutline,
    /// ViewComfy 󰹪
    ViewComfy,
    /// ViewComfyOutline 󱒈
    ViewComfyOutline,
    /// ViewCompact 󰹫
    ViewCompact,
    /// ViewCompactOutline 󰹬
    ViewCompactOutline,
    /// ViewDashboard 󰕮
    ViewDashboard,
    /// ViewDashboardEdit 󱥇
    ViewDashboardEdit,
    /// ViewDashboardEditOutline 󱥈
    ViewDashboardEditOutline,
    /// ViewDashboardOutline 󰨝
    ViewDashboardOutline,
    /// ViewDashboardVariant 󰡃
    ViewDashboardVariant,
    /// ViewDashboardVariantOutline 󱒉
    ViewDashboardVariantOutline,
    /// ViewDay 󰕯
    ViewDay,
    /// ViewDayOutline 󱒊
    ViewDayOutline,
    /// ViewGallery 󱢈
    ViewGallery,
    /// ViewGalleryOutline 󱢉
    ViewGalleryOutline,
    /// ViewGrid 󰕰
    ViewGrid,
    /// ViewGridOutline 󱇙
    ViewGridOutline,
    /// ViewGridPlus 󰾍
    ViewGridPlus,
    /// ViewGridPlusOutline 󱇚
    ViewGridPlusOutline,
    /// ViewHeadline 󰕱
    ViewHeadline,
    /// ViewList 󰕲
    ViewList,
    /// ViewListOutline 󱒋
    ViewListOutline,
    /// ViewModule 󰕳
    ViewModule,
    /// ViewModuleOutline 󱒌
    ViewModuleOutline,
    /// ViewParallel 󰜨
    ViewParallel,
    /// ViewParallelOutline 󱒍
    ViewParallelOutline,
    /// ViewQuilt 󰕴
    ViewQuilt,
    /// ViewQuiltOutline 󱒎
    ViewQuiltOutline,
    /// ViewSequential 󰜩
    ViewSequential,
    /// ViewSequentialOutline 󱒏
    ViewSequentialOutline,
    /// ViewSplitHorizontal 󰯋
    ViewSplitHorizontal,
    /// ViewSplitVertical 󰯌
    ViewSplitVertical,
    /// ViewStream 󰕵
    ViewStream,
    /// ViewStreamOutline 󱒐
    ViewStreamOutline,
    /// ViewWeek 󰕶
    ViewWeek,
    /// ViewWeekOutline 󱒑
    ViewWeekOutline,
    /// Vimeo 󰕷
    Vimeo,
    /// VimeoSquare 
    VimeoSquare,
    /// Violin 󰘏
    Violin,
    /// VirtualReality 󰢔
    VirtualReality,
    /// Virus 
    Virus,
    /// VirusOff 󱣡
    VirusOff,
    /// VirusOffOutline 󱣢
    VirusOffOutline,
    /// VirusOne 󱎶
    VirusOne,
    /// VirusOutline 󱎷
    VirusOutline,
    /// Vk 
    Vk,
    /// Vlc 󰕼
    Vlc,
    /// Vm 
    Vm,
    /// VmActive 
    VmActive,
    /// VmConnect 
    VmConnect,
    /// VmOutline 
    VmOutline,
    /// VmRunning 
    VmRunning,
    /// Voicemail 󰕽
    Voicemail,
    /// Void 
    Void,
    /// Volcano 󱪃
    Volcano,
    /// VolcanoOutline 󱪄
    VolcanoOutline,
    /// Volleyball 󰦴
    Volleyball,
    /// VolumeDown 
    VolumeDown,
    /// VolumeHigh 󰕾
    VolumeHigh,
    /// VolumeLow 󰕿
    VolumeLow,
    /// VolumeMedium 󰖀
    VolumeMedium,
    /// VolumeMinus 󰝞
    VolumeMinus,
    /// VolumeMute 󰝟
    VolumeMute,
    /// VolumeOff 󰖁
    VolumeOff,
    /// VolumePlus 󰝝
    VolumePlus,
    /// VolumeSource 󱄠
    VolumeSource,
    /// VolumeUp 
    VolumeUp,
    /// VolumeVariantOff 󰸈
    VolumeVariantOff,
    /// VolumeVibrate 󱄡
    VolumeVibrate,
    /// Vote 󰨟
    Vote,
    /// VoteOutline 󰨠
    VoteOutline,
    /// Vpn 󰖂
    Vpn,
    /// VsCodium 
    VsCodium,
    /// Vuejs 󰡄
    Vuejs,
    /// Vuetify 󰹭
    Vuetify,
    /// Walk 󰖃
    Walk,
    /// Walking 
    Walking,
    /// Wall 󰟾
    Wall,
    /// WallFire 󱨑
    WallFire,
    /// WallSconce 󰤜
    WallSconce,
    /// WallSconceFlat 󰤝
    WallSconceFlat,
    /// WallSconceFlatOutline 󱟉
    WallSconceFlatOutline,
    /// WallSconceFlatVariant 󰐜
    WallSconceFlatVariant,
    /// WallSconceFlatVariantOutline 󱟊
    WallSconceFlatVariantOutline,
    /// WallSconceOutline 󱟋
    WallSconceOutline,
    /// WallSconceRound 󰝈
    WallSconceRound,
    /// WallSconceRoundOutline 󱟌
    WallSconceRoundOutline,
    /// WallSconceRoundVariant 󰤞
    WallSconceRoundVariant,
    /// WallSconceRoundVariantOutline 󱟍
    WallSconceRoundVariantOutline,
    /// Wallet 
    Wallet,
    /// WalletGiftcard 󰖅
    WalletGiftcard,
    /// WalletMembership 󰖆
    WalletMembership,
    /// WalletOne 󰖄
    WalletOne,
    /// WalletOutline 󰯝
    WalletOutline,
    /// WalletPlus 󰾎
    WalletPlus,
    /// WalletPlusOutline 󰾏
    WalletPlusOutline,
    /// WalletTravel 󰖇
    WalletTravel,
    /// Wallpaper 󰸉
    Wallpaper,
    /// Wan 󰖈
    Wan,
    /// Wand 
    Wand,
    /// Wardrobe 󰾐
    Wardrobe,
    /// WardrobeOutline 󰾑
    WardrobeOutline,
    /// Warehouse 󰾁
    Warehouse,
    /// Warning 
    Warning,
    /// WarningSign 
    WarningSign,
    /// WashingMachine 󰜪
    WashingMachine,
    /// WashingMachineAlert 󱆼
    WashingMachineAlert,
    /// WashingMachineOff 󱆽
    WashingMachineOff,
    /// Watch 
    Watch,
    /// WatchExport 󰖊
    WatchExport,
    /// WatchExportVariant 󰢕
    WatchExportVariant,
    /// WatchImport 󰖋
    WatchImport,
    /// WatchImportVariant 󰢖
    WatchImportVariant,
    /// WatchOne 󰖉
    WatchOne,
    /// WatchVariant 󰢗
    WatchVariant,
    /// WatchVibrate 󰚱
    WatchVibrate,
    /// WatchVibrateOff 󰳚
    WatchVibrateOff,
    /// Water 󰖌
    Water,
    /// WaterAlert 󱔂
    WaterAlert,
    /// WaterAlertOutline 󱔃
    WaterAlertOutline,
    /// WaterBoiler 󰾒
    WaterBoiler,
    /// WaterBoilerAlert 󱆳
    WaterBoilerAlert,
    /// WaterBoilerOff 󱆴
    WaterBoilerOff,
    /// WaterCheck 󱔄
    WaterCheck,
    /// WaterCheckOutline 󱔅
    WaterCheckOutline,
    /// WaterCircle 󱠆
    WaterCircle,
    /// WaterMinus 󱔆
    WaterMinus,
    /// WaterMinusOutline 󱔇
    WaterMinusOutline,
    /// WaterOff 󰖍
    WaterOff,
    /// WaterOffOutline 󱔈
    WaterOffOutline,
    /// WaterOpacity 󱡕
    WaterOpacity,
    /// WaterOutline 󰸊
    WaterOutline,
    /// WaterPercent 󰖎
    WaterPercent,
    /// WaterPercentAlert 󱔉
    WaterPercentAlert,
    /// WaterPlus 󱔊
    WaterPlus,
    /// WaterPlusOutline 󱔋
    WaterPlusOutline,
    /// WaterPolo 󱊠
    WaterPolo,
    /// WaterPump 󰖏
    WaterPump,
    /// WaterPumpOff 󰾓
    WaterPumpOff,
    /// WaterRemove 󱔌
    WaterRemove,
    /// WaterRemoveOutline 󱔍
    WaterRemoveOutline,
    /// WaterSync 󱟆
    WaterSync,
    /// WaterThermometer 󱪅
    WaterThermometer,
    /// WaterThermometerOutline 󱪆
    WaterThermometerOutline,
    /// WaterWell 󱁫
    WaterWell,
    /// WaterWellOutline 󱁬
    WaterWellOutline,
    /// Waterfall 󱡉
    Waterfall,
    /// WateringCan 󱒁
    WateringCan,
    /// WateringCanOutline 󱒂
    WateringCanOutline,
    /// Watermark 󰘒
    Watermark,
    /// Wave 󰼮
    Wave,
    /// Waveform 󱑽
    Waveform,
    /// Waves 󰞍
    Waves,
    /// WavesArrowLeft 󱡙
    WavesArrowLeft,
    /// WavesArrowRight 󱡚
    WavesArrowRight,
    /// WavesArrowUp 󱡛
    WavesArrowUp,
    /// Wayland 
    Wayland,
    /// Waze 󰯞
    Waze,
    /// WeatherCloudy 󰖐
    WeatherCloudy,
    /// WeatherCloudyAlert 󰼯
    WeatherCloudyAlert,
    /// WeatherCloudyArrowRight 󰹮
    WeatherCloudyArrowRight,
    /// WeatherCloudyClock 󱣶
    WeatherCloudyClock,
    /// WeatherFog 󰖑
    WeatherFog,
    /// WeatherHail 󰖒
    WeatherHail,
    /// WeatherHazy 󰼰
    WeatherHazy,
    /// WeatherHurricane 󰢘
    WeatherHurricane,
    /// WeatherLightning 󰖓
    WeatherLightning,
    /// WeatherLightningRainy 󰙾
    WeatherLightningRainy,
    /// WeatherNight 󰖔
    WeatherNight,
    /// WeatherNightPartlyCloudy 󰼱
    WeatherNightPartlyCloudy,
    /// WeatherPartlyCloudy 󰖕
    WeatherPartlyCloudy,
    /// WeatherPartlyLightning 󰼲
    WeatherPartlyLightning,
    /// WeatherPartlyRainy 󰼳
    WeatherPartlyRainy,
    /// WeatherPartlySnowy 󰼴
    WeatherPartlySnowy,
    /// WeatherPartlySnowyRainy 󰼵
    WeatherPartlySnowyRainy,
    /// WeatherPouring 󰖖
    WeatherPouring,
    /// WeatherRainy 󰖗
    WeatherRainy,
    /// WeatherSnowy 󰖘
    WeatherSnowy,
    /// WeatherSnowyHeavy 󰼶
    WeatherSnowyHeavy,
    /// WeatherSnowyRainy 󰙿
    WeatherSnowyRainy,
    /// WeatherSunny 󰖙
    WeatherSunny,
    /// WeatherSunnyAlert 󰼷
    WeatherSunnyAlert,
    /// WeatherSunnyOff 󱓤
    WeatherSunnyOff,
    /// WeatherSunset 󰖚
    WeatherSunset,
    /// WeatherSunsetDown 󰖛
    WeatherSunsetDown,
    /// WeatherSunsetUp 󰖜
    WeatherSunsetUp,
    /// WeatherTornado 󰼸
    WeatherTornado,
    /// WeatherWindy 󰖝
    WeatherWindy,
    /// WeatherWindyVariant 󰖞
    WeatherWindyVariant,
    /// Web 󰖟
    Web,
    /// WebBox 󰾔
    WebBox,
    /// WebCancel 󱞐
    WebCancel,
    /// WebCheck 󰞉
    WebCheck,
    /// WebClock 󱉊
    WebClock,
    /// WebMinus 󱂠
    WebMinus,
    /// WebOff 󰪎
    WebOff,
    /// WebPlus 󰀳
    WebPlus,
    /// WebRefresh 󱞑
    WebRefresh,
    /// WebRemove 󰕑
    WebRemove,
    /// WebSync 󱞒
    WebSync,
    /// Webcam 󰖠
    Webcam,
    /// WebcamOff 󱜷
    WebcamOff,
    /// Webhook 
    Webhook,
    /// WebhookOne 󰘯
    WebhookOne,
    /// Webpack 󰜫
    Webpack,
    /// Webrtc 󱉈
    Webrtc,
    /// Wechat 󰘑
    Wechat,
    /// Weibo 
    Weibo,
    /// Weight 󰖡
    Weight,
    /// WeightGram 󰴿
    WeightGram,
    /// WeightKilogram 󰖢
    WeightKilogram,
    /// WeightLifter 󱅝
    WeightLifter,
    /// WeightPound 󰦵
    WeightPound,
    /// Whatsapp 󰖣
    Whatsapp,
    /// WheelBarrow 󱓲
    WheelBarrow,
    /// Wheelchair 󱪇
    Wheelchair,
    /// WheelchairAccessibility 󰖤
    WheelchairAccessibility,
    /// Whistle 󰦶
    Whistle,
    /// WhistleOutline 󱊼
    WhistleOutline,
    /// WhiteBalanceAuto 󰖥
    WhiteBalanceAuto,
    /// WhiteBalanceIncandescent 󰖦
    WhiteBalanceIncandescent,
    /// WhiteBalanceIridescent 󰖧
    WhiteBalanceIridescent,
    /// WhiteBalanceSunny 󰖨
    WhiteBalanceSunny,
    /// Whitespace 
    Whitespace,
    /// WholeWord 
    WholeWord,
    /// Widgets 󰜬
    Widgets,
    /// WidgetsOutline 󱍕
    WidgetsOutline,
    /// Wifi 󰖩
    Wifi,
    /// WifiAlert 󱚵
    WifiAlert,
    /// WifiArrowDown 󱚶
    WifiArrowDown,
    /// WifiArrowLeft 󱚷
    WifiArrowLeft,
    /// WifiArrowLeftRight 󱚸
    WifiArrowLeftRight,
    /// WifiArrowRight 󱚹
    WifiArrowRight,
    /// WifiArrowUp 󱚺
    WifiArrowUp,
    /// WifiArrowUpDown 󱚻
    WifiArrowUpDown,
    /// WifiCancel 󱚼
    WifiCancel,
    /// WifiCheck 󱚽
    WifiCheck,
    /// WifiCog 󱚾
    WifiCog,
    /// WifiLock 󱚿
    WifiLock,
    /// WifiLockOpen 󱛀
    WifiLockOpen,
    /// WifiMarker 󱛁
    WifiMarker,
    /// WifiMinus 󱛂
    WifiMinus,
    /// WifiOff 󰖪
    WifiOff,
    /// WifiPlus 󱛃
    WifiPlus,
    /// WifiRefresh 󱛄
    WifiRefresh,
    /// WifiRemove 󱛅
    WifiRemove,
    /// WifiSettings 󱛆
    WifiSettings,
    /// WifiStar 󰸋
    WifiStar,
    /// WifiStrengthAlertOutline 󰤫
    WifiStrengthAlertOutline,
    /// WifiStrengthFour 󰤨
    WifiStrengthFour,
    /// WifiStrengthFourAlert 󰤩
    WifiStrengthFourAlert,
    /// WifiStrengthFourLock 󰤪
    WifiStrengthFourLock,
    /// WifiStrengthFourLockOpen 󱛎
    WifiStrengthFourLockOpen,
    /// WifiStrengthLockOpenOutline 󱛏
    WifiStrengthLockOpenOutline,
    /// WifiStrengthLockOutline 󰤬
    WifiStrengthLockOutline,
    /// WifiStrengthOff 󰤭
    WifiStrengthOff,
    /// WifiStrengthOffOutline 󰤮
    WifiStrengthOffOutline,
    /// WifiStrengthOne 󰤟
    WifiStrengthOne,
    /// WifiStrengthOneAlert 󰤠
    WifiStrengthOneAlert,
    /// WifiStrengthOneLock 󰤡
    WifiStrengthOneLock,
    /// WifiStrengthOneLockOpen 󱛋
    WifiStrengthOneLockOpen,
    /// WifiStrengthOutline 󰤯
    WifiStrengthOutline,
    /// WifiStrengthThree 󰤥
    WifiStrengthThree,
    /// WifiStrengthThreeAlert 󰤦
    WifiStrengthThreeAlert,
    /// WifiStrengthThreeLock 󰤧
    WifiStrengthThreeLock,
    /// WifiStrengthThreeLockOpen 󱛍
    WifiStrengthThreeLockOpen,
    /// WifiStrengthTwo 󰤢
    WifiStrengthTwo,
    /// WifiStrengthTwoAlert 󰤣
    WifiStrengthTwoAlert,
    /// WifiStrengthTwoLock 󰤤
    WifiStrengthTwoLock,
    /// WifiStrengthTwoLockOpen 󱛌
    WifiStrengthTwoLockOpen,
    /// WifiSync 󱛇
    WifiSync,
    /// Wikimedia 
    Wikimedia,
    /// Wikipedia 󰖬
    Wikipedia,
    /// Wind 
    Wind,
    /// WindPower 󱪈
    WindPower,
    /// WindPowerOutline 󱪉
    WindPowerOutline,
    /// WindTurbine 󰶥
    WindTurbine,
    /// WindTurbineAlert 󱦫
    WindTurbineAlert,
    /// WindTurbineCheck 󱦬
    WindTurbineCheck,
    /// Window 
    Window,
    /// WindowClose 󰖭
    WindowClose,
    /// WindowClosed 󰖮
    WindowClosed,
    /// WindowClosedVariant 󱇛
    WindowClosedVariant,
    /// WindowMaximize 󰖯
    WindowMaximize,
    /// WindowMinimize 󰖰
    WindowMinimize,
    /// WindowOpen 󰖱
    WindowOpen,
    /// WindowOpenVariant 󱇜
    WindowOpenVariant,
    /// WindowRestore 󰖲
    WindowRestore,
    /// WindowShutter 󱄜
    WindowShutter,
    /// WindowShutterAlert 󱄝
    WindowShutterAlert,
    /// WindowShutterCog 󱪊
    WindowShutterCog,
    /// WindowShutterOpen 󱄞
    WindowShutterOpen,
    /// WindowShutterSettings 󱪋
    WindowShutterSettings,
    /// Windows 
    Windows,
    /// Windsock 󱗺
    Windsock,
    /// Wiper 󰫩
    Wiper,
    /// WiperWash 󰶦
    WiperWash,
    /// WiperWashAlert 󱣟
    WiperWashAlert,
    /// WizardHat 󱑷
    WizardHat,
    /// WordWrap 
    WordWrap,
    /// Wordpress 󰖴
    Wordpress,
    /// Workflow 
    Workflow,
    /// WorkspaceTrusted 
    WorkspaceTrusted,
    /// WorkspaceUnknown 
    WorkspaceUnknown,
    /// WorkspaceUntrusted 
    WorkspaceUntrusted,
    /// Wrap 󰖶
    Wrap,
    /// WrapDisabled 󰯟
    WrapDisabled,
    /// Wrench 
    Wrench,
    /// WrenchClock 󱦣
    WrenchClock,
    /// WrenchOne 󰖷
    WrenchOne,
    /// WrenchOutline 󰯠
    WrenchOutline,
    /// Wthreec 
    Wthreec,
    /// X 
    X,
    /// XCircle 
    XCircle,
    /// XCircleFill 
    XCircleFill,
    /// Xamarin 󰡅
    Xamarin,
    /// Xbox 
    Xbox,
    /// Xerolinux 
    Xerolinux,
    /// Xfce 
    Xfce,
    /// Xing 
    Xing,
    /// XingSign 
    XingSign,
    /// Xml 󰗀
    Xml,
    /// Xmonad 
    Xmonad,
    /// Xmpp 󰟿
    Xmpp,
    /// Xorg 
    Xorg,
    /// Yahoo 󰭏
    Yahoo,
    /// Yeast 󰗁
    Yeast,
    /// YinYang 󰚀
    YinYang,
    /// Yoga 󱅼
    Yoga,
    /// Youtube 
    Youtube,
    /// YoutubeGaming 󰡈
    YoutubeGaming,
    /// YoutubeOne 󰗃
    YoutubeOne,
    /// YoutubePlay 
    YoutubePlay,
    /// YoutubeSign 
    YoutubeSign,
    /// YoutubeStudio 󰡇
    YoutubeStudio,
    /// YoutubeSubscription 󰵀
    YoutubeSubscription,
    /// YoutubeTv 󰑈
    YoutubeTv,
    /// Yurt 󱔖
    Yurt,
    /// ZWave 󰫪
    ZWave,
    /// Zap ⚡
    Zap,
    /// Zend 󰫫
    Zend,
    /// Zigbee 󰵁
    Zigbee,
    /// ZipBox 󰗄
    ZipBox,
    /// ZipBoxOutline 󰿺
    ZipBoxOutline,
    /// ZipDisk 󰨣
    ZipDisk,
    /// ZodiacAquarius 󰩽
    ZodiacAquarius,
    /// ZodiacAries 󰩾
    ZodiacAries,
    /// ZodiacCancer 󰩿
    ZodiacCancer,
    /// ZodiacCapricorn 󰪀
    ZodiacCapricorn,
    /// ZodiacGemini 󰪁
    ZodiacGemini,
    /// ZodiacLeo 󰪂
    ZodiacLeo,
    /// ZodiacLibra 󰪃
    ZodiacLibra,
    /// ZodiacPisces 󰪄
    ZodiacPisces,
    /// ZodiacSagittarius 󰪅
    ZodiacSagittarius,
    /// ZodiacScorpio 󰪆
    ZodiacScorpio,
    /// ZodiacTaurus 󰪇
    ZodiacTaurus,
    /// ZodiacVirgo 󰪈
    ZodiacVirgo,
    /// ZoomIn 
    ZoomIn,
    /// ZoomInOne 
    ZoomInOne,
    /// ZoomOut 
    ZoomOut,
    /// ZoomOutOne 
    ZoomOutOne,
    /// ZorinOs 
    ZorinOs,
}

/// Converts an [`Nerd`] into a [`char`]
#[must_use]
pub const fn icon_to_char(icon: Nerd) -> char {
    match icon {
        Nerd::AbTesting => '\u{f01c9}',
        Nerd::Abacus => '\u{f16e0}',
        Nerd::AbjadArabic => '\u{f1328}',
        Nerd::AbjadHebrew => '\u{f1329}',
        Nerd::AbugidaDevanagari => '\u{f132a}',
        Nerd::AbugidaThai => '\u{f132b}',
        Nerd::AccessPoint => '\u{f0003}',
        Nerd::AccessPointCheck => '\u{f1538}',
        Nerd::AccessPointMinus => '\u{f1539}',
        Nerd::AccessPointNetwork => '\u{f0002}',
        Nerd::AccessPointNetworkOff => '\u{f0be1}',
        Nerd::AccessPointOff => '\u{f1511}',
        Nerd::AccessPointPlus => '\u{f153a}',
        Nerd::AccessPointRemove => '\u{f153b}',
        Nerd::Accessibility => '\u{f406}',
        Nerd::AccessibilityInset => '\u{f40b}',
        Nerd::Account => '\u{eb99}',
        Nerd::AccountAlert => '\u{f0005}',
        Nerd::AccountAlertOutline => '\u{f0b50}',
        Nerd::AccountArrowDown => '\u{f1868}',
        Nerd::AccountArrowDownOutline => '\u{f1869}',
        Nerd::AccountArrowLeft => '\u{f0b51}',
        Nerd::AccountArrowLeftOutline => '\u{f0b52}',
        Nerd::AccountArrowRight => '\u{f0b53}',
        Nerd::AccountArrowRightOutline => '\u{f0b54}',
        Nerd::AccountArrowUp => '\u{f1867}',
        Nerd::AccountArrowUpOutline => '\u{f186a}',
        Nerd::AccountBox => '\u{f0006}',
        Nerd::AccountBoxMultiple => '\u{f0934}',
        Nerd::AccountBoxMultipleOutline => '\u{f100a}',
        Nerd::AccountBoxOutline => '\u{f0007}',
        Nerd::AccountCancel => '\u{f12df}',
        Nerd::AccountCancelOutline => '\u{f12e0}',
        Nerd::AccountCash => '\u{f1097}',
        Nerd::AccountCashOutline => '\u{f1098}',
        Nerd::AccountCheck => '\u{f0008}',
        Nerd::AccountCheckOutline => '\u{f0be2}',
        Nerd::AccountChild => '\u{f0a89}',
        Nerd::AccountChildCircle => '\u{f0a8a}',
        Nerd::AccountChildOutline => '\u{f10c8}',
        Nerd::AccountCircle => '\u{f0009}',
        Nerd::AccountCircleOutline => '\u{f0b55}',
        Nerd::AccountClock => '\u{f0b56}',
        Nerd::AccountClockOutline => '\u{f0b57}',
        Nerd::AccountCog => '\u{f1370}',
        Nerd::AccountCogOutline => '\u{f1371}',
        Nerd::AccountConvert => '\u{f000a}',
        Nerd::AccountConvertOutline => '\u{f1301}',
        Nerd::AccountCowboyHat => '\u{f0e9b}',
        Nerd::AccountCowboyHatOutline => '\u{f17f3}',
        Nerd::AccountDetails => '\u{f0631}',
        Nerd::AccountDetailsOutline => '\u{f1372}',
        Nerd::AccountEdit => '\u{f06bc}',
        Nerd::AccountEditOutline => '\u{f0ffb}',
        Nerd::AccountEye => '\u{f0420}',
        Nerd::AccountEyeOutline => '\u{f127b}',
        Nerd::AccountFilter => '\u{f0936}',
        Nerd::AccountFilterOutline => '\u{f0f9d}',
        Nerd::AccountGroup => '\u{f0849}',
        Nerd::AccountGroupOutline => '\u{f0b58}',
        Nerd::AccountHardHat => '\u{f05b5}',
        Nerd::AccountHardHatOutline => '\u{f1a1f}',
        Nerd::AccountHeart => '\u{f0899}',
        Nerd::AccountHeartOutline => '\u{f0be3}',
        Nerd::AccountInjury => '\u{f1815}',
        Nerd::AccountInjuryOutline => '\u{f1816}',
        Nerd::AccountKey => '\u{f000b}',
        Nerd::AccountKeyOutline => '\u{f0be4}',
        Nerd::AccountLock => '\u{f115e}',
        Nerd::AccountLockOpen => '\u{f1960}',
        Nerd::AccountLockOpenOutline => '\u{f1961}',
        Nerd::AccountLockOutline => '\u{f115f}',
        Nerd::AccountMinus => '\u{f000d}',
        Nerd::AccountMinusOutline => '\u{f0aec}',
        Nerd::AccountMultiple => '\u{f000e}',
        Nerd::AccountMultipleCheck => '\u{f08c5}',
        Nerd::AccountMultipleCheckOutline => '\u{f11fe}',
        Nerd::AccountMultipleMinus => '\u{f05d3}',
        Nerd::AccountMultipleMinusOutline => '\u{f0be5}',
        Nerd::AccountMultipleOutline => '\u{f000f}',
        Nerd::AccountMultiplePlus => '\u{f0010}',
        Nerd::AccountMultiplePlusOutline => '\u{f0800}',
        Nerd::AccountMultipleRemove => '\u{f120a}',
        Nerd::AccountMultipleRemoveOutline => '\u{f120b}',
        Nerd::AccountMusic => '\u{f0803}',
        Nerd::AccountMusicOutline => '\u{f0ce9}',
        Nerd::AccountNetwork => '\u{f0011}',
        Nerd::AccountNetworkOutline => '\u{f0be6}',
        Nerd::AccountOff => '\u{f0012}',
        Nerd::AccountOffOutline => '\u{f0be7}',
        Nerd::AccountOne => '\u{f0004}',
        Nerd::AccountOutline => '\u{f0013}',
        Nerd::AccountPlus => '\u{f0014}',
        Nerd::AccountPlusOutline => '\u{f0801}',
        Nerd::AccountQuestion => '\u{f0b59}',
        Nerd::AccountQuestionOutline => '\u{f0b5a}',
        Nerd::AccountReactivate => '\u{f152b}',
        Nerd::AccountReactivateOutline => '\u{f152c}',
        Nerd::AccountRemove => '\u{f0015}',
        Nerd::AccountRemoveOutline => '\u{f0aed}',
        Nerd::AccountSchool => '\u{f1a20}',
        Nerd::AccountSchoolOutline => '\u{f1a21}',
        Nerd::AccountSearch => '\u{f0016}',
        Nerd::AccountSearchOutline => '\u{f0935}',
        Nerd::AccountSettings => '\u{f0630}',
        Nerd::AccountSettingsOutline => '\u{f10c9}',
        Nerd::AccountStar => '\u{f0017}',
        Nerd::AccountStarOutline => '\u{f0be8}',
        Nerd::AccountSupervisor => '\u{f0a8b}',
        Nerd::AccountSupervisorCircle => '\u{f0a8c}',
        Nerd::AccountSupervisorCircleOutline => '\u{f14ec}',
        Nerd::AccountSupervisorOutline => '\u{f112d}',
        Nerd::AccountSwitch => '\u{f0019}',
        Nerd::AccountSwitchOutline => '\u{f04cb}',
        Nerd::AccountSync => '\u{f191b}',
        Nerd::AccountSyncOutline => '\u{f191c}',
        Nerd::AccountTie => '\u{f0ce3}',
        Nerd::AccountTieHat => '\u{f1898}',
        Nerd::AccountTieHatOutline => '\u{f1899}',
        Nerd::AccountTieOutline => '\u{f10ca}',
        Nerd::AccountTieVoice => '\u{f1308}',
        Nerd::AccountTieVoiceOff => '\u{f130a}',
        Nerd::AccountTieVoiceOffOutline => '\u{f130b}',
        Nerd::AccountTieVoiceOutline => '\u{f1309}',
        Nerd::AccountTieWoman => '\u{f1a8c}',
        Nerd::AccountVoice => '\u{f05cb}',
        Nerd::AccountVoiceOff => '\u{f0ed4}',
        Nerd::AccountWrench => '\u{f189a}',
        Nerd::AccountWrenchOutline => '\u{f189b}',
        Nerd::ActivateBreakpoints => '\u{ea97}',
        Nerd::Add => '\u{ea60}',
        Nerd::Adjust => '\u{f042}',
        Nerd::AdjustOne => '\u{f001a}',
        Nerd::Adn => '\u{f170}',
        Nerd::Advertisements => '\u{f192a}',
        Nerd::AdvertisementsOff => '\u{f192b}',
        Nerd::AirConditioner => '\u{f001b}',
        Nerd::AirFilter => '\u{f0d43}',
        Nerd::AirHorn => '\u{f0dac}',
        Nerd::AirHumidifier => '\u{f1099}',
        Nerd::AirHumidifierOff => '\u{f1466}',
        Nerd::AirPurifier => '\u{f0d44}',
        Nerd::Airbag => '\u{f0be9}',
        Nerd::Airballoon => '\u{f001c}',
        Nerd::AirballoonOutline => '\u{f100b}',
        Nerd::Airplane => '\u{f001d}',
        Nerd::AirplaneAlert => '\u{f187a}',
        Nerd::AirplaneCheck => '\u{f187b}',
        Nerd::AirplaneClock => '\u{f187c}',
        Nerd::AirplaneCog => '\u{f187d}',
        Nerd::AirplaneEdit => '\u{f187e}',
        Nerd::AirplaneLanding => '\u{f05d4}',
        Nerd::AirplaneMarker => '\u{f187f}',
        Nerd::AirplaneMinus => '\u{f1880}',
        Nerd::AirplaneOff => '\u{f001e}',
        Nerd::AirplanePlus => '\u{f1881}',
        Nerd::AirplaneRemove => '\u{f1882}',
        Nerd::AirplaneSearch => '\u{f1883}',
        Nerd::AirplaneSettings => '\u{f1884}',
        Nerd::AirplaneTakeoff => '\u{f05d5}',
        Nerd::Airport => '\u{f084b}',
        Nerd::Alarm => '\u{f0020}',
        Nerd::AlarmBell => '\u{f078e}',
        Nerd::AlarmCheck => '\u{f0021}',
        Nerd::AlarmLight => '\u{f078f}',
        Nerd::AlarmLightOff => '\u{f171e}',
        Nerd::AlarmLightOffOutline => '\u{f171f}',
        Nerd::AlarmLightOutline => '\u{f0bea}',
        Nerd::AlarmMultiple => '\u{f0022}',
        Nerd::AlarmNote => '\u{f0e71}',
        Nerd::AlarmNoteOff => '\u{f0e72}',
        Nerd::AlarmOff => '\u{f0023}',
        Nerd::AlarmPanel => '\u{f15c4}',
        Nerd::AlarmPanelOutline => '\u{f15c5}',
        Nerd::AlarmPlus => '\u{f0024}',
        Nerd::AlarmSnooze => '\u{f068e}',
        Nerd::Album => '\u{f0025}',
        Nerd::Alert => '\u{f421}',
        Nerd::AlertBox => '\u{f0027}',
        Nerd::AlertBoxOutline => '\u{f0ce4}',
        Nerd::AlertCircle => '\u{f0028}',
        Nerd::AlertCircleCheck => '\u{f11ed}',
        Nerd::AlertCircleCheckOutline => '\u{f11ee}',
        Nerd::AlertCircleOutline => '\u{f05d6}',
        Nerd::AlertDecagram => '\u{f06bd}',
        Nerd::AlertDecagramOutline => '\u{f0ce5}',
        Nerd::AlertFill => '\u{f40c}',
        Nerd::AlertMinus => '\u{f14bb}',
        Nerd::AlertMinusOutline => '\u{f14be}',
        Nerd::AlertOctagon => '\u{f0029}',
        Nerd::AlertOctagonOutline => '\u{f0ce6}',
        Nerd::AlertOctagram => '\u{f0767}',
        Nerd::AlertOctagramOutline => '\u{f0ce7}',
        Nerd::AlertOne => '\u{f0026}',
        Nerd::AlertOutline => '\u{f002a}',
        Nerd::AlertPlus => '\u{f14ba}',
        Nerd::AlertPlusOutline => '\u{f14bd}',
        Nerd::AlertRemove => '\u{f14bc}',
        Nerd::AlertRemoveOutline => '\u{f14bf}',
        Nerd::AlertRhombus => '\u{f11ce}',
        Nerd::AlertRhombusOutline => '\u{f11cf}',
        Nerd::Alien => '\u{f089a}',
        Nerd::AlienOutline => '\u{f10cb}',
        Nerd::AlignCenter => '\u{f037}',
        Nerd::AlignHorizontalCenter => '\u{f11c3}',
        Nerd::AlignHorizontalDistribute => '\u{f1962}',
        Nerd::AlignHorizontalLeft => '\u{f11c2}',
        Nerd::AlignHorizontalRight => '\u{f11c4}',
        Nerd::AlignJustify => '\u{f039}',
        Nerd::AlignLeft => '\u{f036}',
        Nerd::AlignRight => '\u{f038}',
        Nerd::AlignVerticalBottom => '\u{f11c5}',
        Nerd::AlignVerticalCenter => '\u{f11c6}',
        Nerd::AlignVerticalDistribute => '\u{f1963}',
        Nerd::AlignVerticalTop => '\u{f11c7}',
        Nerd::AllInclusive => '\u{f06be}',
        Nerd::AllInclusiveBox => '\u{f188d}',
        Nerd::AllInclusiveBoxOutline => '\u{f188e}',
        Nerd::Allergy => '\u{f1258}',
        Nerd::AlmaLinux => '\u{f31d}',
        Nerd::Alpha => '\u{f002b}',
        Nerd::AlphaA => '\u{f0aee}',
        Nerd::AlphaABox => '\u{f0b08}',
        Nerd::AlphaABoxOutline => '\u{f0beb}',
        Nerd::AlphaACircle => '\u{f0bec}',
        Nerd::AlphaACircleOutline => '\u{f0bed}',
        Nerd::AlphaB => '\u{f0aef}',
        Nerd::AlphaBBox => '\u{f0b09}',
        Nerd::AlphaBBoxOutline => '\u{f0bee}',
        Nerd::AlphaBCircle => '\u{f0bef}',
        Nerd::AlphaBCircleOutline => '\u{f0bf0}',
        Nerd::AlphaC => '\u{f0af0}',
        Nerd::AlphaCBox => '\u{f0b0a}',
        Nerd::AlphaCBoxOutline => '\u{f0bf1}',
        Nerd::AlphaCCircle => '\u{f0bf2}',
        Nerd::AlphaCCircleOutline => '\u{f0bf3}',
        Nerd::AlphaD => '\u{f0af1}',
        Nerd::AlphaDBox => '\u{f0b0b}',
        Nerd::AlphaDBoxOutline => '\u{f0bf4}',
        Nerd::AlphaDCircle => '\u{f0bf5}',
        Nerd::AlphaDCircleOutline => '\u{f0bf6}',
        Nerd::AlphaE => '\u{f0af2}',
        Nerd::AlphaEBox => '\u{f0b0c}',
        Nerd::AlphaEBoxOutline => '\u{f0bf7}',
        Nerd::AlphaECircle => '\u{f0bf8}',
        Nerd::AlphaECircleOutline => '\u{f0bf9}',
        Nerd::AlphaF => '\u{f0af3}',
        Nerd::AlphaFBox => '\u{f0b0d}',
        Nerd::AlphaFBoxOutline => '\u{f0bfa}',
        Nerd::AlphaFCircle => '\u{f0bfb}',
        Nerd::AlphaFCircleOutline => '\u{f0bfc}',
        Nerd::AlphaG => '\u{f0af4}',
        Nerd::AlphaGBox => '\u{f0b0e}',
        Nerd::AlphaGBoxOutline => '\u{f0bfd}',
        Nerd::AlphaGCircle => '\u{f0bfe}',
        Nerd::AlphaGCircleOutline => '\u{f0bff}',
        Nerd::AlphaH => '\u{f0af5}',
        Nerd::AlphaHBox => '\u{f0b0f}',
        Nerd::AlphaHBoxOutline => '\u{f0c00}',
        Nerd::AlphaHCircle => '\u{f0c01}',
        Nerd::AlphaHCircleOutline => '\u{f0c02}',
        Nerd::AlphaI => '\u{f0af6}',
        Nerd::AlphaIBox => '\u{f0b10}',
        Nerd::AlphaIBoxOutline => '\u{f0c03}',
        Nerd::AlphaICircle => '\u{f0c04}',
        Nerd::AlphaICircleOutline => '\u{f0c05}',
        Nerd::AlphaIOne => '\u{f1088}',
        Nerd::AlphaJ => '\u{f0af7}',
        Nerd::AlphaJBox => '\u{f0b11}',
        Nerd::AlphaJBoxOutline => '\u{f0c06}',
        Nerd::AlphaJCircle => '\u{f0c07}',
        Nerd::AlphaJCircleOutline => '\u{f0c08}',
        Nerd::AlphaK => '\u{f0af8}',
        Nerd::AlphaKBox => '\u{f0b12}',
        Nerd::AlphaKBoxOutline => '\u{f0c09}',
        Nerd::AlphaKCircle => '\u{f0c0a}',
        Nerd::AlphaKCircleOutline => '\u{f0c0b}',
        Nerd::AlphaL => '\u{f0af9}',
        Nerd::AlphaLBox => '\u{f0b13}',
        Nerd::AlphaLBoxOutline => '\u{f0c0c}',
        Nerd::AlphaLCircle => '\u{f0c0d}',
        Nerd::AlphaLCircleOutline => '\u{f0c0e}',
        Nerd::AlphaLOne => '\u{f13a6}',
        Nerd::AlphaM => '\u{f0afa}',
        Nerd::AlphaMBox => '\u{f0b14}',
        Nerd::AlphaMBoxOutline => '\u{f0c0f}',
        Nerd::AlphaMCircle => '\u{f0c10}',
        Nerd::AlphaMCircleOutline => '\u{f0c11}',
        Nerd::AlphaN => '\u{f0afb}',
        Nerd::AlphaNBox => '\u{f0b15}',
        Nerd::AlphaNBoxOutline => '\u{f0c12}',
        Nerd::AlphaNCircle => '\u{f0c13}',
        Nerd::AlphaNCircleOutline => '\u{f0c14}',
        Nerd::AlphaO => '\u{f0afc}',
        Nerd::AlphaOBox => '\u{f0b16}',
        Nerd::AlphaOBoxOutline => '\u{f0c15}',
        Nerd::AlphaOCircle => '\u{f0c16}',
        Nerd::AlphaOCircleOne => '\u{f0c9e}',
        Nerd::AlphaOCircleOutline => '\u{f0c17}',
        Nerd::AlphaOCircleOutlineOne => '\u{f0c9f}',
        Nerd::AlphaOOne => '\u{f0b39}',
        Nerd::AlphaP => '\u{f0afd}',
        Nerd::AlphaPBox => '\u{f0b17}',
        Nerd::AlphaPBoxOutline => '\u{f0c18}',
        Nerd::AlphaPCircle => '\u{f0c19}',
        Nerd::AlphaPCircleOutline => '\u{f0c1a}',
        Nerd::AlphaQ => '\u{f0afe}',
        Nerd::AlphaQBox => '\u{f0b18}',
        Nerd::AlphaQBoxOutline => '\u{f0c1b}',
        Nerd::AlphaQCircle => '\u{f0c1c}',
        Nerd::AlphaQCircleOutline => '\u{f0c1d}',
        Nerd::AlphaR => '\u{f0aff}',
        Nerd::AlphaRBox => '\u{f0b19}',
        Nerd::AlphaRBoxOutline => '\u{f0c1e}',
        Nerd::AlphaRCircle => '\u{f0c1f}',
        Nerd::AlphaRCircleOutline => '\u{f0c20}',
        Nerd::AlphaS => '\u{f0b00}',
        Nerd::AlphaSBox => '\u{f0b1a}',
        Nerd::AlphaSBoxOutline => '\u{f0c21}',
        Nerd::AlphaSCircle => '\u{f0c22}',
        Nerd::AlphaSCircleOutline => '\u{f0c23}',
        Nerd::AlphaT => '\u{f0b01}',
        Nerd::AlphaTBox => '\u{f0b1b}',
        Nerd::AlphaTBoxOutline => '\u{f0c24}',
        Nerd::AlphaTCircle => '\u{f0c25}',
        Nerd::AlphaTCircleOutline => '\u{f0c26}',
        Nerd::AlphaU => '\u{f0b02}',
        Nerd::AlphaUBox => '\u{f0b1c}',
        Nerd::AlphaUBoxOutline => '\u{f0c27}',
        Nerd::AlphaUCircle => '\u{f0c28}',
        Nerd::AlphaUCircleOutline => '\u{f0c29}',
        Nerd::AlphaV => '\u{f0b03}',
        Nerd::AlphaVBox => '\u{f0b1d}',
        Nerd::AlphaVBoxOutline => '\u{f0c2a}',
        Nerd::AlphaVCircle => '\u{f0c2b}',
        Nerd::AlphaVCircleOutline => '\u{f0c2c}',
        Nerd::AlphaVOne => '\u{f108c}',
        Nerd::AlphaW => '\u{f0b04}',
        Nerd::AlphaWBox => '\u{f0b1e}',
        Nerd::AlphaWBoxOutline => '\u{f0c2d}',
        Nerd::AlphaWCircle => '\u{f0c2e}',
        Nerd::AlphaWCircleOutline => '\u{f0c2f}',
        Nerd::AlphaX => '\u{f0b05}',
        Nerd::AlphaXBox => '\u{f0b1f}',
        Nerd::AlphaXBoxOutline => '\u{f0c30}',
        Nerd::AlphaXCircle => '\u{f0c31}',
        Nerd::AlphaXCircleOutline => '\u{f0c32}',
        Nerd::AlphaXOne => '\u{f1091}',
        Nerd::AlphaY => '\u{f0b06}',
        Nerd::AlphaYBox => '\u{f0b20}',
        Nerd::AlphaYBoxOutline => '\u{f0c33}',
        Nerd::AlphaYCircle => '\u{f0c34}',
        Nerd::AlphaYCircleOutline => '\u{f0c35}',
        Nerd::AlphaZ => '\u{f0b07}',
        Nerd::AlphaZBox => '\u{f0b21}',
        Nerd::AlphaZBoxOutline => '\u{f0c36}',
        Nerd::AlphaZCircle => '\u{f0c37}',
        Nerd::AlphaZCircleOutline => '\u{f0c38}',
        Nerd::AlphabetAurebesh => '\u{f132c}',
        Nerd::AlphabetCyrillic => '\u{f132d}',
        Nerd::AlphabetGreek => '\u{f132e}',
        Nerd::AlphabetLatin => '\u{f132f}',
        Nerd::AlphabetPiqad => '\u{f1330}',
        Nerd::AlphabetTengwar => '\u{f1337}',
        Nerd::Alphabetical => '\u{f002c}',
        Nerd::AlphabeticalOff => '\u{f100c}',
        Nerd::AlphabeticalVariant => '\u{f100d}',
        Nerd::AlphabeticalVariantOff => '\u{f100e}',
        Nerd::Alpine => '\u{f300}',
        Nerd::Altimeter => '\u{f05d7}',
        Nerd::Ambulance => '\u{f0f9}',
        Nerd::AmbulanceOne => '\u{f002f}',
        Nerd::Ammunition => '\u{f0ce8}',
        Nerd::Ampersand => '\u{f0a8d}',
        Nerd::Amplifier => '\u{f0030}',
        Nerd::AmplifierOff => '\u{f11b5}',
        Nerd::Anchor => '\u{f13d}',
        Nerd::AnchorOne => '\u{f0031}',
        Nerd::Android => '\u{f17b}',
        Nerd::AndroidMessages => '\u{f0d45}',
        Nerd::AndroidOne => '\u{f0032}',
        Nerd::AndroidStudio => '\u{f0034}',
        Nerd::AngleAcute => '\u{f0937}',
        Nerd::AngleDown => '\u{f107}',
        Nerd::AngleLeft => '\u{f104}',
        Nerd::AngleObtuse => '\u{f0938}',
        Nerd::AngleRight => '\u{f0939}',
        Nerd::AngleUp => '\u{f106}',
        Nerd::Angular => '\u{f06b2}',
        Nerd::Angularjs => '\u{f06bf}',
        Nerd::Animation => '\u{f05d8}',
        Nerd::AnimationOutline => '\u{f0a8f}',
        Nerd::AnimationPlay => '\u{f093a}',
        Nerd::AnimationPlayOutline => '\u{f0a90}',
        Nerd::Ansible => '\u{f109a}',
        Nerd::Antenna => '\u{f1119}',
        Nerd::Anvil => '\u{f089b}',
        Nerd::AoscOs => '\u{f301}',
        Nerd::ApacheKafka => '\u{f100f}',
        Nerd::Api => '\u{f109b}',
        Nerd::ApiOff => '\u{f1257}',
        Nerd::Apple => '\u{f302}',
        Nerd::AppleFinder => '\u{f0036}',
        Nerd::AppleFruit => '\u{e29e}',
        Nerd::AppleIcloud => '\u{f0038}',
        Nerd::AppleIos => '\u{f0037}',
        Nerd::AppleKeyboardCaps => '\u{f0632}',
        Nerd::AppleKeyboardCommand => '\u{f0633}',
        Nerd::AppleKeyboardControl => '\u{f0634}',
        Nerd::AppleKeyboardOption => '\u{f0635}',
        Nerd::AppleKeyboardShift => '\u{f0636}',
        Nerd::AppleOne => '\u{f0035}',
        Nerd::AppleSafari => '\u{f0039}',
        Nerd::Application => '\u{f08c6}',
        Nerd::ApplicationArray => '\u{f10f5}',
        Nerd::ApplicationArrayOutline => '\u{f10f6}',
        Nerd::ApplicationBraces => '\u{f10f7}',
        Nerd::ApplicationBracesOutline => '\u{f10f8}',
        Nerd::ApplicationBrackets => '\u{f0c8b}',
        Nerd::ApplicationBracketsOutline => '\u{f0c8c}',
        Nerd::ApplicationCog => '\u{f0675}',
        Nerd::ApplicationCogOutline => '\u{f1577}',
        Nerd::ApplicationEdit => '\u{f00ae}',
        Nerd::ApplicationEditOutline => '\u{f0619}',
        Nerd::ApplicationExport => '\u{f0dad}',
        Nerd::ApplicationImport => '\u{f0dae}',
        Nerd::ApplicationOutline => '\u{f0614}',
        Nerd::ApplicationParentheses => '\u{f10f9}',
        Nerd::ApplicationParenthesesOutline => '\u{f10fa}',
        Nerd::ApplicationSettings => '\u{f0b60}',
        Nerd::ApplicationSettingsOutline => '\u{f1555}',
        Nerd::ApplicationVariable => '\u{f10fb}',
        Nerd::ApplicationVariableOutline => '\u{f10fc}',
        Nerd::ApproximatelyEqual => '\u{f0f9e}',
        Nerd::ApproximatelyEqualBox => '\u{f0f9f}',
        Nerd::Apps => '\u{f40e}',
        Nerd::AppsBox => '\u{f0d46}',
        Nerd::AppsOne => '\u{f003b}',
        Nerd::Arch => '\u{f08c7}',
        Nerd::ArchLinux => '\u{f303}',
        Nerd::Archcraft => '\u{f345}',
        Nerd::Archive => '\u{ea98}',
        Nerd::ArchiveAlert => '\u{f14fd}',
        Nerd::ArchiveAlertOutline => '\u{f14fe}',
        Nerd::ArchiveArrowDown => '\u{f1259}',
        Nerd::ArchiveArrowDownOutline => '\u{f125a}',
        Nerd::ArchiveArrowUp => '\u{f125b}',
        Nerd::ArchiveArrowUpOutline => '\u{f125c}',
        Nerd::ArchiveCancel => '\u{f174b}',
        Nerd::ArchiveCancelOutline => '\u{f174c}',
        Nerd::ArchiveCheck => '\u{f174d}',
        Nerd::ArchiveCheckOutline => '\u{f174e}',
        Nerd::ArchiveClock => '\u{f174f}',
        Nerd::ArchiveClockOutline => '\u{f1750}',
        Nerd::ArchiveCog => '\u{f1751}',
        Nerd::ArchiveCogOutline => '\u{f1752}',
        Nerd::ArchiveEdit => '\u{f1753}',
        Nerd::ArchiveEditOutline => '\u{f1754}',
        Nerd::ArchiveEye => '\u{f1755}',
        Nerd::ArchiveEyeOutline => '\u{f1756}',
        Nerd::ArchiveLock => '\u{f1757}',
        Nerd::ArchiveLockOpen => '\u{f1758}',
        Nerd::ArchiveLockOpenOutline => '\u{f1759}',
        Nerd::ArchiveLockOutline => '\u{f175a}',
        Nerd::ArchiveMarker => '\u{f175b}',
        Nerd::ArchiveMarkerOutline => '\u{f175c}',
        Nerd::ArchiveMinus => '\u{f175d}',
        Nerd::ArchiveMinusOutline => '\u{f175e}',
        Nerd::ArchiveMusic => '\u{f175f}',
        Nerd::ArchiveMusicOutline => '\u{f1760}',
        Nerd::ArchiveOff => '\u{f1761}',
        Nerd::ArchiveOffOutline => '\u{f1762}',
        Nerd::ArchiveOne => '\u{f187}',
        Nerd::ArchiveOutline => '\u{f120e}',
        Nerd::ArchivePlus => '\u{f1763}',
        Nerd::ArchivePlusOutline => '\u{f1764}',
        Nerd::ArchiveRefresh => '\u{f1765}',
        Nerd::ArchiveRefreshOutline => '\u{f1766}',
        Nerd::ArchiveRemove => '\u{f1767}',
        Nerd::ArchiveRemoveOutline => '\u{f1768}',
        Nerd::ArchiveSearch => '\u{f1769}',
        Nerd::ArchiveSearchOutline => '\u{f176a}',
        Nerd::ArchiveSettings => '\u{f176b}',
        Nerd::ArchiveSettingsOutline => '\u{f176c}',
        Nerd::ArchiveStar => '\u{f176d}',
        Nerd::ArchiveStarOutline => '\u{f176e}',
        Nerd::ArchiveSync => '\u{f176f}',
        Nerd::ArchiveSyncOutline => '\u{f1770}',
        Nerd::ArchiveThree => '\u{f003c}',
        Nerd::ArchiveTwo => '\u{f411}',
        Nerd::Archlabs => '\u{f31e}',
        Nerd::Arcolinux => '\u{f346}',
        Nerd::Arduino => '\u{f34b}',
        Nerd::ArmFlex => '\u{f0fd7}',
        Nerd::ArmFlexOutline => '\u{f0fd6}',
        Nerd::ArrangeBringForward => '\u{f003d}',
        Nerd::ArrangeBringToFront => '\u{f003e}',
        Nerd::ArrangeSendBackward => '\u{f003f}',
        Nerd::ArrangeSendToBack => '\u{f0040}',
        Nerd::ArrowAll => '\u{f0041}',
        Nerd::ArrowBoth => '\u{ea99}',
        Nerd::ArrowBothOne => '\u{f416}',
        Nerd::ArrowBottomLeft => '\u{f0042}',
        Nerd::ArrowBottomLeftBoldBox => '\u{f1964}',
        Nerd::ArrowBottomLeftBoldBoxOutline => '\u{f1965}',
        Nerd::ArrowBottomLeftBoldOutline => '\u{f09b7}',
        Nerd::ArrowBottomLeftThick => '\u{f09b8}',
        Nerd::ArrowBottomLeftThin => '\u{f19b6}',
        Nerd::ArrowBottomLeftThinCircleOutline => '\u{f1596}',
        Nerd::ArrowBottomRight => '\u{f0043}',
        Nerd::ArrowBottomRightBoldBox => '\u{f1966}',
        Nerd::ArrowBottomRightBoldBoxOutline => '\u{f1967}',
        Nerd::ArrowBottomRightBoldOutline => '\u{f09b9}',
        Nerd::ArrowBottomRightThick => '\u{f09ba}',
        Nerd::ArrowBottomRightThin => '\u{f19b7}',
        Nerd::ArrowBottomRightThinCircleOutline => '\u{f1595}',
        Nerd::ArrowCircleAltLeft => '\u{f190}',
        Nerd::ArrowCollapse => '\u{f0615}',
        Nerd::ArrowCollapseAll => '\u{f0044}',
        Nerd::ArrowCollapseDown => '\u{f0792}',
        Nerd::ArrowCollapseHorizontal => '\u{f084c}',
        Nerd::ArrowCollapseLeft => '\u{f0793}',
        Nerd::ArrowCollapseRight => '\u{f0794}',
        Nerd::ArrowCollapseUp => '\u{f0795}',
        Nerd::ArrowCollapseVertical => '\u{f084d}',
        Nerd::ArrowDecision => '\u{f09bb}',
        Nerd::ArrowDecisionAuto => '\u{f09bc}',
        Nerd::ArrowDecisionAutoOutline => '\u{f09bd}',
        Nerd::ArrowDecisionOutline => '\u{f09be}',
        Nerd::ArrowDown => '\u{f063}',
        Nerd::ArrowDownBold => '\u{f072e}',
        Nerd::ArrowDownBoldBox => '\u{f072f}',
        Nerd::ArrowDownBoldBoxOutline => '\u{f0730}',
        Nerd::ArrowDownBoldCircle => '\u{f0047}',
        Nerd::ArrowDownBoldCircleOutline => '\u{f0048}',
        Nerd::ArrowDownBoldHexagonOutline => '\u{f0049}',
        Nerd::ArrowDownBoldOutline => '\u{f09bf}',
        Nerd::ArrowDownBox => '\u{f06c0}',
        Nerd::ArrowDownCircle => '\u{f0cdb}',
        Nerd::ArrowDownCircleOutline => '\u{f0cdc}',
        Nerd::ArrowDownDropCircle => '\u{f004a}',
        Nerd::ArrowDownDropCircleOutline => '\u{f004b}',
        Nerd::ArrowDownLeft => '\u{f424}',
        Nerd::ArrowDownLeftBold => '\u{f17a2}',
        Nerd::ArrowDownLeftOne => '\u{f17a1}',
        Nerd::ArrowDownOne => '\u{f433}',
        Nerd::ArrowDownRight => '\u{f43e}',
        Nerd::ArrowDownRightBold => '\u{f17a4}',
        Nerd::ArrowDownRightOne => '\u{f17a3}',
        Nerd::ArrowDownThick => '\u{f0046}',
        Nerd::ArrowDownThin => '\u{f19b3}',
        Nerd::ArrowDownThinCircleOutline => '\u{f1599}',
        Nerd::ArrowDownTwo => '\u{f0045}',
        Nerd::ArrowExpand => '\u{f0616}',
        Nerd::ArrowExpandAll => '\u{f004c}',
        Nerd::ArrowExpandDown => '\u{f0796}',
        Nerd::ArrowExpandHorizontal => '\u{f084e}',
        Nerd::ArrowExpandLeft => '\u{f0797}',
        Nerd::ArrowExpandRight => '\u{f0798}',
        Nerd::ArrowExpandUp => '\u{f0799}',
        Nerd::ArrowExpandVertical => '\u{f084f}',
        Nerd::ArrowHorizontalLock => '\u{f115b}',
        Nerd::ArrowLeft => '\u{f060}',
        Nerd::ArrowLeftBold => '\u{f0731}',
        Nerd::ArrowLeftBoldBox => '\u{f0732}',
        Nerd::ArrowLeftBoldBoxOutline => '\u{f0733}',
        Nerd::ArrowLeftBoldCircle => '\u{f004f}',
        Nerd::ArrowLeftBoldCircleOutline => '\u{f0050}',
        Nerd::ArrowLeftBoldHexagonOutline => '\u{f0051}',
        Nerd::ArrowLeftBoldOutline => '\u{f09c0}',
        Nerd::ArrowLeftBottom => '\u{f17a5}',
        Nerd::ArrowLeftBottomBold => '\u{f17a6}',
        Nerd::ArrowLeftBox => '\u{f06c1}',
        Nerd::ArrowLeftCircle => '\u{f0cdd}',
        Nerd::ArrowLeftCircleOutline => '\u{f0cde}',
        Nerd::ArrowLeftDropCircle => '\u{f0052}',
        Nerd::ArrowLeftDropCircleOutline => '\u{f0053}',
        Nerd::ArrowLeftOne => '\u{f434}',
        Nerd::ArrowLeftRight => '\u{f0e73}',
        Nerd::ArrowLeftRightBold => '\u{f0e74}',
        Nerd::ArrowLeftRightBoldOutline => '\u{f09c1}',
        Nerd::ArrowLeftThick => '\u{f004e}',
        Nerd::ArrowLeftThin => '\u{f19b1}',
        Nerd::ArrowLeftThinCircleOutline => '\u{f159a}',
        Nerd::ArrowLeftTop => '\u{f17a7}',
        Nerd::ArrowLeftTopBold => '\u{f17a8}',
        Nerd::ArrowLeftTwo => '\u{f004d}',
        Nerd::ArrowProjectile => '\u{f1840}',
        Nerd::ArrowProjectileMultiple => '\u{f183f}',
        Nerd::ArrowRight => '\u{f061}',
        Nerd::ArrowRightBold => '\u{f0734}',
        Nerd::ArrowRightBoldBox => '\u{f0735}',
        Nerd::ArrowRightBoldBoxOutline => '\u{f0736}',
        Nerd::ArrowRightBoldCircle => '\u{f0056}',
        Nerd::ArrowRightBoldCircleOutline => '\u{f0057}',
        Nerd::ArrowRightBoldHexagonOutline => '\u{f0058}',
        Nerd::ArrowRightBoldOutline => '\u{f09c2}',
        Nerd::ArrowRightBottom => '\u{f17a9}',
        Nerd::ArrowRightBottomBold => '\u{f17aa}',
        Nerd::ArrowRightBox => '\u{f06c2}',
        Nerd::ArrowRightCircle => '\u{f0cdf}',
        Nerd::ArrowRightCircleOutline => '\u{f0ce0}',
        Nerd::ArrowRightDropCircle => '\u{f0059}',
        Nerd::ArrowRightDropCircleOutline => '\u{f005a}',
        Nerd::ArrowRightOne => '\u{f432}',
        Nerd::ArrowRightThick => '\u{f0055}',
        Nerd::ArrowRightThin => '\u{f19b0}',
        Nerd::ArrowRightThinCircleOutline => '\u{f1598}',
        Nerd::ArrowRightTop => '\u{f17ab}',
        Nerd::ArrowRightTopBold => '\u{f17ac}',
        Nerd::ArrowRightTwo => '\u{f0054}',
        Nerd::ArrowSmallDown => '\u{ea9d}',
        Nerd::ArrowSmallLeft => '\u{ea9e}',
        Nerd::ArrowSmallRight => '\u{ea9f}',
        Nerd::ArrowSmallUp => '\u{eaa0}',
        Nerd::ArrowSplitHorizontal => '\u{f093b}',
        Nerd::ArrowSplitVertical => '\u{f093c}',
        Nerd::ArrowSwap => '\u{ebcb}',
        Nerd::ArrowSwitch => '\u{f443}',
        Nerd::ArrowTopLeft => '\u{f005b}',
        Nerd::ArrowTopLeftBoldBox => '\u{f1968}',
        Nerd::ArrowTopLeftBoldBoxOutline => '\u{f1969}',
        Nerd::ArrowTopLeftBoldOutline => '\u{f09c3}',
        Nerd::ArrowTopLeftBottomRight => '\u{f0e75}',
        Nerd::ArrowTopLeftBottomRightBold => '\u{f0e76}',
        Nerd::ArrowTopLeftThick => '\u{f09c4}',
        Nerd::ArrowTopLeftThin => '\u{f19b5}',
        Nerd::ArrowTopLeftThinCircleOutline => '\u{f1593}',
        Nerd::ArrowTopRight => '\u{f005c}',
        Nerd::ArrowTopRightBoldBox => '\u{f196a}',
        Nerd::ArrowTopRightBoldBoxOutline => '\u{f196b}',
        Nerd::ArrowTopRightBoldOutline => '\u{f09c5}',
        Nerd::ArrowTopRightBottomLeft => '\u{f0e77}',
        Nerd::ArrowTopRightBottomLeftBold => '\u{f0e78}',
        Nerd::ArrowTopRightThick => '\u{f09c6}',
        Nerd::ArrowTopRightThin => '\u{f19b4}',
        Nerd::ArrowTopRightThinCircleOutline => '\u{f1594}',
        Nerd::ArrowUDownLeft => '\u{f17ad}',
        Nerd::ArrowUDownLeftBold => '\u{f17ae}',
        Nerd::ArrowUDownRight => '\u{f17af}',
        Nerd::ArrowUDownRightBold => '\u{f17b0}',
        Nerd::ArrowULeftBottom => '\u{f17b1}',
        Nerd::ArrowULeftBottomBold => '\u{f17b2}',
        Nerd::ArrowULeftTop => '\u{f17b3}',
        Nerd::ArrowULeftTopBold => '\u{f17b4}',
        Nerd::ArrowURightBottom => '\u{f17b5}',
        Nerd::ArrowURightBottomBold => '\u{f17b6}',
        Nerd::ArrowURightTop => '\u{f17b7}',
        Nerd::ArrowURightTopBold => '\u{f17b8}',
        Nerd::ArrowUUpLeft => '\u{f17b9}',
        Nerd::ArrowUUpLeftBold => '\u{f17ba}',
        Nerd::ArrowUUpRight => '\u{f17bb}',
        Nerd::ArrowUUpRightBold => '\u{f17bc}',
        Nerd::ArrowUp => '\u{f062}',
        Nerd::ArrowUpBold => '\u{f0737}',
        Nerd::ArrowUpBoldBox => '\u{f0738}',
        Nerd::ArrowUpBoldBoxOutline => '\u{f0739}',
        Nerd::ArrowUpBoldCircle => '\u{f005f}',
        Nerd::ArrowUpBoldCircleOutline => '\u{f0060}',
        Nerd::ArrowUpBoldHexagonOutline => '\u{f0061}',
        Nerd::ArrowUpBoldOutline => '\u{f09c7}',
        Nerd::ArrowUpBox => '\u{f06c3}',
        Nerd::ArrowUpCircle => '\u{f0ce1}',
        Nerd::ArrowUpCircleOutline => '\u{f0ce2}',
        Nerd::ArrowUpDown => '\u{f0e79}',
        Nerd::ArrowUpDownBold => '\u{f0e7a}',
        Nerd::ArrowUpDownBoldOutline => '\u{f09c8}',
        Nerd::ArrowUpDropCircle => '\u{f0062}',
        Nerd::ArrowUpDropCircleOutline => '\u{f0063}',
        Nerd::ArrowUpLeft => '\u{f45c}',
        Nerd::ArrowUpLeftBold => '\u{f17be}',
        Nerd::ArrowUpLeftOne => '\u{f17bd}',
        Nerd::ArrowUpOne => '\u{f431}',
        Nerd::ArrowUpRight => '\u{f46c}',
        Nerd::ArrowUpRightBold => '\u{f17c0}',
        Nerd::ArrowUpRightOne => '\u{f17bf}',
        Nerd::ArrowUpThick => '\u{f005e}',
        Nerd::ArrowUpThin => '\u{f19b2}',
        Nerd::ArrowUpThinCircleOutline => '\u{f1597}',
        Nerd::ArrowUpTwo => '\u{f005d}',
        Nerd::ArrowVerticalLock => '\u{f115c}',
        Nerd::ArtixLinux => '\u{f31f}',
        Nerd::Artstation => '\u{f0b5b}',
        Nerd::AspectRatio => '\u{f0a24}',
        Nerd::Assistant => '\u{f0064}',
        Nerd::Asterisk => '\u{f069}',
        Nerd::AsteriskCircleOutline => '\u{f1a27}',
        Nerd::AsteriskOne => '\u{f06c4}',
        Nerd::At => '\u{f0065}',
        Nerd::Atlassian => '\u{f0804}',
        Nerd::Atm => '\u{f0d47}',
        Nerd::Atom => '\u{e27f}',
        Nerd::AtomOne => '\u{f0768}',
        Nerd::AtomVariant => '\u{f0e7b}',
        Nerd::Attachment => '\u{f0066}',
        Nerd::AttachmentCheck => '\u{f1ac1}',
        Nerd::AttachmentLock => '\u{f19c4}',
        Nerd::AttachmentMinus => '\u{f1ac2}',
        Nerd::AttachmentOff => '\u{f1ac3}',
        Nerd::AttachmentPlus => '\u{f1ac4}',
        Nerd::AttachmentRemove => '\u{f1ac5}',
        Nerd::AudioInputRca => '\u{f186b}',
        Nerd::AudioInputStereoMinijack => '\u{f186c}',
        Nerd::AudioInputXlr => '\u{f186d}',
        Nerd::AudioVideo => '\u{f093d}',
        Nerd::AudioVideoOff => '\u{f11b6}',
        Nerd::AugmentedReality => '\u{f0850}',
        Nerd::AutoDownload => '\u{f137e}',
        Nerd::AutoFix => '\u{f0068}',
        Nerd::AutoUpload => '\u{f0069}',
        Nerd::Autorenew => '\u{f006a}',
        Nerd::AutorenewOff => '\u{f19e7}',
        Nerd::AvTimer => '\u{f006b}',
        Nerd::Away => '\u{e007}',
        Nerd::AwesomeWm => '\u{f354}',
        Nerd::Aws => '\u{f0e0f}',
        Nerd::Axe => '\u{f08c8}',
        Nerd::AxeBattle => '\u{f1842}',
        Nerd::Axis => '\u{f0d48}',
        Nerd::AxisArrow => '\u{f0d49}',
        Nerd::AxisArrowInfo => '\u{f140e}',
        Nerd::AxisArrowLock => '\u{f0d4a}',
        Nerd::AxisLock => '\u{f0d4b}',
        Nerd::AxisXArrow => '\u{f0d4c}',
        Nerd::AxisXArrowLock => '\u{f0d4d}',
        Nerd::AxisXRotateClockwise => '\u{f0d4e}',
        Nerd::AxisXRotateCounterclockwise => '\u{f0d4f}',
        Nerd::AxisXYArrowLock => '\u{f0d50}',
        Nerd::AxisYArrow => '\u{f0d51}',
        Nerd::AxisYArrowLock => '\u{f0d52}',
        Nerd::AxisYRotateClockwise => '\u{f0d53}',
        Nerd::AxisYRotateCounterclockwise => '\u{f0d54}',
        Nerd::AxisZArrow => '\u{f0d55}',
        Nerd::AxisZArrowLock => '\u{f0d56}',
        Nerd::AxisZRotateClockwise => '\u{f0d57}',
        Nerd::AxisZRotateCounterclockwise => '\u{f0d58}',
        Nerd::Azure => '\u{ebd8}',
        Nerd::AzureDevops => '\u{ebe8}',
        Nerd::Babel => '\u{f0a25}',
        Nerd::Baby => '\u{f006c}',
        Nerd::BabyBottle => '\u{f0f39}',
        Nerd::BabyBottleOutline => '\u{f0f3a}',
        Nerd::BabyBuggy => '\u{f13e0}',
        Nerd::BabyCarriage => '\u{f068f}',
        Nerd::BabyCarriageOff => '\u{f0fa0}',
        Nerd::BabyFace => '\u{f0e7c}',
        Nerd::BabyFaceOutline => '\u{f0e7d}',
        Nerd::Backburger => '\u{f006d}',
        Nerd::Backspace => '\u{f006e}',
        Nerd::BackspaceOutline => '\u{f0b5c}',
        Nerd::BackspaceReverse => '\u{f0e7e}',
        Nerd::BackspaceReverseOutline => '\u{f0e7f}',
        Nerd::BackupRestore => '\u{f006f}',
        Nerd::Backward => '\u{f04a}',
        Nerd::Bacteria => '\u{e280}',
        Nerd::BacteriaOne => '\u{f0ed5}',
        Nerd::BacteriaOutline => '\u{f0ed6}',
        Nerd::BadgeAccount => '\u{f0da7}',
        Nerd::BadgeAccountAlert => '\u{f0da8}',
        Nerd::BadgeAccountAlertOutline => '\u{f0da9}',
        Nerd::BadgeAccountHorizontal => '\u{f0e0d}',
        Nerd::BadgeAccountHorizontalOutline => '\u{f0e0e}',
        Nerd::BadgeAccountOutline => '\u{f0daa}',
        Nerd::Badminton => '\u{f0851}',
        Nerd::BagCarryOn => '\u{f0f3b}',
        Nerd::BagCarryOnCheck => '\u{f0d65}',
        Nerd::BagCarryOnOff => '\u{f0f3c}',
        Nerd::BagChecked => '\u{f0f3d}',
        Nerd::BagPersonal => '\u{f0e10}',
        Nerd::BagPersonalOff => '\u{f0e11}',
        Nerd::BagPersonalOffOutline => '\u{f0e12}',
        Nerd::BagPersonalOutline => '\u{f0e13}',
        Nerd::BagSuitcase => '\u{f158b}',
        Nerd::BagSuitcaseOff => '\u{f158d}',
        Nerd::BagSuitcaseOffOutline => '\u{f158e}',
        Nerd::BagSuitcaseOutline => '\u{f158c}',
        Nerd::Baguette => '\u{f0f3e}',
        Nerd::Balcony => '\u{f1817}',
        Nerd::Balloon => '\u{f0a26}',
        Nerd::Ballot => '\u{f09c9}',
        Nerd::BallotOutline => '\u{f09ca}',
        Nerd::BallotRecount => '\u{f0c39}',
        Nerd::BallotRecountOutline => '\u{f0c3a}',
        Nerd::BanCircle => '\u{f05e}',
        Nerd::Banana => '\u{e281}',
        Nerd::Bandage => '\u{f0daf}',
        Nerd::Bank => '\u{f0070}',
        Nerd::BankCheck => '\u{f1655}',
        Nerd::BankMinus => '\u{f0db0}',
        Nerd::BankOff => '\u{f1656}',
        Nerd::BankOffOutline => '\u{f1657}',
        Nerd::BankOutline => '\u{f0e80}',
        Nerd::BankPlus => '\u{f0db1}',
        Nerd::BankRemove => '\u{f0db2}',
        Nerd::BankTransfer => '\u{f0a27}',
        Nerd::BankTransferIn => '\u{f0a28}',
        Nerd::BankTransferOut => '\u{f0a29}',
        Nerd::BarChart => '\u{f080}',
        Nerd::Barcode => '\u{f02a}',
        Nerd::BarcodeOff => '\u{f1236}',
        Nerd::BarcodeOne => '\u{f0071}',
        Nerd::BarcodeScan => '\u{f0072}',
        Nerd::Barley => '\u{f0073}',
        Nerd::BarleyOff => '\u{f0b5d}',
        Nerd::Barn => '\u{f0b5e}',
        Nerd::Barrel => '\u{f0074}',
        Nerd::BarrelOutline => '\u{f1a28}',
        Nerd::Baseball => '\u{f0852}',
        Nerd::BaseballBat => '\u{f0853}',
        Nerd::BaseballDiamond => '\u{f15ec}',
        Nerd::BaseballDiamondOutline => '\u{f15ed}',
        Nerd::Bash => '\u{f1183}',
        Nerd::Basket => '\u{f0076}',
        Nerd::BasketCheck => '\u{f18e5}',
        Nerd::BasketCheckOutline => '\u{f18e6}',
        Nerd::BasketFill => '\u{f0077}',
        Nerd::BasketMinus => '\u{f1523}',
        Nerd::BasketMinusOutline => '\u{f1524}',
        Nerd::BasketOff => '\u{f1525}',
        Nerd::BasketOffOutline => '\u{f1526}',
        Nerd::BasketOutline => '\u{f1181}',
        Nerd::BasketPlus => '\u{f1527}',
        Nerd::BasketPlusOutline => '\u{f1528}',
        Nerd::BasketRemove => '\u{f1529}',
        Nerd::BasketRemoveOutline => '\u{f152a}',
        Nerd::BasketUnfill => '\u{f0078}',
        Nerd::Basketball => '\u{f0806}',
        Nerd::BasketballHoop => '\u{f0c3b}',
        Nerd::BasketballHoopOutline => '\u{f0c3c}',
        Nerd::Bat => '\u{f0b5f}',
        Nerd::Bath => '\u{e282}',
        Nerd::Bathtub => '\u{f1818}',
        Nerd::BathtubOutline => '\u{f1819}',
        Nerd::Battery => '\u{f0079}',
        Nerd::BatteryAlert => '\u{f0083}',
        Nerd::BatteryAlertBluetooth => '\u{f0947}',
        Nerd::BatteryAlertVariant => '\u{f10cc}',
        Nerd::BatteryAlertVariantOutline => '\u{f10cd}',
        Nerd::BatteryArrowDown => '\u{f17de}',
        Nerd::BatteryArrowDownOutline => '\u{f17df}',
        Nerd::BatteryArrowUp => '\u{f17e0}',
        Nerd::BatteryArrowUpOutline => '\u{f17e1}',
        Nerd::BatteryBluetooth => '\u{f0948}',
        Nerd::BatteryBluetoothVariant => '\u{f0949}',
        Nerd::BatteryCharging => '\u{f0084}',
        Nerd::BatteryChargingEightzero => '\u{f008a}',
        Nerd::BatteryChargingFivezero => '\u{f089d}',
        Nerd::BatteryChargingFourzero => '\u{f0088}',
        Nerd::BatteryChargingHigh => '\u{f12a6}',
        Nerd::BatteryChargingLow => '\u{f12a4}',
        Nerd::BatteryChargingMedium => '\u{f12a5}',
        Nerd::BatteryChargingNinezero => '\u{f008b}',
        Nerd::BatteryChargingOnezero => '\u{f089c}',
        Nerd::BatteryChargingOnezerozero => '\u{f0085}',
        Nerd::BatteryChargingOutline => '\u{f089f}',
        Nerd::BatteryChargingSevenzero => '\u{f089e}',
        Nerd::BatteryChargingSixzero => '\u{f0089}',
        Nerd::BatteryChargingThreezero => '\u{f0087}',
        Nerd::BatteryChargingTwozero => '\u{f0086}',
        Nerd::BatteryChargingWireless => '\u{f0807}',
        Nerd::BatteryChargingWirelessAlert => '\u{f0811}',
        Nerd::BatteryChargingWirelessEightzero => '\u{f080f}',
        Nerd::BatteryChargingWirelessFivezero => '\u{f080c}',
        Nerd::BatteryChargingWirelessFourzero => '\u{f080b}',
        Nerd::BatteryChargingWirelessNinezero => '\u{f0810}',
        Nerd::BatteryChargingWirelessOnezero => '\u{f0808}',
        Nerd::BatteryChargingWirelessOutline => '\u{f0812}',
        Nerd::BatteryChargingWirelessSevenzero => '\u{f080e}',
        Nerd::BatteryChargingWirelessSixzero => '\u{f080d}',
        Nerd::BatteryChargingWirelessThreezero => '\u{f080a}',
        Nerd::BatteryChargingWirelessTwozero => '\u{f0809}',
        Nerd::BatteryCheck => '\u{f17e2}',
        Nerd::BatteryCheckOutline => '\u{f17e3}',
        Nerd::BatteryClock => '\u{f19e5}',
        Nerd::BatteryClockOutline => '\u{f19e6}',
        Nerd::BatteryEightzero => '\u{f0081}',
        Nerd::BatteryEightzeroBluetooth => '\u{f0945}',
        Nerd::BatteryFivezero => '\u{f007e}',
        Nerd::BatteryFivezeroBluetooth => '\u{f0942}',
        Nerd::BatteryFourzero => '\u{f007d}',
        Nerd::BatteryFourzeroBluetooth => '\u{f0941}',
        Nerd::BatteryHeart => '\u{f120f}',
        Nerd::BatteryHeartOutline => '\u{f1210}',
        Nerd::BatteryHeartVariant => '\u{f1211}',
        Nerd::BatteryHigh => '\u{f12a3}',
        Nerd::BatteryLock => '\u{f179c}',
        Nerd::BatteryLockOpen => '\u{f179d}',
        Nerd::BatteryLow => '\u{f12a1}',
        Nerd::BatteryMedium => '\u{f12a2}',
        Nerd::BatteryMinus => '\u{f17e4}',
        Nerd::BatteryMinusOutline => '\u{f17e5}',
        Nerd::BatteryMinusVariant => '\u{f008c}',
        Nerd::BatteryNegative => '\u{f008d}',
        Nerd::BatteryNinezero => '\u{f0082}',
        Nerd::BatteryNinezeroBluetooth => '\u{f0946}',
        Nerd::BatteryOff => '\u{f125d}',
        Nerd::BatteryOffOutline => '\u{f125e}',
        Nerd::BatteryOnezero => '\u{f007a}',
        Nerd::BatteryOnezeroBluetooth => '\u{f093e}',
        Nerd::BatteryOutline => '\u{f008e}',
        Nerd::BatteryPlus => '\u{f17e6}',
        Nerd::BatteryPlusOutline => '\u{f17e7}',
        Nerd::BatteryPlusVariant => '\u{f008f}',
        Nerd::BatteryPositive => '\u{f0090}',
        Nerd::BatteryRemove => '\u{f17e8}',
        Nerd::BatteryRemoveOutline => '\u{f17e9}',
        Nerd::BatterySevenzero => '\u{f0080}',
        Nerd::BatterySevenzeroBluetooth => '\u{f0944}',
        Nerd::BatterySixzero => '\u{f007f}',
        Nerd::BatterySixzeroBluetooth => '\u{f0943}',
        Nerd::BatterySync => '\u{f1834}',
        Nerd::BatterySyncOutline => '\u{f1835}',
        Nerd::BatteryThreezero => '\u{f007c}',
        Nerd::BatteryThreezeroBluetooth => '\u{f0940}',
        Nerd::BatteryTwozero => '\u{f007b}',
        Nerd::BatteryTwozeroBluetooth => '\u{f093f}',
        Nerd::BatteryUnknown => '\u{f0091}',
        Nerd::BatteryUnknownBluetooth => '\u{f094a}',
        Nerd::Beach => '\u{f0092}',
        Nerd::Beaker => '\u{ea79}',
        Nerd::BeakerAlert => '\u{f1229}',
        Nerd::BeakerAlertOutline => '\u{f122a}',
        Nerd::BeakerCheck => '\u{f122b}',
        Nerd::BeakerCheckOutline => '\u{f122c}',
        Nerd::BeakerMinus => '\u{f122d}',
        Nerd::BeakerMinusOutline => '\u{f122e}',
        Nerd::BeakerOne => '\u{f0c3}',
        Nerd::BeakerOutline => '\u{f0690}',
        Nerd::BeakerPlus => '\u{f122f}',
        Nerd::BeakerPlusOutline => '\u{f1230}',
        Nerd::BeakerQuestion => '\u{f1231}',
        Nerd::BeakerQuestionOutline => '\u{f1232}',
        Nerd::BeakerRemove => '\u{f1233}',
        Nerd::BeakerRemoveOutline => '\u{f1234}',
        Nerd::BeakerStop => '\u{ebe1}',
        Nerd::BeakerThree => '\u{f0cea}',
        Nerd::BeakerTwo => '\u{f499}',
        Nerd::Bed => '\u{e283}',
        Nerd::BedDouble => '\u{f0fd4}',
        Nerd::BedDoubleOutline => '\u{f0fd3}',
        Nerd::BedEmpty => '\u{f08a0}',
        Nerd::BedKing => '\u{f0fd2}',
        Nerd::BedKingOutline => '\u{f0fd1}',
        Nerd::BedOne => '\u{f02e3}',
        Nerd::BedOutline => '\u{f0099}',
        Nerd::BedQueen => '\u{f0fd0}',
        Nerd::BedQueenOutline => '\u{f0fdb}',
        Nerd::BedSingle => '\u{f106d}',
        Nerd::BedSingleOutline => '\u{f106e}',
        Nerd::Bee => '\u{f0fa1}',
        Nerd::BeeFlower => '\u{f0fa2}',
        Nerd::BeehiveOffOutline => '\u{f13ed}',
        Nerd::BeehiveOutline => '\u{f10ce}',
        Nerd::Beekeeper => '\u{f14e2}',
        Nerd::Beer => '\u{f0fc}',
        Nerd::BeerOne => '\u{f0098}',
        Nerd::BeerOutline => '\u{f130c}',
        Nerd::Bell => '\u{eaa2}',
        Nerd::BellAlert => '\u{f0d59}',
        Nerd::BellAlertOutline => '\u{f0e81}',
        Nerd::BellAlt => '\u{f0f3}',
        Nerd::BellBadge => '\u{f116b}',
        Nerd::BellBadgeOutline => '\u{f0178}',
        Nerd::BellCancel => '\u{f13e7}',
        Nerd::BellCancelOutline => '\u{f13e8}',
        Nerd::BellCheck => '\u{f11e5}',
        Nerd::BellCheckOutline => '\u{f11e6}',
        Nerd::BellCircle => '\u{f0d5a}',
        Nerd::BellCircleOutline => '\u{f0d5b}',
        Nerd::BellCog => '\u{f1a29}',
        Nerd::BellCogOutline => '\u{f1a2a}',
        Nerd::BellDot => '\u{eb9a}',
        Nerd::BellFill => '\u{f476}',
        Nerd::BellMinus => '\u{f13e9}',
        Nerd::BellMinusOutline => '\u{f13ea}',
        Nerd::BellOff => '\u{f009b}',
        Nerd::BellOffOutline => '\u{f0a91}',
        Nerd::BellOne => '\u{f0a2}',
        Nerd::BellOutline => '\u{f009c}',
        Nerd::BellPlus => '\u{f009d}',
        Nerd::BellPlusOutline => '\u{f0a92}',
        Nerd::BellRemove => '\u{f13eb}',
        Nerd::BellRemoveOutline => '\u{f13ec}',
        Nerd::BellRing => '\u{f009e}',
        Nerd::BellRingOutline => '\u{f009f}',
        Nerd::BellSlash => '\u{f478}',
        Nerd::BellSleep => '\u{f00a0}',
        Nerd::BellSleepOutline => '\u{f0a93}',
        Nerd::BellThree => '\u{f009a}',
        Nerd::BellTwo => '\u{f49a}',
        Nerd::Benzene => '\u{e284}',
        Nerd::Beta => '\u{f00a1}',
        Nerd::Betamax => '\u{f09cb}',
        Nerd::Biathlon => '\u{f0e14}',
        Nerd::Bicycle => '\u{f109c}',
        Nerd::BicycleBasket => '\u{f1235}',
        Nerd::BicycleCargo => '\u{f189c}',
        Nerd::BicycleElectric => '\u{f15b4}',
        Nerd::BicyclePennyFarthing => '\u{f15e9}',
        Nerd::Bigger => '\u{e285}',
        Nerd::Biglinux => '\u{f347}',
        Nerd::Bike => '\u{f00a3}',
        Nerd::BikeFast => '\u{f111f}',
        Nerd::Billboard => '\u{f1010}',
        Nerd::Billiards => '\u{f0b61}',
        Nerd::BilliardsRack => '\u{f0b62}',
        Nerd::Binoculars => '\u{f00a5}',
        Nerd::Bio => '\u{f00a6}',
        Nerd::Biohazard => '\u{e286}',
        Nerd::BiohazardOne => '\u{f00a7}',
        Nerd::Bird => '\u{f15c6}',
        Nerd::Bitbucket => '\u{f00a8}',
        Nerd::BitbucketSign => '\u{f172}',
        Nerd::Bitcoin => '\u{f0813}',
        Nerd::BlackMesa => '\u{f00a9}',
        Nerd::Blender => '\u{f0ceb}',
        Nerd::BlenderOutline => '\u{f181a}',
        Nerd::BlenderSoftware => '\u{f00ab}',
        Nerd::Blinds => '\u{f00ac}',
        Nerd::BlindsHorizontal => '\u{f1a2b}',
        Nerd::BlindsHorizontalClosed => '\u{f1a2c}',
        Nerd::BlindsOpen => '\u{f1011}',
        Nerd::BlindsVertical => '\u{f1a2d}',
        Nerd::BlindsVerticalClosed => '\u{f1a2e}',
        Nerd::BlockHelper => '\u{f00ad}',
        Nerd::Blocked => '\u{f479}',
        Nerd::BloggerCircle => '\u{e287}',
        Nerd::BloggerSquare => '\u{e288}',
        Nerd::BloodBag => '\u{f0cec}',
        Nerd::Bluetooth => '\u{f00af}',
        Nerd::BluetoothAudio => '\u{f00b0}',
        Nerd::BluetoothConnect => '\u{f00b1}',
        Nerd::BluetoothOff => '\u{f00b2}',
        Nerd::BluetoothSettings => '\u{f00b3}',
        Nerd::BluetoothTransfer => '\u{f00b4}',
        Nerd::Blur => '\u{f00b5}',
        Nerd::BlurLinear => '\u{f00b6}',
        Nerd::BlurOff => '\u{f00b7}',
        Nerd::BlurRadial => '\u{f00b8}',
        Nerd::Bold => '\u{eaa3}',
        Nerd::BoldOne => '\u{f032}',
        Nerd::BoldTwo => '\u{f49d}',
        Nerd::Bolt => '\u{f0e7}',
        Nerd::BoltOne => '\u{f0db3}',
        Nerd::Bomb => '\u{f0691}',
        Nerd::BombOff => '\u{f06c5}',
        Nerd::Bone => '\u{f00b9}',
        Nerd::BoneOff => '\u{f19e0}',
        Nerd::Bones => '\u{e289}',
        Nerd::Book => '\u{eaa4}',
        Nerd::BookAccount => '\u{f13ad}',
        Nerd::BookAccountOutline => '\u{f13ae}',
        Nerd::BookAlert => '\u{f167c}',
        Nerd::BookAlertOutline => '\u{f167d}',
        Nerd::BookAlphabet => '\u{f061d}',
        Nerd::BookArrowDown => '\u{f167e}',
        Nerd::BookArrowDownOutline => '\u{f167f}',
        Nerd::BookArrowLeft => '\u{f1680}',
        Nerd::BookArrowLeftOutline => '\u{f1681}',
        Nerd::BookArrowRight => '\u{f1682}',
        Nerd::BookArrowRightOutline => '\u{f1683}',
        Nerd::BookArrowUp => '\u{f1684}',
        Nerd::BookArrowUpOutline => '\u{f1685}',
        Nerd::BookCancel => '\u{f1686}',
        Nerd::BookCancelOutline => '\u{f1687}',
        Nerd::BookCheck => '\u{f14f3}',
        Nerd::BookCheckOutline => '\u{f14f4}',
        Nerd::BookClock => '\u{f1688}',
        Nerd::BookClockOutline => '\u{f1689}',
        Nerd::BookCog => '\u{f168a}',
        Nerd::BookCogOutline => '\u{f168b}',
        Nerd::BookCross => '\u{f00a2}',
        Nerd::BookEdit => '\u{f168c}',
        Nerd::BookEditOutline => '\u{f168d}',
        Nerd::BookEducation => '\u{f16c9}',
        Nerd::BookEducationOutline => '\u{f16ca}',
        Nerd::BookHeart => '\u{f1a1d}',
        Nerd::BookHeartOutline => '\u{f1a1e}',
        Nerd::BookInformationVariant => '\u{f106f}',
        Nerd::BookLock => '\u{f079a}',
        Nerd::BookLockOpen => '\u{f079b}',
        Nerd::BookLockOpenOutline => '\u{f168e}',
        Nerd::BookLockOutline => '\u{f168f}',
        Nerd::BookMarker => '\u{f1690}',
        Nerd::BookMarkerOutline => '\u{f1691}',
        Nerd::BookMinus => '\u{f05d9}',
        Nerd::BookMinusMultiple => '\u{f0a94}',
        Nerd::BookMinusMultipleOutline => '\u{f090b}',
        Nerd::BookMinusOutline => '\u{f1692}',
        Nerd::BookMultiple => '\u{f00bb}',
        Nerd::BookMultipleOutline => '\u{f0436}',
        Nerd::BookMusic => '\u{f0067}',
        Nerd::BookMusicOutline => '\u{f1693}',
        Nerd::BookOff => '\u{f1694}',
        Nerd::BookOffOutline => '\u{f1695}',
        Nerd::BookOne => '\u{f02d}',
        Nerd::BookOpen => '\u{e28a}',
        Nerd::BookOpenBlankVariant => '\u{f00be}',
        Nerd::BookOpenO => '\u{e28b}',
        Nerd::BookOpenOne => '\u{f00bd}',
        Nerd::BookOpenOutline => '\u{f0b63}',
        Nerd::BookOpenPageVariant => '\u{f05da}',
        Nerd::BookOpenPageVariantOutline => '\u{f15d6}',
        Nerd::BookOpenVariant => '\u{f14f7}',
        Nerd::BookOutline => '\u{f0b64}',
        Nerd::BookPlay => '\u{f0e82}',
        Nerd::BookPlayOutline => '\u{f0e83}',
        Nerd::BookPlus => '\u{f05db}',
        Nerd::BookPlusMultiple => '\u{f0a95}',
        Nerd::BookPlusMultipleOutline => '\u{f0ade}',
        Nerd::BookPlusOutline => '\u{f1696}',
        Nerd::BookRefresh => '\u{f1697}',
        Nerd::BookRefreshOutline => '\u{f1698}',
        Nerd::BookRemove => '\u{f0a97}',
        Nerd::BookRemoveMultiple => '\u{f0a96}',
        Nerd::BookRemoveMultipleOutline => '\u{f04ca}',
        Nerd::BookRemoveOutline => '\u{f1699}',
        Nerd::BookSearch => '\u{f0e84}',
        Nerd::BookSearchOutline => '\u{f0e85}',
        Nerd::BookSettings => '\u{f169a}',
        Nerd::BookSettingsOutline => '\u{f169b}',
        Nerd::BookSync => '\u{f169c}',
        Nerd::BookSyncOutline => '\u{f16c8}',
        Nerd::BookThree => '\u{f00ba}',
        Nerd::BookTwo => '\u{f405}',
        Nerd::BookVariant => '\u{f00bf}',
        Nerd::BookVariantMultiple => '\u{f00bc}',
        Nerd::Bookmark => '\u{eaa5}',
        Nerd::BookmarkBoxMultiple => '\u{f196c}',
        Nerd::BookmarkBoxMultipleOutline => '\u{f196d}',
        Nerd::BookmarkCheck => '\u{f00c1}',
        Nerd::BookmarkCheckOutline => '\u{f137b}',
        Nerd::BookmarkEmpty => '\u{f097}',
        Nerd::BookmarkFill => '\u{f47a}',
        Nerd::BookmarkMinus => '\u{f09cc}',
        Nerd::BookmarkMinusOutline => '\u{f09cd}',
        Nerd::BookmarkMultiple => '\u{f0e15}',
        Nerd::BookmarkMultipleOutline => '\u{f0e16}',
        Nerd::BookmarkMusic => '\u{f00c2}',
        Nerd::BookmarkMusicOutline => '\u{f1379}',
        Nerd::BookmarkOff => '\u{f09ce}',
        Nerd::BookmarkOffOutline => '\u{f09cf}',
        Nerd::BookmarkOne => '\u{f02e}',
        Nerd::BookmarkOutline => '\u{f00c3}',
        Nerd::BookmarkPlus => '\u{f00c5}',
        Nerd::BookmarkPlusOutline => '\u{f00c4}',
        Nerd::BookmarkRemove => '\u{f00c6}',
        Nerd::BookmarkRemoveOutline => '\u{f137a}',
        Nerd::BookmarkSlash => '\u{f533}',
        Nerd::BookmarkSlashFill => '\u{f493}',
        Nerd::BookmarkThree => '\u{f00c0}',
        Nerd::BookmarkTwo => '\u{f461}',
        Nerd::Bookshelf => '\u{f125f}',
        Nerd::BoomGate => '\u{f0e86}',
        Nerd::BoomGateAlert => '\u{f0e87}',
        Nerd::BoomGateAlertOutline => '\u{f0e88}',
        Nerd::BoomGateArrowDown => '\u{f0e89}',
        Nerd::BoomGateArrowDownOutline => '\u{f0e8a}',
        Nerd::BoomGateArrowUp => '\u{f0e8c}',
        Nerd::BoomGateArrowUpOutline => '\u{f0e8d}',
        Nerd::BoomGateOutline => '\u{f0e8b}',
        Nerd::BoomGateUp => '\u{f17f9}',
        Nerd::BoomGateUpOutline => '\u{f17fa}',
        Nerd::Boombox => '\u{f05dc}',
        Nerd::Boomerang => '\u{f10cf}',
        Nerd::Bootstrap => '\u{f06c6}',
        Nerd::BorderAll => '\u{f00c7}',
        Nerd::BorderAllVariant => '\u{f08a1}',
        Nerd::BorderBottom => '\u{f00c8}',
        Nerd::BorderBottomVariant => '\u{f08a2}',
        Nerd::BorderColor => '\u{f00c9}',
        Nerd::BorderHorizontal => '\u{f00ca}',
        Nerd::BorderInside => '\u{f00cb}',
        Nerd::BorderLeft => '\u{f00cc}',
        Nerd::BorderLeftVariant => '\u{f08a3}',
        Nerd::BorderNone => '\u{f00cd}',
        Nerd::BorderNoneVariant => '\u{f08a4}',
        Nerd::BorderOutside => '\u{f00ce}',
        Nerd::BorderRight => '\u{f00cf}',
        Nerd::BorderRightVariant => '\u{f08a5}',
        Nerd::BorderStyle => '\u{f00d0}',
        Nerd::BorderTop => '\u{f00d1}',
        Nerd::BorderTopVariant => '\u{f08a6}',
        Nerd::BorderVertical => '\u{f00d2}',
        Nerd::BottleSoda => '\u{f1070}',
        Nerd::BottleSodaClassic => '\u{f1071}',
        Nerd::BottleSodaClassicOutline => '\u{f1363}',
        Nerd::BottleSodaOutline => '\u{f1072}',
        Nerd::BottleTonic => '\u{f112e}',
        Nerd::BottleTonicOutline => '\u{f112f}',
        Nerd::BottleTonicPlus => '\u{f1130}',
        Nerd::BottleTonicPlusOutline => '\u{f1131}',
        Nerd::BottleTonicSkull => '\u{f1132}',
        Nerd::BottleTonicSkullOutline => '\u{f1133}',
        Nerd::BottleWine => '\u{f0854}',
        Nerd::BottleWineOutline => '\u{f1310}',
        Nerd::BowArrow => '\u{f1841}',
        Nerd::BowTie => '\u{f0678}',
        Nerd::Bowl => '\u{f028e}',
        Nerd::BowlMix => '\u{f0617}',
        Nerd::BowlMixOutline => '\u{f02e4}',
        Nerd::BowlOutline => '\u{f02a9}',
        Nerd::Bowling => '\u{f00d3}',
        Nerd::Box => '\u{f00d4}',
        Nerd::BoxCutter => '\u{f00d5}',
        Nerd::BoxCutterOff => '\u{f0b4a}',
        Nerd::BoxShadow => '\u{f0637}',
        Nerd::BoxingGlove => '\u{f0b65}',
        Nerd::BracketDot => '\u{ebe5}',
        Nerd::BracketError => '\u{ebe6}',
        Nerd::Braille => '\u{f09d0}',
        Nerd::Brain => '\u{e28c}',
        Nerd::BrainOne => '\u{f09d1}',
        Nerd::Bread => '\u{e28d}',
        Nerd::BreadSlice => '\u{f0cee}',
        Nerd::BreadSliceOutline => '\u{f0cef}',
        Nerd::Bridge => '\u{f0618}',
        Nerd::Briefcase => '\u{eaac}',
        Nerd::BriefcaseAccount => '\u{f0cf0}',
        Nerd::BriefcaseAccountOutline => '\u{f0cf1}',
        Nerd::BriefcaseArrowLeftRight => '\u{f1a8d}',
        Nerd::BriefcaseArrowLeftRightOutline => '\u{f1a8e}',
        Nerd::BriefcaseArrowUpDown => '\u{f1a8f}',
        Nerd::BriefcaseArrowUpDownOutline => '\u{f1a90}',
        Nerd::BriefcaseCheck => '\u{f00d7}',
        Nerd::BriefcaseCheckOutline => '\u{f131e}',
        Nerd::BriefcaseClock => '\u{f10d0}',
        Nerd::BriefcaseClockOutline => '\u{f10d1}',
        Nerd::BriefcaseDownload => '\u{f00d8}',
        Nerd::BriefcaseDownloadOutline => '\u{f0c3d}',
        Nerd::BriefcaseEdit => '\u{f0a98}',
        Nerd::BriefcaseEditOutline => '\u{f0c3e}',
        Nerd::BriefcaseEye => '\u{f17d9}',
        Nerd::BriefcaseEyeOutline => '\u{f17da}',
        Nerd::BriefcaseMinus => '\u{f0a2a}',
        Nerd::BriefcaseMinusOutline => '\u{f0c3f}',
        Nerd::BriefcaseOff => '\u{f1658}',
        Nerd::BriefcaseOffOutline => '\u{f1659}',
        Nerd::BriefcaseOne => '\u{f0b1}',
        Nerd::BriefcaseOutline => '\u{f0814}',
        Nerd::BriefcasePlus => '\u{f0a2b}',
        Nerd::BriefcasePlusOutline => '\u{f0c40}',
        Nerd::BriefcaseRemove => '\u{f0a2c}',
        Nerd::BriefcaseRemoveOutline => '\u{f0c41}',
        Nerd::BriefcaseSearch => '\u{f0a2d}',
        Nerd::BriefcaseSearchOutline => '\u{f0c42}',
        Nerd::BriefcaseThree => '\u{f00d6}',
        Nerd::BriefcaseTwo => '\u{f491}',
        Nerd::BriefcaseUpload => '\u{f00d9}',
        Nerd::BriefcaseUploadOutline => '\u{f0c43}',
        Nerd::BriefcaseVariant => '\u{f1494}',
        Nerd::BriefcaseVariantOff => '\u{f165a}',
        Nerd::BriefcaseVariantOffOutline => '\u{f165b}',
        Nerd::BriefcaseVariantOutline => '\u{f1495}',
        Nerd::BrightnessAuto => '\u{f00e1}',
        Nerd::BrightnessFive => '\u{f00de}',
        Nerd::BrightnessFour => '\u{f00dd}',
        Nerd::BrightnessOne => '\u{f00da}',
        Nerd::BrightnessPercent => '\u{f0cf2}',
        Nerd::BrightnessSeven => '\u{f00e0}',
        Nerd::BrightnessSix => '\u{f00df}',
        Nerd::BrightnessThree => '\u{f00dc}',
        Nerd::BrightnessTwo => '\u{f00db}',
        Nerd::Broadcast => '\u{eaad}',
        Nerd::BroadcastOff => '\u{f1721}',
        Nerd::BroadcastOne => '\u{f43c}',
        Nerd::BroadcastTwo => '\u{f1720}',
        Nerd::Broom => '\u{f00e2}',
        Nerd::Browser => '\u{eaae}',
        Nerd::BrowserOne => '\u{f488}',
        Nerd::Brush => '\u{f00e3}',
        Nerd::BrushOff => '\u{f1771}',
        Nerd::BrushOutline => '\u{f1a0d}',
        Nerd::BrushVariant => '\u{f1813}',
        Nerd::Bspwm => '\u{f355}',
        Nerd::Btc => '\u{f15a}',
        Nerd::Bucket => '\u{f1415}',
        Nerd::BucketOutline => '\u{f1416}',
        Nerd::Budgie => '\u{f320}',
        Nerd::Buffet => '\u{f0578}',
        Nerd::Bug => '\u{eaaf}',
        Nerd::BugCheck => '\u{f0a2e}',
        Nerd::BugCheckOutline => '\u{f0a2f}',
        Nerd::BugOne => '\u{f188}',
        Nerd::BugOutline => '\u{f0a30}',
        Nerd::BugThree => '\u{f00e4}',
        Nerd::BugTwo => '\u{f46f}',
        Nerd::Bugle => '\u{f0db4}',
        Nerd::Building => '\u{f0f7}',
        Nerd::BulkheadLight => '\u{f1a2f}',
        Nerd::Bulldozer => '\u{f0b22}',
        Nerd::Bullet => '\u{f0cf3}',
        Nerd::BulletinBoard => '\u{f00e5}',
        Nerd::Bullhorn => '\u{f0a1}',
        Nerd::BullhornOne => '\u{f00e6}',
        Nerd::BullhornOutline => '\u{f0b23}',
        Nerd::BullhornVariant => '\u{f196e}',
        Nerd::BullhornVariantOutline => '\u{f196f}',
        Nerd::Bullseye => '\u{f140}',
        Nerd::BullseyeArrow => '\u{f08c9}',
        Nerd::BullseyeOne => '\u{f05dd}',
        Nerd::Bulma => '\u{f12e7}',
        Nerd::BunkBed => '\u{f1302}',
        Nerd::BunkBedOutline => '\u{f0097}',
        Nerd::Bus => '\u{f00e7}',
        Nerd::BusAlert => '\u{f0a99}',
        Nerd::BusArticulatedEnd => '\u{f079c}',
        Nerd::BusArticulatedFront => '\u{f079d}',
        Nerd::BusClock => '\u{f08ca}',
        Nerd::BusDoubleDecker => '\u{f079e}',
        Nerd::BusElectric => '\u{f191d}',
        Nerd::BusMarker => '\u{f1212}',
        Nerd::BusMultiple => '\u{f0f3f}',
        Nerd::BusSchool => '\u{f079f}',
        Nerd::BusSide => '\u{f07a0}',
        Nerd::BusStop => '\u{f1012}',
        Nerd::BusStopCovered => '\u{f1013}',
        Nerd::BusStopUncovered => '\u{f1014}',
        Nerd::Butterfly => '\u{e28e}',
        Nerd::ButterflyOne => '\u{f1589}',
        Nerd::ButterflyOutline => '\u{f158a}',
        Nerd::CabinAFrame => '\u{f188c}',
        Nerd::CableData => '\u{f1394}',
        Nerd::Cache => '\u{f49b}',
        Nerd::Cached => '\u{f00e8}',
        Nerd::Cactus => '\u{f0db5}',
        Nerd::Cake => '\u{f00e9}',
        Nerd::CakeLayered => '\u{f00ea}',
        Nerd::CakeVariant => '\u{f00eb}',
        Nerd::CakeVariantOutline => '\u{f17f0}',
        Nerd::Calculator => '\u{f00ec}',
        Nerd::CalculatorVariant => '\u{f0a9a}',
        Nerd::CalculatorVariantOutline => '\u{f15a6}',
        Nerd::Calendar => '\u{eab0}',
        Nerd::CalendarAccount => '\u{f0ed7}',
        Nerd::CalendarAccountOutline => '\u{f0ed8}',
        Nerd::CalendarAlert => '\u{f0a31}',
        Nerd::CalendarArrowLeft => '\u{f1134}',
        Nerd::CalendarArrowRight => '\u{f1135}',
        Nerd::CalendarBlank => '\u{f00ee}',
        Nerd::CalendarBlankMultiple => '\u{f1073}',
        Nerd::CalendarBlankOutline => '\u{f0b66}',
        Nerd::CalendarCheck => '\u{f00ef}',
        Nerd::CalendarCheckOutline => '\u{f0c44}',
        Nerd::CalendarClock => '\u{f00f0}',
        Nerd::CalendarClockOutline => '\u{f16e1}',
        Nerd::CalendarCollapseHorizontal => '\u{f189d}',
        Nerd::CalendarCursor => '\u{f157b}',
        Nerd::CalendarEdit => '\u{f08a7}',
        Nerd::CalendarEmpty => '\u{f133}',
        Nerd::CalendarEnd => '\u{f166c}',
        Nerd::CalendarExpandHorizontal => '\u{f189e}',
        Nerd::CalendarExport => '\u{f0b24}',
        Nerd::CalendarHeart => '\u{f09d2}',
        Nerd::CalendarImport => '\u{f0b25}',
        Nerd::CalendarLock => '\u{f1641}',
        Nerd::CalendarLockOutline => '\u{f1642}',
        Nerd::CalendarMinus => '\u{f0d5c}',
        Nerd::CalendarMonth => '\u{f0e17}',
        Nerd::CalendarMonthOutline => '\u{f0e18}',
        Nerd::CalendarMultiple => '\u{f00f1}',
        Nerd::CalendarMultipleCheck => '\u{f00f2}',
        Nerd::CalendarMultiselect => '\u{f0a32}',
        Nerd::CalendarOne => '\u{f073}',
        Nerd::CalendarOutline => '\u{f0b67}',
        Nerd::CalendarPlus => '\u{f00f3}',
        Nerd::CalendarQuestion => '\u{f0692}',
        Nerd::CalendarRange => '\u{f0679}',
        Nerd::CalendarRangeOutline => '\u{f0b68}',
        Nerd::CalendarRefresh => '\u{f01e1}',
        Nerd::CalendarRefreshOutline => '\u{f0203}',
        Nerd::CalendarRemove => '\u{f00f4}',
        Nerd::CalendarRemoveOutline => '\u{f0c45}',
        Nerd::CalendarSearch => '\u{f094c}',
        Nerd::CalendarStar => '\u{f09d3}',
        Nerd::CalendarStart => '\u{f166d}',
        Nerd::CalendarSync => '\u{f0e8e}',
        Nerd::CalendarSyncOutline => '\u{f0e8f}',
        Nerd::CalendarText => '\u{f00f5}',
        Nerd::CalendarTextOutline => '\u{f0c46}',
        Nerd::CalendarThree => '\u{f00ed}',
        Nerd::CalendarToday => '\u{f00f6}',
        Nerd::CalendarTodayOutline => '\u{f1a30}',
        Nerd::CalendarTwo => '\u{f455}',
        Nerd::CalendarWeek => '\u{f0a33}',
        Nerd::CalendarWeekBegin => '\u{f0a34}',
        Nerd::CalendarWeekBeginOutline => '\u{f1a31}',
        Nerd::CalendarWeekEnd => '\u{f1a32}',
        Nerd::CalendarWeekEndOutline => '\u{f1a33}',
        Nerd::CalendarWeekOutline => '\u{f1a34}',
        Nerd::CalendarWeekend => '\u{f0ed9}',
        Nerd::CalendarWeekendOutline => '\u{f0eda}',
        Nerd::CallIncoming => '\u{eb92}',
        Nerd::CallMade => '\u{f00f7}',
        Nerd::CallMerge => '\u{f00f8}',
        Nerd::CallMissed => '\u{f00f9}',
        Nerd::CallOutgoing => '\u{eb93}',
        Nerd::CallReceived => '\u{f00fa}',
        Nerd::CallSplit => '\u{f00fb}',
        Nerd::Camcorder => '\u{f00fc}',
        Nerd::CamcorderOff => '\u{f00ff}',
        Nerd::Camera => '\u{f030}',
        Nerd::CameraAccount => '\u{f08cb}',
        Nerd::CameraBurst => '\u{f0693}',
        Nerd::CameraControl => '\u{f0b69}',
        Nerd::CameraDocument => '\u{f1871}',
        Nerd::CameraDocumentOff => '\u{f1872}',
        Nerd::CameraEnhance => '\u{f0101}',
        Nerd::CameraEnhanceOutline => '\u{f0b6a}',
        Nerd::CameraFlip => '\u{f15d9}',
        Nerd::CameraFlipOutline => '\u{f15da}',
        Nerd::CameraFront => '\u{f0102}',
        Nerd::CameraFrontVariant => '\u{f0103}',
        Nerd::CameraGopro => '\u{f07a1}',
        Nerd::CameraImage => '\u{f08cc}',
        Nerd::CameraIris => '\u{f0104}',
        Nerd::CameraLock => '\u{f1a14}',
        Nerd::CameraLockOutline => '\u{f1a15}',
        Nerd::CameraMarker => '\u{f19a7}',
        Nerd::CameraMarkerOutline => '\u{f19a8}',
        Nerd::CameraMeteringCenter => '\u{f07a2}',
        Nerd::CameraMeteringMatrix => '\u{f07a3}',
        Nerd::CameraMeteringPartial => '\u{f07a4}',
        Nerd::CameraMeteringSpot => '\u{f07a5}',
        Nerd::CameraOff => '\u{f05df}',
        Nerd::CameraOffOutline => '\u{f19bf}',
        Nerd::CameraOne => '\u{f0100}',
        Nerd::CameraOutline => '\u{f0d5d}',
        Nerd::CameraPartyMode => '\u{f0105}',
        Nerd::CameraPlus => '\u{f0edb}',
        Nerd::CameraPlusOutline => '\u{f0edc}',
        Nerd::CameraRear => '\u{f0106}',
        Nerd::CameraRearVariant => '\u{f0107}',
        Nerd::CameraRetake => '\u{f0e19}',
        Nerd::CameraRetakeOutline => '\u{f0e1a}',
        Nerd::CameraRetro => '\u{f083}',
        Nerd::CameraSwitch => '\u{f0108}',
        Nerd::CameraSwitchOutline => '\u{f084a}',
        Nerd::CameraTimer => '\u{f0109}',
        Nerd::CameraWireless => '\u{f0db6}',
        Nerd::CameraWirelessOutline => '\u{f0db7}',
        Nerd::Campfire => '\u{f0edd}',
        Nerd::Cancel => '\u{f073a}',
        Nerd::Candelabra => '\u{f17d2}',
        Nerd::CandelabraFire => '\u{f17d3}',
        Nerd::Candle => '\u{f05e2}',
        Nerd::Candy => '\u{f1970}',
        Nerd::CandyOff => '\u{f1971}',
        Nerd::CandyOffOutline => '\u{f1972}',
        Nerd::CandyOutline => '\u{f1973}',
        Nerd::Candycane => '\u{f010a}',
        Nerd::Cannabis => '\u{f07a6}',
        Nerd::CannabisOff => '\u{f166e}',
        Nerd::CapsLock => '\u{f0a9b}',
        Nerd::Car => '\u{f010b}',
        Nerd::CarArrowLeft => '\u{f13b2}',
        Nerd::CarArrowRight => '\u{f13b3}',
        Nerd::CarBack => '\u{f0e1b}',
        Nerd::CarBattery => '\u{f010c}',
        Nerd::CarBrakeAbs => '\u{f0c47}',
        Nerd::CarBrakeAlert => '\u{f0c48}',
        Nerd::CarBrakeFluidLevel => '\u{f1909}',
        Nerd::CarBrakeHold => '\u{f0d5e}',
        Nerd::CarBrakeLowPressure => '\u{f190a}',
        Nerd::CarBrakeParking => '\u{f0d5f}',
        Nerd::CarBrakeRetarder => '\u{f1017}',
        Nerd::CarBrakeTemperature => '\u{f190b}',
        Nerd::CarBrakeWornLinings => '\u{f190c}',
        Nerd::CarChildSeat => '\u{f0fa3}',
        Nerd::CarClock => '\u{f1974}',
        Nerd::CarClutch => '\u{f1018}',
        Nerd::CarCog => '\u{f13cc}',
        Nerd::CarConnected => '\u{f010d}',
        Nerd::CarConvertible => '\u{f07a7}',
        Nerd::CarCoolantLevel => '\u{f1019}',
        Nerd::CarCruiseControl => '\u{f0d60}',
        Nerd::CarDefrostFront => '\u{f0d61}',
        Nerd::CarDefrostRear => '\u{f0d62}',
        Nerd::CarDoor => '\u{f0b6b}',
        Nerd::CarDoorLock => '\u{f109d}',
        Nerd::CarElectric => '\u{f0b6c}',
        Nerd::CarElectricOutline => '\u{f15b5}',
        Nerd::CarEmergency => '\u{f160f}',
        Nerd::CarEsp => '\u{f0c49}',
        Nerd::CarEstate => '\u{f07a8}',
        Nerd::CarHatchback => '\u{f07a9}',
        Nerd::CarInfo => '\u{f11be}',
        Nerd::CarKey => '\u{f0b6d}',
        Nerd::CarLiftedPickup => '\u{f152d}',
        Nerd::CarLightAlert => '\u{f190d}',
        Nerd::CarLightDimmed => '\u{f0c4a}',
        Nerd::CarLightFog => '\u{f0c4b}',
        Nerd::CarLightHigh => '\u{f0c4c}',
        Nerd::CarLimousine => '\u{f08cd}',
        Nerd::CarMultiple => '\u{f0b6e}',
        Nerd::CarOff => '\u{f0e1c}',
        Nerd::CarOutline => '\u{f14ed}',
        Nerd::CarParkingLights => '\u{f0d63}',
        Nerd::CarPickup => '\u{f07aa}',
        Nerd::CarSeat => '\u{f0fa4}',
        Nerd::CarSeatCooler => '\u{f0fa5}',
        Nerd::CarSeatHeater => '\u{f0fa6}',
        Nerd::CarSelect => '\u{f1879}',
        Nerd::CarSettings => '\u{f13cd}',
        Nerd::CarShiftPattern => '\u{f0f40}',
        Nerd::CarSide => '\u{f07ab}',
        Nerd::CarSpeedLimiter => '\u{f190e}',
        Nerd::CarSports => '\u{f07ac}',
        Nerd::CarThreePlus => '\u{f1016}',
        Nerd::CarTireAlert => '\u{f0c4d}',
        Nerd::CarTractionControl => '\u{f0d64}',
        Nerd::CarTurbocharger => '\u{f101a}',
        Nerd::CarTwoPlus => '\u{f1015}',
        Nerd::CarWash => '\u{f010e}',
        Nerd::CarWindshield => '\u{f101b}',
        Nerd::CarWindshieldOutline => '\u{f101c}',
        Nerd::CarWireless => '\u{f1878}',
        Nerd::CarWrench => '\u{f1814}',
        Nerd::Carabiner => '\u{f14c0}',
        Nerd::Caravan => '\u{f07ad}',
        Nerd::Card => '\u{f0b6f}',
        Nerd::CardAccountDetails => '\u{f05d2}',
        Nerd::CardAccountDetailsOutline => '\u{f0dab}',
        Nerd::CardAccountDetailsStar => '\u{f02a3}',
        Nerd::CardAccountDetailsStarOutline => '\u{f06db}',
        Nerd::CardAccountMail => '\u{f018e}',
        Nerd::CardAccountMailOutline => '\u{f0e98}',
        Nerd::CardAccountPhone => '\u{f0e99}',
        Nerd::CardAccountPhoneOutline => '\u{f0e9a}',
        Nerd::CardBulleted => '\u{f0b70}',
        Nerd::CardBulletedOff => '\u{f0b71}',
        Nerd::CardBulletedOffOutline => '\u{f0b72}',
        Nerd::CardBulletedOutline => '\u{f0b73}',
        Nerd::CardBulletedSettings => '\u{f0b74}',
        Nerd::CardBulletedSettingsOutline => '\u{f0b75}',
        Nerd::CardMinus => '\u{f1600}',
        Nerd::CardMinusOutline => '\u{f1601}',
        Nerd::CardMultiple => '\u{f17f1}',
        Nerd::CardMultipleOutline => '\u{f17f2}',
        Nerd::CardOff => '\u{f1602}',
        Nerd::CardOffOutline => '\u{f1603}',
        Nerd::CardOutline => '\u{f0b76}',
        Nerd::CardPlus => '\u{f11ff}',
        Nerd::CardPlusOutline => '\u{f1200}',
        Nerd::CardRemove => '\u{f1604}',
        Nerd::CardRemoveOutline => '\u{f1605}',
        Nerd::CardSearch => '\u{f1074}',
        Nerd::CardSearchOutline => '\u{f1075}',
        Nerd::CardText => '\u{f0b77}',
        Nerd::CardTextOutline => '\u{f0b78}',
        Nerd::Cards => '\u{f0638}',
        Nerd::CardsClub => '\u{f08ce}',
        Nerd::CardsClubOutline => '\u{f189f}',
        Nerd::CardsDiamond => '\u{f08cf}',
        Nerd::CardsDiamondOutline => '\u{f101d}',
        Nerd::CardsOutline => '\u{f0639}',
        Nerd::CardsPlaying => '\u{f18a1}',
        Nerd::CardsPlayingClub => '\u{f18a2}',
        Nerd::CardsPlayingClubMultiple => '\u{f18a3}',
        Nerd::CardsPlayingClubMultipleOutline => '\u{f18a4}',
        Nerd::CardsPlayingClubOutline => '\u{f18a5}',
        Nerd::CardsPlayingDiamond => '\u{f18a6}',
        Nerd::CardsPlayingDiamondMultiple => '\u{f18a7}',
        Nerd::CardsPlayingDiamondMultipleOutline => '\u{f18a8}',
        Nerd::CardsPlayingDiamondOutline => '\u{f18a9}',
        Nerd::CardsPlayingHeart => '\u{f18aa}',
        Nerd::CardsPlayingHeartMultiple => '\u{f18ab}',
        Nerd::CardsPlayingHeartMultipleOutline => '\u{f18ac}',
        Nerd::CardsPlayingHeartOutline => '\u{f18ad}',
        Nerd::CardsPlayingOutline => '\u{f063a}',
        Nerd::CardsPlayingSpade => '\u{f18ae}',
        Nerd::CardsPlayingSpadeMultiple => '\u{f18af}',
        Nerd::CardsPlayingSpadeMultipleOutline => '\u{f18b0}',
        Nerd::CardsPlayingSpadeOutline => '\u{f18b1}',
        Nerd::CardsSpade => '\u{f08d1}',
        Nerd::CardsSpadeOutline => '\u{f18b2}',
        Nerd::CardsVariant => '\u{f06c7}',
        Nerd::CaretDown => '\u{f0d7}',
        Nerd::CaretLeft => '\u{f0d9}',
        Nerd::CaretRight => '\u{f0da}',
        Nerd::CaretUp => '\u{f0d8}',
        Nerd::Carot => '\u{e28f}',
        Nerd::Carrot => '\u{f010f}',
        Nerd::Cart => '\u{f0110}',
        Nerd::CartArrowDown => '\u{f0d66}',
        Nerd::CartArrowRight => '\u{f0c4e}',
        Nerd::CartArrowUp => '\u{f0d67}',
        Nerd::CartCheck => '\u{f15ea}',
        Nerd::CartHeart => '\u{f18e0}',
        Nerd::CartMinus => '\u{f0d68}',
        Nerd::CartOff => '\u{f066b}',
        Nerd::CartOutline => '\u{f0111}',
        Nerd::CartPlus => '\u{f0112}',
        Nerd::CartRemove => '\u{f0d69}',
        Nerd::CartVariant => '\u{f15eb}',
        Nerd::CaseSensitive => '\u{eab1}',
        Nerd::CaseSensitiveAlt => '\u{f0113}',
        Nerd::Cash => '\u{f0114}',
        Nerd::CashCheck => '\u{f14ee}',
        Nerd::CashClock => '\u{f1a91}',
        Nerd::CashFast => '\u{f185c}',
        Nerd::CashLock => '\u{f14ea}',
        Nerd::CashLockOpen => '\u{f14eb}',
        Nerd::CashMarker => '\u{f0db8}',
        Nerd::CashMinus => '\u{f1260}',
        Nerd::CashMultiple => '\u{f0116}',
        Nerd::CashOnezerozero => '\u{f0115}',
        Nerd::CashPlus => '\u{f1261}',
        Nerd::CashRefund => '\u{f0a9c}',
        Nerd::CashRegister => '\u{f0cf4}',
        Nerd::CashRemove => '\u{f1262}',
        Nerd::CashSync => '\u{f1a92}',
        Nerd::Cassette => '\u{f09d4}',
        Nerd::Cast => '\u{f0118}',
        Nerd::CastAudio => '\u{f101e}',
        Nerd::CastAudioVariant => '\u{f1749}',
        Nerd::CastConnected => '\u{f0119}',
        Nerd::CastEducation => '\u{f0e1d}',
        Nerd::CastOff => '\u{f078a}',
        Nerd::CastVariant => '\u{f001f}',
        Nerd::Castle => '\u{f011a}',
        Nerd::Cat => '\u{f011b}',
        Nerd::CcBy => '\u{e290}',
        Nerd::CcCc => '\u{e291}',
        Nerd::CcNc => '\u{e292}',
        Nerd::CcNcEu => '\u{e293}',
        Nerd::CcNcJp => '\u{e294}',
        Nerd::CcNd => '\u{e295}',
        Nerd::CcRemix => '\u{e296}',
        Nerd::CcSa => '\u{e297}',
        Nerd::CcShare => '\u{e298}',
        Nerd::CcZero => '\u{e299}',
        Nerd::Cctv => '\u{f07ae}',
        Nerd::CctvOff => '\u{f185f}',
        Nerd::CeilingFan => '\u{f1797}',
        Nerd::CeilingFanLight => '\u{f1798}',
        Nerd::CeilingLight => '\u{f0769}',
        Nerd::CeilingLightMultiple => '\u{f18dd}',
        Nerd::CeilingLightMultipleOutline => '\u{f18de}',
        Nerd::CeilingLightOutline => '\u{f17c7}',
        Nerd::Cellphone => '\u{f011c}',
        Nerd::CellphoneArrowDown => '\u{f09d5}',
        Nerd::CellphoneArrowDownVariant => '\u{f19c5}',
        Nerd::CellphoneBasic => '\u{f011e}',
        Nerd::CellphoneCharging => '\u{f1397}',
        Nerd::CellphoneCheck => '\u{f17fd}',
        Nerd::CellphoneCog => '\u{f0951}',
        Nerd::CellphoneDock => '\u{f011f}',
        Nerd::CellphoneInformation => '\u{f0f41}',
        Nerd::CellphoneKey => '\u{f094e}',
        Nerd::CellphoneLink => '\u{f0121}',
        Nerd::CellphoneLinkOff => '\u{f0122}',
        Nerd::CellphoneLock => '\u{f094f}',
        Nerd::CellphoneMarker => '\u{f183a}',
        Nerd::CellphoneMessage => '\u{f08d3}',
        Nerd::CellphoneMessageOff => '\u{f10d2}',
        Nerd::CellphoneNfc => '\u{f0e90}',
        Nerd::CellphoneNfcOff => '\u{f12d8}',
        Nerd::CellphoneOff => '\u{f0950}',
        Nerd::CellphonePlay => '\u{f101f}',
        Nerd::CellphoneRemove => '\u{f094d}',
        Nerd::CellphoneScreenshot => '\u{f0a35}',
        Nerd::CellphoneSettings => '\u{f0123}',
        Nerd::CellphoneSound => '\u{f0952}',
        Nerd::CellphoneText => '\u{f08d2}',
        Nerd::CellphoneWireless => '\u{f0815}',
        Nerd::Centos => '\u{f111a}',
        Nerd::Certificate => '\u{f0a3}',
        Nerd::CertificateOne => '\u{f0124}',
        Nerd::CertificateOutline => '\u{f1188}',
        Nerd::ChairRolling => '\u{f0f48}',
        Nerd::ChairSchool => '\u{f0125}',
        Nerd::Chandelier => '\u{f1793}',
        Nerd::Charity => '\u{f0c4f}',
        Nerd::ChartArc => '\u{f0126}',
        Nerd::ChartAreaspline => '\u{f0127}',
        Nerd::ChartAreasplineVariant => '\u{f0e91}',
        Nerd::ChartBar => '\u{f0128}',
        Nerd::ChartBarStacked => '\u{f076a}',
        Nerd::ChartBellCurve => '\u{f0c50}',
        Nerd::ChartBellCurveCumulative => '\u{f0fa7}',
        Nerd::ChartBox => '\u{f154d}',
        Nerd::ChartBoxOutline => '\u{f154e}',
        Nerd::ChartBoxPlusOutline => '\u{f154f}',
        Nerd::ChartBubble => '\u{f05e3}',
        Nerd::ChartDonut => '\u{f07af}',
        Nerd::ChartDonutVariant => '\u{f07b0}',
        Nerd::ChartGantt => '\u{f066c}',
        Nerd::ChartHistogram => '\u{f0129}',
        Nerd::ChartLine => '\u{f012a}',
        Nerd::ChartLineStacked => '\u{f076b}',
        Nerd::ChartLineVariant => '\u{f07b1}',
        Nerd::ChartMultiline => '\u{f08d4}',
        Nerd::ChartMultiple => '\u{f1213}',
        Nerd::ChartPie => '\u{f012b}',
        Nerd::ChartPpf => '\u{f1380}',
        Nerd::ChartSankey => '\u{f11df}',
        Nerd::ChartSankeyVariant => '\u{f11e0}',
        Nerd::ChartScatterPlot => '\u{f0e92}',
        Nerd::ChartScatterPlotHexbin => '\u{f066d}',
        Nerd::ChartTimeline => '\u{f066e}',
        Nerd::ChartTimelineVariant => '\u{f0e93}',
        Nerd::ChartTimelineVariantShimmer => '\u{f15b6}',
        Nerd::ChartTree => '\u{f0e94}',
        Nerd::ChartWaterfall => '\u{f1918}',
        Nerd::Chat => '\u{f0b79}',
        Nerd::ChatAlert => '\u{f0b7a}',
        Nerd::ChatAlertOutline => '\u{f12c9}',
        Nerd::ChatMinus => '\u{f1410}',
        Nerd::ChatMinusOutline => '\u{f1413}',
        Nerd::ChatOutline => '\u{f0ede}',
        Nerd::ChatPlus => '\u{f140f}',
        Nerd::ChatPlusOutline => '\u{f1412}',
        Nerd::ChatProcessing => '\u{f0b7b}',
        Nerd::ChatProcessingOutline => '\u{f12ca}',
        Nerd::ChatQuestion => '\u{f1738}',
        Nerd::ChatQuestionOutline => '\u{f1739}',
        Nerd::ChatRemove => '\u{f1411}',
        Nerd::ChatRemoveOutline => '\u{f1414}',
        Nerd::ChatSleep => '\u{f12d1}',
        Nerd::ChatSleepOutline => '\u{f12d2}',
        Nerd::Check => '\u{eab2}',
        Nerd::CheckAll => '\u{ebb1}',
        Nerd::CheckAllOne => '\u{f012d}',
        Nerd::CheckBold => '\u{f0e1e}',
        Nerd::CheckCircle => '\u{f49e}',
        Nerd::CheckCircleFill => '\u{f4a4}',
        Nerd::CheckCircleOne => '\u{f05e0}',
        Nerd::CheckCircleOutline => '\u{f05e1}',
        Nerd::CheckDecagram => '\u{f0791}',
        Nerd::CheckDecagramOutline => '\u{f1740}',
        Nerd::CheckEmpty => '\u{f096}',
        Nerd::CheckMinus => '\u{f147}',
        Nerd::CheckNetwork => '\u{f0c53}',
        Nerd::CheckNetworkOutline => '\u{f0c54}',
        Nerd::CheckOne => '\u{f046}',
        Nerd::CheckOutline => '\u{f0855}',
        Nerd::CheckSign => '\u{f14a}',
        Nerd::CheckThree => '\u{f012c}',
        Nerd::CheckTwo => '\u{f42e}',
        Nerd::CheckUnderline => '\u{f0e1f}',
        Nerd::CheckUnderlineCircle => '\u{f0e20}',
        Nerd::CheckUnderlineCircleOutline => '\u{f0e21}',
        Nerd::Checkbook => '\u{f0a9d}',
        Nerd::Checkbox => '\u{f4a7}',
        Nerd::CheckboxBlank => '\u{f012e}',
        Nerd::CheckboxBlankBadge => '\u{f1176}',
        Nerd::CheckboxBlankBadgeOutline => '\u{f0117}',
        Nerd::CheckboxBlankCircle => '\u{f012f}',
        Nerd::CheckboxBlankCircleOne => '\u{f0765}',
        Nerd::CheckboxBlankCircleOutline => '\u{f0130}',
        Nerd::CheckboxBlankCircleOutlineOne => '\u{f043d}',
        Nerd::CheckboxBlankCircleOutlineTwo => '\u{f0766}',
        Nerd::CheckboxBlankOff => '\u{f12ec}',
        Nerd::CheckboxBlankOffOutline => '\u{f12ed}',
        Nerd::CheckboxBlankOutline => '\u{f0131}',
        Nerd::CheckboxIntermediate => '\u{f0856}',
        Nerd::CheckboxMarked => '\u{f0132}',
        Nerd::CheckboxMarkedCircle => '\u{f0133}',
        Nerd::CheckboxMarkedCircleOutline => '\u{f0134}',
        Nerd::CheckboxMarkedCirclePlusOutline => '\u{f1927}',
        Nerd::CheckboxMarkedOutline => '\u{f0135}',
        Nerd::CheckboxMultipleBlank => '\u{f0136}',
        Nerd::CheckboxMultipleBlankCircle => '\u{f063b}',
        Nerd::CheckboxMultipleBlankCircleOutline => '\u{f063c}',
        Nerd::CheckboxMultipleBlankOutline => '\u{f0137}',
        Nerd::CheckboxMultipleMarked => '\u{f0138}',
        Nerd::CheckboxMultipleMarkedCircle => '\u{f063d}',
        Nerd::CheckboxMultipleMarkedCircleOutline => '\u{f063e}',
        Nerd::CheckboxMultipleMarkedOutline => '\u{f0139}',
        Nerd::CheckboxMultipleOutline => '\u{f0c51}',
        Nerd::CheckboxOutline => '\u{f0c52}',
        Nerd::Checkerboard => '\u{f013a}',
        Nerd::CheckerboardMinus => '\u{f1202}',
        Nerd::CheckerboardPlus => '\u{f1201}',
        Nerd::CheckerboardRemove => '\u{f1203}',
        Nerd::Checklist => '\u{eab3}',
        Nerd::ChecklistO => '\u{e29a}',
        Nerd::ChecklistOne => '\u{f45e}',
        Nerd::Cheese => '\u{f12b9}',
        Nerd::CheeseOff => '\u{f13ee}',
        Nerd::ChefHat => '\u{f0b7c}',
        Nerd::ChemicalWeapon => '\u{f013b}',
        Nerd::Cherry => '\u{e29b}',
        Nerd::ChessBishop => '\u{e29c}',
        Nerd::ChessBishopOne => '\u{f085c}',
        Nerd::ChessHorse => '\u{e25f}',
        Nerd::ChessKing => '\u{e260}',
        Nerd::ChessKingOne => '\u{f0857}',
        Nerd::ChessKnight => '\u{f0858}',
        Nerd::ChessPawn => '\u{e261}',
        Nerd::ChessPawnOne => '\u{f0859}',
        Nerd::ChessQueen => '\u{e262}',
        Nerd::ChessQueenOne => '\u{f085a}',
        Nerd::ChessRook => '\u{f085b}',
        Nerd::ChessTower => '\u{e263}',
        Nerd::Chesse => '\u{e264}',
        Nerd::ChevronDoubleDown => '\u{f013c}',
        Nerd::ChevronDoubleLeft => '\u{f013d}',
        Nerd::ChevronDoubleRight => '\u{f013e}',
        Nerd::ChevronDoubleUp => '\u{f013f}',
        Nerd::ChevronDown => '\u{f078}',
        Nerd::ChevronDownBox => '\u{f09d6}',
        Nerd::ChevronDownBoxOutline => '\u{f09d7}',
        Nerd::ChevronDownCircle => '\u{f0b26}',
        Nerd::ChevronDownCircleOutline => '\u{f0b27}',
        Nerd::ChevronDownOne => '\u{f47c}',
        Nerd::ChevronDownTwo => '\u{f0140}',
        Nerd::ChevronLeft => '\u{f053}',
        Nerd::ChevronLeftBox => '\u{f09d8}',
        Nerd::ChevronLeftBoxOutline => '\u{f09d9}',
        Nerd::ChevronLeftCircle => '\u{f0b28}',
        Nerd::ChevronLeftCircleOutline => '\u{f0b29}',
        Nerd::ChevronLeftOne => '\u{f47d}',
        Nerd::ChevronLeftTwo => '\u{f0141}',
        Nerd::ChevronRight => '\u{f054}',
        Nerd::ChevronRightBox => '\u{f09da}',
        Nerd::ChevronRightBoxOutline => '\u{f09db}',
        Nerd::ChevronRightCircle => '\u{f0b2a}',
        Nerd::ChevronRightCircleOutline => '\u{f0b2b}',
        Nerd::ChevronRightOne => '\u{f460}',
        Nerd::ChevronRightTwo => '\u{f0142}',
        Nerd::ChevronSignDown => '\u{f13a}',
        Nerd::ChevronSignLeft => '\u{f137}',
        Nerd::ChevronSignRight => '\u{f138}',
        Nerd::ChevronSignUp => '\u{f139}',
        Nerd::ChevronTripleDown => '\u{f0db9}',
        Nerd::ChevronTripleLeft => '\u{f0dba}',
        Nerd::ChevronTripleRight => '\u{f0dbb}',
        Nerd::ChevronTripleUp => '\u{f0dbc}',
        Nerd::ChevronUp => '\u{f077}',
        Nerd::ChevronUpBox => '\u{f09dc}',
        Nerd::ChevronUpBoxOutline => '\u{f09dd}',
        Nerd::ChevronUpCircle => '\u{f0b2c}',
        Nerd::ChevronUpCircleOutline => '\u{f0b2d}',
        Nerd::ChevronUpOne => '\u{f47b}',
        Nerd::ChevronUpTwo => '\u{f0143}',
        Nerd::ChickenThigh => '\u{e29f}',
        Nerd::ChiliAlert => '\u{f17ea}',
        Nerd::ChiliAlertOutline => '\u{f17eb}',
        Nerd::ChiliHot => '\u{f07b2}',
        Nerd::ChiliHotOutline => '\u{f17ec}',
        Nerd::ChiliMedium => '\u{f07b3}',
        Nerd::ChiliMediumOutline => '\u{f17ed}',
        Nerd::ChiliMild => '\u{f07b4}',
        Nerd::ChiliMildOutline => '\u{f17ee}',
        Nerd::ChiliOff => '\u{f1467}',
        Nerd::ChiliOffOutline => '\u{f17ef}',
        Nerd::Chilli => '\u{e265}',
        Nerd::Chip => '\u{e266}',
        Nerd::ChipOne => '\u{f061a}',
        Nerd::ChromeClose => '\u{eab8}',
        Nerd::ChromeMaximize => '\u{eab9}',
        Nerd::ChromeMinimize => '\u{eaba}',
        Nerd::ChromeRestore => '\u{eabb}',
        Nerd::Church => '\u{f0144}',
        Nerd::Cicling => '\u{e267}',
        Nerd::Cigar => '\u{f1189}',
        Nerd::CigarOff => '\u{f141b}',
        Nerd::Cinnamon => '\u{f35f}',
        Nerd::Circle => '\u{f111}',
        Nerd::CircleArrowDown => '\u{f0ab}',
        Nerd::CircleArrowLeft => '\u{f0a8}',
        Nerd::CircleArrowRight => '\u{f0a9}',
        Nerd::CircleArrowUp => '\u{f0aa}',
        Nerd::CircleBlank => '\u{f10c}',
        Nerd::CircleBox => '\u{f15dc}',
        Nerd::CircleBoxOutline => '\u{f15dd}',
        Nerd::CircleDouble => '\u{f0e95}',
        Nerd::CircleEditOutline => '\u{f08d5}',
        Nerd::CircleExpand => '\u{f0e96}',
        Nerd::CircleFilled => '\u{ea71}',
        Nerd::CircleHalf => '\u{f1395}',
        Nerd::CircleHalfFull => '\u{f1396}',
        Nerd::CircleLargeFilled => '\u{ebb4}',
        Nerd::CircleLargeOutline => '\u{ebb5}',
        Nerd::CircleMedium => '\u{f09de}',
        Nerd::CircleMultiple => '\u{f0b38}',
        Nerd::CircleMultipleOutline => '\u{f0695}',
        Nerd::CircleOffOutline => '\u{f10d3}',
        Nerd::CircleOne => '\u{f4aa}',
        Nerd::CircleOpacity => '\u{f1853}',
        Nerd::CircleOutline => '\u{eabc}',
        Nerd::CircleSlash => '\u{eabd}',
        Nerd::CircleSlashOne => '\u{f468}',
        Nerd::CircleSliceEight => '\u{f0aa5}',
        Nerd::CircleSliceFive => '\u{f0aa2}',
        Nerd::CircleSliceFour => '\u{f0aa1}',
        Nerd::CircleSliceOne => '\u{f0a9e}',
        Nerd::CircleSliceSeven => '\u{f0aa4}',
        Nerd::CircleSliceSix => '\u{f0aa3}',
        Nerd::CircleSliceThree => '\u{f0aa0}',
        Nerd::CircleSliceTwo => '\u{f0a9f}',
        Nerd::CircleSmall => '\u{f09df}',
        Nerd::CircuitBoard => '\u{eabe}',
        Nerd::CircularSaw => '\u{f0e22}',
        Nerd::City => '\u{f0146}',
        Nerd::CityVariant => '\u{f0a36}',
        Nerd::CityVariantOutline => '\u{f0a37}',
        Nerd::CleanCode => '\u{e000}',
        Nerd::ClearAll => '\u{eabf}',
        Nerd::Clipboard => '\u{f0147}',
        Nerd::ClipboardAccount => '\u{f0148}',
        Nerd::ClipboardAccountOutline => '\u{f0c55}',
        Nerd::ClipboardAlert => '\u{f0149}',
        Nerd::ClipboardAlertOutline => '\u{f0cf7}',
        Nerd::ClipboardArrowDown => '\u{f014a}',
        Nerd::ClipboardArrowDownOutline => '\u{f0c56}',
        Nerd::ClipboardArrowLeft => '\u{f014b}',
        Nerd::ClipboardArrowLeftOutline => '\u{f0cf8}',
        Nerd::ClipboardArrowRight => '\u{f0cf9}',
        Nerd::ClipboardArrowRightOutline => '\u{f0cfa}',
        Nerd::ClipboardArrowUp => '\u{f0c57}',
        Nerd::ClipboardArrowUpOutline => '\u{f0c58}',
        Nerd::ClipboardCheck => '\u{f014e}',
        Nerd::ClipboardCheckMultiple => '\u{f1263}',
        Nerd::ClipboardCheckMultipleOutline => '\u{f1264}',
        Nerd::ClipboardCheckOutline => '\u{f08a8}',
        Nerd::ClipboardClock => '\u{f16e2}',
        Nerd::ClipboardClockOutline => '\u{f16e3}',
        Nerd::ClipboardEdit => '\u{f14e5}',
        Nerd::ClipboardEditOutline => '\u{f14e6}',
        Nerd::ClipboardFile => '\u{f1265}',
        Nerd::ClipboardFileOutline => '\u{f1266}',
        Nerd::ClipboardFlow => '\u{f06c8}',
        Nerd::ClipboardFlowOutline => '\u{f1117}',
        Nerd::ClipboardList => '\u{f10d4}',
        Nerd::ClipboardListOutline => '\u{f10d5}',
        Nerd::ClipboardMinus => '\u{f1618}',
        Nerd::ClipboardMinusOutline => '\u{f1619}',
        Nerd::ClipboardMultiple => '\u{f1267}',
        Nerd::ClipboardMultipleOutline => '\u{f1268}',
        Nerd::ClipboardOff => '\u{f161a}',
        Nerd::ClipboardOffOutline => '\u{f161b}',
        Nerd::ClipboardOutline => '\u{f014c}',
        Nerd::ClipboardPlay => '\u{f0c59}',
        Nerd::ClipboardPlayMultiple => '\u{f1269}',
        Nerd::ClipboardPlayMultipleOutline => '\u{f126a}',
        Nerd::ClipboardPlayOutline => '\u{f0c5a}',
        Nerd::ClipboardPlus => '\u{f0751}',
        Nerd::ClipboardPlusOutline => '\u{f131f}',
        Nerd::ClipboardPulse => '\u{f085d}',
        Nerd::ClipboardPulseOutline => '\u{f085e}',
        Nerd::ClipboardRemove => '\u{f161c}',
        Nerd::ClipboardRemoveOutline => '\u{f161d}',
        Nerd::ClipboardSearch => '\u{f161e}',
        Nerd::ClipboardSearchOutline => '\u{f161f}',
        Nerd::ClipboardText => '\u{f014d}',
        Nerd::ClipboardTextClock => '\u{f18f9}',
        Nerd::ClipboardTextClockOutline => '\u{f18fa}',
        Nerd::ClipboardTextMultiple => '\u{f126b}',
        Nerd::ClipboardTextMultipleOutline => '\u{f126c}',
        Nerd::ClipboardTextOff => '\u{f1620}',
        Nerd::ClipboardTextOffOutline => '\u{f1621}',
        Nerd::ClipboardTextOutline => '\u{f0a38}',
        Nerd::ClipboardTextPlay => '\u{f0c5b}',
        Nerd::ClipboardTextPlayOutline => '\u{f0c5c}',
        Nerd::ClipboardTextSearch => '\u{f1622}',
        Nerd::ClipboardTextSearchOutline => '\u{f1623}',
        Nerd::Clippy => '\u{eac0}',
        Nerd::ClippyOne => '\u{f014f}',
        Nerd::Clock => '\u{f43a}',
        Nerd::ClockAlert => '\u{f0955}',
        Nerd::ClockAlertOutline => '\u{f05ce}',
        Nerd::ClockCheck => '\u{f0fa8}',
        Nerd::ClockCheckOutline => '\u{f0fa9}',
        Nerd::ClockDigital => '\u{f0e97}',
        Nerd::ClockEdit => '\u{f19ba}',
        Nerd::ClockEditOutline => '\u{f19bb}',
        Nerd::ClockEnd => '\u{f0151}',
        Nerd::ClockFast => '\u{f0152}',
        Nerd::ClockFill => '\u{f4ab}',
        Nerd::ClockIn => '\u{f0153}',
        Nerd::ClockMinus => '\u{f1863}',
        Nerd::ClockMinusOutline => '\u{f1864}',
        Nerd::ClockOne => '\u{f0954}',
        Nerd::ClockOut => '\u{f0154}',
        Nerd::ClockOutline => '\u{f0150}',
        Nerd::ClockPlus => '\u{f1861}',
        Nerd::ClockPlusOutline => '\u{f1862}',
        Nerd::ClockRemove => '\u{f1865}',
        Nerd::ClockRemoveOutline => '\u{f1866}',
        Nerd::ClockStart => '\u{f0155}',
        Nerd::ClockTimeEight => '\u{f1446}',
        Nerd::ClockTimeEightOutline => '\u{f1452}',
        Nerd::ClockTimeEleven => '\u{f1449}',
        Nerd::ClockTimeElevenOutline => '\u{f1455}',
        Nerd::ClockTimeFive => '\u{f1443}',
        Nerd::ClockTimeFiveOutline => '\u{f144f}',
        Nerd::ClockTimeFour => '\u{f1442}',
        Nerd::ClockTimeFourOutline => '\u{f144e}',
        Nerd::ClockTimeNine => '\u{f1447}',
        Nerd::ClockTimeNineOutline => '\u{f1453}',
        Nerd::ClockTimeOne => '\u{f143f}',
        Nerd::ClockTimeOneOutline => '\u{f144b}',
        Nerd::ClockTimeSeven => '\u{f1445}',
        Nerd::ClockTimeSevenOutline => '\u{f1451}',
        Nerd::ClockTimeSix => '\u{f1444}',
        Nerd::ClockTimeSixOutline => '\u{f1450}',
        Nerd::ClockTimeTen => '\u{f1448}',
        Nerd::ClockTimeTenOutline => '\u{f1454}',
        Nerd::ClockTimeThree => '\u{f1441}',
        Nerd::ClockTimeThreeOutline => '\u{f144d}',
        Nerd::ClockTimeTwelve => '\u{f144a}',
        Nerd::ClockTimeTwelveOutline => '\u{f1456}',
        Nerd::ClockTimeTwo => '\u{f1440}',
        Nerd::ClockTimeTwoOutline => '\u{f144c}',
        Nerd::Close => '\u{ea76}',
        Nerd::CloseAll => '\u{eac1}',
        Nerd::CloseBox => '\u{f0157}',
        Nerd::CloseBoxMultiple => '\u{f0c5d}',
        Nerd::CloseBoxMultipleOutline => '\u{f0c5e}',
        Nerd::CloseBoxOutline => '\u{f0158}',
        Nerd::CloseCircle => '\u{f0159}',
        Nerd::CloseCircleMultiple => '\u{f062a}',
        Nerd::CloseCircleMultipleOutline => '\u{f0883}',
        Nerd::CloseCircleOutline => '\u{f015a}',
        Nerd::CloseNetwork => '\u{f015b}',
        Nerd::CloseNetworkOutline => '\u{f0c5f}',
        Nerd::CloseOctagon => '\u{f015c}',
        Nerd::CloseOctagonOutline => '\u{f015d}',
        Nerd::CloseOne => '\u{f0156}',
        Nerd::CloseOutline => '\u{f06c9}',
        Nerd::CloseThick => '\u{f1398}',
        Nerd::ClosedCaption => '\u{f015e}',
        Nerd::ClosedCaptionOutline => '\u{f0dbd}',
        Nerd::Cloud => '\u{e268}',
        Nerd::CloudAlert => '\u{f09e0}',
        Nerd::CloudBraces => '\u{f07b5}',
        Nerd::CloudCheck => '\u{f0160}',
        Nerd::CloudCheckOutline => '\u{f12cc}',
        Nerd::CloudCircle => '\u{f0161}',
        Nerd::CloudDownload => '\u{f0ed}',
        Nerd::CloudDownloadOne => '\u{f0162}',
        Nerd::CloudDownloadOutline => '\u{f0b7d}',
        Nerd::CloudFour => '\u{f015f}',
        Nerd::CloudLock => '\u{f11f1}',
        Nerd::CloudLockOutline => '\u{f11f2}',
        Nerd::CloudOffOutline => '\u{f0164}',
        Nerd::CloudOffline => '\u{f4ad}',
        Nerd::CloudOne => '\u{ebaa}',
        Nerd::CloudOutline => '\u{f0163}',
        Nerd::CloudPercent => '\u{f1a35}',
        Nerd::CloudPercentOutline => '\u{f1a36}',
        Nerd::CloudPrint => '\u{f0165}',
        Nerd::CloudPrintOutline => '\u{f0166}',
        Nerd::CloudQuestion => '\u{f0a39}',
        Nerd::CloudRefresh => '\u{f052a}',
        Nerd::CloudSearch => '\u{f0956}',
        Nerd::CloudSearchOutline => '\u{f0957}',
        Nerd::CloudSync => '\u{f063f}',
        Nerd::CloudSyncOutline => '\u{f12d6}',
        Nerd::CloudTags => '\u{f07b6}',
        Nerd::CloudThree => '\u{f4ac}',
        Nerd::CloudTwo => '\u{f0c2}',
        Nerd::CloudUpload => '\u{f0ee}',
        Nerd::CloudUploadOne => '\u{f0167}',
        Nerd::CloudUploadOutline => '\u{f0b7e}',
        Nerd::Clover => '\u{f0816}',
        Nerd::CoachLamp => '\u{f1020}',
        Nerd::CoachLampVariant => '\u{f1a37}',
        Nerd::CoatRack => '\u{f109e}',
        Nerd::Cockroach => '\u{e269}',
        Nerd::Code => '\u{eac4}',
        Nerd::CodeArray => '\u{f0168}',
        Nerd::CodeBraces => '\u{f0169}',
        Nerd::CodeBracesBox => '\u{f10d6}',
        Nerd::CodeBrackets => '\u{f016a}',
        Nerd::CodeEqual => '\u{f016b}',
        Nerd::CodeFork => '\u{f126}',
        Nerd::CodeGreaterThan => '\u{f016c}',
        Nerd::CodeGreaterThanOrEqual => '\u{f016d}',
        Nerd::CodeJson => '\u{f0626}',
        Nerd::CodeLessThan => '\u{f016e}',
        Nerd::CodeLessThanOrEqual => '\u{f016f}',
        Nerd::CodeNotEqual => '\u{f0170}',
        Nerd::CodeNotEqualVariant => '\u{f0171}',
        Nerd::CodeOfConduct => '\u{f4ae}',
        Nerd::CodeOne => '\u{f121}',
        Nerd::CodeParentheses => '\u{f0172}',
        Nerd::CodeParenthesesBox => '\u{f10d7}',
        Nerd::CodeReview => '\u{f4af}',
        Nerd::CodeSquare => '\u{f4b0}',
        Nerd::CodeString => '\u{f0173}',
        Nerd::CodeTags => '\u{f0174}',
        Nerd::CodeTagsCheck => '\u{f0694}',
        Nerd::CodeTwo => '\u{f44f}',
        Nerd::Codeberg => '\u{f330}',
        Nerd::Codepen => '\u{f0175}',
        Nerd::Codescan => '\u{f4b1}',
        Nerd::CodescanCheckmark => '\u{f4b2}',
        Nerd::Codespaces => '\u{f4b3}',
        Nerd::CoffeBeans => '\u{e26a}',
        Nerd::Coffee => '\u{f0f4}',
        Nerd::CoffeeMaker => '\u{f109f}',
        Nerd::CoffeeMakerCheck => '\u{f1931}',
        Nerd::CoffeeMakerCheckOutline => '\u{f1932}',
        Nerd::CoffeeMakerOutline => '\u{f181b}',
        Nerd::CoffeeOff => '\u{f0faa}',
        Nerd::CoffeeOffOutline => '\u{f0fab}',
        Nerd::CoffeeOne => '\u{f0176}',
        Nerd::CoffeeOutline => '\u{f06ca}',
        Nerd::CoffeeToGo => '\u{f0177}',
        Nerd::CoffeeToGoOutline => '\u{f130e}',
        Nerd::Coffin => '\u{f0b7f}',
        Nerd::Cog => '\u{f013}',
        Nerd::CogBox => '\u{f0494}',
        Nerd::CogClockwise => '\u{f11dd}',
        Nerd::CogCounterclockwise => '\u{f11de}',
        Nerd::CogOff => '\u{f13ce}',
        Nerd::CogOffOutline => '\u{f13cf}',
        Nerd::CogOne => '\u{f0493}',
        Nerd::CogOutline => '\u{f08bb}',
        Nerd::CogPause => '\u{f1933}',
        Nerd::CogPauseOutline => '\u{f1934}',
        Nerd::CogPlay => '\u{f1935}',
        Nerd::CogPlayOutline => '\u{f1936}',
        Nerd::CogRefresh => '\u{f145e}',
        Nerd::CogRefreshOutline => '\u{f145f}',
        Nerd::CogStop => '\u{f1937}',
        Nerd::CogStopOutline => '\u{f1938}',
        Nerd::CogSync => '\u{f1460}',
        Nerd::CogSyncOutline => '\u{f1461}',
        Nerd::CogTransfer => '\u{f105b}',
        Nerd::CogTransferOutline => '\u{f105c}',
        Nerd::Cogs => '\u{f085}',
        Nerd::CogsOne => '\u{f08d6}',
        Nerd::Coins => '\u{e26b}',
        Nerd::Collage => '\u{f0640}',
        Nerd::Collapse => '\u{f150}',
        Nerd::CollapseAll => '\u{eac5}',
        Nerd::CollapseAllOne => '\u{f0aa6}',
        Nerd::CollapseAllOutline => '\u{f0aa7}',
        Nerd::CollapseAlt => '\u{f117}',
        Nerd::CollapseTop => '\u{f151}',
        Nerd::ColorHelper => '\u{f0179}',
        Nerd::ColorMode => '\u{eac6}',
        Nerd::Columns => '\u{f0db}',
        Nerd::ColumnsOne => '\u{f4b4}',
        Nerd::Comb => '\u{e26c}',
        Nerd::Combine => '\u{ebb6}',
        Nerd::Comet => '\u{e26d}',
        Nerd::Comma => '\u{f0e23}',
        Nerd::CommaBox => '\u{f0e2b}',
        Nerd::CommaBoxOutline => '\u{f0e24}',
        Nerd::CommaCircle => '\u{f0e25}',
        Nerd::CommaCircleOutline => '\u{f0e26}',
        Nerd::CommandPalette => '\u{f4b5}',
        Nerd::Comment => '\u{ea6b}',
        Nerd::CommentAccount => '\u{f017b}',
        Nerd::CommentAccountOutline => '\u{f017c}',
        Nerd::CommentAlert => '\u{f017d}',
        Nerd::CommentAlertOutline => '\u{f017e}',
        Nerd::CommentAlt => '\u{f0e5}',
        Nerd::CommentArrowLeft => '\u{f09e1}',
        Nerd::CommentArrowLeftOutline => '\u{f09e2}',
        Nerd::CommentArrowRight => '\u{f09e3}',
        Nerd::CommentArrowRightOutline => '\u{f09e4}',
        Nerd::CommentBookmark => '\u{f15ae}',
        Nerd::CommentBookmarkOutline => '\u{f15af}',
        Nerd::CommentCheck => '\u{f017f}',
        Nerd::CommentCheckOutline => '\u{f0180}',
        Nerd::CommentDiscussion => '\u{eac7}',
        Nerd::CommentDiscussionOne => '\u{f442}',
        Nerd::CommentEdit => '\u{f11bf}',
        Nerd::CommentEditOutline => '\u{f12c4}',
        Nerd::CommentEye => '\u{f0a3a}',
        Nerd::CommentEyeOutline => '\u{f0a3b}',
        Nerd::CommentFlash => '\u{f15b0}',
        Nerd::CommentFlashOutline => '\u{f15b1}',
        Nerd::CommentMinus => '\u{f15df}',
        Nerd::CommentMinusOutline => '\u{f15e0}',
        Nerd::CommentMultiple => '\u{f085f}',
        Nerd::CommentMultipleOutline => '\u{f0181}',
        Nerd::CommentOff => '\u{f15e1}',
        Nerd::CommentOffOutline => '\u{f15e2}',
        Nerd::CommentOne => '\u{f075}',
        Nerd::CommentOutline => '\u{f0182}',
        Nerd::CommentPlus => '\u{f09e5}',
        Nerd::CommentPlusOutline => '\u{f0183}',
        Nerd::CommentProcessing => '\u{f0184}',
        Nerd::CommentProcessingOutline => '\u{f0185}',
        Nerd::CommentQuestion => '\u{f0817}',
        Nerd::CommentQuestionOutline => '\u{f0186}',
        Nerd::CommentQuote => '\u{f1021}',
        Nerd::CommentQuoteOutline => '\u{f1022}',
        Nerd::CommentRemove => '\u{f05de}',
        Nerd::CommentRemoveOutline => '\u{f0187}',
        Nerd::CommentSearch => '\u{f0a3c}',
        Nerd::CommentSearchOutline => '\u{f0a3d}',
        Nerd::CommentText => '\u{f0188}',
        Nerd::CommentTextMultiple => '\u{f0860}',
        Nerd::CommentTextMultipleOutline => '\u{f0861}',
        Nerd::CommentTextOutline => '\u{f0189}',
        Nerd::CommentThree => '\u{f017a}',
        Nerd::CommentTwo => '\u{f41f}',
        Nerd::Comments => '\u{f086}',
        Nerd::CommentsAlt => '\u{f0e6}',
        Nerd::Commit => '\u{f4b6}',
        Nerd::Compare => '\u{f018a}',
        Nerd::CompareHorizontal => '\u{f1492}',
        Nerd::CompareRemove => '\u{f18b3}',
        Nerd::CompareVertical => '\u{f1493}',
        Nerd::Compass => '\u{ebd5}',
        Nerd::CompassActive => '\u{ebd7}',
        Nerd::CompassDot => '\u{ebd6}',
        Nerd::CompassOff => '\u{f0b80}',
        Nerd::CompassOffOutline => '\u{f0b81}',
        Nerd::CompassOne => '\u{f14e}',
        Nerd::CompassOutline => '\u{f018c}',
        Nerd::CompassRose => '\u{f1382}',
        Nerd::CompassTwo => '\u{f018b}',
        Nerd::Compost => '\u{f1a38}',
        Nerd::Cone => '\u{f194c}',
        Nerd::ConeOff => '\u{f194d}',
        Nerd::Connection => '\u{f1616}',
        Nerd::Console => '\u{f018d}',
        Nerd::ConsoleLine => '\u{f07b7}',
        Nerd::ConsoleNetwork => '\u{f08a9}',
        Nerd::ConsoleNetworkOutline => '\u{f0c60}',
        Nerd::Consolidate => '\u{f10d8}',
        Nerd::ContactlessPayment => '\u{f0d6a}',
        Nerd::ContactlessPaymentCircle => '\u{f0321}',
        Nerd::ContactlessPaymentCircleOutline => '\u{f0408}',
        Nerd::Contacts => '\u{f06cb}',
        Nerd::ContactsOutline => '\u{f05b8}',
        Nerd::Contain => '\u{f0a3e}',
        Nerd::ContainEnd => '\u{f0a3f}',
        Nerd::ContainStart => '\u{f0a40}',
        Nerd::Container => '\u{f4b7}',
        Nerd::ContentCopy => '\u{f018f}',
        Nerd::ContentCut => '\u{f0190}',
        Nerd::ContentDuplicate => '\u{f0191}',
        Nerd::ContentPaste => '\u{f0192}',
        Nerd::ContentSave => '\u{f0193}',
        Nerd::ContentSaveAlert => '\u{f0f42}',
        Nerd::ContentSaveAlertOutline => '\u{f0f43}',
        Nerd::ContentSaveAll => '\u{f0194}',
        Nerd::ContentSaveAllOutline => '\u{f0f44}',
        Nerd::ContentSaveCheck => '\u{f18ea}',
        Nerd::ContentSaveCheckOutline => '\u{f18eb}',
        Nerd::ContentSaveCog => '\u{f145b}',
        Nerd::ContentSaveCogOutline => '\u{f145c}',
        Nerd::ContentSaveEdit => '\u{f0cfb}',
        Nerd::ContentSaveEditOutline => '\u{f0cfc}',
        Nerd::ContentSaveMove => '\u{f0e27}',
        Nerd::ContentSaveMoveOutline => '\u{f0e28}',
        Nerd::ContentSaveOff => '\u{f1643}',
        Nerd::ContentSaveOffOutline => '\u{f1644}',
        Nerd::ContentSaveOutline => '\u{f0818}',
        Nerd::ContentSaveSettings => '\u{f061b}',
        Nerd::ContentSaveSettingsOutline => '\u{f0b2e}',
        Nerd::Contrast => '\u{f0195}',
        Nerd::ContrastBox => '\u{f0196}',
        Nerd::ContrastCircle => '\u{f0197}',
        Nerd::ControllerClassic => '\u{f0b82}',
        Nerd::ControllerClassicOutline => '\u{f0b83}',
        Nerd::Cookie => '\u{f0198}',
        Nerd::CookieAlert => '\u{f16d0}',
        Nerd::CookieAlertOutline => '\u{f16d1}',
        Nerd::CookieCheck => '\u{f16d2}',
        Nerd::CookieCheckOutline => '\u{f16d3}',
        Nerd::CookieClock => '\u{f16e4}',
        Nerd::CookieClockOutline => '\u{f16e5}',
        Nerd::CookieCog => '\u{f16d4}',
        Nerd::CookieCogOutline => '\u{f16d5}',
        Nerd::CookieEdit => '\u{f16e6}',
        Nerd::CookieEditOutline => '\u{f16e7}',
        Nerd::CookieLock => '\u{f16e8}',
        Nerd::CookieLockOutline => '\u{f16e9}',
        Nerd::CookieMinus => '\u{f16da}',
        Nerd::CookieMinusOutline => '\u{f16db}',
        Nerd::CookieOff => '\u{f16ea}',
        Nerd::CookieOffOutline => '\u{f16eb}',
        Nerd::CookieOutline => '\u{f16de}',
        Nerd::CookiePlus => '\u{f16d6}',
        Nerd::CookiePlusOutline => '\u{f16d7}',
        Nerd::CookieRefresh => '\u{f16ec}',
        Nerd::CookieRefreshOutline => '\u{f16ed}',
        Nerd::CookieRemove => '\u{f16d8}',
        Nerd::CookieRemoveOutline => '\u{f16d9}',
        Nerd::CookieSettings => '\u{f16dc}',
        Nerd::CookieSettingsOutline => '\u{f16dd}',
        Nerd::CoolantTemperature => '\u{f03c8}',
        Nerd::Copilot => '\u{f4b8}',
        Nerd::CopilotError => '\u{f4b9}',
        Nerd::CopilotWarning => '\u{f4ba}',
        Nerd::Copy => '\u{ebcc}',
        Nerd::CopyOne => '\u{f0c5}',
        Nerd::CopyTwo => '\u{f4bb}',
        Nerd::Copyleft => '\u{f1939}',
        Nerd::Copyright => '\u{f05e6}',
        Nerd::Cordova => '\u{f0958}',
        Nerd::Coreos => '\u{f305}',
        Nerd::Corn => '\u{f07b8}',
        Nerd::CornOff => '\u{f13ef}',
        Nerd::CosineWave => '\u{f1479}',
        Nerd::Counter => '\u{f0199}',
        Nerd::Countertop => '\u{f181c}',
        Nerd::CountertopOutline => '\u{f181d}',
        Nerd::Cow => '\u{f019a}',
        Nerd::CowOff => '\u{f18fc}',
        Nerd::Cpu => '\u{f4bc}',
        Nerd::CpuSixfourBit => '\u{f0ee0}',
        Nerd::CpuThreetwoBit => '\u{f0edf}',
        Nerd::Cradle => '\u{f198b}',
        Nerd::CradleOutline => '\u{f1991}',
        Nerd::Crane => '\u{f0862}',
        Nerd::Creation => '\u{f0674}',
        Nerd::CreativeCommons => '\u{f0d6b}',
        Nerd::CreditCard => '\u{f09d}',
        Nerd::CreditCardCheck => '\u{f13d0}',
        Nerd::CreditCardCheckOutline => '\u{f13d1}',
        Nerd::CreditCardChip => '\u{f190f}',
        Nerd::CreditCardChipOutline => '\u{f1910}',
        Nerd::CreditCardClock => '\u{f0ee1}',
        Nerd::CreditCardClockOutline => '\u{f0ee2}',
        Nerd::CreditCardEdit => '\u{f17d7}',
        Nerd::CreditCardEditOutline => '\u{f17d8}',
        Nerd::CreditCardFast => '\u{f1911}',
        Nerd::CreditCardFastOutline => '\u{f1912}',
        Nerd::CreditCardLock => '\u{f18e7}',
        Nerd::CreditCardLockOutline => '\u{f18e8}',
        Nerd::CreditCardMarker => '\u{f06a8}',
        Nerd::CreditCardMarkerOutline => '\u{f0dbe}',
        Nerd::CreditCardMinus => '\u{f0fac}',
        Nerd::CreditCardMinusOutline => '\u{f0fad}',
        Nerd::CreditCardMultiple => '\u{f0ff0}',
        Nerd::CreditCardMultipleOutline => '\u{f019c}',
        Nerd::CreditCardOff => '\u{f0ff1}',
        Nerd::CreditCardOffOutline => '\u{f05e4}',
        Nerd::CreditCardOne => '\u{f439}',
        Nerd::CreditCardOutline => '\u{f019b}',
        Nerd::CreditCardPlus => '\u{f0ff2}',
        Nerd::CreditCardPlusOutline => '\u{f0676}',
        Nerd::CreditCardRefresh => '\u{f1645}',
        Nerd::CreditCardRefreshOutline => '\u{f1646}',
        Nerd::CreditCardRefund => '\u{f0ff3}',
        Nerd::CreditCardRefundOutline => '\u{f0aa8}',
        Nerd::CreditCardRemove => '\u{f0fae}',
        Nerd::CreditCardRemoveOutline => '\u{f0faf}',
        Nerd::CreditCardScan => '\u{f0ff4}',
        Nerd::CreditCardScanOutline => '\u{f019d}',
        Nerd::CreditCardSearch => '\u{f1647}',
        Nerd::CreditCardSearchOutline => '\u{f1648}',
        Nerd::CreditCardSettings => '\u{f0ff5}',
        Nerd::CreditCardSettingsOutline => '\u{f08d7}',
        Nerd::CreditCardSync => '\u{f1649}',
        Nerd::CreditCardSyncOutline => '\u{f164a}',
        Nerd::CreditCardTwo => '\u{f0fef}',
        Nerd::CreditCardWireless => '\u{f0802}',
        Nerd::CreditCardWirelessOff => '\u{f057a}',
        Nerd::CreditCardWirelessOffOutline => '\u{f057b}',
        Nerd::CreditCardWirelessOutline => '\u{f0d6c}',
        Nerd::Cricket => '\u{f0d6d}',
        Nerd::Crop => '\u{f125}',
        Nerd::CropFree => '\u{f019f}',
        Nerd::CropLandscape => '\u{f01a0}',
        Nerd::CropOne => '\u{f019e}',
        Nerd::CropPortrait => '\u{f01a1}',
        Nerd::CropRotate => '\u{f0696}',
        Nerd::CropSquare => '\u{f01a2}',
        Nerd::Cross => '\u{f0953}',
        Nerd::CrossBolnisi => '\u{f0ced}',
        Nerd::CrossCeltic => '\u{f0cf5}',
        Nerd::CrossOutline => '\u{f0cf6}',
        Nerd::CrossReference => '\u{f4bd}',
        Nerd::Crosshairs => '\u{f01a3}',
        Nerd::CrosshairsGps => '\u{f01a4}',
        Nerd::CrosshairsOff => '\u{f0f45}',
        Nerd::CrosshairsQuestion => '\u{f1136}',
        Nerd::Crowd => '\u{f1975}',
        Nerd::Crown => '\u{e26e}',
        Nerd::CrownCircle => '\u{f17dc}',
        Nerd::CrownCircleOutline => '\u{f17dd}',
        Nerd::CrownOne => '\u{f01a5}',
        Nerd::CrownOutline => '\u{f11d0}',
        Nerd::Cryengine => '\u{f0959}',
        Nerd::CrystalBall => '\u{f0b2f}',
        Nerd::CrystalLinux => '\u{f348}',
        Nerd::Cssthree => '\u{f13c}',
        Nerd::Cube => '\u{f01a6}',
        Nerd::CubeOff => '\u{f141c}',
        Nerd::CubeOffOutline => '\u{f141d}',
        Nerd::CubeOutline => '\u{f01a7}',
        Nerd::CubeScan => '\u{f0b84}',
        Nerd::CubeSend => '\u{f01a8}',
        Nerd::CubeUnfolded => '\u{f01a9}',
        Nerd::Cup => '\u{f01aa}',
        Nerd::CupCoffe => '\u{e26f}',
        Nerd::CupOff => '\u{f05e5}',
        Nerd::CupOffOutline => '\u{f137d}',
        Nerd::CupOutline => '\u{f130f}',
        Nerd::CupWater => '\u{f01ab}',
        Nerd::Cupboard => '\u{f0f46}',
        Nerd::CupboardOutline => '\u{f0f47}',
        Nerd::Cupcake => '\u{f095a}',
        Nerd::Curling => '\u{f0863}',
        Nerd::CurrencyBdt => '\u{f0864}',
        Nerd::CurrencyBrl => '\u{f0b85}',
        Nerd::CurrencyBtc => '\u{f01ac}',
        Nerd::CurrencyCny => '\u{f07ba}',
        Nerd::CurrencyEth => '\u{f07bb}',
        Nerd::CurrencyEur => '\u{f01ad}',
        Nerd::CurrencyEurOff => '\u{f1315}',
        Nerd::CurrencyFra => '\u{f1a39}',
        Nerd::CurrencyGbp => '\u{f01ae}',
        Nerd::CurrencyIls => '\u{f0c61}',
        Nerd::CurrencyInr => '\u{f01af}',
        Nerd::CurrencyJpy => '\u{f07bc}',
        Nerd::CurrencyKrw => '\u{f07bd}',
        Nerd::CurrencyKzt => '\u{f0865}',
        Nerd::CurrencyMnt => '\u{f1512}',
        Nerd::CurrencyNgn => '\u{f01b0}',
        Nerd::CurrencyPhp => '\u{f09e6}',
        Nerd::CurrencyRial => '\u{f0e9c}',
        Nerd::CurrencyRub => '\u{f01b1}',
        Nerd::CurrencyRupee => '\u{f1976}',
        Nerd::CurrencySign => '\u{f07be}',
        Nerd::CurrencyTry => '\u{f01b2}',
        Nerd::CurrencyTwd => '\u{f07bf}',
        Nerd::CurrencyUsd => '\u{f01c1}',
        Nerd::CurrencyUsdOff => '\u{f067a}',
        Nerd::CurrentAc => '\u{f1480}',
        Nerd::CurrentDc => '\u{f095c}',
        Nerd::CursorDefault => '\u{f01c0}',
        Nerd::CursorDefaultClick => '\u{f0cfd}',
        Nerd::CursorDefaultClickOutline => '\u{f0cfe}',
        Nerd::CursorDefaultGesture => '\u{f1127}',
        Nerd::CursorDefaultGestureOutline => '\u{f1128}',
        Nerd::CursorDefaultOutline => '\u{f01bf}',
        Nerd::CursorMove => '\u{f01be}',
        Nerd::CursorPointer => '\u{f01bd}',
        Nerd::CursorText => '\u{f05e7}',
        Nerd::Curtains => '\u{f1846}',
        Nerd::CurtainsClosed => '\u{f1847}',
        Nerd::Cut => '\u{f0c4}',
        Nerd::Cylinder => '\u{f194e}',
        Nerd::CylinderOff => '\u{f194f}',
        Nerd::DanceBallroom => '\u{f15fb}',
        Nerd::DancePole => '\u{f1578}',
        Nerd::Dash => '\u{eacc}',
        Nerd::DashOne => '\u{f48b}',
        Nerd::Dashboard => '\u{eacd}',
        Nerd::DashboardOne => '\u{f0e4}',
        Nerd::DataMatrix => '\u{f153c}',
        Nerd::DataMatrixEdit => '\u{f153d}',
        Nerd::DataMatrixMinus => '\u{f153e}',
        Nerd::DataMatrixPlus => '\u{f153f}',
        Nerd::DataMatrixRemove => '\u{f1540}',
        Nerd::DataMatrixScan => '\u{f1541}',
        Nerd::Database => '\u{eace}',
        Nerd::DatabaseAlert => '\u{f163a}',
        Nerd::DatabaseAlertOutline => '\u{f1624}',
        Nerd::DatabaseArrowDown => '\u{f163b}',
        Nerd::DatabaseArrowDownOutline => '\u{f1625}',
        Nerd::DatabaseArrowLeft => '\u{f163c}',
        Nerd::DatabaseArrowLeftOutline => '\u{f1626}',
        Nerd::DatabaseArrowRight => '\u{f163d}',
        Nerd::DatabaseArrowRightOutline => '\u{f1627}',
        Nerd::DatabaseArrowUp => '\u{f163e}',
        Nerd::DatabaseArrowUpOutline => '\u{f1628}',
        Nerd::DatabaseCheck => '\u{f0aa9}',
        Nerd::DatabaseCheckOutline => '\u{f1629}',
        Nerd::DatabaseClock => '\u{f163f}',
        Nerd::DatabaseClockOutline => '\u{f162a}',
        Nerd::DatabaseCog => '\u{f164b}',
        Nerd::DatabaseCogOutline => '\u{f164c}',
        Nerd::DatabaseEdit => '\u{f0b86}',
        Nerd::DatabaseEditOutline => '\u{f162b}',
        Nerd::DatabaseExport => '\u{f095e}',
        Nerd::DatabaseExportOutline => '\u{f162c}',
        Nerd::DatabaseEye => '\u{f191f}',
        Nerd::DatabaseEyeOff => '\u{f1920}',
        Nerd::DatabaseEyeOffOutline => '\u{f1921}',
        Nerd::DatabaseEyeOutline => '\u{f1922}',
        Nerd::DatabaseImport => '\u{f095d}',
        Nerd::DatabaseImportOutline => '\u{f162d}',
        Nerd::DatabaseLock => '\u{f0aaa}',
        Nerd::DatabaseLockOutline => '\u{f162e}',
        Nerd::DatabaseMarker => '\u{f12f6}',
        Nerd::DatabaseMarkerOutline => '\u{f162f}',
        Nerd::DatabaseMinus => '\u{f01bb}',
        Nerd::DatabaseMinusOutline => '\u{f1630}',
        Nerd::DatabaseOff => '\u{f1640}',
        Nerd::DatabaseOffOutline => '\u{f1631}',
        Nerd::DatabaseOne => '\u{f472}',
        Nerd::DatabaseOutline => '\u{f1632}',
        Nerd::DatabasePlus => '\u{f01ba}',
        Nerd::DatabasePlusOutline => '\u{f1633}',
        Nerd::DatabaseRefresh => '\u{f05c2}',
        Nerd::DatabaseRefreshOutline => '\u{f1634}',
        Nerd::DatabaseRemove => '\u{f0d00}',
        Nerd::DatabaseRemoveOutline => '\u{f1635}',
        Nerd::DatabaseSearch => '\u{f0866}',
        Nerd::DatabaseSearchOutline => '\u{f1636}',
        Nerd::DatabaseSettings => '\u{f0d01}',
        Nerd::DatabaseSettingsOutline => '\u{f1637}',
        Nerd::DatabaseSync => '\u{f0cff}',
        Nerd::DatabaseSyncOutline => '\u{f1638}',
        Nerd::DatabaseTwo => '\u{f01bc}',
        Nerd::DeathStar => '\u{f08d8}',
        Nerd::DeathStarVariant => '\u{f08d9}',
        Nerd::DeathlyHallows => '\u{f0b87}',
        Nerd::Debian => '\u{f08da}',
        Nerd::Debug => '\u{ead8}',
        Nerd::DebugAll => '\u{ebdc}',
        Nerd::DebugAlt => '\u{eb91}',
        Nerd::DebugAltSmall => '\u{eba8}',
        Nerd::DebugBreakpointConditional => '\u{eaa7}',
        Nerd::DebugBreakpointConditionalUnverified => '\u{eaa6}',
        Nerd::DebugBreakpointData => '\u{eaa9}',
        Nerd::DebugBreakpointDataUnverified => '\u{eaa8}',
        Nerd::DebugBreakpointFunction => '\u{eb88}',
        Nerd::DebugBreakpointFunctionUnverified => '\u{eb87}',
        Nerd::DebugBreakpointLog => '\u{eaab}',
        Nerd::DebugBreakpointLogUnverified => '\u{eaaa}',
        Nerd::DebugBreakpointUnsupported => '\u{eb8c}',
        Nerd::DebugConsole => '\u{eb9b}',
        Nerd::DebugContinue => '\u{eacf}',
        Nerd::DebugContinueSmall => '\u{ebe0}',
        Nerd::DebugCoverage => '\u{ebdd}',
        Nerd::DebugDisconnect => '\u{ead0}',
        Nerd::DebugLineByLine => '\u{ebd0}',
        Nerd::DebugPause => '\u{ead1}',
        Nerd::DebugRerun => '\u{ebc0}',
        Nerd::DebugRestart => '\u{ead2}',
        Nerd::DebugRestartFrame => '\u{eb90}',
        Nerd::DebugReverseContinue => '\u{eb8e}',
        Nerd::DebugStackframe => '\u{eb8b}',
        Nerd::DebugStackframeActive => '\u{eb89}',
        Nerd::DebugStackframeDot => '\u{eb8a}',
        Nerd::DebugStart => '\u{ead3}',
        Nerd::DebugStepBack => '\u{eb8f}',
        Nerd::DebugStepInto => '\u{ead4}',
        Nerd::DebugStepIntoOne => '\u{f01b9}',
        Nerd::DebugStepOut => '\u{ead5}',
        Nerd::DebugStepOutOne => '\u{f01b8}',
        Nerd::DebugStepOver => '\u{ead6}',
        Nerd::DebugStepOverOne => '\u{f01b7}',
        Nerd::DebugStop => '\u{ead7}',
        Nerd::Decagram => '\u{f076c}',
        Nerd::DecagramOutline => '\u{f076d}',
        Nerd::Decimal => '\u{f10a1}',
        Nerd::DecimalComma => '\u{f10a2}',
        Nerd::DecimalCommaDecrease => '\u{f10a3}',
        Nerd::DecimalCommaIncrease => '\u{f10a4}',
        Nerd::DecimalDecrease => '\u{f01b6}',
        Nerd::DecimalIncrease => '\u{f01b5}',
        Nerd::Deepin => '\u{f321}',
        Nerd::Delete => '\u{f01b4}',
        Nerd::DeleteAlert => '\u{f10a5}',
        Nerd::DeleteAlertOutline => '\u{f10a6}',
        Nerd::DeleteCircle => '\u{f0683}',
        Nerd::DeleteCircleOutline => '\u{f0b88}',
        Nerd::DeleteClock => '\u{f1556}',
        Nerd::DeleteClockOutline => '\u{f1557}',
        Nerd::DeleteEmpty => '\u{f06cc}',
        Nerd::DeleteEmptyOutline => '\u{f0e9d}',
        Nerd::DeleteForever => '\u{f05e8}',
        Nerd::DeleteForeverOutline => '\u{f0b89}',
        Nerd::DeleteOff => '\u{f10a7}',
        Nerd::DeleteOffOutline => '\u{f10a8}',
        Nerd::DeleteOutline => '\u{f09e7}',
        Nerd::DeleteRestore => '\u{f0819}',
        Nerd::DeleteSweep => '\u{f05e9}',
        Nerd::DeleteSweepOutline => '\u{f0c62}',
        Nerd::DeleteVariant => '\u{f01b3}',
        Nerd::Delta => '\u{f01c2}',
        Nerd::Dependabot => '\u{f4be}',
        Nerd::Desk => '\u{f1239}',
        Nerd::DeskLamp => '\u{f095f}',
        Nerd::Deskphone => '\u{f01c3}',
        Nerd::Desktop => '\u{f108}',
        Nerd::DesktopClassic => '\u{f07c0}',
        Nerd::DesktopDownload => '\u{ea78}',
        Nerd::DesktopDownloadOne => '\u{f498}',
        Nerd::DesktopMac => '\u{f01c4}',
        Nerd::DesktopMacDashboard => '\u{f09e8}',
        Nerd::DesktopTower => '\u{f01c5}',
        Nerd::DesktopTowerMonitor => '\u{f0aab}',
        Nerd::Details => '\u{f01c6}',
        Nerd::DevTo => '\u{f0d6e}',
        Nerd::DeveloperBoard => '\u{f0697}',
        Nerd::Deviantart => '\u{f01c7}',
        Nerd::DeviceCamera => '\u{eada}',
        Nerd::DeviceCameraOne => '\u{f446}',
        Nerd::DeviceCameraVideo => '\u{ead9}',
        Nerd::DeviceCameraVideoOne => '\u{f447}',
        Nerd::DeviceDesktop => '\u{f4a9}',
        Nerd::DeviceMobile => '\u{eadb}',
        Nerd::DeviceMobileOne => '\u{f42c}',
        Nerd::Devices => '\u{f0fb0}',
        Nerd::Devuan => '\u{f307}',
        Nerd::Dharmachakra => '\u{f094b}',
        Nerd::Diabetes => '\u{f1126}',
        Nerd::Dialpad => '\u{f061c}',
        Nerd::Diameter => '\u{f0c63}',
        Nerd::DiameterOutline => '\u{f0c64}',
        Nerd::DiameterVariant => '\u{f0c65}',
        Nerd::Diamond => '\u{f4bf}',
        Nerd::DiamondOne => '\u{f0b8a}',
        Nerd::DiamondOutline => '\u{f0b8b}',
        Nerd::DiamondStone => '\u{f01c8}',
        Nerd::Dice => '\u{e270}',
        Nerd::DiceDeight => '\u{f1152}',
        Nerd::DiceDeightOutline => '\u{f05ec}',
        Nerd::DiceDfour => '\u{f1150}',
        Nerd::DiceDfourOutline => '\u{f05eb}',
        Nerd::DiceDonetwo => '\u{f1154}',
        Nerd::DiceDonetwoOutline => '\u{f0867}',
        Nerd::DiceDonezero => '\u{f1153}',
        Nerd::DiceDonezeroOutline => '\u{f076f}',
        Nerd::DiceDsix => '\u{f1151}',
        Nerd::DiceDsixOutline => '\u{f05ed}',
        Nerd::DiceDtwozero => '\u{f1155}',
        Nerd::DiceDtwozeroOutline => '\u{f05ea}',
        Nerd::DiceFive => '\u{f01ce}',
        Nerd::DiceFiveOutline => '\u{f114e}',
        Nerd::DiceFour => '\u{f01cd}',
        Nerd::DiceFourOutline => '\u{f114d}',
        Nerd::DiceMultiple => '\u{f076e}',
        Nerd::DiceMultipleOutline => '\u{f1156}',
        Nerd::DiceOne => '\u{f01ca}',
        Nerd::DiceOneOutline => '\u{f114a}',
        Nerd::DiceSix => '\u{f01cf}',
        Nerd::DiceSixOutline => '\u{f114f}',
        Nerd::DiceThree => '\u{f01cc}',
        Nerd::DiceThreeOutline => '\u{f114c}',
        Nerd::DiceTwo => '\u{f01cb}',
        Nerd::DiceTwoOutline => '\u{f114b}',
        Nerd::Diff => '\u{eae1}',
        Nerd::DiffAdded => '\u{eadc}',
        Nerd::DiffAddedOne => '\u{f457}',
        Nerd::DiffIgnored => '\u{eadd}',
        Nerd::DiffIgnoredOne => '\u{f474}',
        Nerd::DiffModified => '\u{eade}',
        Nerd::DiffModifiedOne => '\u{f459}',
        Nerd::DiffOne => '\u{f440}',
        Nerd::DiffRemoved => '\u{eadf}',
        Nerd::DiffRemovedOne => '\u{f458}',
        Nerd::DiffRenamed => '\u{eae0}',
        Nerd::DiffRenamedOne => '\u{f45a}',
        Nerd::DigitalOcean => '\u{f1237}',
        Nerd::DipSwitch => '\u{f07c1}',
        Nerd::Directions => '\u{f01d0}',
        Nerd::DirectionsFork => '\u{f0641}',
        Nerd::Disc => '\u{f05ee}',
        Nerd::DiscAlert => '\u{f01d1}',
        Nerd::DiscPlayer => '\u{f0960}',
        Nerd::Discard => '\u{eae2}',
        Nerd::Disco => '\u{e271}',
        Nerd::Discord => '\u{f066f}',
        Nerd::DiscussionClosed => '\u{f4c0}',
        Nerd::DiscussionDuplicate => '\u{f4c1}',
        Nerd::DiscussionOutdated => '\u{f4c2}',
        Nerd::Dishwasher => '\u{f0aac}',
        Nerd::DishwasherAlert => '\u{f11b8}',
        Nerd::DishwasherOff => '\u{f11b9}',
        Nerd::Disqus => '\u{f01d2}',
        Nerd::DistributeHorizontalCenter => '\u{f11c9}',
        Nerd::DistributeHorizontalLeft => '\u{f11c8}',
        Nerd::DistributeHorizontalRight => '\u{f11ca}',
        Nerd::DistributeVerticalBottom => '\u{f11cb}',
        Nerd::DistributeVerticalCenter => '\u{f11cc}',
        Nerd::DistributeVerticalTop => '\u{f11cd}',
        Nerd::Diversify => '\u{f1877}',
        Nerd::Diving => '\u{f1977}',
        Nerd::DivingFlippers => '\u{f0dbf}',
        Nerd::DivingHelmet => '\u{f0dc0}',
        Nerd::DivingScuba => '\u{f0dc1}',
        Nerd::DivingScubaFlag => '\u{f0dc2}',
        Nerd::DivingScubaTank => '\u{f0dc3}',
        Nerd::DivingScubaTankMultiple => '\u{f0dc4}',
        Nerd::DivingSnorkel => '\u{f0dc5}',
        Nerd::Division => '\u{f01d4}',
        Nerd::DivisionBox => '\u{f01d5}',
        Nerd::Dlna => '\u{f0a41}',
        Nerd::Dna => '\u{e272}',
        Nerd::DnaOne => '\u{f0684}',
        Nerd::Dns => '\u{f01d6}',
        Nerd::DnsOutline => '\u{f0b8c}',
        Nerd::DockBottom => '\u{f10a9}',
        Nerd::DockLeft => '\u{f10aa}',
        Nerd::DockRight => '\u{f10ab}',
        Nerd::DockTop => '\u{f1513}',
        Nerd::DockWindow => '\u{f10ac}',
        Nerd::Docker => '\u{f0868}',
        Nerd::Doctor => '\u{f0a42}',
        Nerd::Dog => '\u{f0a43}',
        Nerd::DogService => '\u{f0aad}',
        Nerd::DogSide => '\u{f0a44}',
        Nerd::DogSideOff => '\u{f16ee}',
        Nerd::Dolby => '\u{f06b3}',
        Nerd::Dolly => '\u{f0e9e}',
        Nerd::Dolphin => '\u{f18b4}',
        Nerd::Domain => '\u{f01d7}',
        Nerd::DomainOff => '\u{f0d6f}',
        Nerd::DomainPlus => '\u{f10ad}',
        Nerd::DomainRemove => '\u{f10ae}',
        Nerd::DomeLight => '\u{f141e}',
        Nerd::DominoMask => '\u{f1023}',
        Nerd::Donkey => '\u{f07c2}',
        Nerd::Donut => '\u{e273}',
        Nerd::Door => '\u{f081a}',
        Nerd::DoorClosed => '\u{f081b}',
        Nerd::DoorClosedLock => '\u{f10af}',
        Nerd::DoorOpen => '\u{f081c}',
        Nerd::DoorSliding => '\u{f181e}',
        Nerd::DoorSlidingLock => '\u{f181f}',
        Nerd::DoorSlidingOpen => '\u{f1820}',
        Nerd::Doorbell => '\u{f12e6}',
        Nerd::DoorbellVideo => '\u{f0869}',
        Nerd::Dot => '\u{f4c3}',
        Nerd::DotCircleAlt => '\u{f192}',
        Nerd::DotFill => '\u{f444}',
        Nerd::DotNet => '\u{f0aae}',
        Nerd::DotsCircle => '\u{f1978}',
        Nerd::DotsGrid => '\u{f15fc}',
        Nerd::DotsHexagon => '\u{f15ff}',
        Nerd::DotsHorizontal => '\u{f01d8}',
        Nerd::DotsHorizontalCircle => '\u{f07c3}',
        Nerd::DotsHorizontalCircleOutline => '\u{f0b8d}',
        Nerd::DotsSquare => '\u{f15fd}',
        Nerd::DotsTriangle => '\u{f15fe}',
        Nerd::DotsVertical => '\u{f01d9}',
        Nerd::DotsVerticalCircle => '\u{f07c4}',
        Nerd::DotsVerticalCircleOutline => '\u{f0b8e}',
        Nerd::DoubleAngleDown => '\u{f103}',
        Nerd::DoubleAngleLeft => '\u{f100}',
        Nerd::DoubleAngleRight => '\u{f101}',
        Nerd::DoubleAngleUp => '\u{f102}',
        Nerd::Download => '\u{f01a}',
        Nerd::DownloadAlt => '\u{f019}',
        Nerd::DownloadBox => '\u{f1462}',
        Nerd::DownloadBoxOutline => '\u{f1463}',
        Nerd::DownloadCircle => '\u{f1464}',
        Nerd::DownloadCircleOutline => '\u{f1465}',
        Nerd::DownloadLock => '\u{f1320}',
        Nerd::DownloadLockOutline => '\u{f1321}',
        Nerd::DownloadMultiple => '\u{f09e9}',
        Nerd::DownloadNetwork => '\u{f06f4}',
        Nerd::DownloadNetworkOutline => '\u{f0c66}',
        Nerd::DownloadOff => '\u{f10b0}',
        Nerd::DownloadOffOutline => '\u{f10b1}',
        Nerd::DownloadOne => '\u{f409}',
        Nerd::DownloadOutline => '\u{f0b8f}',
        Nerd::DownloadTwo => '\u{f01da}',
        Nerd::Drag => '\u{f01db}',
        Nerd::DragHorizontal => '\u{f01dc}',
        Nerd::DragHorizontalVariant => '\u{f12f0}',
        Nerd::DragVariant => '\u{f0b90}',
        Nerd::DragVertical => '\u{f01dd}',
        Nerd::DragVerticalVariant => '\u{f12f1}',
        Nerd::DramaMasks => '\u{f0d02}',
        Nerd::Draw => '\u{f0f49}',
        Nerd::DrawPen => '\u{f19b9}',
        Nerd::Drawing => '\u{f01de}',
        Nerd::DrawingBox => '\u{f01df}',
        Nerd::Dress => '\u{e274}',
        Nerd::Dresser => '\u{f0f4a}',
        Nerd::DresserOutline => '\u{f0f4b}',
        Nerd::Dribble => '\u{f17d}',
        Nerd::Drone => '\u{f01e2}',
        Nerd::Drop => '\u{e275}',
        Nerd::Dropbox => '\u{f16b}',
        Nerd::DropboxOne => '\u{f01e3}',
        Nerd::Drupal => '\u{f01e4}',
        Nerd::Duck => '\u{f01e5}',
        Nerd::Dumbbell => '\u{f01e6}',
        Nerd::DumpTruck => '\u{f0c67}',
        Nerd::Duplicate => '\u{f4c4}',
        Nerd::Dwm => '\u{f356}',
        Nerd::EarHearing => '\u{f07c5}',
        Nerd::EarHearingLoop => '\u{f1aee}',
        Nerd::EarHearingOff => '\u{f0a45}',
        Nerd::Earbuds => '\u{f184f}',
        Nerd::EarbudsOff => '\u{f1850}',
        Nerd::EarbudsOffOutline => '\u{f1851}',
        Nerd::EarbudsOutline => '\u{f1852}',
        Nerd::Earth => '\u{f01e7}',
        Nerd::EarthArrowRight => '\u{f1311}',
        Nerd::EarthBox => '\u{f06cd}',
        Nerd::EarthBoxMinus => '\u{f1407}',
        Nerd::EarthBoxOff => '\u{f06ce}',
        Nerd::EarthBoxPlus => '\u{f1406}',
        Nerd::EarthBoxRemove => '\u{f1408}',
        Nerd::EarthMinus => '\u{f1404}',
        Nerd::EarthOff => '\u{f01e8}',
        Nerd::EarthPlus => '\u{f1403}',
        Nerd::EarthRemove => '\u{f1405}',
        Nerd::Edit => '\u{ea73}',
        Nerd::EditOne => '\u{f044}',
        Nerd::EditSign => '\u{f14b}',
        Nerd::EditorLayout => '\u{eae3}',
        Nerd::Egg => '\u{f0aaf}',
        Nerd::EggEaster => '\u{f0ab0}',
        Nerd::EggFried => '\u{f184a}',
        Nerd::EggOff => '\u{f13f0}',
        Nerd::EggOffOutline => '\u{f13f1}',
        Nerd::EggOutline => '\u{f13f2}',
        Nerd::EiffelTower => '\u{f156b}',
        Nerd::EightTrack => '\u{f09ea}',
        Nerd::Eject => '\u{f052}',
        Nerd::EjectOne => '\u{f01ea}',
        Nerd::EjectOutline => '\u{f0b91}',
        Nerd::ElectricSwitch => '\u{f0e9f}',
        Nerd::ElectricSwitchClosed => '\u{f10d9}',
        Nerd::ElectronFramework => '\u{f1024}',
        Nerd::ElementaryOs => '\u{f309}',
        Nerd::Elephant => '\u{f07c6}',
        Nerd::ElevationDecline => '\u{f01eb}',
        Nerd::ElevationRise => '\u{f01ec}',
        Nerd::Elevator => '\u{f01ed}',
        Nerd::ElevatorDown => '\u{f12c2}',
        Nerd::ElevatorPassenger => '\u{f1381}',
        Nerd::ElevatorPassengerOff => '\u{f1979}',
        Nerd::ElevatorPassengerOffOutline => '\u{f197a}',
        Nerd::ElevatorPassengerOutline => '\u{f197b}',
        Nerd::ElevatorUp => '\u{f12c1}',
        Nerd::Ellipse => '\u{f0ea0}',
        Nerd::EllipseOutline => '\u{f0ea1}',
        Nerd::Ellipsis => '\u{ea7c}',
        Nerd::EllipsisHorizontal => '\u{f141}',
        Nerd::EllipsisOne => '\u{f475}',
        Nerd::EllipsisVertical => '\u{f142}',
        Nerd::Ello => '\u{e276}',
        Nerd::Email => '\u{f01ee}',
        Nerd::EmailAlert => '\u{f06cf}',
        Nerd::EmailAlertOutline => '\u{f0d42}',
        Nerd::EmailBox => '\u{f0d03}',
        Nerd::EmailCheck => '\u{f0ab1}',
        Nerd::EmailCheckOutline => '\u{f0ab2}',
        Nerd::EmailEdit => '\u{f0ee3}',
        Nerd::EmailEditOutline => '\u{f0ee4}',
        Nerd::EmailFast => '\u{f186f}',
        Nerd::EmailFastOutline => '\u{f1870}',
        Nerd::EmailLock => '\u{f01f1}',
        Nerd::EmailMarkAsUnread => '\u{f0b92}',
        Nerd::EmailMinus => '\u{f0ee5}',
        Nerd::EmailMinusOutline => '\u{f0ee6}',
        Nerd::EmailMultiple => '\u{f0ee7}',
        Nerd::EmailMultipleOutline => '\u{f0ee8}',
        Nerd::EmailNewsletter => '\u{f0fb1}',
        Nerd::EmailOff => '\u{f13e3}',
        Nerd::EmailOffOutline => '\u{f13e4}',
        Nerd::EmailOpen => '\u{f01ef}',
        Nerd::EmailOpenMultiple => '\u{f0ee9}',
        Nerd::EmailOpenMultipleOutline => '\u{f0eea}',
        Nerd::EmailOpenOutline => '\u{f05ef}',
        Nerd::EmailOutline => '\u{f01f0}',
        Nerd::EmailPlus => '\u{f09eb}',
        Nerd::EmailPlusOutline => '\u{f09ec}',
        Nerd::EmailReceive => '\u{f10da}',
        Nerd::EmailReceiveOutline => '\u{f10db}',
        Nerd::EmailRemove => '\u{f1661}',
        Nerd::EmailRemoveOutline => '\u{f1662}',
        Nerd::EmailSeal => '\u{f195b}',
        Nerd::EmailSealOutline => '\u{f195c}',
        Nerd::EmailSearch => '\u{f0961}',
        Nerd::EmailSearchOutline => '\u{f0962}',
        Nerd::EmailSend => '\u{f10dc}',
        Nerd::EmailSendOutline => '\u{f10dd}',
        Nerd::EmailSync => '\u{f12c7}',
        Nerd::EmailSyncOutline => '\u{f12c8}',
        Nerd::EmailVariant => '\u{f05f0}',
        Nerd::Ember => '\u{f0b30}',
        Nerd::Emby => '\u{f06b4}',
        Nerd::Emoticon => '\u{f0c68}',
        Nerd::EmoticonAngry => '\u{f0c69}',
        Nerd::EmoticonAngryOutline => '\u{f0c6a}',
        Nerd::EmoticonConfused => '\u{f10de}',
        Nerd::EmoticonConfusedOutline => '\u{f10df}',
        Nerd::EmoticonCool => '\u{f0c6b}',
        Nerd::EmoticonCoolOutline => '\u{f01f3}',
        Nerd::EmoticonCry => '\u{f0c6c}',
        Nerd::EmoticonCryOutline => '\u{f0c6d}',
        Nerd::EmoticonDead => '\u{f0c6e}',
        Nerd::EmoticonDeadOutline => '\u{f069b}',
        Nerd::EmoticonDevil => '\u{f0c6f}',
        Nerd::EmoticonDevilOutline => '\u{f01f4}',
        Nerd::EmoticonExcited => '\u{f0c70}',
        Nerd::EmoticonExcitedOutline => '\u{f069c}',
        Nerd::EmoticonFrown => '\u{f0f4c}',
        Nerd::EmoticonFrownOutline => '\u{f0f4d}',
        Nerd::EmoticonHappy => '\u{f0c71}',
        Nerd::EmoticonHappyOutline => '\u{f01f5}',
        Nerd::EmoticonKiss => '\u{f0c72}',
        Nerd::EmoticonKissOutline => '\u{f0c73}',
        Nerd::EmoticonLol => '\u{f1214}',
        Nerd::EmoticonLolOutline => '\u{f1215}',
        Nerd::EmoticonNeutral => '\u{f0c74}',
        Nerd::EmoticonNeutralOutline => '\u{f01f6}',
        Nerd::EmoticonOutline => '\u{f01f2}',
        Nerd::EmoticonPoop => '\u{f01f7}',
        Nerd::EmoticonPoopOutline => '\u{f0c75}',
        Nerd::EmoticonSad => '\u{f0c76}',
        Nerd::EmoticonSadOutline => '\u{f01f8}',
        Nerd::EmoticonSick => '\u{f157c}',
        Nerd::EmoticonSickOutline => '\u{f157d}',
        Nerd::EmoticonTongue => '\u{f01f9}',
        Nerd::EmoticonTongueOutline => '\u{f0c77}',
        Nerd::EmoticonWink => '\u{f0c78}',
        Nerd::EmoticonWinkOutline => '\u{f0c79}',
        Nerd::EmptyWindow => '\u{eae4}',
        Nerd::EndeavourOs => '\u{f322}',
        Nerd::Engine => '\u{f01fa}',
        Nerd::EngineOff => '\u{f0a46}',
        Nerd::EngineOffOutline => '\u{f0a47}',
        Nerd::EngineOutline => '\u{f01fb}',
        Nerd::Enlightenment => '\u{f357}',
        Nerd::Envelope => '\u{f003}',
        Nerd::EnvelopeAlt => '\u{f0e0}',
        Nerd::EnvelopeOpen => '\u{e277}',
        Nerd::EnvelopeOpenO => '\u{e278}',
        Nerd::Epsilon => '\u{f10e0}',
        Nerd::Equal => '\u{e279}',
        Nerd::EqualBigger => '\u{e27a}',
        Nerd::EqualBox => '\u{f01fd}',
        Nerd::EqualOne => '\u{f01fc}',
        Nerd::Equalizer => '\u{f0ea2}',
        Nerd::EqualizerOutline => '\u{f0ea3}',
        Nerd::Eraser => '\u{f01fe}',
        Nerd::EraserVariant => '\u{f0642}',
        Nerd::Error => '\u{ea87}',
        Nerd::Escalator => '\u{f01ff}',
        Nerd::EscalatorBox => '\u{f1399}',
        Nerd::EscalatorDown => '\u{f12c0}',
        Nerd::EscalatorUp => '\u{f12bf}',
        Nerd::Eslint => '\u{f0c7a}',
        Nerd::Et => '\u{f0ab3}',
        Nerd::Ethereum => '\u{f086a}',
        Nerd::Ethernet => '\u{f0200}',
        Nerd::EthernetCable => '\u{f0201}',
        Nerd::EthernetCableOff => '\u{f0202}',
        Nerd::Eur => '\u{f153}',
        Nerd::EvPlugCcsone => '\u{f1519}',
        Nerd::EvPlugCcstwo => '\u{f151a}',
        Nerd::EvPlugChademo => '\u{f151b}',
        Nerd::EvPlugTesla => '\u{f151c}',
        Nerd::EvPlugTypeone => '\u{f151d}',
        Nerd::EvPlugTypetwo => '\u{f151e}',
        Nerd::EvStation => '\u{f05f1}',
        Nerd::Evernote => '\u{f0204}',
        Nerd::Excavator => '\u{f1025}',
        Nerd::Exchange => '\u{f0ec}',
        Nerd::Exclamation => '\u{f12a}',
        Nerd::ExclamationOne => '\u{f0205}',
        Nerd::ExclamationSign => '\u{f06a}',
        Nerd::ExclamationThick => '\u{f1238}',
        Nerd::Exclude => '\u{eae5}',
        Nerd::ExitRun => '\u{f0a48}',
        Nerd::ExitToApp => '\u{f0206}',
        Nerd::ExitToAppOne => '\u{f05fc}',
        Nerd::ExpandAll => '\u{eb95}',
        Nerd::ExpandAllOne => '\u{f0ab4}',
        Nerd::ExpandAllOutline => '\u{f0ab5}',
        Nerd::ExpandAlt => '\u{f116}',
        Nerd::ExpansionCard => '\u{f08ae}',
        Nerd::ExpansionCardVariant => '\u{f0fb2}',
        Nerd::Exponent => '\u{f0963}',
        Nerd::ExponentBox => '\u{f0964}',
        Nerd::Export => '\u{ebac}',
        Nerd::ExportOne => '\u{f0207}',
        Nerd::ExportVariant => '\u{f0b93}',
        Nerd::Extensions => '\u{eae6}',
        Nerd::ExternalInterruption => '\u{e00a}',
        Nerd::ExternalLink => '\u{f08e}',
        Nerd::Eye => '\u{ea70}',
        Nerd::EyeArrowLeft => '\u{f18fd}',
        Nerd::EyeArrowLeftOutline => '\u{f18fe}',
        Nerd::EyeArrowRight => '\u{f18ff}',
        Nerd::EyeArrowRightOutline => '\u{f1900}',
        Nerd::EyeCheck => '\u{f0d04}',
        Nerd::EyeCheckOutline => '\u{f0d05}',
        Nerd::EyeCircle => '\u{f0b94}',
        Nerd::EyeCircleOutline => '\u{f0b95}',
        Nerd::EyeClose => '\u{f070}',
        Nerd::EyeClosed => '\u{eae7}',
        Nerd::EyeClosedOne => '\u{f4c5}',
        Nerd::EyeMinus => '\u{f1026}',
        Nerd::EyeMinusOutline => '\u{f1027}',
        Nerd::EyeOff => '\u{f0209}',
        Nerd::EyeOffOutline => '\u{f06d1}',
        Nerd::EyeOne => '\u{f441}',
        Nerd::EyeOpen => '\u{f06e}',
        Nerd::EyeOutline => '\u{f06d0}',
        Nerd::EyePlus => '\u{f086b}',
        Nerd::EyePlusOutline => '\u{f086c}',
        Nerd::EyeRefresh => '\u{f197c}',
        Nerd::EyeRefreshOutline => '\u{f197d}',
        Nerd::EyeRemove => '\u{f15e3}',
        Nerd::EyeRemoveOutline => '\u{f15e4}',
        Nerd::EyeSettings => '\u{f086d}',
        Nerd::EyeSettingsOutline => '\u{f086e}',
        Nerd::EyeTwo => '\u{f0208}',
        Nerd::Eyedropper => '\u{f020a}',
        Nerd::EyedropperMinus => '\u{f13dd}',
        Nerd::EyedropperOff => '\u{f13df}',
        Nerd::EyedropperPlus => '\u{f13dc}',
        Nerd::EyedropperRemove => '\u{f13de}',
        Nerd::EyedropperVariant => '\u{f020b}',
        Nerd::FDroid => '\u{f36a}',
        Nerd::FaceAgent => '\u{f0d70}',
        Nerd::FaceMan => '\u{f0643}',
        Nerd::FaceManOutline => '\u{f0b96}',
        Nerd::FaceManProfile => '\u{f0644}',
        Nerd::FaceManShimmer => '\u{f15cc}',
        Nerd::FaceManShimmerOutline => '\u{f15cd}',
        Nerd::FaceMask => '\u{f1586}',
        Nerd::FaceMaskOutline => '\u{f1587}',
        Nerd::FaceRecognition => '\u{f0c7b}',
        Nerd::FaceWoman => '\u{f1077}',
        Nerd::FaceWomanOutline => '\u{f1078}',
        Nerd::FaceWomanProfile => '\u{f1076}',
        Nerd::FaceWomanShimmer => '\u{f15ce}',
        Nerd::FaceWomanShimmerOutline => '\u{f15cf}',
        Nerd::Facebook => '\u{f09a}',
        Nerd::FacebookGaming => '\u{f07dd}',
        Nerd::FacebookMessenger => '\u{f020e}',
        Nerd::FacebookOne => '\u{f020c}',
        Nerd::FacebookSign => '\u{f082}',
        Nerd::FacebookWorkplace => '\u{f0b31}',
        Nerd::FacetimeVideo => '\u{f03d}',
        Nerd::Factory => '\u{f020f}',
        Nerd::FamilyTree => '\u{f160e}',
        Nerd::Fan => '\u{f0210}',
        Nerd::FanAlert => '\u{f146c}',
        Nerd::FanAuto => '\u{f171d}',
        Nerd::FanChevronDown => '\u{f146d}',
        Nerd::FanChevronUp => '\u{f146e}',
        Nerd::FanClock => '\u{f1a3a}',
        Nerd::FanMinus => '\u{f1470}',
        Nerd::FanOff => '\u{f081d}',
        Nerd::FanPlus => '\u{f146f}',
        Nerd::FanRemove => '\u{f1471}',
        Nerd::FanSpeedOne => '\u{f1472}',
        Nerd::FanSpeedThree => '\u{f1474}',
        Nerd::FanSpeedTwo => '\u{f1473}',
        Nerd::FastBackward => '\u{f049}',
        Nerd::FastForward => '\u{f0211}',
        Nerd::FastForwardFive => '\u{f11f8}',
        Nerd::FastForwardOnefive => '\u{f193a}',
        Nerd::FastForwardOnezero => '\u{f0d71}',
        Nerd::FastForwardOutline => '\u{f06d2}',
        Nerd::FastForwardSixzero => '\u{f160b}',
        Nerd::FastForwardThreezero => '\u{f0d06}',
        Nerd::Fax => '\u{f0212}',
        Nerd::Feather => '\u{f06d3}',
        Nerd::FeatureSearch => '\u{f0a49}',
        Nerd::FeatureSearchOutline => '\u{f0a4a}',
        Nerd::Fedora => '\u{f08db}',
        Nerd::FedoraInverse => '\u{f30b}',
        Nerd::FeedDiscussion => '\u{f4c6}',
        Nerd::FeedForked => '\u{f4c7}',
        Nerd::FeedHeart => '\u{f4c8}',
        Nerd::FeedMerged => '\u{f4c9}',
        Nerd::FeedPerson => '\u{f4ca}',
        Nerd::FeedRepo => '\u{f4cb}',
        Nerd::FeedRocket => '\u{f4cc}',
        Nerd::FeedStar => '\u{f4cd}',
        Nerd::FeedTag => '\u{f4ce}',
        Nerd::FeedTrophy => '\u{f4cf}',
        Nerd::Feedback => '\u{eb96}',
        Nerd::Feedly => '\u{e27b}',
        Nerd::Female => '\u{f182}',
        Nerd::Fence => '\u{f179a}',
        Nerd::FenceElectric => '\u{f17f6}',
        Nerd::Fencing => '\u{f14c1}',
        Nerd::Ferris => '\u{f323}',
        Nerd::FerrisWheel => '\u{f0ea4}',
        Nerd::Ferry => '\u{f0213}',
        Nerd::FighterJet => '\u{f0fb}',
        Nerd::File => '\u{ea7b}',
        Nerd::FileAccount => '\u{f073b}',
        Nerd::FileAccountOutline => '\u{f1028}',
        Nerd::FileAdded => '\u{f4d0}',
        Nerd::FileAlert => '\u{f0a4b}',
        Nerd::FileAlertOutline => '\u{f0a4c}',
        Nerd::FileAlt => '\u{f016}',
        Nerd::FileArrowLeftRight => '\u{f1a93}',
        Nerd::FileArrowLeftRightOutline => '\u{f1a94}',
        Nerd::FileArrowUpDown => '\u{f1a95}',
        Nerd::FileArrowUpDownOutline => '\u{f1a96}',
        Nerd::FileBadge => '\u{f4d1}',
        Nerd::FileBinary => '\u{eae8}',
        Nerd::FileBinaryOne => '\u{f471}',
        Nerd::FileCabinet => '\u{f0ab6}',
        Nerd::FileCad => '\u{f0eeb}',
        Nerd::FileCadBox => '\u{f0eec}',
        Nerd::FileCancel => '\u{f0dc6}',
        Nerd::FileCancelOutline => '\u{f0dc7}',
        Nerd::FileCertificate => '\u{f1186}',
        Nerd::FileCertificateOutline => '\u{f1187}',
        Nerd::FileChart => '\u{f0215}',
        Nerd::FileChartCheck => '\u{f19c6}',
        Nerd::FileChartCheckOutline => '\u{f19c7}',
        Nerd::FileChartOutline => '\u{f1029}',
        Nerd::FileCheck => '\u{f0216}',
        Nerd::FileCheckOutline => '\u{f0e29}',
        Nerd::FileClock => '\u{f12e1}',
        Nerd::FileClockOutline => '\u{f12e2}',
        Nerd::FileCloud => '\u{f0217}',
        Nerd::FileCloudOutline => '\u{f102a}',
        Nerd::FileCode => '\u{eae9}',
        Nerd::FileCodeOne => '\u{eafb}',
        Nerd::FileCodeOutline => '\u{f102b}',
        Nerd::FileCodeThree => '\u{f022e}',
        Nerd::FileCodeTwo => '\u{f40d}',
        Nerd::FileCog => '\u{f107b}',
        Nerd::FileCogOutline => '\u{f107c}',
        Nerd::FileCompare => '\u{f08aa}',
        Nerd::FileDelimited => '\u{f0218}',
        Nerd::FileDelimitedOutline => '\u{f0ea5}',
        Nerd::FileDiff => '\u{f4d2}',
        Nerd::FileDirectory => '\u{f413}',
        Nerd::FileDirectoryFill => '\u{f4d3}',
        Nerd::FileDirectoryOpenFill => '\u{f4d4}',
        Nerd::FileDocument => '\u{f0219}',
        Nerd::FileDocumentAlert => '\u{f1a97}',
        Nerd::FileDocumentAlertOutline => '\u{f1a98}',
        Nerd::FileDocumentCheck => '\u{f1a99}',
        Nerd::FileDocumentCheckOutline => '\u{f1a9a}',
        Nerd::FileDocumentEdit => '\u{f0dc8}',
        Nerd::FileDocumentEditOutline => '\u{f0dc9}',
        Nerd::FileDocumentMinus => '\u{f1a9b}',
        Nerd::FileDocumentMinusOutline => '\u{f1a9c}',
        Nerd::FileDocumentMultiple => '\u{f1517}',
        Nerd::FileDocumentMultipleOutline => '\u{f1518}',
        Nerd::FileDocumentOutline => '\u{f09ee}',
        Nerd::FileDocumentPlus => '\u{f1a9d}',
        Nerd::FileDocumentPlusOutline => '\u{f1a9e}',
        Nerd::FileDocumentRemove => '\u{f1a9f}',
        Nerd::FileDocumentRemoveOutline => '\u{f1aa0}',
        Nerd::FileDownload => '\u{f0965}',
        Nerd::FileDownloadOutline => '\u{f0966}',
        Nerd::FileEdit => '\u{f11e7}',
        Nerd::FileEditOutline => '\u{f11e8}',
        Nerd::FileExcel => '\u{f021b}',
        Nerd::FileExcelBox => '\u{f021c}',
        Nerd::FileExcelBoxOutline => '\u{f102c}',
        Nerd::FileExcelOutline => '\u{f102d}',
        Nerd::FileExport => '\u{e27c}',
        Nerd::FileExportOne => '\u{f021d}',
        Nerd::FileExportOutline => '\u{f102e}',
        Nerd::FileEye => '\u{f0dca}',
        Nerd::FileEyeOutline => '\u{f0dcb}',
        Nerd::FileFind => '\u{f021e}',
        Nerd::FileFindOutline => '\u{f0b97}',
        Nerd::FileGifBox => '\u{f0d78}',
        Nerd::FileHidden => '\u{f0613}',
        Nerd::FileImage => '\u{f021f}',
        Nerd::FileImageMarker => '\u{f1772}',
        Nerd::FileImageMarkerOutline => '\u{f1773}',
        Nerd::FileImageMinus => '\u{f193b}',
        Nerd::FileImageMinusOutline => '\u{f193c}',
        Nerd::FileImageOutline => '\u{f0eb0}',
        Nerd::FileImagePlus => '\u{f193d}',
        Nerd::FileImagePlusOutline => '\u{f193e}',
        Nerd::FileImageRemove => '\u{f193f}',
        Nerd::FileImageRemoveOutline => '\u{f1940}',
        Nerd::FileImport => '\u{e27d}',
        Nerd::FileImportOne => '\u{f0220}',
        Nerd::FileImportOutline => '\u{f102f}',
        Nerd::FileJpgBox => '\u{f0225}',
        Nerd::FileKey => '\u{f1184}',
        Nerd::FileKeyOutline => '\u{f1185}',
        Nerd::FileLink => '\u{f1177}',
        Nerd::FileLinkOutline => '\u{f1178}',
        Nerd::FileLock => '\u{f0221}',
        Nerd::FileLockOpen => '\u{f19c8}',
        Nerd::FileLockOpenOutline => '\u{f19c9}',
        Nerd::FileLockOutline => '\u{f1030}',
        Nerd::FileMarker => '\u{f1774}',
        Nerd::FileMarkerOutline => '\u{f1775}',
        Nerd::FileMedia => '\u{eaea}',
        Nerd::FileMediaOne => '\u{f40f}',
        Nerd::FileMinus => '\u{f1aa1}',
        Nerd::FileMinusOutline => '\u{f1aa2}',
        Nerd::FileMove => '\u{f0ab9}',
        Nerd::FileMoveOutline => '\u{f1031}',
        Nerd::FileMoved => '\u{f4d5}',
        Nerd::FileMultiple => '\u{f0222}',
        Nerd::FileMultipleOutline => '\u{f1032}',
        Nerd::FileMusic => '\u{f0223}',
        Nerd::FileMusicOutline => '\u{f0e2a}',
        Nerd::FileOne => '\u{f15b}',
        Nerd::FileOutline => '\u{f0224}',
        Nerd::FilePdf => '\u{eaeb}',
        Nerd::FilePdfBox => '\u{f0226}',
        Nerd::FilePercent => '\u{f081e}',
        Nerd::FilePercentOutline => '\u{f1033}',
        Nerd::FilePhone => '\u{f1179}',
        Nerd::FilePhoneOutline => '\u{f117a}',
        Nerd::FilePlus => '\u{f0752}',
        Nerd::FilePlusOutline => '\u{f0eed}',
        Nerd::FilePngBox => '\u{f0e2d}',
        Nerd::FilePowerpoint => '\u{f0227}',
        Nerd::FilePowerpointBox => '\u{f0228}',
        Nerd::FilePowerpointBoxOutline => '\u{f1034}',
        Nerd::FilePowerpointOutline => '\u{f1035}',
        Nerd::FilePresentationBox => '\u{f0229}',
        Nerd::FileQuestion => '\u{f086f}',
        Nerd::FileQuestionOutline => '\u{f1036}',
        Nerd::FileRefresh => '\u{f0918}',
        Nerd::FileRefreshOutline => '\u{f0541}',
        Nerd::FileRemove => '\u{f0b98}',
        Nerd::FileRemoveOutline => '\u{f1037}',
        Nerd::FileRemoved => '\u{f4d6}',
        Nerd::FileReplace => '\u{f0b32}',
        Nerd::FileReplaceOutline => '\u{f0b33}',
        Nerd::FileRestore => '\u{f0670}',
        Nerd::FileRestoreOutline => '\u{f1038}',
        Nerd::FileRotateLeft => '\u{f1a3b}',
        Nerd::FileRotateLeftOutline => '\u{f1a3c}',
        Nerd::FileRotateRight => '\u{f1a3d}',
        Nerd::FileRotateRightOutline => '\u{f1a3e}',
        Nerd::FileSearch => '\u{f0c7c}',
        Nerd::FileSearchOutline => '\u{f0c7d}',
        Nerd::FileSend => '\u{f022a}',
        Nerd::FileSendOutline => '\u{f1039}',
        Nerd::FileSettings => '\u{f1079}',
        Nerd::FileSettingsOutline => '\u{f107a}',
        Nerd::FileSign => '\u{f19c3}',
        Nerd::FileStar => '\u{f103a}',
        Nerd::FileStarOutline => '\u{f103b}',
        Nerd::FileSubmodule => '\u{eaec}',
        Nerd::FileSubmoduleOne => '\u{f414}',
        Nerd::FileSwap => '\u{f0fb4}',
        Nerd::FileSwapOutline => '\u{f0fb5}',
        Nerd::FileSymlinkDirectory => '\u{eaed}',
        Nerd::FileSymlinkDirectoryOne => '\u{f482}',
        Nerd::FileSymlinkFile => '\u{eaee}',
        Nerd::FileSymlinkFileOne => '\u{f481}',
        Nerd::FileSync => '\u{f1216}',
        Nerd::FileSyncOutline => '\u{f1217}',
        Nerd::FileTable => '\u{f0c7e}',
        Nerd::FileTableBox => '\u{f10e1}',
        Nerd::FileTableBoxMultiple => '\u{f10e2}',
        Nerd::FileTableBoxMultipleOutline => '\u{f10e3}',
        Nerd::FileTableBoxOutline => '\u{f10e4}',
        Nerd::FileTableOutline => '\u{f0c7f}',
        Nerd::FileText => '\u{f15c}',
        Nerd::FileTextAlt => '\u{f0f6}',
        Nerd::FileThree => '\u{f0214}',
        Nerd::FileTree => '\u{f0645}',
        Nerd::FileTreeOutline => '\u{f13d2}',
        Nerd::FileTwo => '\u{f4a5}',
        Nerd::FileUndo => '\u{f08dc}',
        Nerd::FileUndoOutline => '\u{f103c}',
        Nerd::FileUpload => '\u{f0a4d}',
        Nerd::FileUploadOutline => '\u{f0a4e}',
        Nerd::FileVideo => '\u{f022b}',
        Nerd::FileVideoOutline => '\u{f0e2c}',
        Nerd::FileWord => '\u{f022c}',
        Nerd::FileWordBox => '\u{f022d}',
        Nerd::FileWordBoxOutline => '\u{f103d}',
        Nerd::FileWordOutline => '\u{f103e}',
        Nerd::FileZip => '\u{eaef}',
        Nerd::FileZipOne => '\u{f410}',
        Nerd::Files => '\u{eaf0}',
        Nerd::Film => '\u{f008}',
        Nerd::FilmOne => '\u{f022f}',
        Nerd::Filmstrip => '\u{f0230}',
        Nerd::FilmstripBox => '\u{f0332}',
        Nerd::FilmstripBoxMultiple => '\u{f0d18}',
        Nerd::FilmstripOff => '\u{f0231}',
        Nerd::Filter => '\u{eaf1}',
        Nerd::FilterCheck => '\u{f18ec}',
        Nerd::FilterCheckOutline => '\u{f18ed}',
        Nerd::FilterCog => '\u{f1aa3}',
        Nerd::FilterCogOutline => '\u{f1aa4}',
        Nerd::FilterFilled => '\u{ebce}',
        Nerd::FilterMenu => '\u{f10e5}',
        Nerd::FilterMenuOutline => '\u{f10e6}',
        Nerd::FilterMinus => '\u{f0eee}',
        Nerd::FilterMinusOutline => '\u{f0eef}',
        Nerd::FilterMultiple => '\u{f1a3f}',
        Nerd::FilterMultipleOutline => '\u{f1a40}',
        Nerd::FilterOff => '\u{f14ef}',
        Nerd::FilterOffOutline => '\u{f14f0}',
        Nerd::FilterOne => '\u{f0b0}',
        Nerd::FilterOutline => '\u{f0233}',
        Nerd::FilterPlus => '\u{f0ef0}',
        Nerd::FilterPlusOutline => '\u{f0ef1}',
        Nerd::FilterRemove => '\u{f0234}',
        Nerd::FilterRemoveOutline => '\u{f0235}',
        Nerd::FilterSettings => '\u{f1aa5}',
        Nerd::FilterSettingsOutline => '\u{f1aa6}',
        Nerd::FilterThree => '\u{f0232}',
        Nerd::FilterTwo => '\u{f4d7}',
        Nerd::FilterVariant => '\u{f0236}',
        Nerd::FilterVariantMinus => '\u{f1112}',
        Nerd::FilterVariantPlus => '\u{f1113}',
        Nerd::FilterVariantRemove => '\u{f103f}',
        Nerd::Finance => '\u{f081f}',
        Nerd::FindReplace => '\u{f06d4}',
        Nerd::Fingerprint => '\u{e23f}',
        Nerd::FingerprintOff => '\u{f0eb1}',
        Nerd::FingerprintOne => '\u{f0237}',
        Nerd::Fire => '\u{f06d}',
        Nerd::FireAlert => '\u{f15d7}',
        Nerd::FireCircle => '\u{f1807}',
        Nerd::FireExtinguisher => '\u{f0ef2}',
        Nerd::FireHydrant => '\u{f1137}',
        Nerd::FireHydrantAlert => '\u{f1138}',
        Nerd::FireHydrantOff => '\u{f1139}',
        Nerd::FireOff => '\u{f1722}',
        Nerd::FireOne => '\u{f0238}',
        Nerd::FireTruck => '\u{f08ab}',
        Nerd::Firebase => '\u{f0967}',
        Nerd::Firefox => '\u{f0239}',
        Nerd::Fireplace => '\u{f0e2e}',
        Nerd::FireplaceOff => '\u{f0e2f}',
        Nerd::Firewire => '\u{f05be}',
        Nerd::Firework => '\u{f0e30}',
        Nerd::FireworkOff => '\u{f1723}',
        Nerd::FiscalHost => '\u{f4d8}',
        Nerd::Fish => '\u{f023a}',
        Nerd::FishOff => '\u{f13f3}',
        Nerd::Fishbowl => '\u{f0ef3}',
        Nerd::FishbowlOutline => '\u{f0ef4}',
        Nerd::FitToPage => '\u{f0ef5}',
        Nerd::FitToPageOutline => '\u{f0ef6}',
        Nerd::FitToScreen => '\u{f18f4}',
        Nerd::FitToScreenOutline => '\u{f18f5}',
        Nerd::Fiveeighteight => '\u{f273}',
        Nerd::Fiveeightfive => '\u{f270}',
        Nerd::Fiveeightfour => '\u{f26e}',
        Nerd::Fiveeightnine => '\u{f274}',
        Nerd::Fiveeightone => '\u{f26b}',
        Nerd::Fiveeightseven => '\u{f272}',
        Nerd::Fiveeightsix => '\u{f271}',
        Nerd::Fiveeightthree => '\u{f26d}',
        Nerd::Fiveeighttwo => '\u{f26c}',
        Nerd::Fiveeightzero => '\u{f26a}',
        Nerd::Fivefiveeight => '\u{f253}',
        Nerd::Fivefivefive => '\u{f250}',
        Nerd::Fivefivefour => '\u{f24e}',
        Nerd::Fivefivenine => '\u{f254}',
        Nerd::Fivefiveone => '\u{f24b}',
        Nerd::Fivefiveseven => '\u{f252}',
        Nerd::Fivefivesix => '\u{f251}',
        Nerd::Fivefivethree => '\u{f24d}',
        Nerd::Fivefivetwo => '\u{f24c}',
        Nerd::Fivefivezero => '\u{f24a}',
        Nerd::Fivefoureight => '\u{f248}',
        Nerd::Fivefourfive => '\u{f245}',
        Nerd::Fivefourfour => '\u{f244}',
        Nerd::Fivefournine => '\u{f249}',
        Nerd::Fivefourone => '\u{f241}',
        Nerd::Fivefourseven => '\u{f247}',
        Nerd::Fivefoursix => '\u{f246}',
        Nerd::Fivefourthree => '\u{f243}',
        Nerd::Fivefourtwo => '\u{f242}',
        Nerd::Fivefourzero => '\u{f240}',
        Nerd::Fivenineeight => '\u{f27d}',
        Nerd::Fiveninefive => '\u{f27a}',
        Nerd::Fiveninefour => '\u{f279}',
        Nerd::Fivenineone => '\u{f276}',
        Nerd::Fivenineseven => '\u{f27c}',
        Nerd::Fiveninesix => '\u{f27b}',
        Nerd::Fiveninethree => '\u{f278}',
        Nerd::Fiveninetwo => '\u{f277}',
        Nerd::Fiveninezero => '\u{f275}',
        Nerd::Fiveoneeight => '\u{f229}',
        Nerd::Fiveonefive => '\u{f226}',
        Nerd::Fiveonefour => '\u{f225}',
        Nerd::Fiveonenine => '\u{f22a}',
        Nerd::Fiveoneone => '\u{f222}',
        Nerd::Fiveoneseven => '\u{f228}',
        Nerd::Fiveonesix => '\u{f227}',
        Nerd::Fiveonethree => '\u{f224}',
        Nerd::Fiveonetwo => '\u{f223}',
        Nerd::Fiveseveneight => '\u{f268}',
        Nerd::Fivesevenfive => '\u{f265}',
        Nerd::Fivesevenfour => '\u{f264}',
        Nerd::Fivesevennine => '\u{f269}',
        Nerd::Fivesevenseven => '\u{f267}',
        Nerd::Fivesevensix => '\u{f266}',
        Nerd::Fiveseventwo => '\u{f262}',
        Nerd::Fivesixeight => '\u{f25d}',
        Nerd::Fivesixfive => '\u{f25a}',
        Nerd::Fivesixfour => '\u{f259}',
        Nerd::Fivesixnine => '\u{f25e}',
        Nerd::Fivesixone => '\u{f256}',
        Nerd::Fivesixseven => '\u{f25c}',
        Nerd::Fivesixsix => '\u{f25b}',
        Nerd::Fivesixthree => '\u{f258}',
        Nerd::Fivesixtwo => '\u{f257}',
        Nerd::Fivesixzero => '\u{f255}',
        Nerd::Fivethreeeight => '\u{f23d}',
        Nerd::Fivethreefive => '\u{f23a}',
        Nerd::Fivethreefour => '\u{f239}',
        Nerd::Fivethreenine => '\u{f23e}',
        Nerd::Fivethreeone => '\u{f236}',
        Nerd::Fivethreeseven => '\u{f23c}',
        Nerd::Fivethreesix => '\u{f23b}',
        Nerd::Fivethreethree => '\u{f238}',
        Nerd::Fivethreetwo => '\u{f237}',
        Nerd::Fivethreezero => '\u{f235}',
        Nerd::Fivetwoeight => '\u{f233}',
        Nerd::Fivetwofive => '\u{f230}',
        Nerd::Fivetwofour => '\u{f22f}',
        Nerd::Fivetwonine => '\u{f234}',
        Nerd::Fivetwoone => '\u{f22c}',
        Nerd::Fivetwoseven => '\u{f232}',
        Nerd::Fivetwosix => '\u{f231}',
        Nerd::Fivetwothree => '\u{f22e}',
        Nerd::Fivetwotwo => '\u{f22d}',
        Nerd::Fivetwozero => '\u{f22b}',
        Nerd::Fivezeroeight => '\u{f21d}',
        Nerd::Fivezerofive => '\u{f21a}',
        Nerd::Fivezerofour => '\u{f219}',
        Nerd::Fivezeronine => '\u{f21e}',
        Nerd::Fivezeroone => '\u{f216}',
        Nerd::Fivezeroseven => '\u{f21c}',
        Nerd::Fivezerosix => '\u{f21b}',
        Nerd::Fivezerothree => '\u{f218}',
        Nerd::Fivezerotwo => '\u{f217}',
        Nerd::Fivezerozero => '\u{f215}',
        Nerd::Flag => '\u{f024}',
        Nerd::FlagAlt => '\u{f11d}',
        Nerd::FlagCheckered => '\u{f023c}',
        Nerd::FlagMinus => '\u{f0b99}',
        Nerd::FlagMinusOutline => '\u{f10b2}',
        Nerd::FlagOff => '\u{f18ee}',
        Nerd::FlagOffOutline => '\u{f18ef}',
        Nerd::FlagOne => '\u{f023b}',
        Nerd::FlagOutline => '\u{f023d}',
        Nerd::FlagPlus => '\u{f0b9a}',
        Nerd::FlagPlusOutline => '\u{f10b3}',
        Nerd::FlagRemove => '\u{f0b9b}',
        Nerd::FlagRemoveOutline => '\u{f10b4}',
        Nerd::FlagTriangle => '\u{f023f}',
        Nerd::FlagVariant => '\u{f0240}',
        Nerd::FlagVariantOutline => '\u{f023e}',
        Nerd::Flame => '\u{eaf2}',
        Nerd::FlameOne => '\u{f490}',
        Nerd::Flare => '\u{f0d72}',
        Nerd::Flash => '\u{f0241}',
        Nerd::FlashAlert => '\u{f0ef7}',
        Nerd::FlashAlertOutline => '\u{f0ef8}',
        Nerd::FlashAuto => '\u{f0242}',
        Nerd::FlashOff => '\u{f0243}',
        Nerd::FlashOutline => '\u{f06d5}',
        Nerd::FlashRedEye => '\u{f067b}',
        Nerd::Flashlight => '\u{f0244}',
        Nerd::FlashlightOff => '\u{f0245}',
        Nerd::Flask => '\u{f0093}',
        Nerd::FlaskEmpty => '\u{f0094}',
        Nerd::FlaskEmptyMinus => '\u{f123a}',
        Nerd::FlaskEmptyMinusOutline => '\u{f123b}',
        Nerd::FlaskEmptyOff => '\u{f13f4}',
        Nerd::FlaskEmptyOffOutline => '\u{f13f5}',
        Nerd::FlaskEmptyOutline => '\u{f0095}',
        Nerd::FlaskEmptyPlus => '\u{f123c}',
        Nerd::FlaskEmptyPlusOutline => '\u{f123d}',
        Nerd::FlaskEmptyRemove => '\u{f123e}',
        Nerd::FlaskEmptyRemoveOutline => '\u{f123f}',
        Nerd::FlaskMinus => '\u{f1240}',
        Nerd::FlaskMinusOutline => '\u{f1241}',
        Nerd::FlaskOff => '\u{f13f6}',
        Nerd::FlaskOffOutline => '\u{f13f7}',
        Nerd::FlaskOutline => '\u{f0096}',
        Nerd::FlaskPlus => '\u{f1242}',
        Nerd::FlaskPlusOutline => '\u{f1243}',
        Nerd::FlaskRemove => '\u{f1244}',
        Nerd::FlaskRemoveOutline => '\u{f1245}',
        Nerd::FlaskRoundBottom => '\u{f124b}',
        Nerd::FlaskRoundBottomEmpty => '\u{f124c}',
        Nerd::FlaskRoundBottomEmptyOutline => '\u{f124d}',
        Nerd::FlaskRoundBottomOutline => '\u{f124e}',
        Nerd::Flathub => '\u{f324}',
        Nerd::FleurDeLis => '\u{f1303}',
        Nerd::Flickr => '\u{f16e}',
        Nerd::FlipHorizontal => '\u{f10e7}',
        Nerd::FlipToBack => '\u{f0247}',
        Nerd::FlipToFront => '\u{f0248}',
        Nerd::FlipVertical => '\u{f10e8}',
        Nerd::FloorLamp => '\u{f08dd}',
        Nerd::FloorLampDual => '\u{f1040}',
        Nerd::FloorLampDualOutline => '\u{f17ce}',
        Nerd::FloorLampOutline => '\u{f17c8}',
        Nerd::FloorLampTorchiere => '\u{f1747}',
        Nerd::FloorLampTorchiereOutline => '\u{f17d6}',
        Nerd::FloorLampTorchiereVariant => '\u{f1041}',
        Nerd::FloorLampTorchiereVariantOutline => '\u{f17cf}',
        Nerd::FloorPlan => '\u{f0821}',
        Nerd::Floppy => '\u{e240}',
        Nerd::FloppyOne => '\u{f0249}',
        Nerd::FloppyVariant => '\u{f09ef}',
        Nerd::Flower => '\u{f024a}',
        Nerd::FlowerOutline => '\u{f09f0}',
        Nerd::FlowerPollen => '\u{f1885}',
        Nerd::FlowerPollenOutline => '\u{f1886}',
        Nerd::FlowerPoppy => '\u{f0d08}',
        Nerd::FlowerTulip => '\u{f09f1}',
        Nerd::FlowerTulipOutline => '\u{f09f2}',
        Nerd::Fluxbox => '\u{f358}',
        Nerd::FocusAuto => '\u{f0f4e}',
        Nerd::FocusField => '\u{f0f4f}',
        Nerd::FocusFieldHorizontal => '\u{f0f50}',
        Nerd::FocusFieldVertical => '\u{f0f51}',
        Nerd::Fold => '\u{eaf5}',
        Nerd::FoldDown => '\u{eaf3}',
        Nerd::FoldDownOne => '\u{f4d9}',
        Nerd::FoldOne => '\u{f48c}',
        Nerd::FoldUp => '\u{eaf4}',
        Nerd::FoldUpOne => '\u{f4da}',
        Nerd::Folder => '\u{ea83}',
        Nerd::FolderAccount => '\u{f024c}',
        Nerd::FolderAccountOutline => '\u{f0b9c}',
        Nerd::FolderActive => '\u{eaf6}',
        Nerd::FolderAlert => '\u{f0dcc}',
        Nerd::FolderAlertOutline => '\u{f0dcd}',
        Nerd::FolderArrowDown => '\u{f19e8}',
        Nerd::FolderArrowDownOutline => '\u{f19e9}',
        Nerd::FolderArrowLeft => '\u{f19ea}',
        Nerd::FolderArrowLeftOutline => '\u{f19eb}',
        Nerd::FolderArrowLeftRight => '\u{f19ec}',
        Nerd::FolderArrowLeftRightOutline => '\u{f19ed}',
        Nerd::FolderArrowRight => '\u{f19ee}',
        Nerd::FolderArrowRightOutline => '\u{f19ef}',
        Nerd::FolderArrowUp => '\u{f19f0}',
        Nerd::FolderArrowUpDown => '\u{f19f1}',
        Nerd::FolderArrowUpDownOutline => '\u{f19f2}',
        Nerd::FolderArrowUpOutline => '\u{f19f3}',
        Nerd::FolderCancel => '\u{f19f4}',
        Nerd::FolderCancelOutline => '\u{f19f5}',
        Nerd::FolderCheck => '\u{f197e}',
        Nerd::FolderCheckOutline => '\u{f197f}',
        Nerd::FolderClock => '\u{f0aba}',
        Nerd::FolderClockOutline => '\u{f0abb}',
        Nerd::FolderClose => '\u{f07b}',
        Nerd::FolderCloseAlt => '\u{f114}',
        Nerd::FolderCog => '\u{f107f}',
        Nerd::FolderCogOutline => '\u{f1080}',
        Nerd::FolderDownload => '\u{f024d}',
        Nerd::FolderDownloadOutline => '\u{f10e9}',
        Nerd::FolderEdit => '\u{f08de}',
        Nerd::FolderEditOutline => '\u{f0dce}',
        Nerd::FolderEye => '\u{f178a}',
        Nerd::FolderEyeOutline => '\u{f178b}',
        Nerd::FolderFile => '\u{f19f6}',
        Nerd::FolderFileOutline => '\u{f19f7}',
        Nerd::FolderGoogleDrive => '\u{f024e}',
        Nerd::FolderHeart => '\u{f10ea}',
        Nerd::FolderHeartOutline => '\u{f10eb}',
        Nerd::FolderHidden => '\u{f179e}',
        Nerd::FolderHome => '\u{f10b5}',
        Nerd::FolderHomeOutline => '\u{f10b6}',
        Nerd::FolderImage => '\u{f024f}',
        Nerd::FolderInformation => '\u{f10b7}',
        Nerd::FolderInformationOutline => '\u{f10b8}',
        Nerd::FolderKey => '\u{f08ac}',
        Nerd::FolderKeyNetwork => '\u{f08ad}',
        Nerd::FolderKeyNetworkOutline => '\u{f0c80}',
        Nerd::FolderKeyOutline => '\u{f10ec}',
        Nerd::FolderLibrary => '\u{ebdf}',
        Nerd::FolderLock => '\u{f0250}',
        Nerd::FolderLockOpen => '\u{f0251}',
        Nerd::FolderLockOpenOutline => '\u{f1aa7}',
        Nerd::FolderLockOutline => '\u{f1aa8}',
        Nerd::FolderMarker => '\u{f126d}',
        Nerd::FolderMarkerOutline => '\u{f126e}',
        Nerd::FolderMove => '\u{f0252}',
        Nerd::FolderMoveOutline => '\u{f1246}',
        Nerd::FolderMultiple => '\u{f0253}',
        Nerd::FolderMultipleImage => '\u{f0254}',
        Nerd::FolderMultipleOutline => '\u{f0255}',
        Nerd::FolderMultiplePlus => '\u{f147e}',
        Nerd::FolderMultiplePlusOutline => '\u{f147f}',
        Nerd::FolderMusic => '\u{f1359}',
        Nerd::FolderMusicOutline => '\u{f135a}',
        Nerd::FolderNetwork => '\u{f0870}',
        Nerd::FolderNetworkOutline => '\u{f0c81}',
        Nerd::FolderOff => '\u{f19f8}',
        Nerd::FolderOffOutline => '\u{f19f9}',
        Nerd::FolderOne => '\u{f024b}',
        Nerd::FolderOpen => '\u{f0770}',
        Nerd::FolderOpenAlt => '\u{f115}',
        Nerd::FolderOpenOutline => '\u{f0dcf}',
        Nerd::FolderOpened => '\u{eaf7}',
        Nerd::FolderOutline => '\u{f0256}',
        Nerd::FolderPlay => '\u{f19fa}',
        Nerd::FolderPlayOutline => '\u{f19fb}',
        Nerd::FolderPlus => '\u{f0257}',
        Nerd::FolderPlusOutline => '\u{f0b9d}',
        Nerd::FolderPound => '\u{f0d09}',
        Nerd::FolderPoundOutline => '\u{f0d0a}',
        Nerd::FolderQuestion => '\u{f19ca}',
        Nerd::FolderQuestionOutline => '\u{f19cb}',
        Nerd::FolderRefresh => '\u{f0749}',
        Nerd::FolderRefreshOutline => '\u{f0542}',
        Nerd::FolderRemove => '\u{f0258}',
        Nerd::FolderRemoveOutline => '\u{f0b9e}',
        Nerd::FolderSearch => '\u{f0968}',
        Nerd::FolderSearchOutline => '\u{f0969}',
        Nerd::FolderSettings => '\u{f107d}',
        Nerd::FolderSettingsOutline => '\u{f107e}',
        Nerd::FolderStar => '\u{f069d}',
        Nerd::FolderStarMultiple => '\u{f13d3}',
        Nerd::FolderStarMultipleOutline => '\u{f13d4}',
        Nerd::FolderStarOutline => '\u{f0b9f}',
        Nerd::FolderSwap => '\u{f0fb6}',
        Nerd::FolderSwapOutline => '\u{f0fb7}',
        Nerd::FolderSync => '\u{f0d0b}',
        Nerd::FolderSyncOutline => '\u{f0d0c}',
        Nerd::FolderTable => '\u{f12e3}',
        Nerd::FolderTableOutline => '\u{f12e4}',
        Nerd::FolderText => '\u{f0c82}',
        Nerd::FolderTextOutline => '\u{f0c83}',
        Nerd::FolderUpload => '\u{f0259}',
        Nerd::FolderUploadOutline => '\u{f10ed}',
        Nerd::FolderWrench => '\u{f19fc}',
        Nerd::FolderWrenchOutline => '\u{f19fd}',
        Nerd::FolderZip => '\u{f06eb}',
        Nerd::FolderZipOutline => '\u{f07b9}',
        Nerd::Foneab => '\u{f1ab}',
        Nerd::Foneafour => '\u{f1a4}',
        Nerd::Foneaone => '\u{f1a1}',
        Nerd::Fonefc => '\u{f1fc}',
        Nerd::Fonefthree => '\u{f1f3}',
        Nerd::Fonesevenone => '\u{f171}',
        Nerd::Font => '\u{f031}',
        Nerd::FontAwesome => '\u{f003a}',
        Nerd::Food => '\u{f0f5}',
        Nerd::FoodApple => '\u{f025b}',
        Nerd::FoodAppleOutline => '\u{f0c84}',
        Nerd::FoodCroissant => '\u{f07c8}',
        Nerd::FoodDrumstick => '\u{f141f}',
        Nerd::FoodDrumstickOff => '\u{f1468}',
        Nerd::FoodDrumstickOffOutline => '\u{f1469}',
        Nerd::FoodDrumstickOutline => '\u{f1420}',
        Nerd::FoodForkDrink => '\u{f05f2}',
        Nerd::FoodHalal => '\u{f1572}',
        Nerd::FoodHotDog => '\u{f184b}',
        Nerd::FoodKosher => '\u{f1573}',
        Nerd::FoodOff => '\u{f05f3}',
        Nerd::FoodOffOutline => '\u{f1915}',
        Nerd::FoodOne => '\u{f025a}',
        Nerd::FoodOutline => '\u{f1916}',
        Nerd::FoodSteak => '\u{f146a}',
        Nerd::FoodSteakOff => '\u{f146b}',
        Nerd::FoodTakeoutBox => '\u{f1836}',
        Nerd::FoodTakeoutBoxOutline => '\u{f1837}',
        Nerd::FoodTurkey => '\u{f171c}',
        Nerd::FoodVariant => '\u{f025c}',
        Nerd::FoodVariantOff => '\u{f13e5}',
        Nerd::FootPrint => '\u{f0f52}',
        Nerd::Football => '\u{f025d}',
        Nerd::FootballAustralian => '\u{f025e}',
        Nerd::FootballHelmet => '\u{f025f}',
        Nerd::Footprint => '\u{e241}',
        Nerd::Forest => '\u{f1897}',
        Nerd::Forgejo => '\u{f335}',
        Nerd::Forklift => '\u{f07c9}',
        Nerd::FormDropdown => '\u{f1400}',
        Nerd::FormSelect => '\u{f1401}',
        Nerd::FormTextarea => '\u{f1095}',
        Nerd::FormTextbox => '\u{f060e}',
        Nerd::FormTextboxLock => '\u{f135d}',
        Nerd::FormTextboxPassword => '\u{f07f5}',
        Nerd::FormatAlignBottom => '\u{f0753}',
        Nerd::FormatAlignCenter => '\u{f0260}',
        Nerd::FormatAlignJustify => '\u{f0261}',
        Nerd::FormatAlignLeft => '\u{f0262}',
        Nerd::FormatAlignMiddle => '\u{f0754}',
        Nerd::FormatAlignRight => '\u{f0263}',
        Nerd::FormatAlignTop => '\u{f0755}',
        Nerd::FormatAnnotationMinus => '\u{f0abc}',
        Nerd::FormatAnnotationPlus => '\u{f0646}',
        Nerd::FormatBold => '\u{f0264}',
        Nerd::FormatClear => '\u{f0265}',
        Nerd::FormatColorFill => '\u{f0266}',
        Nerd::FormatColorHighlight => '\u{f0e31}',
        Nerd::FormatColorMarkerCancel => '\u{f1313}',
        Nerd::FormatColorText => '\u{f069e}',
        Nerd::FormatColumns => '\u{f08df}',
        Nerd::FormatFloatCenter => '\u{f0267}',
        Nerd::FormatFloatLeft => '\u{f0268}',
        Nerd::FormatFloatNone => '\u{f0269}',
        Nerd::FormatFloatRight => '\u{f026a}',
        Nerd::FormatFont => '\u{f06d6}',
        Nerd::FormatFontSizeDecrease => '\u{f09f3}',
        Nerd::FormatFontSizeIncrease => '\u{f09f4}',
        Nerd::FormatHeaderDecrease => '\u{f0271}',
        Nerd::FormatHeaderEqual => '\u{f0272}',
        Nerd::FormatHeaderFive => '\u{f026f}',
        Nerd::FormatHeaderFour => '\u{f026e}',
        Nerd::FormatHeaderIncrease => '\u{f0273}',
        Nerd::FormatHeaderOne => '\u{f026b}',
        Nerd::FormatHeaderPound => '\u{f0274}',
        Nerd::FormatHeaderSix => '\u{f0270}',
        Nerd::FormatHeaderThree => '\u{f026d}',
        Nerd::FormatHeaderTwo => '\u{f026c}',
        Nerd::FormatHorizontalAlignCenter => '\u{f061e}',
        Nerd::FormatHorizontalAlignLeft => '\u{f061f}',
        Nerd::FormatHorizontalAlignRight => '\u{f0620}',
        Nerd::FormatIndentDecrease => '\u{f0275}',
        Nerd::FormatIndentIncrease => '\u{f0276}',
        Nerd::FormatItalic => '\u{f0277}',
        Nerd::FormatLetterCase => '\u{f0b34}',
        Nerd::FormatLetterCaseLower => '\u{f0b35}',
        Nerd::FormatLetterCaseUpper => '\u{f0b36}',
        Nerd::FormatLetterEndsWith => '\u{f0fb8}',
        Nerd::FormatLetterMatches => '\u{f0fb9}',
        Nerd::FormatLetterSpacing => '\u{f1956}',
        Nerd::FormatLetterStartsWith => '\u{f0fba}',
        Nerd::FormatLineSpacing => '\u{f0278}',
        Nerd::FormatLineStyle => '\u{f05c8}',
        Nerd::FormatLineWeight => '\u{f05c9}',
        Nerd::FormatListBulleted => '\u{f0279}',
        Nerd::FormatListBulletedSquare => '\u{f0dd0}',
        Nerd::FormatListBulletedTriangle => '\u{f0eb2}',
        Nerd::FormatListBulletedType => '\u{f027a}',
        Nerd::FormatListCheckbox => '\u{f096a}',
        Nerd::FormatListChecks => '\u{f0756}',
        Nerd::FormatListGroup => '\u{f1860}',
        Nerd::FormatListNumbered => '\u{f027b}',
        Nerd::FormatListNumberedRtl => '\u{f0d0d}',
        Nerd::FormatListText => '\u{f126f}',
        Nerd::FormatOverline => '\u{f0eb3}',
        Nerd::FormatPageBreak => '\u{f06d7}',
        Nerd::FormatPageSplit => '\u{f1917}',
        Nerd::FormatPaint => '\u{f027c}',
        Nerd::FormatParagraph => '\u{f027d}',
        Nerd::FormatPilcrow => '\u{f06d8}',
        Nerd::FormatQuoteClose => '\u{f027e}',
        Nerd::FormatQuoteCloseOutline => '\u{f11a8}',
        Nerd::FormatQuoteOpen => '\u{f0757}',
        Nerd::FormatQuoteOpenOutline => '\u{f11a7}',
        Nerd::FormatRotateNinezero => '\u{f06aa}',
        Nerd::FormatSection => '\u{f069f}',
        Nerd::FormatSize => '\u{f027f}',
        Nerd::FormatStrikethrough => '\u{f0280}',
        Nerd::FormatStrikethroughVariant => '\u{f0281}',
        Nerd::FormatSubscript => '\u{f0282}',
        Nerd::FormatSuperscript => '\u{f0283}',
        Nerd::FormatText => '\u{f0284}',
        Nerd::FormatTextRotationAngleDown => '\u{f0fbb}',
        Nerd::FormatTextRotationAngleUp => '\u{f0fbc}',
        Nerd::FormatTextRotationDown => '\u{f0d73}',
        Nerd::FormatTextRotationDownVertical => '\u{f0fbd}',
        Nerd::FormatTextRotationNone => '\u{f0d74}',
        Nerd::FormatTextRotationUp => '\u{f0fbe}',
        Nerd::FormatTextRotationVertical => '\u{f0fbf}',
        Nerd::FormatTextVariant => '\u{f0e32}',
        Nerd::FormatTextVariantOutline => '\u{f150f}',
        Nerd::FormatTextWrappingClip => '\u{f0d0e}',
        Nerd::FormatTextWrappingOverflow => '\u{f0d0f}',
        Nerd::FormatTextWrappingWrap => '\u{f0d10}',
        Nerd::FormatTextbox => '\u{f0d11}',
        Nerd::FormatTextdirectionLToR => '\u{f0285}',
        Nerd::FormatTextdirectionRToL => '\u{f0286}',
        Nerd::FormatTitle => '\u{f05f4}',
        Nerd::FormatUnderline => '\u{f0287}',
        Nerd::FormatUnderlineWavy => '\u{f18e9}',
        Nerd::FormatVerticalAlignBottom => '\u{f0621}',
        Nerd::FormatVerticalAlignCenter => '\u{f0622}',
        Nerd::FormatVerticalAlignTop => '\u{f0623}',
        Nerd::FormatWrapInline => '\u{f0288}',
        Nerd::FormatWrapSquare => '\u{f0289}',
        Nerd::FormatWrapTight => '\u{f028a}',
        Nerd::FormatWrapTopBottom => '\u{f028b}',
        Nerd::Forum => '\u{f028c}',
        Nerd::ForumMinus => '\u{f1aa9}',
        Nerd::ForumMinusOutline => '\u{f1aaa}',
        Nerd::ForumOutline => '\u{f0822}',
        Nerd::ForumPlus => '\u{f1aab}',
        Nerd::ForumPlusOutline => '\u{f1aac}',
        Nerd::ForumRemove => '\u{f1aad}',
        Nerd::ForumRemoveOutline => '\u{f1aae}',
        Nerd::Forward => '\u{f04e}',
        Nerd::ForwardOne => '\u{f028d}',
        Nerd::Forwardburger => '\u{f0d75}',
        Nerd::Fosdem => '\u{f36b}',
        Nerd::Fountain => '\u{f096b}',
        Nerd::FountainPen => '\u{f0d12}',
        Nerd::FountainPenTip => '\u{f0d13}',
        Nerd::Foureighteight => '\u{f208}',
        Nerd::Foureightfive => '\u{f205}',
        Nerd::Foureightfour => '\u{f204}',
        Nerd::Foureightnine => '\u{f209}',
        Nerd::Foureightone => '\u{f201}',
        Nerd::Foureightseven => '\u{f207}',
        Nerd::Foureightsix => '\u{f206}',
        Nerd::Foureightthree => '\u{f203}',
        Nerd::Foureighttwo => '\u{f202}',
        Nerd::Foureightzero => '\u{f200}',
        Nerd::Fourfiveeight => '\u{f1e8}',
        Nerd::Fourfivefive => '\u{f1e5}',
        Nerd::Fourfivefour => '\u{f1e4}',
        Nerd::Fourfivenine => '\u{f1e9}',
        Nerd::Fourfiveone => '\u{f1e1}',
        Nerd::Fourfiveseven => '\u{f1e7}',
        Nerd::Fourfivesix => '\u{f1e6}',
        Nerd::Fourfivethree => '\u{f1e3}',
        Nerd::Fourfivetwo => '\u{f1e2}',
        Nerd::Fourfoureight => '\u{f1dd}',
        Nerd::Fourfourfive => '\u{f1da}',
        Nerd::Fourfourfour => '\u{f1d9}',
        Nerd::Fourfournine => '\u{f1de}',
        Nerd::Fourfourseven => '\u{f1dc}',
        Nerd::Fourfoursix => '\u{f1db}',
        Nerd::Fourfourthree => '\u{f1d8}',
        Nerd::Fournineeight => '\u{f213}',
        Nerd::Fourninefour => '\u{f20e}',
        Nerd::Fourninenine => '\u{f214}',
        Nerd::Fournineone => '\u{f20b}',
        Nerd::Fourninesix => '\u{f211}',
        Nerd::Fourninethree => '\u{f20d}',
        Nerd::Fourninetwo => '\u{f20c}',
        Nerd::Fourninezero => '\u{f20a}',
        Nerd::Fouroneeight => '\u{f1bd}',
        Nerd::Fouronefive => '\u{f1ba}',
        Nerd::Fouronefour => '\u{f1b9}',
        Nerd::Fouronenine => '\u{f1be}',
        Nerd::Fouroneone => '\u{f1b6}',
        Nerd::Fouroneseven => '\u{f1bc}',
        Nerd::Fouronesix => '\u{f1bb}',
        Nerd::Fouronethree => '\u{f1b8}',
        Nerd::Fouronetwo => '\u{f1b7}',
        Nerd::Fouronezero => '\u{f1b5}',
        Nerd::Fourseveneight => '\u{f1fd}',
        Nerd::Foursevenfive => '\u{f1fa}',
        Nerd::Foursevenfour => '\u{f1f9}',
        Nerd::Foursevennine => '\u{f1fe}',
        Nerd::Foursevenone => '\u{f1f6}',
        Nerd::Foursevensix => '\u{f1fb}',
        Nerd::Fourseventhree => '\u{f1f8}',
        Nerd::Fourseventwo => '\u{f1f7}',
        Nerd::Foursevenzero => '\u{f1f5}',
        Nerd::Foursixfour => '\u{f1ee}',
        Nerd::Foursixnine => '\u{f1f4}',
        Nerd::Foursixone => '\u{f1eb}',
        Nerd::Foursixseven => '\u{f1f2}',
        Nerd::Foursixsix => '\u{f1f1}',
        Nerd::Foursixthree => '\u{f1ed}',
        Nerd::Foursixtwo => '\u{f1ec}',
        Nerd::Foursixzero => '\u{f1ea}',
        Nerd::Foursquare => '\u{f180}',
        Nerd::Fourthreeeight => '\u{f1d3}',
        Nerd::Fourthreefour => '\u{f1ce}',
        Nerd::Fourthreenine => '\u{f1d4}',
        Nerd::Fourthreeone => '\u{f1cb}',
        Nerd::Fourthreethree => '\u{f1cd}',
        Nerd::Fourthreetwo => '\u{f1cc}',
        Nerd::Fourthreezero => '\u{f1ca}',
        Nerd::Fourtwoeight => '\u{f1c8}',
        Nerd::Fourtwofive => '\u{f1c5}',
        Nerd::Fourtwofour => '\u{f1c4}',
        Nerd::Fourtwonine => '\u{f1c9}',
        Nerd::Fourtwoseven => '\u{f1c7}',
        Nerd::Fourtwosix => '\u{f1c6}',
        Nerd::Fourtwothree => '\u{f1c3}',
        Nerd::Fourtwotwo => '\u{f1c2}',
        Nerd::Fourzeroeight => '\u{f1b3}',
        Nerd::Fourzerofour => '\u{f1ae}',
        Nerd::Fourzeronine => '\u{f1b4}',
        Nerd::Fourzeroseven => '\u{f1b2}',
        Nerd::Fourzerosix => '\u{f1b1}',
        Nerd::Fourzerothree => '\u{f1ad}',
        Nerd::Fourzerotwo => '\u{f1ac}',
        Nerd::Fourzerozero => '\u{f1aa}',
        Nerd::FractionOneHalf => '\u{f1992}',
        Nerd::Freebsd => '\u{f08e0}',
        Nerd::Freecad => '\u{f336}',
        Nerd::Freecodecamp => '\u{e242}',
        Nerd::FreedesktopOrg => '\u{f360}',
        Nerd::FrenchFries => '\u{f1957}',
        Nerd::FrequentlyAskedQuestions => '\u{f0eb4}',
        Nerd::Fridge => '\u{f0290}',
        Nerd::FridgeAlert => '\u{f11b1}',
        Nerd::FridgeAlertOutline => '\u{f11b2}',
        Nerd::FridgeBottom => '\u{f0292}',
        Nerd::FridgeIndustrial => '\u{f15ee}',
        Nerd::FridgeIndustrialAlert => '\u{f15ef}',
        Nerd::FridgeIndustrialAlertOutline => '\u{f15f0}',
        Nerd::FridgeIndustrialOff => '\u{f15f1}',
        Nerd::FridgeIndustrialOffOutline => '\u{f15f2}',
        Nerd::FridgeIndustrialOutline => '\u{f15f3}',
        Nerd::FridgeOff => '\u{f11af}',
        Nerd::FridgeOffOutline => '\u{f11b0}',
        Nerd::FridgeOutline => '\u{f028f}',
        Nerd::FridgeTop => '\u{f0291}',
        Nerd::FridgeVariant => '\u{f15f4}',
        Nerd::FridgeVariantAlert => '\u{f15f5}',
        Nerd::FridgeVariantAlertOutline => '\u{f15f6}',
        Nerd::FridgeVariantOff => '\u{f15f7}',
        Nerd::FridgeVariantOffOutline => '\u{f15f8}',
        Nerd::FridgeVariantOutline => '\u{f15f9}',
        Nerd::Frown => '\u{f119}',
        Nerd::FruitCherries => '\u{f1042}',
        Nerd::FruitCherriesOff => '\u{f13f8}',
        Nerd::FruitCitrus => '\u{f1043}',
        Nerd::FruitCitrusOff => '\u{f13f9}',
        Nerd::FruitGrapes => '\u{f1044}',
        Nerd::FruitGrapesOutline => '\u{f1045}',
        Nerd::FruitPear => '\u{f1a0e}',
        Nerd::FruitPineapple => '\u{f1046}',
        Nerd::FruitWatermelon => '\u{f1047}',
        Nerd::Ftwoonetwo => '\u{f212}',
        Nerd::Ftwoonezero => '\u{f210}',
        Nerd::Ftwosevene => '\u{f27e}',
        Nerd::Ftwosixone => '\u{f261}',
        Nerd::Ftwosixthree => '\u{f263}',
        Nerd::Ftwosixzero => '\u{f260}',
        Nerd::Fuel => '\u{f07ca}',
        Nerd::FuelCell => '\u{f18b5}',
        Nerd::Fullscreen => '\u{f0b2}',
        Nerd::FullscreenExit => '\u{f0294}',
        Nerd::FullscreenOne => '\u{f0293}',
        Nerd::Function => '\u{f0295}',
        Nerd::FunctionVariant => '\u{f0871}',
        Nerd::FuriganaHorizontal => '\u{f1081}',
        Nerd::FuriganaVertical => '\u{f1082}',
        Nerd::Fuse => '\u{f0c85}',
        Nerd::FuseAlert => '\u{f142d}',
        Nerd::FuseBlade => '\u{f0c86}',
        Nerd::FuseOff => '\u{f142c}',
        Nerd::Fzerofe => '\u{f0fe}',
        Nerd::Galaxy => '\u{e243}',
        Nerd::Galery => '\u{e244}',
        Nerd::Gamepad => '\u{f11b}',
        Nerd::GamepadCircle => '\u{f0e33}',
        Nerd::GamepadCircleDown => '\u{f0e34}',
        Nerd::GamepadCircleLeft => '\u{f0e35}',
        Nerd::GamepadCircleOutline => '\u{f0e36}',
        Nerd::GamepadCircleRight => '\u{f0e37}',
        Nerd::GamepadCircleUp => '\u{f0e38}',
        Nerd::GamepadDown => '\u{f0e39}',
        Nerd::GamepadLeft => '\u{f0e3a}',
        Nerd::GamepadOne => '\u{f0296}',
        Nerd::GamepadOutline => '\u{f1919}',
        Nerd::GamepadRight => '\u{f0e3b}',
        Nerd::GamepadRound => '\u{f0e3c}',
        Nerd::GamepadRoundDown => '\u{f0e3d}',
        Nerd::GamepadRoundLeft => '\u{f0e3e}',
        Nerd::GamepadRoundOutline => '\u{f0e3f}',
        Nerd::GamepadRoundRight => '\u{f0e40}',
        Nerd::GamepadRoundUp => '\u{f0e41}',
        Nerd::GamepadSquare => '\u{f0eb5}',
        Nerd::GamepadSquareOutline => '\u{f0eb6}',
        Nerd::GamepadUp => '\u{f0e42}',
        Nerd::GamepadVariant => '\u{f0297}',
        Nerd::GamepadVariantOutline => '\u{f0eb7}',
        Nerd::Gamma => '\u{f10ee}',
        Nerd::GantryCrane => '\u{f0dd1}',
        Nerd::Garage => '\u{f06d9}',
        Nerd::GarageAlert => '\u{f0872}',
        Nerd::GarageAlertVariant => '\u{f12d5}',
        Nerd::GarageLock => '\u{f17fb}',
        Nerd::GarageOpen => '\u{f06da}',
        Nerd::GarageOpenVariant => '\u{f12d4}',
        Nerd::GarageVariant => '\u{f12d3}',
        Nerd::GarageVariantLock => '\u{f17fc}',
        Nerd::GarudaLinux => '\u{f337}',
        Nerd::GasBurner => '\u{f1a1b}',
        Nerd::GasCylinder => '\u{f0647}',
        Nerd::GasStation => '\u{f0298}',
        Nerd::GasStationOff => '\u{f1409}',
        Nerd::GasStationOffOutline => '\u{f140a}',
        Nerd::GasStationOutline => '\u{f0eb8}',
        Nerd::Gate => '\u{f0299}',
        Nerd::GateAlert => '\u{f17f8}',
        Nerd::GateAnd => '\u{f08e1}',
        Nerd::GateArrowLeft => '\u{f17f7}',
        Nerd::GateArrowRight => '\u{f1169}',
        Nerd::GateNand => '\u{f08e2}',
        Nerd::GateNor => '\u{f08e3}',
        Nerd::GateNot => '\u{f08e4}',
        Nerd::GateOpen => '\u{f116a}',
        Nerd::GateOr => '\u{f08e5}',
        Nerd::GateXnor => '\u{f08e6}',
        Nerd::GateXor => '\u{f08e7}',
        Nerd::Gatsby => '\u{f0e43}',
        Nerd::Gauge => '\u{f029a}',
        Nerd::GaugeEmpty => '\u{f0873}',
        Nerd::GaugeFull => '\u{f0874}',
        Nerd::GaugeLow => '\u{f0875}',
        Nerd::Gavel => '\u{f029b}',
        Nerd::Gbp => '\u{f154}',
        Nerd::Gear => '\u{eaf8}',
        Nerd::GearOne => '\u{f423}',
        Nerd::GenderFemale => '\u{f029c}',
        Nerd::GenderMale => '\u{f029d}',
        Nerd::GenderMaleFemale => '\u{f029e}',
        Nerd::GenderMaleFemaleVariant => '\u{f113f}',
        Nerd::GenderNonBinary => '\u{f1140}',
        Nerd::GenderTransgender => '\u{f029f}',
        Nerd::Gentoo => '\u{f08e8}',
        Nerd::Gesture => '\u{f07cb}',
        Nerd::GestureDoubleTap => '\u{f073c}',
        Nerd::GesturePinch => '\u{f0abd}',
        Nerd::GestureSpread => '\u{f0abe}',
        Nerd::GestureSwipe => '\u{f0d76}',
        Nerd::GestureSwipeDown => '\u{f073d}',
        Nerd::GestureSwipeHorizontal => '\u{f0abf}',
        Nerd::GestureSwipeLeft => '\u{f073e}',
        Nerd::GestureSwipeRight => '\u{f073f}',
        Nerd::GestureSwipeUp => '\u{f0740}',
        Nerd::GestureSwipeVertical => '\u{f0ac0}',
        Nerd::GestureTap => '\u{f0741}',
        Nerd::GestureTapBox => '\u{f12a9}',
        Nerd::GestureTapButton => '\u{f12a8}',
        Nerd::GestureTapHold => '\u{f0d77}',
        Nerd::GestureTwoDoubleTap => '\u{f0742}',
        Nerd::GestureTwoTap => '\u{f0743}',
        Nerd::Ghost => '\u{f02a0}',
        Nerd::GhostOff => '\u{f09f5}',
        Nerd::GhostOffOutline => '\u{f165c}',
        Nerd::GhostOutline => '\u{f165d}',
        Nerd::Gift => '\u{eaf9}',
        Nerd::GiftCard => '\u{e2a0}',
        Nerd::GiftOff => '\u{f16ef}',
        Nerd::GiftOffOutline => '\u{f16f0}',
        Nerd::GiftOne => '\u{f06b}',
        Nerd::GiftOpen => '\u{f16f1}',
        Nerd::GiftOpenOutline => '\u{f16f2}',
        Nerd::GiftOutline => '\u{f02a1}',
        Nerd::GiftThree => '\u{f0e44}',
        Nerd::GiftTwo => '\u{f436}',
        Nerd::Gimp => '\u{f338}',
        Nerd::GistSecret => '\u{eafa}',
        Nerd::Git => '\u{f02a2}',
        Nerd::GitBranch => '\u{f418}',
        Nerd::GitCommit => '\u{eafc}',
        Nerd::GitCommitOne => '\u{f417}',
        Nerd::GitCompare => '\u{eafd}',
        Nerd::GitCompareOne => '\u{f47f}',
        Nerd::GitMerge => '\u{eafe}',
        Nerd::GitMergeOne => '\u{f419}',
        Nerd::GitMergeQueue => '\u{f4db}',
        Nerd::GitPullRequest => '\u{ea64}',
        Nerd::GitPullRequestClosed => '\u{ebda}',
        Nerd::GitPullRequestClosedOne => '\u{f4dc}',
        Nerd::GitPullRequestCreate => '\u{ebbc}',
        Nerd::GitPullRequestDraft => '\u{ebdb}',
        Nerd::GitPullRequestDraftOne => '\u{f4dd}',
        Nerd::GitPullRequestOne => '\u{f407}',
        Nerd::Gitea => '\u{f339}',
        Nerd::Github => '\u{ea84}',
        Nerd::GithubAction => '\u{eaff}',
        Nerd::GithubAlt => '\u{f113}',
        Nerd::GithubInverted => '\u{eba1}',
        Nerd::GithubOne => '\u{f09b}',
        Nerd::GithubSign => '\u{f092}',
        Nerd::GithubTwo => '\u{f02a4}',
        Nerd::Gitlab => '\u{f0ba0}',
        Nerd::Gittip => '\u{f184}',
        Nerd::Glass => '\u{e245}',
        Nerd::GlassCocktail => '\u{f0356}',
        Nerd::GlassCocktailOff => '\u{f15e6}',
        Nerd::GlassFlute => '\u{f02a5}',
        Nerd::GlassFragile => '\u{f1873}',
        Nerd::GlassMug => '\u{f02a6}',
        Nerd::GlassMugOff => '\u{f15e7}',
        Nerd::GlassMugVariant => '\u{f1116}',
        Nerd::GlassMugVariantOff => '\u{f15e8}',
        Nerd::GlassOne => '\u{f000}',
        Nerd::GlassPintOutline => '\u{f130d}',
        Nerd::GlassStange => '\u{f02a7}',
        Nerd::GlassTulip => '\u{f02a8}',
        Nerd::GlassWine => '\u{f0876}',
        Nerd::Glasses => '\u{f02aa}',
        Nerd::Globe => '\u{eb01}',
        Nerd::GlobeLight => '\u{f12d7}',
        Nerd::GlobeModel => '\u{f08e9}',
        Nerd::GlobeOne => '\u{f0ac}',
        Nerd::GlobeTwo => '\u{f484}',
        Nerd::Gmail => '\u{f02ab}',
        Nerd::Gnome => '\u{f02ac}',
        Nerd::GnuGuix => '\u{f325}',
        Nerd::GoKart => '\u{f0d79}',
        Nerd::GoKartTrack => '\u{f0d7a}',
        Nerd::GoToFile => '\u{ea94}',
        Nerd::Goal => '\u{f4de}',
        Nerd::Gog => '\u{f0ba1}',
        Nerd::Gold => '\u{f124f}',
        Nerd::Golf => '\u{f0823}',
        Nerd::GolfCart => '\u{f11a4}',
        Nerd::GolfTee => '\u{f1083}',
        Nerd::Gondola => '\u{f0686}',
        Nerd::Goodreads => '\u{f0d7b}',
        Nerd::Google => '\u{f02ad}',
        Nerd::GoogleAds => '\u{f0c87}',
        Nerd::GoogleAnalytics => '\u{f07cc}',
        Nerd::GoogleAssistant => '\u{f07cd}',
        Nerd::GoogleCardboard => '\u{f02ae}',
        Nerd::GoogleChrome => '\u{f02af}',
        Nerd::GoogleCircles => '\u{f02b0}',
        Nerd::GoogleCirclesCommunities => '\u{f02b1}',
        Nerd::GoogleCirclesExtended => '\u{f02b2}',
        Nerd::GoogleCirclesGroup => '\u{f02b3}',
        Nerd::GoogleClassroom => '\u{f02c0}',
        Nerd::GoogleCloud => '\u{f11f6}',
        Nerd::GoogleController => '\u{f02b4}',
        Nerd::GoogleControllerOff => '\u{f02b5}',
        Nerd::GoogleDownasaur => '\u{f1362}',
        Nerd::GoogleDrive => '\u{e246}',
        Nerd::GoogleDriveOne => '\u{f02b6}',
        Nerd::GoogleEarth => '\u{f02b7}',
        Nerd::GoogleFit => '\u{f096c}',
        Nerd::GoogleGlass => '\u{f02b8}',
        Nerd::GoogleHangouts => '\u{f02c9}',
        Nerd::GoogleHome => '\u{f0824}',
        Nerd::GoogleKeep => '\u{f06dc}',
        Nerd::GoogleLens => '\u{f09f6}',
        Nerd::GoogleMaps => '\u{f05f5}',
        Nerd::GoogleMyBusiness => '\u{f1048}',
        Nerd::GoogleNearby => '\u{f02b9}',
        Nerd::GooglePlay => '\u{e247}',
        Nerd::GooglePlayOne => '\u{f02bc}',
        Nerd::GooglePlus => '\u{f02bd}',
        Nerd::GooglePlusSign => '\u{f0d4}',
        Nerd::GooglePodcast => '\u{f0eb9}',
        Nerd::GoogleSpreadsheet => '\u{f09f7}',
        Nerd::GoogleStreetView => '\u{f0c88}',
        Nerd::GoogleTranslate => '\u{f02bf}',
        Nerd::Gps => '\u{e248}',
        Nerd::Grabber => '\u{eb02}',
        Nerd::GrabberOne => '\u{f4a6}',
        Nerd::GradientHorizontal => '\u{f174a}',
        Nerd::GradientVertical => '\u{f06a0}',
        Nerd::Grain => '\u{f0d7c}',
        Nerd::Graph => '\u{eb03}',
        Nerd::GraphLeft => '\u{ebad}',
        Nerd::GraphLine => '\u{ebe2}',
        Nerd::GraphOne => '\u{f437}',
        Nerd::GraphOutline => '\u{f104a}',
        Nerd::GraphScatter => '\u{ebe3}',
        Nerd::GraphTwo => '\u{f1049}',
        Nerd::Graphql => '\u{f0877}',
        Nerd::Grass => '\u{f1510}',
        Nerd::Grav => '\u{e249}',
        Nerd::GraveStone => '\u{f0ba2}',
        Nerd::GreasePencil => '\u{f0648}',
        Nerd::GreaterThan => '\u{f096d}',
        Nerd::GreaterThanOrEqual => '\u{f096e}',
        Nerd::Greenhouse => '\u{f002d}',
        Nerd::Grid => '\u{f02c1}',
        Nerd::GridLarge => '\u{f0758}',
        Nerd::GridOff => '\u{f02c2}',
        Nerd::Grill => '\u{f0e45}',
        Nerd::GrillOutline => '\u{f118a}',
        Nerd::Gripper => '\u{eb04}',
        Nerd::Group => '\u{f0c0}',
        Nerd::GroupByRefType => '\u{eb97}',
        Nerd::GroupOne => '\u{f02c3}',
        Nerd::Gtk => '\u{f362}',
        Nerd::Guitar => '\u{e24a}',
        Nerd::GuitarAcoustic => '\u{f0771}',
        Nerd::GuitarElectric => '\u{f02c4}',
        Nerd::GuitarPick => '\u{f02c5}',
        Nerd::GuitarPickOutline => '\u{f02c6}',
        Nerd::Gut => '\u{e24b}',
        Nerd::GuyFawkesMask => '\u{f0825}',
        Nerd::Gymnastics => '\u{f1a41}',
        Nerd::HSign => '\u{f0fd}',
        Nerd::Hail => '\u{f0ac1}',
        Nerd::HairDryer => '\u{f10ef}',
        Nerd::HairDryerOutline => '\u{f10f0}',
        Nerd::Halloween => '\u{f0ba3}',
        Nerd::Halter => '\u{e24c}',
        Nerd::Hamburger => '\u{e24d}',
        Nerd::HamburgerCheck => '\u{f1776}',
        Nerd::HamburgerMinus => '\u{f1777}',
        Nerd::HamburgerOff => '\u{f1778}',
        Nerd::HamburgerOne => '\u{f0685}',
        Nerd::HamburgerPlus => '\u{f1779}',
        Nerd::HamburgerRemove => '\u{f177a}',
        Nerd::Hammer => '\u{f08ea}',
        Nerd::HammerScrewdriver => '\u{f1322}',
        Nerd::HammerSickle => '\u{f1887}',
        Nerd::HammerWrench => '\u{f1323}',
        Nerd::HandBackLeft => '\u{f0e46}',
        Nerd::HandBackLeftOff => '\u{f1830}',
        Nerd::HandBackLeftOffOutline => '\u{f1832}',
        Nerd::HandBackLeftOutline => '\u{f182c}',
        Nerd::HandBackRight => '\u{f0e47}',
        Nerd::HandBackRightOff => '\u{f1831}',
        Nerd::HandBackRightOffOutline => '\u{f1833}',
        Nerd::HandBackRightOutline => '\u{f182d}',
        Nerd::HandClap => '\u{f194b}',
        Nerd::HandClapOff => '\u{f1a42}',
        Nerd::HandCoin => '\u{f188f}',
        Nerd::HandCoinOutline => '\u{f1890}',
        Nerd::HandDown => '\u{f0a7}',
        Nerd::HandExtended => '\u{f18b6}',
        Nerd::HandExtendedOutline => '\u{f18b7}',
        Nerd::HandFrontLeft => '\u{f182b}',
        Nerd::HandFrontLeftOutline => '\u{f182e}',
        Nerd::HandFrontRight => '\u{f0a4f}',
        Nerd::HandFrontRightOutline => '\u{f182f}',
        Nerd::HandHeart => '\u{f10f1}',
        Nerd::HandHeartOutline => '\u{f157e}',
        Nerd::HandLeft => '\u{f0a5}',
        Nerd::HandOkay => '\u{f0a50}',
        Nerd::HandPeace => '\u{f0a51}',
        Nerd::HandPeaceVariant => '\u{f0a52}',
        Nerd::HandPointingDown => '\u{f0a53}',
        Nerd::HandPointingLeft => '\u{f0a54}',
        Nerd::HandPointingRight => '\u{f02c7}',
        Nerd::HandPointingUp => '\u{f0a55}',
        Nerd::HandRight => '\u{f0a4}',
        Nerd::HandSaw => '\u{f0e48}',
        Nerd::HandUp => '\u{f0a6}',
        Nerd::HandWash => '\u{f157f}',
        Nerd::HandWashOutline => '\u{f1580}',
        Nerd::HandWater => '\u{f139f}',
        Nerd::HandWave => '\u{f1821}',
        Nerd::HandWaveOutline => '\u{f1822}',
        Nerd::Handball => '\u{f0f53}',
        Nerd::Handcuffs => '\u{f113e}',
        Nerd::HandsPray => '\u{f0579}',
        Nerd::Handshake => '\u{f1218}',
        Nerd::HandshakeOutline => '\u{f15a1}',
        Nerd::Hanger => '\u{f02c8}',
        Nerd::HardHat => '\u{f096f}',
        Nerd::Harddisk => '\u{f02ca}',
        Nerd::HarddiskPlus => '\u{f104b}',
        Nerd::HarddiskRemove => '\u{f104c}',
        Nerd::Hash => '\u{f4df}',
        Nerd::Hat => '\u{e24e}',
        Nerd::HatFedora => '\u{f0ba4}',
        Nerd::HazardLights => '\u{f0c89}',
        Nerd::Hdd => '\u{f0a0}',
        Nerd::Hdr => '\u{f0d7d}',
        Nerd::HdrOff => '\u{f0d7e}',
        Nerd::Head => '\u{f135e}',
        Nerd::HeadAlert => '\u{f1338}',
        Nerd::HeadAlertOutline => '\u{f1339}',
        Nerd::HeadCheck => '\u{f133a}',
        Nerd::HeadCheckOutline => '\u{f133b}',
        Nerd::HeadCog => '\u{f133c}',
        Nerd::HeadCogOutline => '\u{f133d}',
        Nerd::HeadDotsHorizontal => '\u{f133e}',
        Nerd::HeadDotsHorizontalOutline => '\u{f133f}',
        Nerd::HeadFlash => '\u{f1340}',
        Nerd::HeadFlashOutline => '\u{f1341}',
        Nerd::HeadHeart => '\u{f1342}',
        Nerd::HeadHeartOutline => '\u{f1343}',
        Nerd::HeadLightbulb => '\u{f1344}',
        Nerd::HeadLightbulbOutline => '\u{f1345}',
        Nerd::HeadMinus => '\u{f1346}',
        Nerd::HeadMinusOutline => '\u{f1347}',
        Nerd::HeadOutline => '\u{f135f}',
        Nerd::HeadPlus => '\u{f1348}',
        Nerd::HeadPlusOutline => '\u{f1349}',
        Nerd::HeadQuestion => '\u{f134a}',
        Nerd::HeadQuestionOutline => '\u{f134b}',
        Nerd::HeadRemove => '\u{f134c}',
        Nerd::HeadRemoveOutline => '\u{f134d}',
        Nerd::HeadSnowflake => '\u{f134e}',
        Nerd::HeadSnowflakeOutline => '\u{f134f}',
        Nerd::HeadSync => '\u{f1350}',
        Nerd::HeadSyncOutline => '\u{f1351}',
        Nerd::Heading => '\u{f4e0}',
        Nerd::Headphones => '\u{f025}',
        Nerd::HeadphonesBluetooth => '\u{f0970}',
        Nerd::HeadphonesBox => '\u{f02cc}',
        Nerd::HeadphonesOff => '\u{f07ce}',
        Nerd::HeadphonesOne => '\u{f02cb}',
        Nerd::HeadphonesSettings => '\u{f02cd}',
        Nerd::Headset => '\u{f02ce}',
        Nerd::HeadsetDock => '\u{f02cf}',
        Nerd::HeadsetOff => '\u{f02d0}',
        Nerd::Heart => '\u{2665}',
        Nerd::HeartBox => '\u{f02d2}',
        Nerd::HeartBoxOutline => '\u{f02d3}',
        Nerd::HeartBroken => '\u{f02d4}',
        Nerd::HeartBrokenOutline => '\u{f0d14}',
        Nerd::HeartCircle => '\u{f0971}',
        Nerd::HeartCircleOutline => '\u{f0972}',
        Nerd::HeartCog => '\u{f1663}',
        Nerd::HeartCogOutline => '\u{f1664}',
        Nerd::HeartEmpty => '\u{f08a}',
        Nerd::HeartFill => '\u{f4e1}',
        Nerd::HeartFlash => '\u{f0ef9}',
        Nerd::HeartFour => '\u{f08d0}',
        Nerd::HeartHalf => '\u{f06df}',
        Nerd::HeartHalfFull => '\u{f06de}',
        Nerd::HeartHalfOutline => '\u{f06e0}',
        Nerd::HeartMinus => '\u{f142f}',
        Nerd::HeartMinusOutline => '\u{f1432}',
        Nerd::HeartMultiple => '\u{f0a56}',
        Nerd::HeartMultipleOutline => '\u{f0a57}',
        Nerd::HeartOff => '\u{f0759}',
        Nerd::HeartOffOutline => '\u{f1434}',
        Nerd::HeartOne => '\u{eb05}',
        Nerd::HeartOutline => '\u{f02d5}',
        Nerd::HeartOutlineOne => '\u{f18a0}',
        Nerd::HeartPlus => '\u{f142e}',
        Nerd::HeartPlusOutline => '\u{f1431}',
        Nerd::HeartPulse => '\u{f05f6}',
        Nerd::HeartRemove => '\u{f1430}',
        Nerd::HeartRemoveOutline => '\u{f1433}',
        Nerd::HeartSettings => '\u{f1665}',
        Nerd::HeartSettingsOutline => '\u{f1666}',
        Nerd::HeartThree => '\u{f02d1}',
        Nerd::HeartTwo => '\u{f004}',
        Nerd::HeatPump => '\u{f1a43}',
        Nerd::HeatPumpOutline => '\u{f1a44}',
        Nerd::HeatWave => '\u{f1a45}',
        Nerd::HeatingCoil => '\u{f1aaf}',
        Nerd::HeavyCircle => '\u{2b58}',
        Nerd::Helicopter => '\u{f0ac2}',
        Nerd::Help => '\u{f02d6}',
        Nerd::HelpBox => '\u{f078b}',
        Nerd::HelpCircle => '\u{f02d7}',
        Nerd::HelpCircleOutline => '\u{f0625}',
        Nerd::HelpNetwork => '\u{f06f5}',
        Nerd::HelpNetworkOutline => '\u{f0c8a}',
        Nerd::HelpRhombus => '\u{f0ba5}',
        Nerd::HelpRhombusOutline => '\u{f0ba6}',
        Nerd::Hexadecimal => '\u{f12a7}',
        Nerd::Hexagon => '\u{e24f}',
        Nerd::HexagonMultiple => '\u{f06e1}',
        Nerd::HexagonMultipleOutline => '\u{f10f2}',
        Nerd::HexagonOne => '\u{f02d8}',
        Nerd::HexagonOutline => '\u{f02d9}',
        Nerd::HexagonSliceFive => '\u{f0ac7}',
        Nerd::HexagonSliceFour => '\u{f0ac6}',
        Nerd::HexagonSliceOne => '\u{f0ac3}',
        Nerd::HexagonSliceSix => '\u{f0ac8}',
        Nerd::HexagonSliceThree => '\u{f0ac5}',
        Nerd::HexagonSliceTwo => '\u{f0ac4}',
        Nerd::Hexagram => '\u{f0ac9}',
        Nerd::HexagramOutline => '\u{f0aca}',
        Nerd::HighDefinition => '\u{f07cf}',
        Nerd::HighDefinitionBox => '\u{f0878}',
        Nerd::HighHeel => '\u{e250}',
        Nerd::Highway => '\u{f05f7}',
        Nerd::Hiking => '\u{f0d7f}',
        Nerd::History => '\u{ea82}',
        Nerd::HistoryOne => '\u{f464}',
        Nerd::HistoryTwo => '\u{f02da}',
        Nerd::HockeyPuck => '\u{f0879}',
        Nerd::HockeySticks => '\u{f087a}',
        Nerd::Hololens => '\u{f02db}',
        Nerd::Home => '\u{eb06}',
        Nerd::HomeAccount => '\u{f0826}',
        Nerd::HomeAlert => '\u{f087b}',
        Nerd::HomeAlertOutline => '\u{f15d0}',
        Nerd::HomeAnalytics => '\u{f0eba}',
        Nerd::HomeAssistant => '\u{f07d0}',
        Nerd::HomeAutomation => '\u{f07d1}',
        Nerd::HomeBattery => '\u{f1901}',
        Nerd::HomeBatteryOutline => '\u{f1902}',
        Nerd::HomeCircle => '\u{f07d2}',
        Nerd::HomeCircleOutline => '\u{f104d}',
        Nerd::HomeCity => '\u{f0d15}',
        Nerd::HomeCityOutline => '\u{f0d16}',
        Nerd::HomeClock => '\u{f1a12}',
        Nerd::HomeClockOutline => '\u{f1a13}',
        Nerd::HomeEdit => '\u{f1159}',
        Nerd::HomeEditOutline => '\u{f115a}',
        Nerd::HomeExportOutline => '\u{f0f9b}',
        Nerd::HomeFill => '\u{f4e2}',
        Nerd::HomeFlood => '\u{f0efa}',
        Nerd::HomeFloorA => '\u{f0d83}',
        Nerd::HomeFloorB => '\u{f0d84}',
        Nerd::HomeFloorG => '\u{f0d85}',
        Nerd::HomeFloorL => '\u{f0d86}',
        Nerd::HomeFloorNegativeOne => '\u{f0dd3}',
        Nerd::HomeFloorOne => '\u{f0d80}',
        Nerd::HomeFloorThree => '\u{f0d82}',
        Nerd::HomeFloorTwo => '\u{f0d81}',
        Nerd::HomeFloorZero => '\u{f0dd2}',
        Nerd::HomeGroup => '\u{f0dd4}',
        Nerd::HomeGroupMinus => '\u{f19c1}',
        Nerd::HomeGroupPlus => '\u{f19c0}',
        Nerd::HomeGroupRemove => '\u{f19c2}',
        Nerd::HomeHeart => '\u{f0827}',
        Nerd::HomeImportOutline => '\u{f0f9c}',
        Nerd::HomeLightbulb => '\u{f1251}',
        Nerd::HomeLightbulbOutline => '\u{f1252}',
        Nerd::HomeLightningBolt => '\u{f1903}',
        Nerd::HomeLightningBoltOutline => '\u{f1904}',
        Nerd::HomeLock => '\u{f08eb}',
        Nerd::HomeLockOpen => '\u{f08ec}',
        Nerd::HomeMapMarker => '\u{f05f8}',
        Nerd::HomeMinus => '\u{f0974}',
        Nerd::HomeMinusOutline => '\u{f13d5}',
        Nerd::HomeModern => '\u{f02dd}',
        Nerd::HomeOff => '\u{f1a46}',
        Nerd::HomeOffOutline => '\u{f1a47}',
        Nerd::HomeOne => '\u{f015}',
        Nerd::HomeOutline => '\u{f06a1}',
        Nerd::HomePlus => '\u{f0975}',
        Nerd::HomePlusOutline => '\u{f13d6}',
        Nerd::HomeRemove => '\u{f1247}',
        Nerd::HomeRemoveOutline => '\u{f13d7}',
        Nerd::HomeRoof => '\u{f112b}',
        Nerd::HomeSearch => '\u{f13b0}',
        Nerd::HomeSearchOutline => '\u{f13b1}',
        Nerd::HomeSwitch => '\u{f1794}',
        Nerd::HomeSwitchOutline => '\u{f1795}',
        Nerd::HomeThermometer => '\u{f0f54}',
        Nerd::HomeThermometerOutline => '\u{f0f55}',
        Nerd::HomeThree => '\u{f02dc}',
        Nerd::HomeTwo => '\u{f46d}',
        Nerd::HomeVariant => '\u{f02de}',
        Nerd::HomeVariantOutline => '\u{f0ba7}',
        Nerd::Hook => '\u{f06e2}',
        Nerd::HookOff => '\u{f06e3}',
        Nerd::HoopHouse => '\u{f0e56}',
        Nerd::Hops => '\u{f02df}',
        Nerd::HorizontalRotateClockwise => '\u{f10f3}',
        Nerd::HorizontalRotateCounterclockwise => '\u{f10f4}',
        Nerd::HorizontalRule => '\u{eb07}',
        Nerd::HorizontalRuleOne => '\u{f45b}',
        Nerd::Horse => '\u{f15bf}',
        Nerd::HorseHuman => '\u{f15c0}',
        Nerd::HorseVariant => '\u{f15c1}',
        Nerd::HorseVariantFast => '\u{f186e}',
        Nerd::Horseshoe => '\u{f0a58}',
        Nerd::Hospital => '\u{f0f8}',
        Nerd::HospitalBox => '\u{f02e0}',
        Nerd::HospitalBoxOutline => '\u{f0ff7}',
        Nerd::HospitalBuilding => '\u{f02e1}',
        Nerd::HospitalMarker => '\u{f02e2}',
        Nerd::HospitalOne => '\u{f0ff6}',
        Nerd::HotTub => '\u{f0828}',
        Nerd::Hotdog => '\u{e251}',
        Nerd::Hourglass => '\u{f4e3}',
        Nerd::HoursTwofour => '\u{f1478}',
        Nerd::Htmlfive => '\u{f13b}',
        Nerd::Hubot => '\u{eb08}',
        Nerd::HubotOne => '\u{f477}',
        Nerd::Hubspot => '\u{f0d17}',
        Nerd::Hulu => '\u{f0829}',
        Nerd::Human => '\u{f02e6}',
        Nerd::HumanBabyChangingTable => '\u{f138b}',
        Nerd::HumanCane => '\u{f1581}',
        Nerd::HumanCapacityDecrease => '\u{f159b}',
        Nerd::HumanCapacityIncrease => '\u{f159c}',
        Nerd::HumanChild => '\u{f02e7}',
        Nerd::HumanDolly => '\u{f1980}',
        Nerd::HumanEdit => '\u{f14e8}',
        Nerd::HumanFemale => '\u{f0649}',
        Nerd::HumanFemaleBoy => '\u{f0a59}',
        Nerd::HumanFemaleDance => '\u{f15c9}',
        Nerd::HumanFemaleFemale => '\u{f0a5a}',
        Nerd::HumanFemaleGirl => '\u{f0a5b}',
        Nerd::HumanGreeting => '\u{f17c4}',
        Nerd::HumanGreetingProximity => '\u{f159d}',
        Nerd::HumanGreetingVariant => '\u{f064a}',
        Nerd::HumanHandsdown => '\u{f064b}',
        Nerd::HumanHandsup => '\u{f064c}',
        Nerd::HumanMale => '\u{f064d}',
        Nerd::HumanMaleBoard => '\u{f0890}',
        Nerd::HumanMaleBoardPoll => '\u{f0846}',
        Nerd::HumanMaleBoy => '\u{f0a5c}',
        Nerd::HumanMaleChild => '\u{f138c}',
        Nerd::HumanMaleFemale => '\u{f02e8}',
        Nerd::HumanMaleFemaleChild => '\u{f1823}',
        Nerd::HumanMaleGirl => '\u{f0a5d}',
        Nerd::HumanMaleHeight => '\u{f0efb}',
        Nerd::HumanMaleHeightVariant => '\u{f0efc}',
        Nerd::HumanMaleMale => '\u{f0a5e}',
        Nerd::HumanNonBinary => '\u{f1848}',
        Nerd::HumanPregnant => '\u{f05cf}',
        Nerd::HumanQueue => '\u{f1571}',
        Nerd::HumanScooter => '\u{f11e9}',
        Nerd::HumanWheelchair => '\u{f138d}',
        Nerd::HumanWhiteCane => '\u{f1981}',
        Nerd::HumbleBundle => '\u{f0744}',
        Nerd::Hvac => '\u{f1352}',
        Nerd::HvacOff => '\u{f159e}',
        Nerd::HydraulicOilLevel => '\u{f1324}',
        Nerd::HydraulicOilTemperature => '\u{f1325}',
        Nerd::HydroPower => '\u{f12e5}',
        Nerd::HydrogenStation => '\u{f1894}',
        Nerd::HyperbolaGnuLinuxLibre => '\u{f33a}',
        Nerd::Hyprland => '\u{f359}',
        Nerd::ICustomAsm => '\u{e6ab}',
        Nerd::ICustomC => '\u{e61e}',
        Nerd::ICustomCommonLisp => '\u{e6b0}',
        Nerd::ICustomCpp => '\u{e61d}',
        Nerd::ICustomCrystal => '\u{e62f}',
        Nerd::ICustomDefault => '\u{e612}',
        Nerd::ICustomElectron => '\u{e62e}',
        Nerd::ICustomElixir => '\u{e62d}',
        Nerd::ICustomElm => '\u{e62c}',
        Nerd::ICustomEmacs => '\u{e632}',
        Nerd::ICustomFennel => '\u{e6af}',
        Nerd::ICustomFolder => '\u{e5ff}',
        Nerd::ICustomFolderConfig => '\u{e5fc}',
        Nerd::ICustomFolderGit => '\u{e5fb}',
        Nerd::ICustomFolderGithub => '\u{e5fd}',
        Nerd::ICustomFolderNpm => '\u{e5fa}',
        Nerd::ICustomFolderOct => '\u{e6ad}',
        Nerd::ICustomFolderOpen => '\u{e5fe}',
        Nerd::ICustomGo => '\u{e626}',
        Nerd::ICustomHome => '\u{e617}',
        Nerd::ICustomKotlin => '\u{e634}',
        Nerd::ICustomMsdos => '\u{e629}',
        Nerd::ICustomNeovim => '\u{e6ae}',
        Nerd::ICustomOrgmode => '\u{e633}',
        Nerd::ICustomPlayArrow => '\u{e602}',
        Nerd::ICustomPurescript => '\u{e630}',
        Nerd::ICustomScheme => '\u{e6b1}',
        Nerd::ICustomToml => '\u{e6b2}',
        Nerd::ICustomVLang => '\u{e6ac}',
        Nerd::ICustomVim => '\u{e62b}',
        Nerd::ICustomWindows => '\u{e62a}',
        Nerd::IIndentLine => '\u{e621}',
        Nerd::ISetiApple => '\u{e635}',
        Nerd::ISetiArgdown => '\u{e636}',
        Nerd::ISetiAsm => '\u{e637}',
        Nerd::ISetiAudio => '\u{e638}',
        Nerd::ISetiBabel => '\u{e639}',
        Nerd::ISetiBazel => '\u{e63a}',
        Nerd::ISetiBicep => '\u{e63b}',
        Nerd::ISetiBower => '\u{e61a}',
        Nerd::ISetiBsl => '\u{e63c}',
        Nerd::ISetiC => '\u{e649}',
        Nerd::ISetiCSharp => '\u{e648}',
        Nerd::ISetiCake => '\u{e63e}',
        Nerd::ISetiCakePhp => '\u{e63d}',
        Nerd::ISetiCheckbox => '\u{e63f}',
        Nerd::ISetiCheckboxUnchecked => '\u{e640}',
        Nerd::ISetiClock => '\u{e641}',
        Nerd::ISetiClojure => '\u{e642}',
        Nerd::ISetiCodeClimate => '\u{e643}',
        Nerd::ISetiCodeSearch => '\u{e644}',
        Nerd::ISetiCoffee => '\u{e61b}',
        Nerd::ISetiColdfusion => '\u{e645}',
        Nerd::ISetiConfig => '\u{e615}',
        Nerd::ISetiCpp => '\u{e646}',
        Nerd::ISetiCrystalEmbedded => '\u{e647}',
        Nerd::ISetiCss => '\u{e614}',
        Nerd::ISetiCsv => '\u{e64a}',
        Nerd::ISetiCu => '\u{e64b}',
        Nerd::ISetiD => '\u{e651}',
        Nerd::ISetiDart => '\u{e64c}',
        Nerd::ISetiDb => '\u{e64d}',
        Nerd::ISetiDefault => '\u{e64e}',
        Nerd::ISetiDeprecationCop => '\u{e64f}',
        Nerd::ISetiDocker => '\u{e650}',
        Nerd::ISetiEditorconfig => '\u{e652}',
        Nerd::ISetiEjs => '\u{e618}',
        Nerd::ISetiElixirScript => '\u{e653}',
        Nerd::ISetiError => '\u{e654}',
        Nerd::ISetiEslint => '\u{e655}',
        Nerd::ISetiEthereum => '\u{e656}',
        Nerd::ISetiFSharp => '\u{e65a}',
        Nerd::ISetiFavicon => '\u{e623}',
        Nerd::ISetiFirebase => '\u{e657}',
        Nerd::ISetiFirefox => '\u{e658}',
        Nerd::ISetiFolder => '\u{e613}',
        Nerd::ISetiFont => '\u{e659}',
        Nerd::ISetiGit => '\u{e65d}',
        Nerd::ISetiGithub => '\u{e65b}',
        Nerd::ISetiGitlab => '\u{e65c}',
        Nerd::ISetiGo => '\u{e627}',
        Nerd::ISetiGodot => '\u{e65f}',
        Nerd::ISetiGotwo => '\u{e65e}',
        Nerd::ISetiGradle => '\u{e660}',
        Nerd::ISetiGrails => '\u{e661}',
        Nerd::ISetiGraphql => '\u{e662}',
        Nerd::ISetiGrunt => '\u{e611}',
        Nerd::ISetiGulp => '\u{e610}',
        Nerd::ISetiHacklang => '\u{e663}',
        Nerd::ISetiHaml => '\u{e664}',
        Nerd::ISetiHappenings => '\u{e665}',
        Nerd::ISetiHaskell => '\u{e61f}',
        Nerd::ISetiHaxe => '\u{e666}',
        Nerd::ISetiHeroku => '\u{e607}',
        Nerd::ISetiHex => '\u{e667}',
        Nerd::ISetiHtml => '\u{e60e}',
        Nerd::ISetiIgnored => '\u{e668}',
        Nerd::ISetiIllustrator => '\u{e669}',
        Nerd::ISetiImage => '\u{e60d}',
        Nerd::ISetiInfo => '\u{e66a}',
        Nerd::ISetiIonic => '\u{e66b}',
        Nerd::ISetiJade => '\u{e66c}',
        Nerd::ISetiJava => '\u{e66d}',
        Nerd::ISetiJavascript => '\u{e60c}',
        Nerd::ISetiJenkins => '\u{e66e}',
        Nerd::ISetiJinja => '\u{e66f}',
        Nerd::ISetiJson => '\u{e60b}',
        Nerd::ISetiJulia => '\u{e624}',
        Nerd::ISetiKarma => '\u{e622}',
        Nerd::ISetiLicense => '\u{e60a}',
        Nerd::ISetiLiquid => '\u{e670}',
        Nerd::ISetiLivescript => '\u{e671}',
        Nerd::ISetiLock => '\u{e672}',
        Nerd::ISetiLua => '\u{e620}',
        Nerd::ISetiMakefile => '\u{e673}',
        Nerd::ISetiMarkdown => '\u{e609}',
        Nerd::ISetiMaven => '\u{e674}',
        Nerd::ISetiMdo => '\u{e675}',
        Nerd::ISetiMustache => '\u{e60f}',
        Nerd::ISetiNewFile => '\u{e676}',
        Nerd::ISetiNim => '\u{e677}',
        Nerd::ISetiNotebook => '\u{e678}',
        Nerd::ISetiNpm => '\u{e616}',
        Nerd::ISetiNunjucks => '\u{e679}',
        Nerd::ISetiOcaml => '\u{e67a}',
        Nerd::ISetiOdata => '\u{e67b}',
        Nerd::ISetiPddl => '\u{e67c}',
        Nerd::ISetiPdf => '\u{e67d}',
        Nerd::ISetiPerl => '\u{e67e}',
        Nerd::ISetiPhotoshop => '\u{e67f}',
        Nerd::ISetiPhp => '\u{e608}',
        Nerd::ISetiPipeline => '\u{e680}',
        Nerd::ISetiPlan => '\u{e681}',
        Nerd::ISetiPlatformio => '\u{e682}',
        Nerd::ISetiPowershell => '\u{e683}',
        Nerd::ISetiPrisma => '\u{e684}',
        Nerd::ISetiProject => '\u{e601}',
        Nerd::ISetiProlog => '\u{e685}',
        Nerd::ISetiPug => '\u{e686}',
        Nerd::ISetiPuppet => '\u{e631}',
        Nerd::ISetiPython => '\u{e606}',
        Nerd::ISetiR => '\u{e68a}',
        Nerd::ISetiRails => '\u{e604}',
        Nerd::ISetiReact => '\u{e625}',
        Nerd::ISetiReasonml => '\u{e687}',
        Nerd::ISetiRescript => '\u{e688}',
        Nerd::ISetiRollup => '\u{e689}',
        Nerd::ISetiRuby => '\u{e605}',
        Nerd::ISetiRust => '\u{e68b}',
        Nerd::ISetiSalesforce => '\u{e68c}',
        Nerd::ISetiSass => '\u{e603}',
        Nerd::ISetiSbt => '\u{e68d}',
        Nerd::ISetiScala => '\u{e68e}',
        Nerd::ISetiSearch => '\u{e68f}',
        Nerd::ISetiSettings => '\u{e690}',
        Nerd::ISetiShell => '\u{e691}',
        Nerd::ISetiSlim => '\u{e692}',
        Nerd::ISetiSmarty => '\u{e693}',
        Nerd::ISetiSpring => '\u{e694}',
        Nerd::ISetiStylelint => '\u{e695}',
        Nerd::ISetiStylus => '\u{e600}',
        Nerd::ISetiSublime => '\u{e696}',
        Nerd::ISetiSvelte => '\u{e697}',
        Nerd::ISetiSvg => '\u{e698}',
        Nerd::ISetiSwift => '\u{e699}',
        Nerd::ISetiTerraform => '\u{e69a}',
        Nerd::ISetiTex => '\u{e69b}',
        Nerd::ISetiTodo => '\u{e69c}',
        Nerd::ISetiTsconfig => '\u{e69d}',
        Nerd::ISetiTwig => '\u{e61c}',
        Nerd::ISetiTypescript => '\u{e628}',
        Nerd::ISetiVala => '\u{e69e}',
        Nerd::ISetiVideo => '\u{e69f}',
        Nerd::ISetiVue => '\u{e6a0}',
        Nerd::ISetiWasm => '\u{e6a1}',
        Nerd::ISetiWat => '\u{e6a2}',
        Nerd::ISetiWebpack => '\u{e6a3}',
        Nerd::ISetiWgt => '\u{e6a4}',
        Nerd::ISetiWord => '\u{e6a5}',
        Nerd::ISetiXls => '\u{e6a6}',
        Nerd::ISetiXml => '\u{e619}',
        Nerd::ISetiYarn => '\u{e6a7}',
        Nerd::ISetiYml => '\u{e6a8}',
        Nerd::ISetiZig => '\u{e6a9}',
        Nerd::ISetiZip => '\u{e6aa}',
        Nerd::IceCream => '\u{e252}',
        Nerd::IceCreamOff => '\u{f0e52}',
        Nerd::IceCreamOne => '\u{f082a}',
        Nerd::IcePop => '\u{f0efd}',
        Nerd::IdBadge => '\u{f4e4}',
        Nerd::IdCard => '\u{e253}',
        Nerd::IdCardOne => '\u{f0fc0}',
        Nerd::Identifier => '\u{f0efe}',
        Nerd::IdeogramCjk => '\u{f1331}',
        Nerd::IdeogramCjkVariant => '\u{f1332}',
        Nerd::Illumos => '\u{f326}',
        Nerd::Image => '\u{f4e5}',
        Nerd::ImageAlbum => '\u{f02ea}',
        Nerd::ImageArea => '\u{f02eb}',
        Nerd::ImageAreaClose => '\u{f02ec}',
        Nerd::ImageAutoAdjust => '\u{f0fc1}',
        Nerd::ImageBroken => '\u{f02ed}',
        Nerd::ImageBrokenVariant => '\u{f02ee}',
        Nerd::ImageEdit => '\u{f11e3}',
        Nerd::ImageEditOutline => '\u{f11e4}',
        Nerd::ImageFilterBlackWhite => '\u{f02f0}',
        Nerd::ImageFilterCenterFocus => '\u{f02f1}',
        Nerd::ImageFilterCenterFocusStrong => '\u{f0eff}',
        Nerd::ImageFilterCenterFocusStrongOutline => '\u{f0f00}',
        Nerd::ImageFilterCenterFocusWeak => '\u{f02f2}',
        Nerd::ImageFilterDrama => '\u{f02f3}',
        Nerd::ImageFilterFrames => '\u{f02f4}',
        Nerd::ImageFilterHdr => '\u{f02f5}',
        Nerd::ImageFilterHdrOne => '\u{f0509}',
        Nerd::ImageFilterNone => '\u{f02f6}',
        Nerd::ImageFilterTiltShift => '\u{f02f7}',
        Nerd::ImageFilterVintage => '\u{f02f8}',
        Nerd::ImageFrame => '\u{f0e49}',
        Nerd::ImageLock => '\u{f1ab0}',
        Nerd::ImageLockOutline => '\u{f1ab1}',
        Nerd::ImageMarker => '\u{f177b}',
        Nerd::ImageMarkerOutline => '\u{f177c}',
        Nerd::ImageMinus => '\u{f1419}',
        Nerd::ImageMove => '\u{f09f8}',
        Nerd::ImageMultiple => '\u{f02f9}',
        Nerd::ImageMultipleOutline => '\u{f02ef}',
        Nerd::ImageOff => '\u{f082b}',
        Nerd::ImageOffOutline => '\u{f11d1}',
        Nerd::ImageOne => '\u{f02e9}',
        Nerd::ImageOutline => '\u{f0976}',
        Nerd::ImagePlus => '\u{f087c}',
        Nerd::ImageRefresh => '\u{f19fe}',
        Nerd::ImageRefreshOutline => '\u{f19ff}',
        Nerd::ImageRemove => '\u{f1418}',
        Nerd::ImageSearch => '\u{f0977}',
        Nerd::ImageSearchOutline => '\u{f0978}',
        Nerd::ImageSizeSelectActual => '\u{f0c8d}',
        Nerd::ImageSizeSelectLarge => '\u{f0c8e}',
        Nerd::ImageSizeSelectSmall => '\u{f0c8f}',
        Nerd::ImageSync => '\u{f1a00}',
        Nerd::ImageSyncOutline => '\u{f1a01}',
        Nerd::ImageText => '\u{f160d}',
        Nerd::Imdb => '\u{e254}',
        Nerd::Import => '\u{f02fa}',
        Nerd::Inbox => '\u{eb09}',
        Nerd::InboxArrowDown => '\u{f02fb}',
        Nerd::InboxArrowDownOutline => '\u{f1270}',
        Nerd::InboxArrowUp => '\u{f03d1}',
        Nerd::InboxArrowUpOutline => '\u{f1271}',
        Nerd::InboxFull => '\u{f1272}',
        Nerd::InboxFullOutline => '\u{f1273}',
        Nerd::InboxMultiple => '\u{f08b0}',
        Nerd::InboxMultipleOutline => '\u{f0ba8}',
        Nerd::InboxOne => '\u{f01c}',
        Nerd::InboxOutline => '\u{f1274}',
        Nerd::InboxRemove => '\u{f159f}',
        Nerd::InboxRemoveOutline => '\u{f15a0}',
        Nerd::InboxThree => '\u{f0687}',
        Nerd::InboxTwo => '\u{f48d}',
        Nerd::Incognito => '\u{f05f9}',
        Nerd::IncognitoCircle => '\u{f1421}',
        Nerd::IncognitoCircleOff => '\u{f1422}',
        Nerd::IncognitoOff => '\u{f0075}',
        Nerd::IndentLeft => '\u{f03b}',
        Nerd::IndentRight => '\u{f03c}',
        Nerd::Induction => '\u{f184c}',
        Nerd::Infinity => '\u{e255}',
        Nerd::InfinityOne => '\u{f4e6}',
        Nerd::InfinityTwo => '\u{f06e4}',
        Nerd::Info => '\u{ea74}',
        Nerd::InfoOne => '\u{f449}',
        Nerd::InfoSign => '\u{f05a}',
        Nerd::Information => '\u{f02fc}',
        Nerd::InformationOff => '\u{f178c}',
        Nerd::InformationOffOutline => '\u{f178d}',
        Nerd::InformationOutline => '\u{f02fd}',
        Nerd::InformationVariant => '\u{f064e}',
        Nerd::Injection => '\u{e2a1}',
        Nerd::Inkscape => '\u{f33b}',
        Nerd::Inr => '\u{f156}',
        Nerd::Inspect => '\u{ebd1}',
        Nerd::Instagram => '\u{f16d}',
        Nerd::InstagramOne => '\u{f02fe}',
        Nerd::InstrumentTriangle => '\u{f104e}',
        Nerd::IntegratedCircuitChip => '\u{f1913}',
        Nerd::InternalInterruption => '\u{e009}',
        Nerd::InvertColors => '\u{f0301}',
        Nerd::InvertColorsOff => '\u{f0e4a}',
        Nerd::Iobroker => '\u{f12e8}',
        Nerd::Ip => '\u{f0a5f}',
        Nerd::IpNetwork => '\u{f0a60}',
        Nerd::IpNetworkOutline => '\u{f0c90}',
        Nerd::IpOutline => '\u{f1982}',
        Nerd::Ipod => '\u{f0c91}',
        Nerd::Iron => '\u{f1824}',
        Nerd::IronBoard => '\u{f1838}',
        Nerd::IronOutline => '\u{f1825}',
        Nerd::Island => '\u{f104f}',
        Nerd::Isle => '\u{e2a2}',
        Nerd::IssueClosed => '\u{f41d}',
        Nerd::IssueDraft => '\u{ebd9}',
        Nerd::IssueDraftOne => '\u{f4e7}',
        Nerd::IssueOpened => '\u{f41b}',
        Nerd::IssueReopened => '\u{eb0b}',
        Nerd::IssueReopenedOne => '\u{f41c}',
        Nerd::IssueTrackedBy => '\u{f4e8}',
        Nerd::IssueTracks => '\u{f4e9}',
        Nerd::Issues => '\u{eb0c}',
        Nerd::Italic => '\u{eb0d}',
        Nerd::ItalicOne => '\u{f033}',
        Nerd::ItalicTwo => '\u{f49f}',
        Nerd::Iterations => '\u{f4ea}',
        Nerd::Ithree => '\u{f35a}',
        Nerd::IvBag => '\u{f10b9}',
        Nerd::Jabber => '\u{f0dd5}',
        Nerd::Java => '\u{e256}',
        Nerd::Jeepney => '\u{f0302}',
        Nerd::Jellyfish => '\u{f0f01}',
        Nerd::JellyfishOutline => '\u{f0f02}',
        Nerd::Jersey => '\u{eb0e}',
        Nerd::Jira => '\u{f0303}',
        Nerd::Jpy => '\u{f157}',
        Nerd::Jquery => '\u{f087d}',
        Nerd::Jsfiddle => '\u{f0304}',
        Nerd::Json => '\u{eb0f}',
        Nerd::JumpRope => '\u{f12ff}',
        Nerd::Jwm => '\u{f35b}',
        Nerd::Kabaddi => '\u{f0d87}',
        Nerd::KaliLinux => '\u{f327}',
        Nerd::Kangaroo => '\u{f1558}',
        Nerd::Karate => '\u{f082c}',
        Nerd::Kayaking => '\u{f08af}',
        Nerd::KdeNeon => '\u{f331}',
        Nerd::KdePlasma => '\u{f332}',
        Nerd::Kdenlive => '\u{f33c}',
        Nerd::KebabHorizontal => '\u{f4eb}',
        Nerd::KebabVertical => '\u{eb10}',
        Nerd::Keg => '\u{f0305}',
        Nerd::Kettle => '\u{f05fa}',
        Nerd::KettleAlert => '\u{f1317}',
        Nerd::KettleAlertOutline => '\u{f1318}',
        Nerd::KettleOff => '\u{f131b}',
        Nerd::KettleOffOutline => '\u{f131c}',
        Nerd::KettleOutline => '\u{f0f56}',
        Nerd::KettlePourOver => '\u{f173c}',
        Nerd::KettleSteam => '\u{f1319}',
        Nerd::KettleSteamOutline => '\u{f131a}',
        Nerd::Kettlebell => '\u{f1300}',
        Nerd::Key => '\u{eb11}',
        Nerd::KeyAlert => '\u{f1983}',
        Nerd::KeyAlertOutline => '\u{f1984}',
        Nerd::KeyArrowRight => '\u{f1312}',
        Nerd::KeyAsterisk => '\u{f4ec}',
        Nerd::KeyChain => '\u{f1574}',
        Nerd::KeyChainVariant => '\u{f1575}',
        Nerd::KeyChange => '\u{f0307}',
        Nerd::KeyLink => '\u{f119f}',
        Nerd::KeyMinus => '\u{f0308}',
        Nerd::KeyOne => '\u{f084}',
        Nerd::KeyOutline => '\u{f0dd6}',
        Nerd::KeyPlus => '\u{f0309}',
        Nerd::KeyRemove => '\u{f030a}',
        Nerd::KeyStar => '\u{f119e}',
        Nerd::KeyThree => '\u{f0306}',
        Nerd::KeyTwo => '\u{f43d}',
        Nerd::KeyVariant => '\u{f030b}',
        Nerd::KeyWireless => '\u{f0fc2}',
        Nerd::Keyboard => '\u{f11c}',
        Nerd::KeyboardBackspace => '\u{f030d}',
        Nerd::KeyboardCaps => '\u{f030e}',
        Nerd::KeyboardClose => '\u{f030f}',
        Nerd::KeyboardEsc => '\u{f12b7}',
        Nerd::KeyboardFeight => '\u{f12b2}',
        Nerd::KeyboardFfive => '\u{f12af}',
        Nerd::KeyboardFfour => '\u{f12ae}',
        Nerd::KeyboardFnine => '\u{f12b3}',
        Nerd::KeyboardFone => '\u{f12ab}',
        Nerd::KeyboardFoneone => '\u{f12b5}',
        Nerd::KeyboardFonetwo => '\u{f12b6}',
        Nerd::KeyboardFonezero => '\u{f12b4}',
        Nerd::KeyboardFseven => '\u{f12b1}',
        Nerd::KeyboardFsix => '\u{f12b0}',
        Nerd::KeyboardFthree => '\u{f12ad}',
        Nerd::KeyboardFtwo => '\u{f12ac}',
        Nerd::KeyboardOff => '\u{f0310}',
        Nerd::KeyboardOffOutline => '\u{f0e4b}',
        Nerd::KeyboardOne => '\u{f030c}',
        Nerd::KeyboardOutline => '\u{f097b}',
        Nerd::KeyboardReturn => '\u{f0311}',
        Nerd::KeyboardSettings => '\u{f09f9}',
        Nerd::KeyboardSettingsOutline => '\u{f09fa}',
        Nerd::KeyboardSpace => '\u{f1050}',
        Nerd::KeyboardTab => '\u{f0312}',
        Nerd::KeyboardTabReverse => '\u{f0325}',
        Nerd::KeyboardVariant => '\u{f0313}',
        Nerd::Khanda => '\u{f10fd}',
        Nerd::Kicad => '\u{f34c}',
        Nerd::Kickstarter => '\u{f0745}',
        Nerd::Kite => '\u{f1985}',
        Nerd::KiteOutline => '\u{f1986}',
        Nerd::Kitesurfing => '\u{f1744}',
        Nerd::Klingon => '\u{f135b}',
        Nerd::Knife => '\u{f09fb}',
        Nerd::KnifeMilitary => '\u{f09fc}',
        Nerd::Koala => '\u{f173f}',
        Nerd::Kodi => '\u{f0314}',
        Nerd::Krita => '\u{f33d}',
        Nerd::Krw => '\u{f159}',
        Nerd::Kubernetes => '\u{f10fe}',
        Nerd::Kubuntu => '\u{f333}',
        Nerd::KubuntuInverse => '\u{f334}',
        Nerd::Label => '\u{f0315}',
        Nerd::LabelMultiple => '\u{f1375}',
        Nerd::LabelMultipleOutline => '\u{f1376}',
        Nerd::LabelOff => '\u{f0acb}',
        Nerd::LabelOffOutline => '\u{f0acc}',
        Nerd::LabelOutline => '\u{f0316}',
        Nerd::LabelPercent => '\u{f12ea}',
        Nerd::LabelPercentOutline => '\u{f12eb}',
        Nerd::LabelVariant => '\u{f0acd}',
        Nerd::LabelVariantOutline => '\u{f0ace}',
        Nerd::Ladder => '\u{f15a2}',
        Nerd::Ladybug => '\u{f082d}',
        Nerd::Lambda => '\u{f0627}',
        Nerd::Lamp => '\u{f06b5}',
        Nerd::LampOutline => '\u{f17d0}',
        Nerd::Lamps => '\u{f1576}',
        Nerd::LampsOutline => '\u{f17d1}',
        Nerd::Lan => '\u{f0317}',
        Nerd::LanCheck => '\u{f12aa}',
        Nerd::LanConnect => '\u{f0318}',
        Nerd::LanDisconnect => '\u{f0319}',
        Nerd::LanPending => '\u{f031a}',
        Nerd::LandFields => '\u{f1ab2}',
        Nerd::LandPlots => '\u{f1ab3}',
        Nerd::LandPlotsCircle => '\u{f1ab4}',
        Nerd::LandPlotsCircleVariant => '\u{f1ab5}',
        Nerd::LandRowsHorizontal => '\u{f1ab6}',
        Nerd::LandRowsVertical => '\u{f1ab7}',
        Nerd::Landslide => '\u{f1a48}',
        Nerd::LandslideOutline => '\u{f1a49}',
        Nerd::LanguageC => '\u{f0671}',
        Nerd::LanguageCpp => '\u{f0672}',
        Nerd::LanguageCsharp => '\u{f031b}',
        Nerd::LanguageCssthree => '\u{f031c}',
        Nerd::LanguageFortran => '\u{f121a}',
        Nerd::LanguageGo => '\u{f07d3}',
        Nerd::LanguageHaskell => '\u{f0c92}',
        Nerd::LanguageHtmlfive => '\u{f031d}',
        Nerd::LanguageJava => '\u{f0b37}',
        Nerd::LanguageJavascript => '\u{f031e}',
        Nerd::LanguageKotlin => '\u{f1219}',
        Nerd::LanguageLua => '\u{f08b1}',
        Nerd::LanguageMarkdown => '\u{f0354}',
        Nerd::LanguageMarkdownOutline => '\u{f0f5b}',
        Nerd::LanguagePhp => '\u{f031f}',
        Nerd::LanguagePython => '\u{f0320}',
        Nerd::LanguageR => '\u{f07d4}',
        Nerd::LanguageRuby => '\u{f0d2d}',
        Nerd::LanguageRubyOnRails => '\u{f0acf}',
        Nerd::LanguageRust => '\u{f1617}',
        Nerd::LanguageSwift => '\u{f06e5}',
        Nerd::LanguageTypescript => '\u{f06e6}',
        Nerd::LanguageXaml => '\u{f0673}',
        Nerd::Laptop => '\u{f109}',
        Nerd::LaptopAccount => '\u{f1a4a}',
        Nerd::LaptopOff => '\u{f06e7}',
        Nerd::LaptopOne => '\u{f0322}',
        Nerd::Laravel => '\u{f0ad0}',
        Nerd::LaserPointer => '\u{f1484}',
        Nerd::Lasso => '\u{f0f03}',
        Nerd::Lastpass => '\u{f0446}',
        Nerd::Latitude => '\u{f0f57}',
        Nerd::Launch => '\u{f0327}',
        Nerd::LavaLamp => '\u{f07d5}',
        Nerd::Law => '\u{eb12}',
        Nerd::LawOne => '\u{f495}',
        Nerd::Layers => '\u{e257}',
        Nerd::LayersActive => '\u{ebd4}',
        Nerd::LayersDot => '\u{ebd3}',
        Nerd::LayersEdit => '\u{f1892}',
        Nerd::LayersMinus => '\u{f0e4c}',
        Nerd::LayersOff => '\u{f0329}',
        Nerd::LayersOffOutline => '\u{f09fd}',
        Nerd::LayersOne => '\u{ebd2}',
        Nerd::LayersOutline => '\u{f09fe}',
        Nerd::LayersPlus => '\u{f0e4d}',
        Nerd::LayersRemove => '\u{f0e4e}',
        Nerd::LayersSearch => '\u{f1206}',
        Nerd::LayersSearchOutline => '\u{f1207}',
        Nerd::LayersTriple => '\u{f0f58}',
        Nerd::LayersTripleOutline => '\u{f0f59}',
        Nerd::LayersTwo => '\u{f0328}',
        Nerd::Layout => '\u{ebeb}',
        Nerd::LeadPencil => '\u{f064f}',
        Nerd::Leaf => '\u{f06c}',
        Nerd::LeafCircle => '\u{f1905}',
        Nerd::LeafCircleOutline => '\u{f1906}',
        Nerd::LeafMaple => '\u{f0c93}',
        Nerd::LeafMapleOff => '\u{f12da}',
        Nerd::LeafOff => '\u{f12d9}',
        Nerd::LeafOne => '\u{f032a}',
        Nerd::Leak => '\u{f0dd7}',
        Nerd::LeakOff => '\u{f0dd8}',
        Nerd::Lecturn => '\u{f1af0}',
        Nerd::LedOff => '\u{f032b}',
        Nerd::LedOn => '\u{f032c}',
        Nerd::LedOutline => '\u{f032d}',
        Nerd::LedStrip => '\u{f07d6}',
        Nerd::LedStripVariant => '\u{f1051}',
        Nerd::LedStripVariantOff => '\u{f1a4b}',
        Nerd::LedVariantOff => '\u{f032e}',
        Nerd::LedVariantOn => '\u{f032f}',
        Nerd::LedVariantOutline => '\u{f0330}',
        Nerd::Leek => '\u{f117d}',
        Nerd::Legal => '\u{f0e3}',
        Nerd::Lemon => '\u{f094}',
        Nerd::LessThan => '\u{f097c}',
        Nerd::LessThanOrEqual => '\u{f097d}',
        Nerd::LevelDown => '\u{f149}',
        Nerd::LevelUp => '\u{f148}',
        Nerd::Library => '\u{eb9c}',
        Nerd::LibraryOne => '\u{f0331}',
        Nerd::LibraryOutline => '\u{f1a22}',
        Nerd::LibraryShelves => '\u{f0ba9}',
        Nerd::License => '\u{f0fc3}',
        Nerd::Lifebuoy => '\u{f087e}',
        Nerd::LightBulb => '\u{f400}',
        Nerd::LightFloodDown => '\u{f1987}',
        Nerd::LightFloodUp => '\u{f1988}',
        Nerd::LightRecessed => '\u{f179b}',
        Nerd::LightSwitch => '\u{f097e}',
        Nerd::LightSwitchOff => '\u{f1a24}',
        Nerd::Lightbulb => '\u{ea61}',
        Nerd::LightbulbAlert => '\u{f19e1}',
        Nerd::LightbulbAlertOutline => '\u{f19e2}',
        Nerd::LightbulbAuto => '\u{f1800}',
        Nerd::LightbulbAutoOutline => '\u{f1801}',
        Nerd::LightbulbAutofix => '\u{eb13}',
        Nerd::LightbulbCfl => '\u{f1208}',
        Nerd::LightbulbCflOff => '\u{f1209}',
        Nerd::LightbulbCflSpiral => '\u{f1275}',
        Nerd::LightbulbCflSpiralOff => '\u{f12c3}',
        Nerd::LightbulbFluorescentTube => '\u{f1804}',
        Nerd::LightbulbFluorescentTubeOutline => '\u{f1805}',
        Nerd::LightbulbGroup => '\u{f1253}',
        Nerd::LightbulbGroupOff => '\u{f12cd}',
        Nerd::LightbulbGroupOffOutline => '\u{f12ce}',
        Nerd::LightbulbGroupOutline => '\u{f1254}',
        Nerd::LightbulbMultiple => '\u{f1255}',
        Nerd::LightbulbMultipleOff => '\u{f12cf}',
        Nerd::LightbulbMultipleOffOutline => '\u{f12d0}',
        Nerd::LightbulbMultipleOutline => '\u{f1256}',
        Nerd::LightbulbNight => '\u{f1a4c}',
        Nerd::LightbulbNightOutline => '\u{f1a4d}',
        Nerd::LightbulbOff => '\u{f0e4f}',
        Nerd::LightbulbOffOutline => '\u{f0e50}',
        Nerd::LightbulbOn => '\u{f06e8}',
        Nerd::LightbulbOnEightzero => '\u{f1a55}',
        Nerd::LightbulbOnFivezero => '\u{f1a52}',
        Nerd::LightbulbOnFourzero => '\u{f1a51}',
        Nerd::LightbulbOnNinezero => '\u{f1a56}',
        Nerd::LightbulbOnOnezero => '\u{f1a4e}',
        Nerd::LightbulbOnOutline => '\u{f06e9}',
        Nerd::LightbulbOnSevenzero => '\u{f1a54}',
        Nerd::LightbulbOnSixzero => '\u{f1a53}',
        Nerd::LightbulbOnThreezero => '\u{f1a50}',
        Nerd::LightbulbOnTwozero => '\u{f1a4f}',
        Nerd::LightbulbOne => '\u{f0335}',
        Nerd::LightbulbOutline => '\u{f0336}',
        Nerd::LightbulbQuestion => '\u{f19e3}',
        Nerd::LightbulbQuestionOutline => '\u{f19e4}',
        Nerd::LightbulbSpot => '\u{f17f4}',
        Nerd::LightbulbSpotOff => '\u{f17f5}',
        Nerd::LightbulbVariant => '\u{f1802}',
        Nerd::LightbulbVariantOutline => '\u{f1803}',
        Nerd::Lighthouse => '\u{f09ff}',
        Nerd::LighthouseOn => '\u{f0a00}',
        Nerd::LightningBolt => '\u{f140b}',
        Nerd::LightningBoltCircle => '\u{f0820}',
        Nerd::LightningBoltOutline => '\u{f140c}',
        Nerd::LineScan => '\u{f0624}',
        Nerd::Lingerie => '\u{f1476}',
        Nerd::Link => '\u{eb15}',
        Nerd::LinkBox => '\u{f0d1a}',
        Nerd::LinkBoxOutline => '\u{f0d1b}',
        Nerd::LinkBoxVariant => '\u{f0d1c}',
        Nerd::LinkBoxVariantOutline => '\u{f0d1d}',
        Nerd::LinkExternal => '\u{eb14}',
        Nerd::LinkExternalOne => '\u{f465}',
        Nerd::LinkLock => '\u{f10ba}',
        Nerd::LinkOff => '\u{f0338}',
        Nerd::LinkOne => '\u{f0c1}',
        Nerd::LinkPlus => '\u{f0c94}',
        Nerd::LinkThree => '\u{f0337}',
        Nerd::LinkTwo => '\u{f44c}',
        Nerd::LinkVariant => '\u{f0339}',
        Nerd::LinkVariantMinus => '\u{f10ff}',
        Nerd::LinkVariantOff => '\u{f033a}',
        Nerd::LinkVariantPlus => '\u{f1100}',
        Nerd::LinkVariantRemove => '\u{f1101}',
        Nerd::Linkedin => '\u{f0e1}',
        Nerd::LinkedinOne => '\u{f033b}',
        Nerd::LinkedinSign => '\u{f08c}',
        Nerd::Linux => '\u{f17c}',
        Nerd::LinuxMint => '\u{f08ed}',
        Nerd::LinuxMintInverse => '\u{f30f}',
        Nerd::LinuxOne => '\u{f033d}',
        Nerd::Lips => '\u{e258}',
        Nerd::Lipstick => '\u{e259}',
        Nerd::LipstickOne => '\u{f13b5}',
        Nerd::LiquidSpot => '\u{f1826}',
        Nerd::Liquor => '\u{f191e}',
        Nerd::List => '\u{f03a}',
        Nerd::ListAlt => '\u{f022}',
        Nerd::ListFilter => '\u{eb83}',
        Nerd::ListFlat => '\u{eb84}',
        Nerd::ListOrdered => '\u{eb16}',
        Nerd::ListOrderedOne => '\u{f452}',
        Nerd::ListSelection => '\u{eb85}',
        Nerd::ListStatus => '\u{f15ab}',
        Nerd::ListTree => '\u{eb86}',
        Nerd::ListUnordered => '\u{eb17}',
        Nerd::ListUnorderedOne => '\u{f451}',
        Nerd::Litecoin => '\u{f0a61}',
        Nerd::LiveShare => '\u{eb18}',
        Nerd::Liver => '\u{e25a}',
        Nerd::Loading => '\u{eb19}',
        Nerd::LoadingOne => '\u{f0772}',
        Nerd::LocOs => '\u{f349}',
        Nerd::Location => '\u{eb1a}',
        Nerd::LocationArrow => '\u{f124}',
        Nerd::LocationEnter => '\u{f0fc4}',
        Nerd::LocationExit => '\u{f0fc5}',
        Nerd::LocationOne => '\u{f450}',
        Nerd::Lock => '\u{ea75}',
        Nerd::LockAlert => '\u{f08ee}',
        Nerd::LockAlertOutline => '\u{f15d1}',
        Nerd::LockCheck => '\u{f139a}',
        Nerd::LockCheckOutline => '\u{f16a8}',
        Nerd::LockClock => '\u{f097f}',
        Nerd::LockMinus => '\u{f16a9}',
        Nerd::LockMinusOutline => '\u{f16aa}',
        Nerd::LockOff => '\u{f1671}',
        Nerd::LockOffOutline => '\u{f1672}',
        Nerd::LockOne => '\u{f023}',
        Nerd::LockOpen => '\u{f033f}',
        Nerd::LockOpenAlert => '\u{f139b}',
        Nerd::LockOpenAlertOutline => '\u{f15d2}',
        Nerd::LockOpenCheck => '\u{f139c}',
        Nerd::LockOpenCheckOutline => '\u{f16ab}',
        Nerd::LockOpenMinus => '\u{f16ac}',
        Nerd::LockOpenMinusOutline => '\u{f16ad}',
        Nerd::LockOpenOutline => '\u{f0340}',
        Nerd::LockOpenPlus => '\u{f16ae}',
        Nerd::LockOpenPlusOutline => '\u{f16af}',
        Nerd::LockOpenRemove => '\u{f16b0}',
        Nerd::LockOpenRemoveOutline => '\u{f16b1}',
        Nerd::LockOpenVariant => '\u{f0fc6}',
        Nerd::LockOpenVariantOutline => '\u{f0fc7}',
        Nerd::LockOutline => '\u{f0341}',
        Nerd::LockPattern => '\u{f06ea}',
        Nerd::LockPlus => '\u{f05fb}',
        Nerd::LockPlusOutline => '\u{f16b2}',
        Nerd::LockQuestion => '\u{f08ef}',
        Nerd::LockRemove => '\u{f16b3}',
        Nerd::LockRemoveOutline => '\u{f16b4}',
        Nerd::LockReset => '\u{f0773}',
        Nerd::LockSmall => '\u{ebe7}',
        Nerd::LockSmart => '\u{f08b2}',
        Nerd::LockThree => '\u{f033e}',
        Nerd::LockTwo => '\u{f456}',
        Nerd::Locker => '\u{f07d7}',
        Nerd::LockerMultiple => '\u{f07d8}',
        Nerd::Log => '\u{f4ed}',
        Nerd::Login => '\u{f0342}',
        Nerd::LogoGist => '\u{f480}',
        Nerd::LogoGithub => '\u{f470}',
        Nerd::Logout => '\u{f0343}',
        Nerd::LogoutVariant => '\u{f05fd}',
        Nerd::Lollipop => '\u{e2a3}',
        Nerd::LongArrowDown => '\u{f175}',
        Nerd::LongArrowLeft => '\u{f177}',
        Nerd::LongArrowRight => '\u{f178}',
        Nerd::LongArrowUp => '\u{f176}',
        Nerd::LongPause => '\u{e006}',
        Nerd::Longitude => '\u{f0f5a}',
        Nerd::Looks => '\u{f0344}',
        Nerd::Lotion => '\u{f1582}',
        Nerd::LotionOutline => '\u{f1583}',
        Nerd::LotionPlus => '\u{f1584}',
        Nerd::LotionPlusOutline => '\u{f1585}',
        Nerd::Loupe => '\u{f0345}',
        Nerd::LoyaltyCard => '\u{e2a4}',
        Nerd::Lumx => '\u{f0346}',
        Nerd::Lung => '\u{e25b}',
        Nerd::Lungs => '\u{f1084}',
        Nerd::Lxde => '\u{f363}',
        Nerd::LxleLinux => '\u{f33e}',
        Nerd::Lxqt => '\u{f364}',
        Nerd::Mace => '\u{f1843}',
        Nerd::MagazinePistol => '\u{f0324}',
        Nerd::MagazineRifle => '\u{f0323}',
        Nerd::Mageia => '\u{f310}',
        Nerd::Magic => '\u{f0d0}',
        Nerd::MagicStaff => '\u{f1844}',
        Nerd::Magnet => '\u{ebae}',
        Nerd::MagnetOn => '\u{f0348}',
        Nerd::MagnetOne => '\u{f076}',
        Nerd::MagnetTwo => '\u{f0347}',
        Nerd::Magnify => '\u{f0349}',
        Nerd::MagnifyClose => '\u{f0980}',
        Nerd::MagnifyExpand => '\u{f1874}',
        Nerd::MagnifyMinus => '\u{f034a}',
        Nerd::MagnifyMinusCursor => '\u{f0a62}',
        Nerd::MagnifyMinusOutline => '\u{f06ec}',
        Nerd::MagnifyPlus => '\u{f034b}',
        Nerd::MagnifyPlusCursor => '\u{f0a63}',
        Nerd::MagnifyPlusOutline => '\u{f06ed}',
        Nerd::MagnifyRemoveCursor => '\u{f120c}',
        Nerd::MagnifyRemoveOutline => '\u{f120d}',
        Nerd::MagnifyScan => '\u{f1276}',
        Nerd::Mail => '\u{eb1c}',
        Nerd::MailOne => '\u{f42f}',
        Nerd::MailRead => '\u{eb1b}',
        Nerd::MailTwo => '\u{f0ebb}',
        Nerd::Mailbox => '\u{f06ee}',
        Nerd::MailboxOpen => '\u{f0d88}',
        Nerd::MailboxOpenOutline => '\u{f0d89}',
        Nerd::MailboxOpenUp => '\u{f0d8a}',
        Nerd::MailboxOpenUpOutline => '\u{f0d8b}',
        Nerd::MailboxOutline => '\u{f0d8c}',
        Nerd::MailboxUp => '\u{f0d8d}',
        Nerd::MailboxUpOutline => '\u{f0d8e}',
        Nerd::MakeupBrushes => '\u{e25c}',
        Nerd::Male => '\u{f183}',
        Nerd::Mandriva => '\u{f311}',
        Nerd::Manjaro => '\u{f160a}',
        Nerd::Map => '\u{f034d}',
        Nerd::MapCheck => '\u{f0ebc}',
        Nerd::MapCheckOutline => '\u{f0ebd}',
        Nerd::MapClock => '\u{f0d1e}',
        Nerd::MapClockOutline => '\u{f0d1f}',
        Nerd::MapLegend => '\u{f0a01}',
        Nerd::MapMarker => '\u{f034e}',
        Nerd::MapMarkerAccount => '\u{f18e3}',
        Nerd::MapMarkerAccountOutline => '\u{f18e4}',
        Nerd::MapMarkerAlert => '\u{f0f05}',
        Nerd::MapMarkerAlertOutline => '\u{f0f06}',
        Nerd::MapMarkerCheck => '\u{f0c95}',
        Nerd::MapMarkerCheckOutline => '\u{f12fb}',
        Nerd::MapMarkerCircle => '\u{f034f}',
        Nerd::MapMarkerDistance => '\u{f08f0}',
        Nerd::MapMarkerDown => '\u{f1102}',
        Nerd::MapMarkerLeft => '\u{f12db}',
        Nerd::MapMarkerLeftOutline => '\u{f12dd}',
        Nerd::MapMarkerMinus => '\u{f0650}',
        Nerd::MapMarkerMinusOutline => '\u{f12f9}',
        Nerd::MapMarkerMultiple => '\u{f0350}',
        Nerd::MapMarkerMultipleOutline => '\u{f1277}',
        Nerd::MapMarkerOff => '\u{f0351}',
        Nerd::MapMarkerOffOutline => '\u{f12fd}',
        Nerd::MapMarkerOutline => '\u{f07d9}',
        Nerd::MapMarkerPath => '\u{f0d20}',
        Nerd::MapMarkerPlus => '\u{f0651}',
        Nerd::MapMarkerPlusOutline => '\u{f12f8}',
        Nerd::MapMarkerQuestion => '\u{f0f07}',
        Nerd::MapMarkerQuestionOutline => '\u{f0f08}',
        Nerd::MapMarkerRadius => '\u{f0352}',
        Nerd::MapMarkerRadiusOutline => '\u{f12fc}',
        Nerd::MapMarkerRemove => '\u{f0f09}',
        Nerd::MapMarkerRemoveOutline => '\u{f12fa}',
        Nerd::MapMarkerRemoveVariant => '\u{f0f0a}',
        Nerd::MapMarkerRight => '\u{f12dc}',
        Nerd::MapMarkerRightOutline => '\u{f12de}',
        Nerd::MapMarkerStar => '\u{f1608}',
        Nerd::MapMarkerStarOutline => '\u{f1609}',
        Nerd::MapMarkerUp => '\u{f1103}',
        Nerd::MapMinus => '\u{f0981}',
        Nerd::MapOutline => '\u{f0982}',
        Nerd::MapPlus => '\u{f0983}',
        Nerd::MapSearch => '\u{f0984}',
        Nerd::MapSearchOutline => '\u{f0985}',
        Nerd::Mapbox => '\u{f0baa}',
        Nerd::Margin => '\u{f0353}',
        Nerd::MarkGithub => '\u{f408}',
        Nerd::Markdown => '\u{eb1d}',
        Nerd::MarkdownOne => '\u{f48a}',
        Nerd::Marker => '\u{f0652}',
        Nerd::MarkerCancel => '\u{f0dd9}',
        Nerd::MarkerCheck => '\u{f0355}',
        Nerd::Mastodon => '\u{f0ad1}',
        Nerd::Mate => '\u{f365}',
        Nerd::MaterialDesign => '\u{f0986}',
        Nerd::MaterialUi => '\u{f0357}',
        Nerd::MathCompass => '\u{f0358}',
        Nerd::MathCos => '\u{f0c96}',
        Nerd::MathIntegral => '\u{f0fc8}',
        Nerd::MathIntegralBox => '\u{f0fc9}',
        Nerd::MathLog => '\u{f1085}',
        Nerd::MathNorm => '\u{f0fca}',
        Nerd::MathNormBox => '\u{f0fcb}',
        Nerd::MathSin => '\u{f0c97}',
        Nerd::MathTan => '\u{f0c98}',
        Nerd::Matrix => '\u{f0628}',
        Nerd::Maxcdn => '\u{f136}',
        Nerd::Maximize => '\u{e25d}',
        Nerd::Meat => '\u{e2a5}',
        Nerd::Medal => '\u{f0987}',
        Nerd::MedalOutline => '\u{f1326}',
        Nerd::MedicalBag => '\u{f06ef}',
        Nerd::MedicalCottonSwab => '\u{f1ab8}',
        Nerd::Medicine => '\u{e221}',
        Nerd::Meditation => '\u{f117b}',
        Nerd::Medkit => '\u{f0fa}',
        Nerd::Megaphone => '\u{eb1e}',
        Nerd::MegaphoneOne => '\u{f45f}',
        Nerd::Meh => '\u{f11a}',
        Nerd::Memory => '\u{f035b}',
        Nerd::Menorah => '\u{f17d4}',
        Nerd::MenorahFire => '\u{f17d5}',
        Nerd::Mention => '\u{eb1f}',
        Nerd::MentionOne => '\u{f486}',
        Nerd::Menu => '\u{eb94}',
        Nerd::MenuDown => '\u{f035d}',
        Nerd::MenuDownOutline => '\u{f06b6}',
        Nerd::MenuLeft => '\u{f035e}',
        Nerd::MenuLeftOutline => '\u{f0a02}',
        Nerd::MenuOne => '\u{f035c}',
        Nerd::MenuOpen => '\u{f0bab}',
        Nerd::MenuRight => '\u{f035f}',
        Nerd::MenuRightOutline => '\u{f0a03}',
        Nerd::MenuSwap => '\u{f0a64}',
        Nerd::MenuSwapOutline => '\u{f0a65}',
        Nerd::MenuUp => '\u{f0360}',
        Nerd::MenuUpOutline => '\u{f06b7}',
        Nerd::Merge => '\u{ebab}',
        Nerd::MergeOne => '\u{f0f5c}',
        Nerd::Message => '\u{f0361}',
        Nerd::MessageAlert => '\u{f0362}',
        Nerd::MessageAlertOutline => '\u{f0a04}',
        Nerd::MessageArrowLeft => '\u{f12f2}',
        Nerd::MessageArrowLeftOutline => '\u{f12f3}',
        Nerd::MessageArrowRight => '\u{f12f4}',
        Nerd::MessageArrowRightOutline => '\u{f12f5}',
        Nerd::MessageBadge => '\u{f1941}',
        Nerd::MessageBadgeOutline => '\u{f1942}',
        Nerd::MessageBookmark => '\u{f15ac}',
        Nerd::MessageBookmarkOutline => '\u{f15ad}',
        Nerd::MessageBulleted => '\u{f06a2}',
        Nerd::MessageBulletedOff => '\u{f06a3}',
        Nerd::MessageCog => '\u{f06f1}',
        Nerd::MessageCogOutline => '\u{f1172}',
        Nerd::MessageDraw => '\u{f0363}',
        Nerd::MessageFast => '\u{f19cc}',
        Nerd::MessageFastOutline => '\u{f19cd}',
        Nerd::MessageFlash => '\u{f15a9}',
        Nerd::MessageFlashOutline => '\u{f15aa}',
        Nerd::MessageImage => '\u{f0364}',
        Nerd::MessageImageOutline => '\u{f116c}',
        Nerd::MessageLock => '\u{f0fcc}',
        Nerd::MessageLockOutline => '\u{f116d}',
        Nerd::MessageMinus => '\u{f116e}',
        Nerd::MessageMinusOutline => '\u{f116f}',
        Nerd::MessageOff => '\u{f164d}',
        Nerd::MessageOffOutline => '\u{f164e}',
        Nerd::MessageOutline => '\u{f0365}',
        Nerd::MessagePlus => '\u{f0653}',
        Nerd::MessagePlusOutline => '\u{f10bb}',
        Nerd::MessageProcessing => '\u{f0366}',
        Nerd::MessageProcessingOutline => '\u{f1170}',
        Nerd::MessageQuestion => '\u{f173a}',
        Nerd::MessageQuestionOutline => '\u{f173b}',
        Nerd::MessageReply => '\u{f0367}',
        Nerd::MessageReplyOutline => '\u{f173d}',
        Nerd::MessageReplyText => '\u{f0368}',
        Nerd::MessageReplyTextOutline => '\u{f173e}',
        Nerd::MessageSettings => '\u{f06f0}',
        Nerd::MessageSettingsOutline => '\u{f1171}',
        Nerd::MessageStar => '\u{f069a}',
        Nerd::MessageStarOutline => '\u{f1250}',
        Nerd::MessageText => '\u{f0369}',
        Nerd::MessageTextClock => '\u{f1173}',
        Nerd::MessageTextClockOutline => '\u{f1174}',
        Nerd::MessageTextFast => '\u{f19ce}',
        Nerd::MessageTextFastOutline => '\u{f19cf}',
        Nerd::MessageTextLock => '\u{f0fcd}',
        Nerd::MessageTextLockOutline => '\u{f1175}',
        Nerd::MessageTextOutline => '\u{f036a}',
        Nerd::MessageVideo => '\u{f036b}',
        Nerd::Meteor => '\u{f0629}',
        Nerd::Meter => '\u{f463}',
        Nerd::MeterElectric => '\u{f1a57}',
        Nerd::MeterElectricOutline => '\u{f1a58}',
        Nerd::MeterGas => '\u{f1a59}',
        Nerd::MeterGasOutline => '\u{f1a5a}',
        Nerd::Metronome => '\u{f07da}',
        Nerd::MetronomeTick => '\u{f07db}',
        Nerd::MicroSd => '\u{f07dc}',
        Nerd::Microphone => '\u{f130}',
        Nerd::MicrophoneMinus => '\u{f08b3}',
        Nerd::MicrophoneOff => '\u{f036d}',
        Nerd::MicrophoneOne => '\u{f036c}',
        Nerd::MicrophoneOutline => '\u{f036e}',
        Nerd::MicrophonePlus => '\u{f08b4}',
        Nerd::MicrophoneQuestion => '\u{f1989}',
        Nerd::MicrophoneQuestionOutline => '\u{f198a}',
        Nerd::MicrophoneSettings => '\u{f036f}',
        Nerd::MicrophoneVariant => '\u{f0370}',
        Nerd::MicrophoneVariantOff => '\u{f0371}',
        Nerd::Microscope => '\u{e222}',
        Nerd::MicroscopeOne => '\u{f0654}',
        Nerd::Microsoft => '\u{f0372}',
        Nerd::MicrosoftAccess => '\u{f138e}',
        Nerd::MicrosoftAzure => '\u{f0805}',
        Nerd::MicrosoftAzureDevops => '\u{f0fd5}',
        Nerd::MicrosoftBing => '\u{f00a4}',
        Nerd::MicrosoftDynamicsThreesixfive => '\u{f0988}',
        Nerd::MicrosoftEdge => '\u{f01e9}',
        Nerd::MicrosoftExcel => '\u{f138f}',
        Nerd::MicrosoftInternetExplorer => '\u{f0300}',
        Nerd::MicrosoftOffice => '\u{f03c6}',
        Nerd::MicrosoftOnedrive => '\u{f03ca}',
        Nerd::MicrosoftOnenote => '\u{f0747}',
        Nerd::MicrosoftOutlook => '\u{f0d22}',
        Nerd::MicrosoftPowerpoint => '\u{f1390}',
        Nerd::MicrosoftSharepoint => '\u{f1391}',
        Nerd::MicrosoftTeams => '\u{f02bb}',
        Nerd::MicrosoftVisualStudio => '\u{f0610}',
        Nerd::MicrosoftVisualStudioCode => '\u{f0a1e}',
        Nerd::MicrosoftWindows => '\u{f05b3}',
        Nerd::MicrosoftWindowsClassic => '\u{f0a21}',
        Nerd::MicrosoftWord => '\u{f1392}',
        Nerd::MicrosoftXbox => '\u{f05b9}',
        Nerd::MicrosoftXboxController => '\u{f05ba}',
        Nerd::MicrosoftXboxControllerBatteryAlert => '\u{f074b}',
        Nerd::MicrosoftXboxControllerBatteryCharging => '\u{f0a22}',
        Nerd::MicrosoftXboxControllerBatteryEmpty => '\u{f074c}',
        Nerd::MicrosoftXboxControllerBatteryFull => '\u{f074d}',
        Nerd::MicrosoftXboxControllerBatteryLow => '\u{f074e}',
        Nerd::MicrosoftXboxControllerBatteryMedium => '\u{f074f}',
        Nerd::MicrosoftXboxControllerBatteryUnknown => '\u{f0750}',
        Nerd::MicrosoftXboxControllerMenu => '\u{f0e6f}',
        Nerd::MicrosoftXboxControllerOff => '\u{f05bb}',
        Nerd::MicrosoftXboxControllerView => '\u{f0e70}',
        Nerd::Microwave => '\u{f0c99}',
        Nerd::MicrowaveOff => '\u{f1423}',
        Nerd::Middleware => '\u{f0f5d}',
        Nerd::MiddlewareOutline => '\u{f0f5e}',
        Nerd::Midi => '\u{f08f1}',
        Nerd::MidiPort => '\u{f08f2}',
        Nerd::Milestone => '\u{eb20}',
        Nerd::MilestoneOne => '\u{f45d}',
        Nerd::MilkBottle => '\u{e223}',
        Nerd::Mine => '\u{f0dda}',
        Nerd::Minecraft => '\u{f0373}',
        Nerd::MiniSd => '\u{f0a05}',
        Nerd::Minidisc => '\u{f0a06}',
        Nerd::Minimize => '\u{e224}',
        Nerd::Minus => '\u{f068}',
        Nerd::MinusBox => '\u{f0375}',
        Nerd::MinusBoxMultiple => '\u{f1141}',
        Nerd::MinusBoxMultipleOutline => '\u{f1142}',
        Nerd::MinusBoxOutline => '\u{f06f2}',
        Nerd::MinusCircle => '\u{f0376}',
        Nerd::MinusCircleMultiple => '\u{f035a}',
        Nerd::MinusCircleMultipleOutline => '\u{f0ad3}',
        Nerd::MinusCircleOff => '\u{f1459}',
        Nerd::MinusCircleOffOutline => '\u{f145a}',
        Nerd::MinusCircleOutline => '\u{f0377}',
        Nerd::MinusNetwork => '\u{f0378}',
        Nerd::MinusNetworkOutline => '\u{f0c9a}',
        Nerd::MinusOne => '\u{f0374}',
        Nerd::MinusSign => '\u{f056}',
        Nerd::MinusSignAlt => '\u{f146}',
        Nerd::MinusThick => '\u{f1639}',
        Nerd::Mirror => '\u{ea69}',
        Nerd::MirrorOne => '\u{f41a}',
        Nerd::MirrorRectangle => '\u{f179f}',
        Nerd::MirrorTwo => '\u{f11fd}',
        Nerd::MirrorVariant => '\u{f17a0}',
        Nerd::MixedMartialArts => '\u{f0d8f}',
        Nerd::MixedReality => '\u{f087f}',
        Nerd::MobilePhone => '\u{f10b}',
        Nerd::Molecule => '\u{e225}',
        Nerd::MoleculeCo => '\u{f12fe}',
        Nerd::MoleculeCotwo => '\u{f07e4}',
        Nerd::MoleculeOne => '\u{f0bac}',
        Nerd::Money => '\u{f0d6}',
        Nerd::Monitor => '\u{f0379}',
        Nerd::MonitorAccount => '\u{f1a5b}',
        Nerd::MonitorArrowDown => '\u{f19d0}',
        Nerd::MonitorArrowDownVariant => '\u{f19d1}',
        Nerd::MonitorCellphone => '\u{f0989}',
        Nerd::MonitorCellphoneStar => '\u{f098a}',
        Nerd::MonitorDashboard => '\u{f0a07}',
        Nerd::MonitorEdit => '\u{f12c6}',
        Nerd::MonitorEye => '\u{f13b4}',
        Nerd::MonitorLock => '\u{f0ddb}',
        Nerd::MonitorMultiple => '\u{f037a}',
        Nerd::MonitorOff => '\u{f0d90}',
        Nerd::MonitorScreenshot => '\u{f0e51}',
        Nerd::MonitorShare => '\u{f1483}',
        Nerd::MonitorShimmer => '\u{f1104}',
        Nerd::MonitorSmall => '\u{f1876}',
        Nerd::MonitorSpeaker => '\u{f0f5f}',
        Nerd::MonitorSpeakerOff => '\u{f0f60}',
        Nerd::MonitorStar => '\u{f0ddc}',
        Nerd::Moon => '\u{f4ee}',
        Nerd::MoonCloud => '\u{e226}',
        Nerd::MoonFirstQuarter => '\u{f0f61}',
        Nerd::MoonFull => '\u{f0f62}',
        Nerd::MoonLastQuarter => '\u{f0f63}',
        Nerd::MoonNew => '\u{f0f64}',
        Nerd::MoonWaningCrescent => '\u{f0f65}',
        Nerd::MoonWaningGibbous => '\u{f0f66}',
        Nerd::MoonWaxingCrescent => '\u{f0f67}',
        Nerd::MoonWaxingGibbous => '\u{f0f68}',
        Nerd::Moped => '\u{f1086}',
        Nerd::MopedElectric => '\u{f15b7}',
        Nerd::MopedElectricOutline => '\u{f15b8}',
        Nerd::MopedOutline => '\u{f15b9}',
        Nerd::More => '\u{f037b}',
        Nerd::MortarBoard => '\u{eb21}',
        Nerd::MortarBoardOne => '\u{f494}',
        Nerd::MortarPestle => '\u{f1748}',
        Nerd::MortarPestlePlus => '\u{f03f1}',
        Nerd::Mosque => '\u{f1827}',
        Nerd::MotherHeart => '\u{f1314}',
        Nerd::MotherNurse => '\u{f0d21}',
        Nerd::Motion => '\u{f15b2}',
        Nerd::MotionOutline => '\u{f15b3}',
        Nerd::MotionPause => '\u{f1590}',
        Nerd::MotionPauseOutline => '\u{f1592}',
        Nerd::MotionPlay => '\u{f158f}',
        Nerd::MotionPlayOutline => '\u{f1591}',
        Nerd::MotionSensor => '\u{f0d91}',
        Nerd::MotionSensorOff => '\u{f1435}',
        Nerd::Motorbike => '\u{f037c}',
        Nerd::MotorbikeElectric => '\u{f15ba}',
        Nerd::Mountains => '\u{e2a6}',
        Nerd::Mouse => '\u{f037d}',
        Nerd::MouseBluetooth => '\u{f098b}',
        Nerd::MouseMoveDown => '\u{f1550}',
        Nerd::MouseMoveUp => '\u{f1551}',
        Nerd::MouseMoveVertical => '\u{f1552}',
        Nerd::MouseOff => '\u{f037e}',
        Nerd::MouseVariant => '\u{f037f}',
        Nerd::MouseVariantOff => '\u{f0380}',
        Nerd::Move => '\u{eb22}',
        Nerd::MoveOne => '\u{f047}',
        Nerd::MoveResize => '\u{f0655}',
        Nerd::MoveResizeVariant => '\u{f0656}',
        Nerd::MoveToBottom => '\u{f4ef}',
        Nerd::MoveToEnd => '\u{f4f0}',
        Nerd::MoveToStart => '\u{f4f1}',
        Nerd::MoveToTop => '\u{f4f2}',
        Nerd::Movie => '\u{f0381}',
        Nerd::MovieCheck => '\u{f16f3}',
        Nerd::MovieCheckOutline => '\u{f16f4}',
        Nerd::MovieCog => '\u{f16f5}',
        Nerd::MovieCogOutline => '\u{f16f6}',
        Nerd::MovieEdit => '\u{f1122}',
        Nerd::MovieEditOutline => '\u{f1123}',
        Nerd::MovieFilter => '\u{f1124}',
        Nerd::MovieFilterOutline => '\u{f1125}',
        Nerd::MovieMinus => '\u{f16f7}',
        Nerd::MovieMinusOutline => '\u{f16f8}',
        Nerd::MovieOff => '\u{f16f9}',
        Nerd::MovieOffOutline => '\u{f16fa}',
        Nerd::MovieOpen => '\u{f0fce}',
        Nerd::MovieOpenCheck => '\u{f16fb}',
        Nerd::MovieOpenCheckOutline => '\u{f16fc}',
        Nerd::MovieOpenCog => '\u{f16fd}',
        Nerd::MovieOpenCogOutline => '\u{f16fe}',
        Nerd::MovieOpenEdit => '\u{f16ff}',
        Nerd::MovieOpenEditOutline => '\u{f1700}',
        Nerd::MovieOpenMinus => '\u{f1701}',
        Nerd::MovieOpenMinusOutline => '\u{f1702}',
        Nerd::MovieOpenOff => '\u{f1703}',
        Nerd::MovieOpenOffOutline => '\u{f1704}',
        Nerd::MovieOpenOutline => '\u{f0fcf}',
        Nerd::MovieOpenPlay => '\u{f1705}',
        Nerd::MovieOpenPlayOutline => '\u{f1706}',
        Nerd::MovieOpenPlus => '\u{f1707}',
        Nerd::MovieOpenPlusOutline => '\u{f1708}',
        Nerd::MovieOpenRemove => '\u{f1709}',
        Nerd::MovieOpenRemoveOutline => '\u{f170a}',
        Nerd::MovieOpenSettings => '\u{f170b}',
        Nerd::MovieOpenSettingsOutline => '\u{f170c}',
        Nerd::MovieOpenStar => '\u{f170d}',
        Nerd::MovieOpenStarOutline => '\u{f170e}',
        Nerd::MovieOutline => '\u{f0ddd}',
        Nerd::MoviePlay => '\u{f170f}',
        Nerd::MoviePlayOutline => '\u{f1710}',
        Nerd::MoviePlus => '\u{f1711}',
        Nerd::MoviePlusOutline => '\u{f1712}',
        Nerd::MovieRemove => '\u{f1713}',
        Nerd::MovieRemoveOutline => '\u{f1714}',
        Nerd::MovieRoll => '\u{f07de}',
        Nerd::MovieSearch => '\u{f11d2}',
        Nerd::MovieSearchOutline => '\u{f11d3}',
        Nerd::MovieSettings => '\u{f1715}',
        Nerd::MovieSettingsOutline => '\u{f1716}',
        Nerd::MovieStar => '\u{f1717}',
        Nerd::MovieStarOutline => '\u{f1718}',
        Nerd::Mower => '\u{f166f}',
        Nerd::MowerBag => '\u{f1670}',
        Nerd::Mpv => '\u{f36e}',
        Nerd::Muffin => '\u{f098c}',
        Nerd::MultiSelect => '\u{f4f3}',
        Nerd::Multicast => '\u{f1893}',
        Nerd::MultipleWindows => '\u{eb23}',
        Nerd::Multiplication => '\u{f0382}',
        Nerd::MultiplicationBox => '\u{f0383}',
        Nerd::Mushroom => '\u{e227}',
        Nerd::MushroomOff => '\u{f13fa}',
        Nerd::MushroomOffOutline => '\u{f13fb}',
        Nerd::MushroomOne => '\u{f07df}',
        Nerd::MushroomOutline => '\u{f07e0}',
        Nerd::Music => '\u{f001}',
        Nerd::MusicAccidentalDoubleFlat => '\u{f0f69}',
        Nerd::MusicAccidentalDoubleSharp => '\u{f0f6a}',
        Nerd::MusicAccidentalFlat => '\u{f0f6b}',
        Nerd::MusicAccidentalNatural => '\u{f0f6c}',
        Nerd::MusicAccidentalSharp => '\u{f0f6d}',
        Nerd::MusicBox => '\u{f0384}',
        Nerd::MusicBoxMultiple => '\u{f0333}',
        Nerd::MusicBoxMultipleOutline => '\u{f0f04}',
        Nerd::MusicBoxOutline => '\u{f0385}',
        Nerd::MusicCircle => '\u{f0386}',
        Nerd::MusicCircleOutline => '\u{f0ad4}',
        Nerd::MusicClefAlto => '\u{f0f6e}',
        Nerd::MusicClefBass => '\u{f0f6f}',
        Nerd::MusicClefTreble => '\u{f0f70}',
        Nerd::MusicNote => '\u{f0387}',
        Nerd::MusicNoteBluetooth => '\u{f05fe}',
        Nerd::MusicNoteBluetoothOff => '\u{f05ff}',
        Nerd::MusicNoteEighthDotted => '\u{f0f71}',
        Nerd::MusicNoteHalf => '\u{f0389}',
        Nerd::MusicNoteHalfDotted => '\u{f0f72}',
        Nerd::MusicNoteOff => '\u{f038a}',
        Nerd::MusicNoteOffOutline => '\u{f0f73}',
        Nerd::MusicNoteOne => '\u{f0388}',
        Nerd::MusicNoteOutline => '\u{f0f74}',
        Nerd::MusicNotePlus => '\u{f0dde}',
        Nerd::MusicNoteQuarter => '\u{f038b}',
        Nerd::MusicNoteQuarterDotted => '\u{f0f75}',
        Nerd::MusicNoteSixteenth => '\u{f038c}',
        Nerd::MusicNoteSixteenthDotted => '\u{f0f76}',
        Nerd::MusicNoteWhole => '\u{f038d}',
        Nerd::MusicNoteWholeDotted => '\u{f0f77}',
        Nerd::MusicOff => '\u{f075b}',
        Nerd::MusicOne => '\u{f075a}',
        Nerd::MusicRestEighth => '\u{f0f78}',
        Nerd::MusicRestHalf => '\u{f0f79}',
        Nerd::MusicRestQuarter => '\u{f0f7a}',
        Nerd::MusicRestSixteenth => '\u{f0f7b}',
        Nerd::MusicRestWhole => '\u{f0f7c}',
        Nerd::Mustache => '\u{e228}',
        Nerd::MustacheOne => '\u{f15de}',
        Nerd::Mute => '\u{eb24}',
        Nerd::MuteOne => '\u{f466}',
        Nerd::MxLinux => '\u{f33f}',
        Nerd::Mysql => '\u{e229}',
        Nerd::Nail => '\u{f0ddf}',
        Nerd::Nas => '\u{f08f3}',
        Nerd::Nativescript => '\u{f0880}',
        Nerd::Nature => '\u{f038e}',
        Nerd::NaturePeople => '\u{f038f}',
        Nerd::Navigation => '\u{f0390}',
        Nerd::NavigationOutline => '\u{f1607}',
        Nerd::NavigationVariantOutline => '\u{f18f1}',
        Nerd::NearMe => '\u{f05cd}',
        Nerd::NearMeOne => '\u{f18f0}',
        Nerd::Necklace => '\u{f0f0b}',
        Nerd::Needle => '\u{f0391}',
        Nerd::NeedleOff => '\u{f19d2}',
        Nerd::Neovim => '\u{f36f}',
        Nerd::Netflix => '\u{f0746}',
        Nerd::Network => '\u{f06f3}',
        Nerd::NetworkOff => '\u{f0c9b}',
        Nerd::NetworkOffOutline => '\u{f0c9c}',
        Nerd::NetworkOutline => '\u{f0c9d}',
        Nerd::NetworkPos => '\u{f1acb}',
        Nerd::NetworkStrengthFour => '\u{f08fa}',
        Nerd::NetworkStrengthFourAlert => '\u{f08fb}',
        Nerd::NetworkStrengthFourCog => '\u{f191a}',
        Nerd::NetworkStrengthOff => '\u{f08fc}',
        Nerd::NetworkStrengthOffOutline => '\u{f08fd}',
        Nerd::NetworkStrengthOne => '\u{f08f4}',
        Nerd::NetworkStrengthOneAlert => '\u{f08f5}',
        Nerd::NetworkStrengthOutline => '\u{f08fe}',
        Nerd::NetworkStrengthThree => '\u{f08f8}',
        Nerd::NetworkStrengthThreeAlert => '\u{f08f9}',
        Nerd::NetworkStrengthTwo => '\u{f08f6}',
        Nerd::NetworkStrengthTwoAlert => '\u{f08f7}',
        Nerd::NewBox => '\u{f0394}',
        Nerd::NewFile => '\u{ea7f}',
        Nerd::NewFolder => '\u{ea80}',
        Nerd::Newline => '\u{ebea}',
        Nerd::Newspaper => '\u{f0395}',
        Nerd::NewspaperCheck => '\u{f1943}',
        Nerd::NewspaperMinus => '\u{f0f0c}',
        Nerd::NewspaperPlus => '\u{f0f0d}',
        Nerd::NewspaperRemove => '\u{f1944}',
        Nerd::NewspaperVariant => '\u{f1001}',
        Nerd::NewspaperVariantMultiple => '\u{f1002}',
        Nerd::NewspaperVariantMultipleOutline => '\u{f1003}',
        Nerd::NewspaperVariantOutline => '\u{f1004}',
        Nerd::Nfc => '\u{f0396}',
        Nerd::NfcSearchVariant => '\u{f0e53}',
        Nerd::NfcTap => '\u{f0397}',
        Nerd::NfcVariant => '\u{f0398}',
        Nerd::NfcVariantOff => '\u{f0e54}',
        Nerd::Ninja => '\u{f0774}',
        Nerd::Nintendo => '\u{e22a}',
        Nerd::NintendoGameBoy => '\u{f1393}',
        Nerd::NintendoSwitch => '\u{f07e1}',
        Nerd::NintendoWii => '\u{f05ab}',
        Nerd::NintendoWiiu => '\u{f072d}',
        Nerd::Nix => '\u{f1105}',
        Nerd::Nixos => '\u{f313}',
        Nerd::NoEntry => '\u{f4f4}',
        Nerd::NoNewline => '\u{eb25}',
        Nerd::Nodejs => '\u{f0399}',
        Nerd::Nonmarkingreturn => '\u{d}',
        Nerd::Noodles => '\u{f117e}',
        Nerd::NorthStar => '\u{f4f5}',
        Nerd::NotEqual => '\u{f098d}',
        Nerd::NotEqualVariant => '\u{f098e}',
        Nerd::Note => '\u{eb26}',
        Nerd::NoteAlert => '\u{f177d}',
        Nerd::NoteAlertOutline => '\u{f177e}',
        Nerd::NoteCheck => '\u{f177f}',
        Nerd::NoteCheckOutline => '\u{f1780}',
        Nerd::NoteEdit => '\u{f1781}',
        Nerd::NoteEditOutline => '\u{f1782}',
        Nerd::NoteMinus => '\u{f164f}',
        Nerd::NoteMinusOutline => '\u{f1650}',
        Nerd::NoteMultiple => '\u{f06b8}',
        Nerd::NoteMultipleOutline => '\u{f06b9}',
        Nerd::NoteOff => '\u{f1783}',
        Nerd::NoteOffOutline => '\u{f1784}',
        Nerd::NoteOne => '\u{f4f6}',
        Nerd::NoteOutline => '\u{f039b}',
        Nerd::NotePlus => '\u{f039c}',
        Nerd::NotePlusOutline => '\u{f039d}',
        Nerd::NoteRemove => '\u{f1651}',
        Nerd::NoteRemoveOutline => '\u{f1652}',
        Nerd::NoteSearch => '\u{f1653}',
        Nerd::NoteSearchOutline => '\u{f1654}',
        Nerd::NoteText => '\u{f039e}',
        Nerd::NoteTextOutline => '\u{f11d7}',
        Nerd::NoteTwo => '\u{f039a}',
        Nerd::Notebook => '\u{ebaf}',
        Nerd::NotebookCheck => '\u{f14f5}',
        Nerd::NotebookCheckOutline => '\u{f14f6}',
        Nerd::NotebookEdit => '\u{f14e7}',
        Nerd::NotebookEditOutline => '\u{f14e9}',
        Nerd::NotebookHeart => '\u{f1a0b}',
        Nerd::NotebookHeartOutline => '\u{f1a0c}',
        Nerd::NotebookMinus => '\u{f1610}',
        Nerd::NotebookMinusOutline => '\u{f1611}',
        Nerd::NotebookMultiple => '\u{f0e55}',
        Nerd::NotebookOne => '\u{f082e}',
        Nerd::NotebookOutline => '\u{f0ebf}',
        Nerd::NotebookPlus => '\u{f1612}',
        Nerd::NotebookPlusOutline => '\u{f1613}',
        Nerd::NotebookRemove => '\u{f1614}',
        Nerd::NotebookRemoveOutline => '\u{f1615}',
        Nerd::NotebookTemplate => '\u{ebbf}',
        Nerd::NotificationClearAll => '\u{f039f}',
        Nerd::Npm => '\u{f06f7}',
        Nerd::Nuke => '\u{f06a4}',
        Nerd::Null => '\u{f07e2}',
        Nerd::Number => '\u{f4f7}',
        Nerd::Numeric => '\u{f03a0}',
        Nerd::NumericEight => '\u{f0b41}',
        Nerd::NumericEightBox => '\u{f03b9}',
        Nerd::NumericEightBoxMultiple => '\u{f0f16}',
        Nerd::NumericEightBoxMultipleOutline => '\u{f03ba}',
        Nerd::NumericEightBoxOutline => '\u{f03bb}',
        Nerd::NumericEightCircle => '\u{f0cae}',
        Nerd::NumericEightCircleOutline => '\u{f0caf}',
        Nerd::NumericFive => '\u{f0b3e}',
        Nerd::NumericFiveBox => '\u{f03b1}',
        Nerd::NumericFiveBoxMultiple => '\u{f0f13}',
        Nerd::NumericFiveBoxMultipleOutline => '\u{f03af}',
        Nerd::NumericFiveBoxOutline => '\u{f03b0}',
        Nerd::NumericFiveCircle => '\u{f0ca8}',
        Nerd::NumericFiveCircleOutline => '\u{f0ca9}',
        Nerd::NumericFour => '\u{f0b3d}',
        Nerd::NumericFourBox => '\u{f03ad}',
        Nerd::NumericFourBoxMultiple => '\u{f0f12}',
        Nerd::NumericFourBoxMultipleOutline => '\u{f03b2}',
        Nerd::NumericFourBoxOutline => '\u{f03ae}',
        Nerd::NumericFourCircle => '\u{f0ca6}',
        Nerd::NumericFourCircleOutline => '\u{f0ca7}',
        Nerd::NumericNegativeOne => '\u{f1052}',
        Nerd::NumericNine => '\u{f0b42}',
        Nerd::NumericNineBox => '\u{f03bc}',
        Nerd::NumericNineBoxMultiple => '\u{f0f17}',
        Nerd::NumericNineBoxMultipleOutline => '\u{f03bd}',
        Nerd::NumericNineBoxOutline => '\u{f03be}',
        Nerd::NumericNineCircle => '\u{f0cb0}',
        Nerd::NumericNineCircleOutline => '\u{f0cb1}',
        Nerd::NumericNinePlus => '\u{f0fee}',
        Nerd::NumericNinePlusBox => '\u{f03bf}',
        Nerd::NumericNinePlusBoxMultiple => '\u{f0f18}',
        Nerd::NumericNinePlusBoxMultipleOutline => '\u{f03c0}',
        Nerd::NumericNinePlusBoxOutline => '\u{f03c1}',
        Nerd::NumericNinePlusCircle => '\u{f0cb2}',
        Nerd::NumericNinePlusCircleOutline => '\u{f0cb3}',
        Nerd::NumericOff => '\u{f19d3}',
        Nerd::NumericOne => '\u{f0b3a}',
        Nerd::NumericOneBox => '\u{f03a4}',
        Nerd::NumericOneBoxMultiple => '\u{f0f0f}',
        Nerd::NumericOneBoxMultipleOutline => '\u{f03a5}',
        Nerd::NumericOneBoxOutline => '\u{f03a6}',
        Nerd::NumericOneCircle => '\u{f0ca0}',
        Nerd::NumericOneCircleOutline => '\u{f0ca1}',
        Nerd::NumericOnezero => '\u{f0fe9}',
        Nerd::NumericOnezeroBox => '\u{f0f7d}',
        Nerd::NumericOnezeroBoxMultiple => '\u{f0fea}',
        Nerd::NumericOnezeroBoxMultipleOutline => '\u{f0feb}',
        Nerd::NumericOnezeroBoxOutline => '\u{f0f7e}',
        Nerd::NumericOnezeroCircle => '\u{f0fec}',
        Nerd::NumericOnezeroCircleOutline => '\u{f0fed}',
        Nerd::NumericPositiveOne => '\u{f15cb}',
        Nerd::NumericSeven => '\u{f0b40}',
        Nerd::NumericSevenBox => '\u{f03b6}',
        Nerd::NumericSevenBoxMultiple => '\u{f0f15}',
        Nerd::NumericSevenBoxMultipleOutline => '\u{f03b7}',
        Nerd::NumericSevenBoxOutline => '\u{f03b8}',
        Nerd::NumericSevenCircle => '\u{f0cac}',
        Nerd::NumericSevenCircleOutline => '\u{f0cad}',
        Nerd::NumericSix => '\u{f0b3f}',
        Nerd::NumericSixBox => '\u{f03b3}',
        Nerd::NumericSixBoxMultiple => '\u{f0f14}',
        Nerd::NumericSixBoxMultipleOutline => '\u{f03b4}',
        Nerd::NumericSixBoxOutline => '\u{f03b5}',
        Nerd::NumericSixCircle => '\u{f0caa}',
        Nerd::NumericSixCircleOutline => '\u{f0cab}',
        Nerd::NumericThree => '\u{f0b3c}',
        Nerd::NumericThreeBox => '\u{f03aa}',
        Nerd::NumericThreeBoxMultiple => '\u{f0f11}',
        Nerd::NumericThreeBoxMultipleOutline => '\u{f03ab}',
        Nerd::NumericThreeBoxOutline => '\u{f03ac}',
        Nerd::NumericThreeCircle => '\u{f0ca4}',
        Nerd::NumericThreeCircleOutline => '\u{f0ca5}',
        Nerd::NumericTwo => '\u{f0b3b}',
        Nerd::NumericTwoBox => '\u{f03a7}',
        Nerd::NumericTwoBoxMultiple => '\u{f0f10}',
        Nerd::NumericTwoBoxMultipleOutline => '\u{f03a8}',
        Nerd::NumericTwoBoxOutline => '\u{f03a9}',
        Nerd::NumericTwoCircle => '\u{f0ca2}',
        Nerd::NumericTwoCircleOutline => '\u{f0ca3}',
        Nerd::NumericZeroBox => '\u{f03a1}',
        Nerd::NumericZeroBoxMultiple => '\u{f0f0e}',
        Nerd::NumericZeroBoxMultipleOutline => '\u{f03a2}',
        Nerd::NumericZeroBoxOutline => '\u{f03a3}',
        Nerd::Nut => '\u{f06f8}',
        Nerd::Nutrition => '\u{f03c2}',
        Nerd::Nuxt => '\u{f1106}',
        Nerd::Oar => '\u{f067c}',
        Nerd::Ocarina => '\u{f0de0}',
        Nerd::Oci => '\u{f12e9}',
        Nerd::Ocr => '\u{f113a}',
        Nerd::Octagon => '\u{f03c3}',
        Nerd::OctagonOutline => '\u{f03c4}',
        Nerd::Octagram => '\u{f06f9}',
        Nerd::OctagramOutline => '\u{f0775}',
        Nerd::Octahedron => '\u{f1950}',
        Nerd::OctahedronOff => '\u{f1951}',
        Nerd::Octoface => '\u{eb27}',
        Nerd::Octoprint => '\u{f34d}',
        Nerd::Odnoklassniki => '\u{f03c5}',
        Nerd::Off => '\u{f011}',
        Nerd::Offer => '\u{f121b}',
        Nerd::OfficeBuilding => '\u{f0991}',
        Nerd::OfficeBuildingCog => '\u{f1949}',
        Nerd::OfficeBuildingCogOutline => '\u{f194a}',
        Nerd::OfficeBuildingMarker => '\u{f1520}',
        Nerd::OfficeBuildingMarkerOutline => '\u{f1521}',
        Nerd::OfficeBuildingOutline => '\u{f151f}',
        Nerd::Oil => '\u{f03c7}',
        Nerd::OilLamp => '\u{f0f19}',
        Nerd::OilLevel => '\u{f1053}',
        Nerd::OilTemperature => '\u{f0ff8}',
        Nerd::Ok => '\u{f00c}',
        Nerd::OkCircle => '\u{f05d}',
        Nerd::OkSign => '\u{f058}',
        Nerd::Ol => '\u{f0cb}',
        Nerd::Om => '\u{f0973}',
        Nerd::Omega => '\u{f03c9}',
        Nerd::OneUp => '\u{f0bad}',
        Nerd::Onepassword => '\u{f0881}',
        Nerd::Opacity => '\u{f05cc}',
        Nerd::OpenInApp => '\u{f03cb}',
        Nerd::OpenInNew => '\u{f03cc}',
        Nerd::OpenPreview => '\u{eb28}',
        Nerd::OpenSourceInitiative => '\u{f0bae}',
        Nerd::Openbsd => '\u{f328}',
        Nerd::Openid => '\u{f03cd}',
        Nerd::Openscad => '\u{f34e}',
        Nerd::Opensuse => '\u{f314}',
        Nerd::Opera => '\u{f03ce}',
        Nerd::Orange => '\u{e2a7}',
        Nerd::Orbit => '\u{f0018}',
        Nerd::OrbitVariant => '\u{f15db}',
        Nerd::OrderAlphabeticalAscending => '\u{f020d}',
        Nerd::OrderAlphabeticalDescending => '\u{f0d07}',
        Nerd::OrderBoolAscending => '\u{f02be}',
        Nerd::OrderBoolAscendingVariant => '\u{f098f}',
        Nerd::OrderBoolDescending => '\u{f1384}',
        Nerd::OrderBoolDescendingVariant => '\u{f0990}',
        Nerd::OrderNumericAscending => '\u{f0545}',
        Nerd::OrderNumericDescending => '\u{f0546}',
        Nerd::Organization => '\u{ea7e}',
        Nerd::OrganizationOne => '\u{f42b}',
        Nerd::Origin => '\u{f0b43}',
        Nerd::Ornament => '\u{f03cf}',
        Nerd::OrnamentVariant => '\u{f03d0}',
        Nerd::Osh => '\u{f34f}',
        Nerd::Oshwa => '\u{f350}',
        Nerd::Osi => '\u{f36c}',
        Nerd::OutdoorLamp => '\u{f1054}',
        Nerd::Output => '\u{eb9d}',
        Nerd::Overscan => '\u{f1005}',
        Nerd::Owl => '\u{f03d2}',
        Nerd::PacMan => '\u{f0baf}',
        Nerd::Package => '\u{eb29}',
        Nerd::PackageDependencies => '\u{f4f8}',
        Nerd::PackageDependents => '\u{f4f9}',
        Nerd::PackageDown => '\u{f03d4}',
        Nerd::PackageOne => '\u{f487}',
        Nerd::PackageTwo => '\u{f03d3}',
        Nerd::PackageUp => '\u{f03d5}',
        Nerd::PackageVariant => '\u{f03d6}',
        Nerd::PackageVariantClosed => '\u{f03d7}',
        Nerd::PackageVariantClosedMinus => '\u{f19d4}',
        Nerd::PackageVariantClosedPlus => '\u{f19d5}',
        Nerd::PackageVariantClosedRemove => '\u{f19d6}',
        Nerd::PackageVariantMinus => '\u{f19d7}',
        Nerd::PackageVariantPlus => '\u{f19d8}',
        Nerd::PackageVariantRemove => '\u{f19d9}',
        Nerd::PageFirst => '\u{f0600}',
        Nerd::PageLast => '\u{f0601}',
        Nerd::PageLayoutBody => '\u{f06fa}',
        Nerd::PageLayoutFooter => '\u{f06fb}',
        Nerd::PageLayoutHeader => '\u{f06fc}',
        Nerd::PageLayoutHeaderFooter => '\u{f0f7f}',
        Nerd::PageLayoutSidebarLeft => '\u{f06fd}',
        Nerd::PageLayoutSidebarRight => '\u{f06fe}',
        Nerd::PageNext => '\u{f0bb0}',
        Nerd::PageNextOutline => '\u{f0bb1}',
        Nerd::PagePrevious => '\u{f0bb2}',
        Nerd::PagePreviousOutline => '\u{f0bb3}',
        Nerd::Pail => '\u{f1417}',
        Nerd::PailMinus => '\u{f1437}',
        Nerd::PailMinusOutline => '\u{f143c}',
        Nerd::PailOff => '\u{f1439}',
        Nerd::PailOffOutline => '\u{f143e}',
        Nerd::PailOutline => '\u{f143a}',
        Nerd::PailPlus => '\u{f1436}',
        Nerd::PailPlusOutline => '\u{f143b}',
        Nerd::PailRemove => '\u{f1438}',
        Nerd::PailRemoveOutline => '\u{f143d}',
        Nerd::Paintbrush => '\u{f48f}',
        Nerd::Paintcan => '\u{eb2a}',
        Nerd::PairProgramming => '\u{e008}',
        Nerd::Palette => '\u{f03d8}',
        Nerd::PaletteAdvanced => '\u{f03d9}',
        Nerd::PaletteColor => '\u{e22b}',
        Nerd::PaletteOutline => '\u{f0e0c}',
        Nerd::PaletteSwatch => '\u{f08b5}',
        Nerd::PaletteSwatchOutline => '\u{f135c}',
        Nerd::PaletteSwatchVariant => '\u{f195a}',
        Nerd::PalmTree => '\u{f1055}',
        Nerd::Pan => '\u{f0bb4}',
        Nerd::PanBottomLeft => '\u{f0bb5}',
        Nerd::PanBottomRight => '\u{f0bb6}',
        Nerd::PanDown => '\u{f0bb7}',
        Nerd::PanHorizontal => '\u{f0bb8}',
        Nerd::PanLeft => '\u{f0bb9}',
        Nerd::PanRight => '\u{f0bba}',
        Nerd::PanTopLeft => '\u{f0bbb}',
        Nerd::PanTopRight => '\u{f0bbc}',
        Nerd::PanUp => '\u{f0bbd}',
        Nerd::PanVertical => '\u{f0bbe}',
        Nerd::Panda => '\u{f03da}',
        Nerd::Pandora => '\u{f03db}',
        Nerd::Panorama => '\u{f03dc}',
        Nerd::PanoramaFisheye => '\u{f03dd}',
        Nerd::PanoramaHorizontal => '\u{f1928}',
        Nerd::PanoramaHorizontalOutline => '\u{f03de}',
        Nerd::PanoramaOutline => '\u{f198c}',
        Nerd::PanoramaSphere => '\u{f198d}',
        Nerd::PanoramaSphereOutline => '\u{f198e}',
        Nerd::PanoramaVariant => '\u{f198f}',
        Nerd::PanoramaVariantOutline => '\u{f1990}',
        Nerd::PanoramaVertical => '\u{f1929}',
        Nerd::PanoramaVerticalOutline => '\u{f03df}',
        Nerd::PanoramaWideAngle => '\u{f195f}',
        Nerd::PanoramaWideAngleOutline => '\u{f03e0}',
        Nerd::PaperAirplane => '\u{f4fa}',
        Nerd::PaperClip => '\u{f0c6}',
        Nerd::PaperCutVertical => '\u{f03e1}',
        Nerd::PaperRoll => '\u{f1157}',
        Nerd::PaperRollOutline => '\u{f1158}',
        Nerd::Paperclip => '\u{f4fb}',
        Nerd::PaperclipCheck => '\u{f1ac6}',
        Nerd::PaperclipLock => '\u{f19da}',
        Nerd::PaperclipMinus => '\u{f1ac7}',
        Nerd::PaperclipOff => '\u{f1ac8}',
        Nerd::PaperclipOne => '\u{f03e2}',
        Nerd::PaperclipPlus => '\u{f1ac9}',
        Nerd::PaperclipRemove => '\u{f1aca}',
        Nerd::ParabolaGnuLinuxLibre => '\u{f340}',
        Nerd::Parachute => '\u{f0cb4}',
        Nerd::ParachuteOutline => '\u{f0cb5}',
        Nerd::Paragliding => '\u{f1745}',
        Nerd::Parking => '\u{f03e3}',
        Nerd::ParrotOs => '\u{f329}',
        Nerd::PartyPopper => '\u{f1056}',
        Nerd::Pass => '\u{eba4}',
        Nerd::PassFilled => '\u{ebb3}',
        Nerd::PasskeyFill => '\u{f4fc}',
        Nerd::Passport => '\u{f07e3}',
        Nerd::PassportBiometric => '\u{f0de1}',
        Nerd::Pasta => '\u{f1160}',
        Nerd::Paste => '\u{f0ea}',
        Nerd::PasteOne => '\u{f429}',
        Nerd::PatioHeater => '\u{f0f80}',
        Nerd::Patreon => '\u{f0882}',
        Nerd::Pause => '\u{f04c}',
        Nerd::PauseCircle => '\u{f03e5}',
        Nerd::PauseCircleOutline => '\u{f03e6}',
        Nerd::PauseOctagon => '\u{f03e7}',
        Nerd::PauseOctagonOutline => '\u{f03e8}',
        Nerd::PauseOne => '\u{f03e4}',
        Nerd::Paw => '\u{f03e9}',
        Nerd::PawOff => '\u{f0657}',
        Nerd::PawOffOutline => '\u{f1676}',
        Nerd::PawOutline => '\u{f1675}',
        Nerd::Peace => '\u{f0884}',
        Nerd::Peach => '\u{e2a8}',
        Nerd::Peanut => '\u{f0ffc}',
        Nerd::PeanutOff => '\u{f0ffd}',
        Nerd::PeanutOffOutline => '\u{f0fff}',
        Nerd::PeanutOutline => '\u{f0ffe}',
        Nerd::Pear => '\u{e2a9}',
        Nerd::Pen => '\u{f03ea}',
        Nerd::PenLock => '\u{f0de2}',
        Nerd::PenMinus => '\u{f0de3}',
        Nerd::PenOff => '\u{f0de4}',
        Nerd::PenPlus => '\u{f0de5}',
        Nerd::PenRemove => '\u{f0de6}',
        Nerd::Pencil => '\u{f040}',
        Nerd::PencilBox => '\u{f03ec}',
        Nerd::PencilBoxMultiple => '\u{f1144}',
        Nerd::PencilBoxMultipleOutline => '\u{f1145}',
        Nerd::PencilBoxOutline => '\u{f03ed}',
        Nerd::PencilCircle => '\u{f06ff}',
        Nerd::PencilCircleOutline => '\u{f0776}',
        Nerd::PencilLock => '\u{f03ee}',
        Nerd::PencilLockOutline => '\u{f0de7}',
        Nerd::PencilMinus => '\u{f0de8}',
        Nerd::PencilMinusOutline => '\u{f0de9}',
        Nerd::PencilOff => '\u{f03ef}',
        Nerd::PencilOffOutline => '\u{f0dea}',
        Nerd::PencilOne => '\u{f448}',
        Nerd::PencilOutline => '\u{f0cb6}',
        Nerd::PencilPlus => '\u{f0deb}',
        Nerd::PencilPlusOutline => '\u{f0dec}',
        Nerd::PencilRemove => '\u{f0ded}',
        Nerd::PencilRemoveOutline => '\u{f0dee}',
        Nerd::PencilRuler => '\u{f1353}',
        Nerd::PencilTwo => '\u{f03eb}',
        Nerd::Penguin => '\u{f0ec0}',
        Nerd::Pentagon => '\u{f0701}',
        Nerd::PentagonOutline => '\u{f0700}',
        Nerd::Pentagram => '\u{f1667}',
        Nerd::People => '\u{f4fd}',
        Nerd::Percent => '\u{f03f0}',
        Nerd::PercentBox => '\u{f1a02}',
        Nerd::PercentBoxOutline => '\u{f1a03}',
        Nerd::PercentCircle => '\u{f1a04}',
        Nerd::PercentCircleOutline => '\u{f1a05}',
        Nerd::PercentOutline => '\u{f1278}',
        Nerd::PeriodicTable => '\u{f08b6}',
        Nerd::Person => '\u{ea67}',
        Nerd::PersonAdd => '\u{ebcd}',
        Nerd::PersonAddOne => '\u{f4fe}',
        Nerd::PersonFill => '\u{f4ff}',
        Nerd::PersonOne => '\u{f415}',
        Nerd::PerspectiveLess => '\u{f0d23}',
        Nerd::PerspectiveMore => '\u{f0d24}',
        Nerd::Ph => '\u{f17c5}',
        Nerd::Phone => '\u{f095}',
        Nerd::PhoneAlert => '\u{f0f1a}',
        Nerd::PhoneAlertOutline => '\u{f118e}',
        Nerd::PhoneBluetooth => '\u{f03f3}',
        Nerd::PhoneBluetoothOutline => '\u{f118f}',
        Nerd::PhoneCancel => '\u{f10bc}',
        Nerd::PhoneCancelOutline => '\u{f1190}',
        Nerd::PhoneCheck => '\u{f11a9}',
        Nerd::PhoneCheckOutline => '\u{f11aa}',
        Nerd::PhoneClassic => '\u{f0602}',
        Nerd::PhoneClassicOff => '\u{f1279}',
        Nerd::PhoneClock => '\u{f19db}',
        Nerd::PhoneDial => '\u{f1559}',
        Nerd::PhoneDialOutline => '\u{f155a}',
        Nerd::PhoneForward => '\u{f03f4}',
        Nerd::PhoneForwardOutline => '\u{f1191}',
        Nerd::PhoneHangup => '\u{f03f5}',
        Nerd::PhoneHangupOutline => '\u{f1192}',
        Nerd::PhoneInTalk => '\u{f03f6}',
        Nerd::PhoneInTalkOutline => '\u{f1182}',
        Nerd::PhoneIncoming => '\u{f03f7}',
        Nerd::PhoneIncomingOutline => '\u{f1193}',
        Nerd::PhoneLock => '\u{f03f8}',
        Nerd::PhoneLockOutline => '\u{f1194}',
        Nerd::PhoneLog => '\u{f03f9}',
        Nerd::PhoneLogOutline => '\u{f1195}',
        Nerd::PhoneMessage => '\u{f1196}',
        Nerd::PhoneMessageOutline => '\u{f1197}',
        Nerd::PhoneMinus => '\u{f0658}',
        Nerd::PhoneMinusOutline => '\u{f1198}',
        Nerd::PhoneMissed => '\u{f03fa}',
        Nerd::PhoneMissedOutline => '\u{f11a5}',
        Nerd::PhoneOff => '\u{f0def}',
        Nerd::PhoneOffOutline => '\u{f11a6}',
        Nerd::PhoneOne => '\u{f03f2}',
        Nerd::PhoneOutgoing => '\u{f03fb}',
        Nerd::PhoneOutgoingOutline => '\u{f1199}',
        Nerd::PhoneOutline => '\u{f0df0}',
        Nerd::PhonePaused => '\u{f03fc}',
        Nerd::PhonePausedOutline => '\u{f119a}',
        Nerd::PhonePlus => '\u{f0659}',
        Nerd::PhonePlusOutline => '\u{f119b}',
        Nerd::PhoneRefresh => '\u{f1993}',
        Nerd::PhoneRefreshOutline => '\u{f1994}',
        Nerd::PhoneRemove => '\u{f152f}',
        Nerd::PhoneRemoveOutline => '\u{f1530}',
        Nerd::PhoneReturn => '\u{f082f}',
        Nerd::PhoneReturnOutline => '\u{f119c}',
        Nerd::PhoneRing => '\u{f11ab}',
        Nerd::PhoneRingOutline => '\u{f11ac}',
        Nerd::PhoneRotateLandscape => '\u{f0885}',
        Nerd::PhoneRotatePortrait => '\u{f0886}',
        Nerd::PhoneSettings => '\u{f03fd}',
        Nerd::PhoneSettingsOutline => '\u{f119d}',
        Nerd::PhoneSign => '\u{f098}',
        Nerd::PhoneSync => '\u{f1995}',
        Nerd::PhoneSyncOutline => '\u{f1996}',
        Nerd::PhoneVoip => '\u{f03fe}',
        Nerd::Pi => '\u{e22c}',
        Nerd::PiBox => '\u{f0400}',
        Nerd::PiHole => '\u{f0df1}',
        Nerd::PiOne => '\u{f03ff}',
        Nerd::Piano => '\u{f067d}',
        Nerd::PianoOff => '\u{f0698}',
        Nerd::Pickaxe => '\u{f08b7}',
        Nerd::Picture => '\u{f03e}',
        Nerd::PictureInPictureBottomRight => '\u{f0e57}',
        Nerd::PictureInPictureBottomRightOutline => '\u{f0e58}',
        Nerd::PictureInPictureTopRight => '\u{f0e59}',
        Nerd::PictureInPictureTopRightOutline => '\u{f0e5a}',
        Nerd::PieChart => '\u{ebe4}',
        Nerd::Pier => '\u{f0887}',
        Nerd::PierCrane => '\u{f0888}',
        Nerd::Pig => '\u{f0401}',
        Nerd::PigVariant => '\u{f1006}',
        Nerd::PigVariantOutline => '\u{f1678}',
        Nerd::PiggyBank => '\u{f1007}',
        Nerd::PiggyBankOutline => '\u{f1679}',
        Nerd::Pill => '\u{f0402}',
        Nerd::PillOff => '\u{f1a5c}',
        Nerd::Pillar => '\u{f0702}',
        Nerd::Pin => '\u{eb2b}',
        Nerd::PinOff => '\u{f0404}',
        Nerd::PinOffOutline => '\u{f0930}',
        Nerd::PinOne => '\u{f435}',
        Nerd::PinOutline => '\u{f0931}',
        Nerd::PinTwo => '\u{f0403}',
        Nerd::PineTree => '\u{f0405}',
        Nerd::PineTreeBox => '\u{f0406}',
        Nerd::PineTreeFire => '\u{f141a}',
        Nerd::Pinned => '\u{eba0}',
        Nerd::PinnedDirty => '\u{ebb2}',
        Nerd::Pinterest => '\u{f0d2}',
        Nerd::PinterestOne => '\u{f0407}',
        Nerd::PinterestSign => '\u{f0d3}',
        Nerd::Pinwheel => '\u{f0ad5}',
        Nerd::PinwheelOutline => '\u{f0ad6}',
        Nerd::Pipe => '\u{f07e5}',
        Nerd::PipeDisconnected => '\u{f07e6}',
        Nerd::PipeLeak => '\u{f0889}',
        Nerd::PipeValve => '\u{f184d}',
        Nerd::PipeWrench => '\u{f1354}',
        Nerd::Pirate => '\u{f0a08}',
        Nerd::Pistol => '\u{f0703}',
        Nerd::Piston => '\u{f088a}',
        Nerd::Pitchfork => '\u{f1553}',
        Nerd::Pizza => '\u{e22d}',
        Nerd::PizzaOne => '\u{f0409}',
        Nerd::Plane => '\u{f072}',
        Nerd::Planet => '\u{e22e}',
        Nerd::Plant => '\u{e22f}',
        Nerd::Play => '\u{eb2c}',
        Nerd::PlayBox => '\u{f127a}',
        Nerd::PlayBoxLock => '\u{f1a16}',
        Nerd::PlayBoxLockOpen => '\u{f1a17}',
        Nerd::PlayBoxLockOpenOutline => '\u{f1a18}',
        Nerd::PlayBoxLockOutline => '\u{f1a19}',
        Nerd::PlayBoxMultiple => '\u{f0d19}',
        Nerd::PlayBoxMultipleOutline => '\u{f13e6}',
        Nerd::PlayBoxOutline => '\u{f040b}',
        Nerd::PlayCircle => '\u{f01d}',
        Nerd::PlayCircleOne => '\u{f040c}',
        Nerd::PlayCircleOutline => '\u{f040d}',
        Nerd::PlayNetwork => '\u{f088b}',
        Nerd::PlayNetworkOutline => '\u{f0cb7}',
        Nerd::PlayOne => '\u{f04b}',
        Nerd::PlayOutline => '\u{f0f1b}',
        Nerd::PlayPause => '\u{f040e}',
        Nerd::PlayProtectedContent => '\u{f040f}',
        Nerd::PlaySign => '\u{f144}',
        Nerd::PlaySpeed => '\u{f08ff}',
        Nerd::PlayThree => '\u{f040a}',
        Nerd::PlayTwo => '\u{f500}',
        Nerd::PlaylistCheck => '\u{f05c7}',
        Nerd::PlaylistEdit => '\u{f0900}',
        Nerd::PlaylistMinus => '\u{f0410}',
        Nerd::PlaylistMusic => '\u{f0cb8}',
        Nerd::PlaylistMusicOutline => '\u{f0cb9}',
        Nerd::PlaylistPlay => '\u{f0411}',
        Nerd::PlaylistPlus => '\u{f0412}',
        Nerd::PlaylistRemove => '\u{f0413}',
        Nerd::PlaylistStar => '\u{f0df2}',
        Nerd::Playstation => '\u{e230}',
        Nerd::Plex => '\u{f06ba}',
        Nerd::Pliers => '\u{f19a4}',
        Nerd::Plug => '\u{eb2d}',
        Nerd::PlugOne => '\u{f492}',
        Nerd::Plus => '\u{f067}',
        Nerd::PlusBox => '\u{f0416}',
        Nerd::PlusBoxMultiple => '\u{f0334}',
        Nerd::PlusBoxMultipleOutline => '\u{f1143}',
        Nerd::PlusBoxOutline => '\u{f0704}',
        Nerd::PlusCircle => '\u{f501}',
        Nerd::PlusCircleMultiple => '\u{f034c}',
        Nerd::PlusCircleMultipleOutline => '\u{f0418}',
        Nerd::PlusCircleOne => '\u{f0417}',
        Nerd::PlusCircleOutline => '\u{f0419}',
        Nerd::PlusLock => '\u{f1a5d}',
        Nerd::PlusLockOpen => '\u{f1a5e}',
        Nerd::PlusMinus => '\u{f0992}',
        Nerd::PlusMinusBox => '\u{f0993}',
        Nerd::PlusMinusVariant => '\u{f14c9}',
        Nerd::PlusNetwork => '\u{f041a}',
        Nerd::PlusNetworkOutline => '\u{f0cba}',
        Nerd::PlusOne => '\u{f44d}',
        Nerd::PlusOutline => '\u{f0705}',
        Nerd::PlusSign => '\u{f055}',
        Nerd::PlusSquareO => '\u{f196}',
        Nerd::PlusThick => '\u{f11ec}',
        Nerd::PlusTwo => '\u{f0415}',
        Nerd::Podcast => '\u{f0994}',
        Nerd::Podium => '\u{f0d25}',
        Nerd::PodiumBronze => '\u{f0d26}',
        Nerd::PodiumGold => '\u{f0d27}',
        Nerd::PodiumSilver => '\u{f0d28}',
        Nerd::PointOfSale => '\u{f0d92}',
        Nerd::Poison => '\u{e231}',
        Nerd::Pokeball => '\u{f041d}',
        Nerd::PokemonGo => '\u{f0a09}',
        Nerd::PokerChip => '\u{f0830}',
        Nerd::Polaroid => '\u{f041e}',
        Nerd::PoliceBadge => '\u{f1167}',
        Nerd::PoliceBadgeOutline => '\u{f1168}',
        Nerd::PoliceStation => '\u{f1839}',
        Nerd::Poll => '\u{f041f}',
        Nerd::Polo => '\u{f14c3}',
        Nerd::Polymer => '\u{f0421}',
        Nerd::PomodoroDone => '\u{e001}',
        Nerd::PomodoroEstimated => '\u{e002}',
        Nerd::PomodoroSquashed => '\u{e004}',
        Nerd::PomodoroTicking => '\u{e003}',
        Nerd::Pool => '\u{f0606}',
        Nerd::PoolThermometer => '\u{f1a5f}',
        Nerd::PopOs => '\u{f32a}',
        Nerd::Popcorn => '\u{e232}',
        Nerd::PopcornOne => '\u{f0422}',
        Nerd::Popsicle => '\u{e233}',
        Nerd::Post => '\u{f1008}',
        Nerd::PostLamp => '\u{f1a60}',
        Nerd::PostOutline => '\u{f1009}',
        Nerd::PostageStamp => '\u{f0cbb}',
        Nerd::Pot => '\u{f02e5}',
        Nerd::PotMix => '\u{f065b}',
        Nerd::PotMixOutline => '\u{f0677}',
        Nerd::PotOutline => '\u{f02ff}',
        Nerd::PotSteam => '\u{f065a}',
        Nerd::PotSteamOutline => '\u{f0326}',
        Nerd::Pound => '\u{f0423}',
        Nerd::PoundBox => '\u{f0424}',
        Nerd::PoundBoxOutline => '\u{f117f}',
        Nerd::Power => '\u{f0425}',
        Nerd::PowerCycle => '\u{f0901}',
        Nerd::PowerOff => '\u{f0902}',
        Nerd::PowerOn => '\u{f0903}',
        Nerd::PowerOnOffSymbol => '\u{23fc}',
        Nerd::PowerOnSymbol => '\u{23fd}',
        Nerd::PowerPlug => '\u{f06a5}',
        Nerd::PowerPlugOff => '\u{f06a6}',
        Nerd::PowerPlugOffOutline => '\u{f1424}',
        Nerd::PowerPlugOutline => '\u{f1425}',
        Nerd::PowerSettings => '\u{f0426}',
        Nerd::PowerSleep => '\u{f0904}',
        Nerd::PowerSleepSymbol => '\u{23fe}',
        Nerd::PowerSocket => '\u{f0427}',
        Nerd::PowerSocketAu => '\u{f0905}',
        Nerd::PowerSocketCh => '\u{f0fb3}',
        Nerd::PowerSocketDe => '\u{f1107}',
        Nerd::PowerSocketEu => '\u{f07e7}',
        Nerd::PowerSocketFr => '\u{f1108}',
        Nerd::PowerSocketIt => '\u{f14ff}',
        Nerd::PowerSocketJp => '\u{f1109}',
        Nerd::PowerSocketUk => '\u{f07e8}',
        Nerd::PowerSocketUs => '\u{f07e9}',
        Nerd::PowerStandby => '\u{f0906}',
        Nerd::PowerSymbol => '\u{23fb}',
        Nerd::Powershell => '\u{f0a0a}',
        Nerd::Prescription => '\u{f0706}',
        Nerd::Presentation => '\u{f0428}',
        Nerd::PresentationPlay => '\u{f0429}',
        Nerd::PreserveCase => '\u{eb2e}',
        Nerd::Pretzel => '\u{f1562}',
        Nerd::Preview => '\u{eb2f}',
        Nerd::PrimitiveSquare => '\u{ea72}',
        Nerd::Print => '\u{f02f}',
        Nerd::Printer => '\u{f042a}',
        Nerd::PrinterAlert => '\u{f042c}',
        Nerd::PrinterCheck => '\u{f1146}',
        Nerd::PrinterEye => '\u{f1458}',
        Nerd::PrinterOff => '\u{f0e5d}',
        Nerd::PrinterOffOutline => '\u{f1785}',
        Nerd::PrinterOutline => '\u{f1786}',
        Nerd::PrinterPos => '\u{f1057}',
        Nerd::PrinterSearch => '\u{f1457}',
        Nerd::PrinterSettings => '\u{f0707}',
        Nerd::PrinterThreed => '\u{f042b}',
        Nerd::PrinterThreedNozzle => '\u{f0e5b}',
        Nerd::PrinterThreedNozzleAlert => '\u{f11c0}',
        Nerd::PrinterThreedNozzleAlertOutline => '\u{f11c1}',
        Nerd::PrinterThreedNozzleHeat => '\u{f18b8}',
        Nerd::PrinterThreedNozzleHeatOutline => '\u{f18b9}',
        Nerd::PrinterThreedNozzleOutline => '\u{f0e5c}',
        Nerd::PrinterWireless => '\u{f0a0b}',
        Nerd::PriorityHigh => '\u{f0603}',
        Nerd::PriorityLow => '\u{f0604}',
        Nerd::ProfessionalHexagon => '\u{f042d}',
        Nerd::ProgressAlert => '\u{f0cbc}',
        Nerd::ProgressCheck => '\u{f0995}',
        Nerd::ProgressClock => '\u{f0996}',
        Nerd::ProgressClose => '\u{f110a}',
        Nerd::ProgressDownload => '\u{f0997}',
        Nerd::ProgressPencil => '\u{f1787}',
        Nerd::ProgressQuestion => '\u{f1522}',
        Nerd::ProgressStar => '\u{f1788}',
        Nerd::ProgressUpload => '\u{f0998}',
        Nerd::ProgressWrench => '\u{f0cbd}',
        Nerd::Project => '\u{eb30}',
        Nerd::ProjectOne => '\u{f502}',
        Nerd::ProjectRoadmap => '\u{f503}',
        Nerd::ProjectSymlink => '\u{f504}',
        Nerd::ProjectTemplate => '\u{f505}',
        Nerd::Projector => '\u{f042e}',
        Nerd::ProjectorOff => '\u{f1a23}',
        Nerd::ProjectorScreen => '\u{f042f}',
        Nerd::ProjectorScreenOff => '\u{f180d}',
        Nerd::ProjectorScreenOffOutline => '\u{f180e}',
        Nerd::ProjectorScreenOutline => '\u{f1724}',
        Nerd::ProjectorScreenVariant => '\u{f180f}',
        Nerd::ProjectorScreenVariantOff => '\u{f1810}',
        Nerd::ProjectorScreenVariantOffOutline => '\u{f1811}',
        Nerd::ProjectorScreenVariantOutline => '\u{f1812}',
        Nerd::PropaneTank => '\u{f1357}',
        Nerd::PropaneTankOutline => '\u{f1358}',
        Nerd::Protocol => '\u{f0fd8}',
        Nerd::PrusaSlicer => '\u{f351}',
        Nerd::Publish => '\u{f06a7}',
        Nerd::PublishOff => '\u{f1945}',
        Nerd::Pulse => '\u{e234}',
        Nerd::PulseOne => '\u{eb31}',
        Nerd::PulseThree => '\u{f0430}',
        Nerd::PulseTwo => '\u{f469}',
        Nerd::Pump => '\u{f1402}',
        Nerd::Pumpkin => '\u{f0bbf}',
        Nerd::PuppyLinux => '\u{f341}',
        Nerd::Purse => '\u{f0f1c}',
        Nerd::PurseOutline => '\u{f0f1d}',
        Nerd::Pushpin => '\u{f08d}',
        Nerd::Puzzle => '\u{f0431}',
        Nerd::PuzzleCheck => '\u{f1426}',
        Nerd::PuzzleCheckOutline => '\u{f1427}',
        Nerd::PuzzleEdit => '\u{f14d3}',
        Nerd::PuzzleEditOutline => '\u{f14d9}',
        Nerd::PuzzleHeart => '\u{f14d4}',
        Nerd::PuzzleHeartOutline => '\u{f14da}',
        Nerd::PuzzleMinus => '\u{f14d1}',
        Nerd::PuzzleMinusOutline => '\u{f14d7}',
        Nerd::PuzzleOutline => '\u{f0a66}',
        Nerd::PuzzlePiece => '\u{f12e}',
        Nerd::PuzzlePlus => '\u{f14d0}',
        Nerd::PuzzlePlusOutline => '\u{f14d6}',
        Nerd::PuzzleRemove => '\u{f14d2}',
        Nerd::PuzzleRemoveOutline => '\u{f14d8}',
        Nerd::PuzzleStar => '\u{f14d5}',
        Nerd::PuzzleStarOutline => '\u{f14db}',
        Nerd::Pyramid => '\u{f1952}',
        Nerd::PyramidOff => '\u{f1953}',
        Nerd::Python => '\u{e235}',
        Nerd::Qi => '\u{f0999}',
        Nerd::Qqchat => '\u{f0605}',
        Nerd::Qrcode => '\u{f029}',
        Nerd::QrcodeEdit => '\u{f08b8}',
        Nerd::QrcodeMinus => '\u{f118c}',
        Nerd::QrcodeOne => '\u{f0432}',
        Nerd::QrcodePlus => '\u{f118b}',
        Nerd::QrcodeRemove => '\u{f118d}',
        Nerd::QrcodeScan => '\u{f0433}',
        Nerd::Qtile => '\u{f35c}',
        Nerd::Quadcopter => '\u{f0434}',
        Nerd::QualityHigh => '\u{f0435}',
        Nerd::QualityLow => '\u{f0a0c}',
        Nerd::QualityMedium => '\u{f0a0d}',
        Nerd::Qubesos => '\u{f342}',
        Nerd::Question => '\u{eb32}',
        Nerd::QuestionOne => '\u{f128}',
        Nerd::QuestionSign => '\u{f059}',
        Nerd::QuestionTwo => '\u{f420}',
        Nerd::Quora => '\u{f0d29}',
        Nerd::QuoraCircle => '\u{e236}',
        Nerd::QuoraSquare => '\u{e237}',
        Nerd::Quote => '\u{eb33}',
        Nerd::QuoteLeft => '\u{f10d}',
        Nerd::QuoteOne => '\u{f453}',
        Nerd::QuoteRight => '\u{f10e}',
        Nerd::Rabbit => '\u{f0907}',
        Nerd::RabbitVariant => '\u{f1a61}',
        Nerd::RabbitVariantOutline => '\u{f1a62}',
        Nerd::RacingHelmet => '\u{f0d93}',
        Nerd::Racquetball => '\u{f0d94}',
        Nerd::Radar => '\u{f0437}',
        Nerd::Radiator => '\u{f0438}',
        Nerd::RadiatorDisabled => '\u{f0ad7}',
        Nerd::RadiatorOff => '\u{f0ad8}',
        Nerd::Radio => '\u{f0439}',
        Nerd::RadioAm => '\u{f0cbe}',
        Nerd::RadioFm => '\u{f0cbf}',
        Nerd::RadioHandheld => '\u{f043a}',
        Nerd::RadioOff => '\u{f121c}',
        Nerd::RadioTower => '\u{eb34}',
        Nerd::RadioTowerOne => '\u{f043b}',
        Nerd::Radioactive => '\u{e238}',
        Nerd::RadioactiveCircle => '\u{f185d}',
        Nerd::RadioactiveCircleOutline => '\u{f185e}',
        Nerd::RadioactiveOff => '\u{f0ec1}',
        Nerd::RadioactiveOne => '\u{f043c}',
        Nerd::RadioboxMarked => '\u{f043e}',
        Nerd::RadiologyBox => '\u{f14c5}',
        Nerd::RadiologyBoxOutline => '\u{f14c6}',
        Nerd::Radius => '\u{f0cc0}',
        Nerd::RadiusOutline => '\u{f0cc1}',
        Nerd::RailroadLight => '\u{f0f1e}',
        Nerd::Raining => '\u{e239}',
        Nerd::Rake => '\u{f1544}',
        Nerd::Random => '\u{f074}',
        Nerd::RaspberryPi => '\u{f043f}',
        Nerd::Raw => '\u{f1a0f}',
        Nerd::RawOff => '\u{f1a10}',
        Nerd::RayEnd => '\u{f0440}',
        Nerd::RayEndArrow => '\u{f0441}',
        Nerd::RayStart => '\u{f0442}',
        Nerd::RayStartArrow => '\u{f0443}',
        Nerd::RayStartEnd => '\u{f0444}',
        Nerd::RayStartVertexEnd => '\u{f15d8}',
        Nerd::RayVertex => '\u{f0445}',
        Nerd::RazorDoubleEdge => '\u{f1997}',
        Nerd::RazorSingleEdge => '\u{f1998}',
        Nerd::React => '\u{f0708}',
        Nerd::Reactions => '\u{eb35}',
        Nerd::Read => '\u{f430}',
        Nerd::ReadOne => '\u{f0447}',
        Nerd::RealHeart => '\u{e23a}',
        Nerd::Receipt => '\u{f0449}',
        Nerd::ReceiptOutline => '\u{f19dc}',
        Nerd::ReceiptTextCheck => '\u{f1a63}',
        Nerd::ReceiptTextCheckOutline => '\u{f1a64}',
        Nerd::ReceiptTextMinus => '\u{f1a65}',
        Nerd::ReceiptTextMinusOutline => '\u{f1a66}',
        Nerd::ReceiptTextPlus => '\u{f1a67}',
        Nerd::ReceiptTextPlusOutline => '\u{f1a68}',
        Nerd::ReceiptTextRemove => '\u{f1a69}',
        Nerd::ReceiptTextRemoveOutline => '\u{f1a6a}',
        Nerd::Record => '\u{eba7}',
        Nerd::RecordCircle => '\u{f0ec2}',
        Nerd::RecordCircleOutline => '\u{f0ec3}',
        Nerd::RecordKeys => '\u{ea65}',
        Nerd::RecordOne => '\u{f044a}',
        Nerd::RecordPlayer => '\u{f099a}',
        Nerd::RecordRec => '\u{f044b}',
        Nerd::Rectangle => '\u{f0e5e}',
        Nerd::RectangleOutline => '\u{f0e5f}',
        Nerd::Recycle => '\u{f044c}',
        Nerd::RecycleVariant => '\u{f139d}',
        Nerd::RedHat => '\u{f316}',
        Nerd::Reddit => '\u{f044d}',
        Nerd::Redhat => '\u{f111b}',
        Nerd::Redo => '\u{ebb0}',
        Nerd::RedoOne => '\u{f044e}',
        Nerd::RedoVariant => '\u{f044f}',
        Nerd::References => '\u{eb36}',
        Nerd::ReflectHorizontal => '\u{f0a0e}',
        Nerd::ReflectVertical => '\u{f0a0f}',
        Nerd::Refresh => '\u{eb37}',
        Nerd::RefreshAuto => '\u{f18f2}',
        Nerd::RefreshCircle => '\u{f1377}',
        Nerd::RefreshOne => '\u{f021}',
        Nerd::RefreshTwo => '\u{f0450}',
        Nerd::Refrigerator => '\u{e23b}',
        Nerd::Regex => '\u{eb38}',
        Nerd::RegexOne => '\u{f0451}',
        Nerd::RegisteredTrademark => '\u{f0a67}',
        Nerd::Reiterate => '\u{f1588}',
        Nerd::RelFilePath => '\u{f506}',
        Nerd::RelationManyToMany => '\u{f1496}',
        Nerd::RelationManyToOne => '\u{f1497}',
        Nerd::RelationManyToOneOrMany => '\u{f1498}',
        Nerd::RelationManyToOnlyOne => '\u{f1499}',
        Nerd::RelationManyToZeroOrMany => '\u{f149a}',
        Nerd::RelationManyToZeroOrOne => '\u{f149b}',
        Nerd::RelationOneOrManyToMany => '\u{f149c}',
        Nerd::RelationOneOrManyToOne => '\u{f149d}',
        Nerd::RelationOneOrManyToOneOrMany => '\u{f149e}',
        Nerd::RelationOneOrManyToOnlyOne => '\u{f149f}',
        Nerd::RelationOneOrManyToZeroOrMany => '\u{f14a0}',
        Nerd::RelationOneOrManyToZeroOrOne => '\u{f14a1}',
        Nerd::RelationOneToMany => '\u{f14a2}',
        Nerd::RelationOneToOne => '\u{f14a3}',
        Nerd::RelationOneToOneOrMany => '\u{f14a4}',
        Nerd::RelationOneToOnlyOne => '\u{f14a5}',
        Nerd::RelationOneToZeroOrMany => '\u{f14a6}',
        Nerd::RelationOneToZeroOrOne => '\u{f14a7}',
        Nerd::RelationOnlyOneToMany => '\u{f14a8}',
        Nerd::RelationOnlyOneToOne => '\u{f14a9}',
        Nerd::RelationOnlyOneToOneOrMany => '\u{f14aa}',
        Nerd::RelationOnlyOneToOnlyOne => '\u{f14ab}',
        Nerd::RelationOnlyOneToZeroOrMany => '\u{f14ac}',
        Nerd::RelationOnlyOneToZeroOrOne => '\u{f14ad}',
        Nerd::RelationZeroOrManyToMany => '\u{f14ae}',
        Nerd::RelationZeroOrManyToOne => '\u{f14af}',
        Nerd::RelationZeroOrManyToOneOrMany => '\u{f14b0}',
        Nerd::RelationZeroOrManyToOnlyOne => '\u{f14b1}',
        Nerd::RelationZeroOrManyToZeroOrMany => '\u{f14b2}',
        Nerd::RelationZeroOrManyToZeroOrOne => '\u{f14b3}',
        Nerd::RelationZeroOrOneToMany => '\u{f14b4}',
        Nerd::RelationZeroOrOneToOne => '\u{f14b5}',
        Nerd::RelationZeroOrOneToOneOrMany => '\u{f14b6}',
        Nerd::RelationZeroOrOneToOnlyOne => '\u{f14b7}',
        Nerd::RelationZeroOrOneToZeroOrMany => '\u{f14b8}',
        Nerd::RelationZeroOrOneToZeroOrOne => '\u{f14b9}',
        Nerd::RelativeScale => '\u{f0452}',
        Nerd::Reload => '\u{f0453}',
        Nerd::ReloadAlert => '\u{f110b}',
        Nerd::Reminder => '\u{f088c}',
        Nerd::Remote => '\u{eb3a}',
        Nerd::RemoteDesktop => '\u{f08b9}',
        Nerd::RemoteExplorer => '\u{eb39}',
        Nerd::RemoteOff => '\u{f0ec4}',
        Nerd::RemoteOne => '\u{f0454}',
        Nerd::RemoteTv => '\u{f0ec5}',
        Nerd::RemoteTvOff => '\u{f0ec6}',
        Nerd::Remove => '\u{eb3b}',
        Nerd::RemoveCircle => '\u{f05c}',
        Nerd::RemoveOne => '\u{f00d}',
        Nerd::RemoveSign => '\u{f057}',
        Nerd::RenameBox => '\u{f0455}',
        Nerd::Renren => '\u{f18b}',
        Nerd::Reorder => '\u{f0c9}',
        Nerd::ReorderHorizontal => '\u{f0688}',
        Nerd::ReorderVertical => '\u{f0689}',
        Nerd::Repeat => '\u{f01e}',
        Nerd::RepeatOff => '\u{f0457}',
        Nerd::RepeatOnce => '\u{f0458}',
        Nerd::RepeatOne => '\u{f0456}',
        Nerd::RepeatVariant => '\u{f0547}',
        Nerd::Replace => '\u{eb3d}',
        Nerd::ReplaceAll => '\u{eb3c}',
        Nerd::Replay => '\u{f0459}',
        Nerd::Reply => '\u{ea7d}',
        Nerd::ReplyAll => '\u{f045b}',
        Nerd::ReplyAllOutline => '\u{f0f1f}',
        Nerd::ReplyCircle => '\u{f11ae}',
        Nerd::ReplyOne => '\u{f112}',
        Nerd::ReplyOutline => '\u{f0f20}',
        Nerd::ReplyThree => '\u{f045a}',
        Nerd::ReplyTwo => '\u{f4a8}',
        Nerd::Repo => '\u{ea62}',
        Nerd::RepoClone => '\u{eb3e}',
        Nerd::RepoCloneOne => '\u{f43f}',
        Nerd::RepoDeleted => '\u{f507}',
        Nerd::RepoForcePush => '\u{eb3f}',
        Nerd::RepoForked => '\u{ea63}',
        Nerd::RepoForkedOne => '\u{f402}',
        Nerd::RepoLocked => '\u{f508}',
        Nerd::RepoOne => '\u{f401}',
        Nerd::RepoPull => '\u{eb40}',
        Nerd::RepoPullOne => '\u{f404}',
        Nerd::RepoPush => '\u{eb41}',
        Nerd::RepoPushOne => '\u{f403}',
        Nerd::RepoTemplate => '\u{f509}',
        Nerd::Report => '\u{eb42}',
        Nerd::ReportOne => '\u{f50a}',
        Nerd::Reprap => '\u{f352}',
        Nerd::Reproduction => '\u{f045c}',
        Nerd::RequestChanges => '\u{eb43}',
        Nerd::Resistor => '\u{f0b44}',
        Nerd::ResistorNodes => '\u{f0b45}',
        Nerd::Resize => '\u{f0a68}',
        Nerd::ResizeBottomRight => '\u{f045d}',
        Nerd::ResizeFull => '\u{f065}',
        Nerd::ResizeHorizontal => '\u{f07e}',
        Nerd::ResizeSmall => '\u{f066}',
        Nerd::ResizeVertical => '\u{f07d}',
        Nerd::Responsive => '\u{f045e}',
        Nerd::Restart => '\u{f0709}',
        Nerd::RestartAlert => '\u{f110c}',
        Nerd::RestartOff => '\u{f0d95}',
        Nerd::Restore => '\u{e23c}',
        Nerd::RestoreAlert => '\u{f110d}',
        Nerd::RestoreOne => '\u{f099b}',
        Nerd::Retweet => '\u{f079}',
        Nerd::Rewind => '\u{f045f}',
        Nerd::RewindFive => '\u{f11f9}',
        Nerd::RewindOnefive => '\u{f1946}',
        Nerd::RewindOnezero => '\u{f0d2a}',
        Nerd::RewindOutline => '\u{f070a}',
        Nerd::RewindSixzero => '\u{f160c}',
        Nerd::RewindThreezero => '\u{f0d96}',
        Nerd::Rhombus => '\u{f070b}',
        Nerd::RhombusMedium => '\u{f0a10}',
        Nerd::RhombusMediumOutline => '\u{f14dc}',
        Nerd::RhombusOutline => '\u{f070c}',
        Nerd::RhombusSplit => '\u{f0a11}',
        Nerd::RhombusSplitOutline => '\u{f14dd}',
        Nerd::Ribbon => '\u{f0460}',
        Nerd::Rice => '\u{f07ea}',
        Nerd::Rickshaw => '\u{f15bb}',
        Nerd::RickshawElectric => '\u{f15bc}',
        Nerd::Ring => '\u{e23d}',
        Nerd::RingOne => '\u{f07eb}',
        Nerd::RiscV => '\u{f353}',
        Nerd::Rivet => '\u{f0e60}',
        Nerd::Road => '\u{f018}',
        Nerd::RoadOne => '\u{f0461}',
        Nerd::RoadVariant => '\u{f0462}',
        Nerd::Robber => '\u{f1058}',
        Nerd::Robot => '\u{f06a9}',
        Nerd::RobotAngry => '\u{f169d}',
        Nerd::RobotAngryOutline => '\u{f169e}',
        Nerd::RobotConfused => '\u{f169f}',
        Nerd::RobotConfusedOutline => '\u{f16a0}',
        Nerd::RobotDead => '\u{f16a1}',
        Nerd::RobotDeadOutline => '\u{f16a2}',
        Nerd::RobotExcited => '\u{f16a3}',
        Nerd::RobotExcitedOutline => '\u{f16a4}',
        Nerd::RobotHappy => '\u{f1719}',
        Nerd::RobotHappyOutline => '\u{f171a}',
        Nerd::RobotIndustrial => '\u{f0b46}',
        Nerd::RobotIndustrialOutline => '\u{f1a1a}',
        Nerd::RobotLove => '\u{f16a5}',
        Nerd::RobotLoveOutline => '\u{f16a6}',
        Nerd::RobotMower => '\u{f11f7}',
        Nerd::RobotMowerOutline => '\u{f11f3}',
        Nerd::RobotOff => '\u{f16a7}',
        Nerd::RobotOffOutline => '\u{f167b}',
        Nerd::RobotOutline => '\u{f167a}',
        Nerd::RobotVacuum => '\u{f070d}',
        Nerd::RobotVacuumVariant => '\u{f0908}',
        Nerd::Rocket => '\u{eb44}',
        Nerd::RocketLaunch => '\u{f14de}',
        Nerd::RocketLaunchOutline => '\u{f14df}',
        Nerd::RocketOne => '\u{f135}',
        Nerd::RocketOutline => '\u{f13af}',
        Nerd::RocketThree => '\u{f0463}',
        Nerd::RocketTwo => '\u{f427}',
        Nerd::RockyLinux => '\u{f32b}',
        Nerd::Rodent => '\u{f1327}',
        Nerd::RollerShade => '\u{f1a6b}',
        Nerd::RollerShadeClosed => '\u{f1a6c}',
        Nerd::RollerSkate => '\u{f0d2b}',
        Nerd::RollerSkateOff => '\u{f0145}',
        Nerd::Rollerblade => '\u{f0d2c}',
        Nerd::RollerbladeOff => '\u{f002e}',
        Nerd::Rollupjs => '\u{f0bc0}',
        Nerd::Rolodex => '\u{f1ab9}',
        Nerd::RolodexOutline => '\u{f1aba}',
        Nerd::RomanNumeralEight => '\u{f108f}',
        Nerd::RomanNumeralFour => '\u{f108b}',
        Nerd::RomanNumeralNine => '\u{f1090}',
        Nerd::RomanNumeralSeven => '\u{f108e}',
        Nerd::RomanNumeralSix => '\u{f108d}',
        Nerd::RomanNumeralThree => '\u{f108a}',
        Nerd::RomanNumeralTwo => '\u{f1089}',
        Nerd::RoomService => '\u{f088d}',
        Nerd::RoomServiceOutline => '\u{f0d97}',
        Nerd::RootFolder => '\u{eb46}',
        Nerd::RootFolderOpened => '\u{eb45}',
        Nerd::RotateLeft => '\u{f0465}',
        Nerd::RotateLeftVariant => '\u{f0466}',
        Nerd::RotateOrbit => '\u{f0d98}',
        Nerd::RotateRight => '\u{f0467}',
        Nerd::RotateRightVariant => '\u{f0468}',
        Nerd::RotateThreed => '\u{f0ec7}',
        Nerd::RotateThreedVariant => '\u{f0464}',
        Nerd::RotateThreesixzero => '\u{f1999}',
        Nerd::RoundedCorner => '\u{f0607}',
        Nerd::Router => '\u{f11e2}',
        Nerd::RouterNetwork => '\u{f1087}',
        Nerd::RouterWireless => '\u{f0469}',
        Nerd::RouterWirelessOff => '\u{f15a3}',
        Nerd::RouterWirelessSettings => '\u{f0a69}',
        Nerd::Routes => '\u{f046a}',
        Nerd::RoutesClock => '\u{f1059}',
        Nerd::Rowing => '\u{f0608}',
        Nerd::Rows => '\u{f50b}',
        Nerd::Rss => '\u{eb47}',
        Nerd::RssBox => '\u{f046c}',
        Nerd::RssOff => '\u{f0f21}',
        Nerd::RssOne => '\u{f09e}',
        Nerd::RssThree => '\u{f046b}',
        Nerd::RssTwo => '\u{f428}',
        Nerd::Rub => '\u{f158}',
        Nerd::Ruby => '\u{e23e}',
        Nerd::RubyO => '\u{e21e}',
        Nerd::RubyOne => '\u{eb48}',
        Nerd::RubyTwo => '\u{f43b}',
        Nerd::Rug => '\u{f1475}',
        Nerd::Rugby => '\u{f0d99}',
        Nerd::Ruler => '\u{e21f}',
        Nerd::RulerOne => '\u{f046d}',
        Nerd::RulerSquare => '\u{f0cc2}',
        Nerd::RulerSquareCompass => '\u{f0ebe}',
        Nerd::Run => '\u{f070e}',
        Nerd::RunAbove => '\u{ebbd}',
        Nerd::RunAll => '\u{eb9e}',
        Nerd::RunBelow => '\u{ebbe}',
        Nerd::RunErrors => '\u{ebde}',
        Nerd::RunFast => '\u{f046e}',
        Nerd::RvTruck => '\u{f11d4}',
        Nerd::Sabayon => '\u{f317}',
        Nerd::Sack => '\u{f0d2e}',
        Nerd::SackPercent => '\u{f0d2f}',
        Nerd::Safe => '\u{f0a6a}',
        Nerd::SafeSquare => '\u{f127c}',
        Nerd::SafeSquareOutline => '\u{f127d}',
        Nerd::SafetyGoggles => '\u{f0d30}',
        Nerd::SailBoat => '\u{f0ec8}',
        Nerd::SailBoatSink => '\u{f1aef}',
        Nerd::Sale => '\u{f046f}',
        Nerd::SaleOutline => '\u{f1a06}',
        Nerd::Salesforce => '\u{f088e}',
        Nerd::Sass => '\u{f07ec}',
        Nerd::Satellite => '\u{f0470}',
        Nerd::SatelliteUplink => '\u{f0909}',
        Nerd::SatelliteVariant => '\u{f0471}',
        Nerd::Sausage => '\u{f08ba}',
        Nerd::SausageOff => '\u{f1789}',
        Nerd::Save => '\u{eb4b}',
        Nerd::SaveAll => '\u{eb49}',
        Nerd::SaveAs => '\u{eb4a}',
        Nerd::SaveOne => '\u{f0c7}',
        Nerd::SawBlade => '\u{f0e61}',
        Nerd::SawtoothWave => '\u{f147a}',
        Nerd::Saxophone => '\u{f0609}',
        Nerd::Scale => '\u{f0472}',
        Nerd::ScaleBalance => '\u{f05d1}',
        Nerd::ScaleBathroom => '\u{f0473}',
        Nerd::ScaleOff => '\u{f105a}',
        Nerd::ScaleUnbalanced => '\u{f19b8}',
        Nerd::ScanHelper => '\u{f13d8}',
        Nerd::Scanner => '\u{f06ab}',
        Nerd::ScannerOff => '\u{f090a}',
        Nerd::ScatterPlot => '\u{f0ec9}',
        Nerd::ScatterPlotOutline => '\u{f0eca}',
        Nerd::Scent => '\u{f1958}',
        Nerd::ScentOff => '\u{f1959}',
        Nerd::School => '\u{f0474}',
        Nerd::SchoolOutline => '\u{f1180}',
        Nerd::ScissorsCutting => '\u{f0a6b}',
        Nerd::Scooter => '\u{f15bd}',
        Nerd::ScooterElectric => '\u{f15be}',
        Nerd::Scoreboard => '\u{f127e}',
        Nerd::ScoreboardOutline => '\u{f127f}',
        Nerd::ScreenFull => '\u{eb4c}',
        Nerd::ScreenFullOne => '\u{f50c}',
        Nerd::ScreenNormal => '\u{eb4d}',
        Nerd::ScreenNormalOne => '\u{f50d}',
        Nerd::ScreenRotation => '\u{f0475}',
        Nerd::ScreenRotationLock => '\u{f0478}',
        Nerd::Screenshot => '\u{f05b}',
        Nerd::ScrewFlatTop => '\u{f0df3}',
        Nerd::ScrewLag => '\u{f0df4}',
        Nerd::ScrewMachineFlatTop => '\u{f0df5}',
        Nerd::ScrewMachineRoundTop => '\u{f0df6}',
        Nerd::ScrewRoundTop => '\u{f0df7}',
        Nerd::Screwdriver => '\u{f0476}',
        Nerd::Script => '\u{f0bc1}',
        Nerd::ScriptOutline => '\u{f0477}',
        Nerd::ScriptText => '\u{f0bc2}',
        Nerd::ScriptTextKey => '\u{f1725}',
        Nerd::ScriptTextKeyOutline => '\u{f1726}',
        Nerd::ScriptTextOutline => '\u{f0bc3}',
        Nerd::ScriptTextPlay => '\u{f1727}',
        Nerd::ScriptTextPlayOutline => '\u{f1728}',
        Nerd::Sd => '\u{f0479}',
        Nerd::Seal => '\u{f047a}',
        Nerd::SealVariant => '\u{f0fd9}',
        Nerd::Search => '\u{ea6d}',
        Nerd::SearchOne => '\u{f002}',
        Nerd::SearchStop => '\u{eb4e}',
        Nerd::SearchTwo => '\u{f422}',
        Nerd::SearchWeb => '\u{f070f}',
        Nerd::Seat => '\u{f0cc3}',
        Nerd::SeatFlat => '\u{f047b}',
        Nerd::SeatFlatAngled => '\u{f047c}',
        Nerd::SeatIndividualSuite => '\u{f047d}',
        Nerd::SeatLegroomExtra => '\u{f047e}',
        Nerd::SeatLegroomNormal => '\u{f047f}',
        Nerd::SeatLegroomReduced => '\u{f0480}',
        Nerd::SeatOutline => '\u{f0cc4}',
        Nerd::SeatPassenger => '\u{f1249}',
        Nerd::SeatReclineExtra => '\u{f0481}',
        Nerd::SeatReclineNormal => '\u{f0482}',
        Nerd::Seatbelt => '\u{f0cc5}',
        Nerd::Security => '\u{f0483}',
        Nerd::SecurityNetwork => '\u{f0484}',
        Nerd::Seed => '\u{f0e62}',
        Nerd::SeedOff => '\u{f13fd}',
        Nerd::SeedOffOutline => '\u{f13fe}',
        Nerd::SeedOutline => '\u{f0e63}',
        Nerd::SeedPlus => '\u{f1a6d}',
        Nerd::SeedPlusOutline => '\u{f1a6e}',
        Nerd::Seesaw => '\u{f15a4}',
        Nerd::Segment => '\u{f0ecb}',
        Nerd::Select => '\u{f0485}',
        Nerd::SelectAll => '\u{f0486}',
        Nerd::SelectColor => '\u{f0d31}',
        Nerd::SelectCompare => '\u{f0ad9}',
        Nerd::SelectDrag => '\u{f0a6c}',
        Nerd::SelectGroup => '\u{f0f82}',
        Nerd::SelectInverse => '\u{f0487}',
        Nerd::SelectMarker => '\u{f1280}',
        Nerd::SelectMultiple => '\u{f1281}',
        Nerd::SelectMultipleMarker => '\u{f1282}',
        Nerd::SelectOff => '\u{f0488}',
        Nerd::SelectPlace => '\u{f0fda}',
        Nerd::SelectRemove => '\u{f17c1}',
        Nerd::SelectSearch => '\u{f1204}',
        Nerd::Selection => '\u{f0489}',
        Nerd::SelectionDrag => '\u{f0a6d}',
        Nerd::SelectionEllipse => '\u{f0d32}',
        Nerd::SelectionEllipseArrowInside => '\u{f0f22}',
        Nerd::SelectionEllipseRemove => '\u{f17c2}',
        Nerd::SelectionMarker => '\u{f1283}',
        Nerd::SelectionMultiple => '\u{f1285}',
        Nerd::SelectionMultipleMarker => '\u{f1284}',
        Nerd::SelectionOff => '\u{f0777}',
        Nerd::SelectionRemove => '\u{f17c3}',
        Nerd::SelectionSearch => '\u{f1205}',
        Nerd::SemanticWeb => '\u{f1316}',
        Nerd::Send => '\u{f048a}',
        Nerd::SendCheck => '\u{f1161}',
        Nerd::SendCheckOutline => '\u{f1162}',
        Nerd::SendCircle => '\u{f0df8}',
        Nerd::SendCircleOutline => '\u{f0df9}',
        Nerd::SendClock => '\u{f1163}',
        Nerd::SendClockOutline => '\u{f1164}',
        Nerd::SendLock => '\u{f07ed}',
        Nerd::SendLockOutline => '\u{f1166}',
        Nerd::SendOutline => '\u{f1165}',
        Nerd::SerialPort => '\u{f065c}',
        Nerd::Server => '\u{eb50}',
        Nerd::ServerEnvironment => '\u{eba3}',
        Nerd::ServerMinus => '\u{f048c}',
        Nerd::ServerNetwork => '\u{f048d}',
        Nerd::ServerNetworkOff => '\u{f048e}',
        Nerd::ServerOff => '\u{f048f}',
        Nerd::ServerOne => '\u{f473}',
        Nerd::ServerPlus => '\u{f0490}',
        Nerd::ServerProcess => '\u{eba2}',
        Nerd::ServerRemove => '\u{f0491}',
        Nerd::ServerSecurity => '\u{f0492}',
        Nerd::ServerTwo => '\u{f048b}',
        Nerd::SetAll => '\u{f0778}',
        Nerd::SetCenter => '\u{f0779}',
        Nerd::SetCenterRight => '\u{f077a}',
        Nerd::SetLeft => '\u{f077b}',
        Nerd::SetLeftCenter => '\u{f077c}',
        Nerd::SetLeftRight => '\u{f077d}',
        Nerd::SetMerge => '\u{f14e0}',
        Nerd::SetNone => '\u{f077e}',
        Nerd::SetRight => '\u{f077f}',
        Nerd::SetSplit => '\u{f14e1}',
        Nerd::SetSquare => '\u{f145d}',
        Nerd::SetTopBox => '\u{f099f}',
        Nerd::Settings => '\u{eb52}',
        Nerd::SettingsGear => '\u{eb51}',
        Nerd::SettingsHelper => '\u{f0a6e}',
        Nerd::Shaker => '\u{f110e}',
        Nerd::ShakerOutline => '\u{f110f}',
        Nerd::Shape => '\u{f0831}',
        Nerd::ShapeCirclePlus => '\u{f065d}',
        Nerd::ShapeOutline => '\u{f0832}',
        Nerd::ShapeOvalPlus => '\u{f11fa}',
        Nerd::ShapePlus => '\u{f0495}',
        Nerd::ShapePolygonPlus => '\u{f065e}',
        Nerd::ShapeRectanglePlus => '\u{f065f}',
        Nerd::ShapeSquarePlus => '\u{f0660}',
        Nerd::ShapeSquareRoundedPlus => '\u{f14fa}',
        Nerd::Share => '\u{f045}',
        Nerd::ShareAll => '\u{f11f4}',
        Nerd::ShareAllOutline => '\u{f11f5}',
        Nerd::ShareAlt => '\u{f064}',
        Nerd::ShareAndroid => '\u{f50f}',
        Nerd::ShareCircle => '\u{f11ad}',
        Nerd::ShareOff => '\u{f0f23}',
        Nerd::ShareOffOutline => '\u{f0f24}',
        Nerd::ShareOne => '\u{f50e}',
        Nerd::ShareOutline => '\u{f0932}',
        Nerd::ShareSign => '\u{f14d}',
        Nerd::ShareTwo => '\u{f0496}',
        Nerd::ShareVariant => '\u{f0497}',
        Nerd::ShareVariantOutline => '\u{f1514}',
        Nerd::Shark => '\u{f18ba}',
        Nerd::SharkFin => '\u{f1673}',
        Nerd::SharkFinOutline => '\u{f1674}',
        Nerd::SharkOff => '\u{f18bb}',
        Nerd::Sheep => '\u{f0cc6}',
        Nerd::Shield => '\u{eb53}',
        Nerd::ShieldAccount => '\u{f088f}',
        Nerd::ShieldAccountOutline => '\u{f0a12}',
        Nerd::ShieldAccountVariant => '\u{f15a7}',
        Nerd::ShieldAccountVariantOutline => '\u{f15a8}',
        Nerd::ShieldAirplane => '\u{f06bb}',
        Nerd::ShieldAirplaneOutline => '\u{f0cc7}',
        Nerd::ShieldAlert => '\u{f0ecc}',
        Nerd::ShieldAlertOutline => '\u{f0ecd}',
        Nerd::ShieldBug => '\u{f13da}',
        Nerd::ShieldBugOutline => '\u{f13db}',
        Nerd::ShieldCar => '\u{f0f83}',
        Nerd::ShieldCheck => '\u{f510}',
        Nerd::ShieldCheckOne => '\u{f0565}',
        Nerd::ShieldCheckOutline => '\u{f0cc8}',
        Nerd::ShieldCross => '\u{f0cc9}',
        Nerd::ShieldCrossOutline => '\u{f0cca}',
        Nerd::ShieldCrown => '\u{f18bc}',
        Nerd::ShieldCrownOutline => '\u{f18bd}',
        Nerd::ShieldEdit => '\u{f11a0}',
        Nerd::ShieldEditOutline => '\u{f11a1}',
        Nerd::ShieldHalf => '\u{f1360}',
        Nerd::ShieldHalfFull => '\u{f0780}',
        Nerd::ShieldHome => '\u{f068a}',
        Nerd::ShieldHomeOutline => '\u{f0ccb}',
        Nerd::ShieldKey => '\u{f0bc4}',
        Nerd::ShieldKeyOutline => '\u{f0bc5}',
        Nerd::ShieldLinkVariant => '\u{f0d33}',
        Nerd::ShieldLinkVariantOutline => '\u{f0d34}',
        Nerd::ShieldLock => '\u{f511}',
        Nerd::ShieldLockOne => '\u{f099d}',
        Nerd::ShieldLockOpen => '\u{f199a}',
        Nerd::ShieldLockOpenOutline => '\u{f199b}',
        Nerd::ShieldLockOutline => '\u{f0ccc}',
        Nerd::ShieldMoon => '\u{f1828}',
        Nerd::ShieldMoonOutline => '\u{f1829}',
        Nerd::ShieldOff => '\u{f099e}',
        Nerd::ShieldOffOutline => '\u{f099c}',
        Nerd::ShieldOne => '\u{f132}',
        Nerd::ShieldOutline => '\u{f0499}',
        Nerd::ShieldPlus => '\u{f0ada}',
        Nerd::ShieldPlusOutline => '\u{f0adb}',
        Nerd::ShieldRefresh => '\u{f00aa}',
        Nerd::ShieldRefreshOutline => '\u{f01e0}',
        Nerd::ShieldRemove => '\u{f0adc}',
        Nerd::ShieldRemoveOutline => '\u{f0add}',
        Nerd::ShieldSearch => '\u{f0d9a}',
        Nerd::ShieldSlash => '\u{f512}',
        Nerd::ShieldStar => '\u{f113b}',
        Nerd::ShieldStarOutline => '\u{f113c}',
        Nerd::ShieldSun => '\u{f105d}',
        Nerd::ShieldSunOutline => '\u{f105e}',
        Nerd::ShieldSword => '\u{f18be}',
        Nerd::ShieldSwordOutline => '\u{f18bf}',
        Nerd::ShieldSync => '\u{f11a2}',
        Nerd::ShieldSyncOutline => '\u{f11a3}',
        Nerd::ShieldThree => '\u{f0498}',
        Nerd::ShieldTwo => '\u{f49c}',
        Nerd::ShieldX => '\u{f513}',
        Nerd::Shimmer => '\u{f1545}',
        Nerd::ShipWheel => '\u{f0833}',
        Nerd::ShippingPallet => '\u{f184e}',
        Nerd::Shirt => '\u{e218}',
        Nerd::ShoeBallet => '\u{f15ca}',
        Nerd::ShoeCleat => '\u{f15c7}',
        Nerd::ShoeFormal => '\u{f0b47}',
        Nerd::ShoeHeel => '\u{f0b48}',
        Nerd::ShoePrint => '\u{f0dfa}',
        Nerd::ShoeSneaker => '\u{f15c8}',
        Nerd::Shopping => '\u{f049a}',
        Nerd::ShoppingCart => '\u{f07a}',
        Nerd::ShoppingMusic => '\u{f049b}',
        Nerd::ShoppingOutline => '\u{f11d5}',
        Nerd::ShoppingSearch => '\u{f0f84}',
        Nerd::ShoppingSearchOutline => '\u{f1a6f}',
        Nerd::Shore => '\u{f14f9}',
        Nerd::ShortPause => '\u{e005}',
        Nerd::Shovel => '\u{f0710}',
        Nerd::ShovelOff => '\u{f0711}',
        Nerd::Shower => '\u{f09a0}',
        Nerd::ShowerHead => '\u{f09a1}',
        Nerd::Shredder => '\u{f049c}',
        Nerd::Shuffle => '\u{f049d}',
        Nerd::ShuffleDisabled => '\u{f049e}',
        Nerd::ShuffleVariant => '\u{f049f}',
        Nerd::Shuriken => '\u{f137f}',
        Nerd::Sickle => '\u{f18c0}',
        Nerd::SidebarCollapse => '\u{f514}',
        Nerd::SidebarExpand => '\u{f515}',
        Nerd::Sigma => '\u{f04a0}',
        Nerd::SigmaLower => '\u{f062b}',
        Nerd::SignBlank => '\u{f0c8}',
        Nerd::SignCaution => '\u{f04a1}',
        Nerd::SignDirection => '\u{f0781}',
        Nerd::SignDirectionMinus => '\u{f1000}',
        Nerd::SignDirectionPlus => '\u{f0fdc}',
        Nerd::SignDirectionRemove => '\u{f0fdd}',
        Nerd::SignIn => '\u{ea6f}',
        Nerd::SignInOne => '\u{f42a}',
        Nerd::SignOut => '\u{ea6e}',
        Nerd::SignOutOne => '\u{f426}',
        Nerd::SignPole => '\u{f14f8}',
        Nerd::SignRealEstate => '\u{f1118}',
        Nerd::SignText => '\u{f0782}',
        Nerd::Signal => '\u{f012}',
        Nerd::SignalCellularOne => '\u{f08bc}',
        Nerd::SignalCellularOutline => '\u{f08bf}',
        Nerd::SignalCellularThree => '\u{f08be}',
        Nerd::SignalCellularTwo => '\u{f08bd}',
        Nerd::SignalDistanceVariant => '\u{f0e64}',
        Nerd::SignalFiveg => '\u{f0a6f}',
        Nerd::SignalFourg => '\u{f0714}',
        Nerd::SignalHspa => '\u{f0715}',
        Nerd::SignalHspaPlus => '\u{f0716}',
        Nerd::SignalOff => '\u{f0783}',
        Nerd::SignalOne => '\u{f04a2}',
        Nerd::SignalThreeg => '\u{f0713}',
        Nerd::SignalTwog => '\u{f0712}',
        Nerd::SignalVariant => '\u{f060a}',
        Nerd::Signature => '\u{f0dfb}',
        Nerd::SignatureFreehand => '\u{f0dfc}',
        Nerd::SignatureImage => '\u{f0dfd}',
        Nerd::SignatureText => '\u{f0dfe}',
        Nerd::Signin => '\u{f090}',
        Nerd::Signout => '\u{f08b}',
        Nerd::Silo => '\u{f0b49}',
        Nerd::Silverware => '\u{f04a3}',
        Nerd::SilverwareClean => '\u{f0fde}',
        Nerd::SilverwareFork => '\u{f04a4}',
        Nerd::SilverwareForkKnife => '\u{f0a70}',
        Nerd::SilverwareSpoon => '\u{f04a5}',
        Nerd::SilverwareVariant => '\u{f04a6}',
        Nerd::Sim => '\u{f04a7}',
        Nerd::SimAlert => '\u{f04a8}',
        Nerd::SimAlertOutline => '\u{f15d3}',
        Nerd::SimOff => '\u{f04a9}',
        Nerd::SimOffOutline => '\u{f15d4}',
        Nerd::SimOutline => '\u{f15d5}',
        Nerd::SimpleIcons => '\u{f131d}',
        Nerd::SinaWeibo => '\u{f0adf}',
        Nerd::SineWave => '\u{f095b}',
        Nerd::SingleSelect => '\u{f516}',
        Nerd::Sitemap => '\u{f0e8}',
        Nerd::SitemapOne => '\u{f04aa}',
        Nerd::SitemapOutline => '\u{f199c}',
        Nerd::Sixoneeight => '\u{f293}',
        Nerd::Sixonefive => '\u{f290}',
        Nerd::Sixonefour => '\u{f28e}',
        Nerd::Sixonenine => '\u{f294}',
        Nerd::Sixoneone => '\u{f28b}',
        Nerd::Sixoneseven => '\u{f292}',
        Nerd::Sixonesix => '\u{f291}',
        Nerd::Sixonethree => '\u{f28d}',
        Nerd::Sixonetwo => '\u{f28c}',
        Nerd::Sixonezero => '\u{f28a}',
        Nerd::Sixtwoeight => '\u{f29d}',
        Nerd::Sixtwofive => '\u{f29a}',
        Nerd::Sixtwofour => '\u{f299}',
        Nerd::Sixtwonine => '\u{f29e}',
        Nerd::Sixtwoone => '\u{f296}',
        Nerd::Sixtwoseven => '\u{f29c}',
        Nerd::Sixtwosix => '\u{f29b}',
        Nerd::Sixtwothree => '\u{f298}',
        Nerd::Sixtwotwo => '\u{f297}',
        Nerd::Sixtwozero => '\u{f295}',
        Nerd::Sixzeroeight => '\u{f288}',
        Nerd::Sixzerofour => '\u{f284}',
        Nerd::Sixzeronine => '\u{f289}',
        Nerd::Sixzeroseven => '\u{f287}',
        Nerd::Sixzerothree => '\u{f283}',
        Nerd::Sixzerotwo => '\u{f282}',
        Nerd::SizeM => '\u{f13a5}',
        Nerd::SizeS => '\u{f13a4}',
        Nerd::SizeXl => '\u{f13a7}',
        Nerd::SizeXs => '\u{f13a3}',
        Nerd::SizeXxl => '\u{f13a8}',
        Nerd::SizeXxs => '\u{f13a2}',
        Nerd::SizeXxxl => '\u{f13a9}',
        Nerd::Skate => '\u{f0d35}',
        Nerd::SkateOff => '\u{f0699}',
        Nerd::Skateboard => '\u{f14c2}',
        Nerd::Skateboarding => '\u{f0501}',
        Nerd::SkewLess => '\u{f0d36}',
        Nerd::SkewMore => '\u{f0d37}',
        Nerd::Ski => '\u{f1304}',
        Nerd::SkiCrossCountry => '\u{f1305}',
        Nerd::SkiWater => '\u{f1306}',
        Nerd::Skip => '\u{f517}',
        Nerd::SkipBackward => '\u{f04ab}',
        Nerd::SkipBackwardOutline => '\u{f0f25}',
        Nerd::SkipFill => '\u{f518}',
        Nerd::SkipForward => '\u{f04ac}',
        Nerd::SkipForwardOutline => '\u{f0f26}',
        Nerd::SkipNext => '\u{f04ad}',
        Nerd::SkipNextCircle => '\u{f0661}',
        Nerd::SkipNextCircleOutline => '\u{f0662}',
        Nerd::SkipNextOutline => '\u{f0f27}',
        Nerd::SkipPrevious => '\u{f04ae}',
        Nerd::SkipPreviousCircle => '\u{f0663}',
        Nerd::SkipPreviousCircleOutline => '\u{f0664}',
        Nerd::SkipPreviousOutline => '\u{f0f28}',
        Nerd::Skull => '\u{f068c}',
        Nerd::SkullCrossbones => '\u{f0bc6}',
        Nerd::SkullCrossbonesOutline => '\u{f0bc7}',
        Nerd::SkullOutline => '\u{f0bc8}',
        Nerd::SkullScan => '\u{f14c7}',
        Nerd::SkullScanOutline => '\u{f14c8}',
        Nerd::Skype => '\u{f17e}',
        Nerd::SkypeBusiness => '\u{f04b0}',
        Nerd::SkypeOne => '\u{f04af}',
        Nerd::Slack => '\u{f04b1}',
        Nerd::Slackware => '\u{f318}',
        Nerd::SlackwareInverse => '\u{f319}',
        Nerd::Slash => '\u{e216}',
        Nerd::SlashForward => '\u{f0fdf}',
        Nerd::SlashForwardBox => '\u{f0fe0}',
        Nerd::Sledding => '\u{f041b}',
        Nerd::Sleep => '\u{f04b2}',
        Nerd::SleepOff => '\u{f04b3}',
        Nerd::Slide => '\u{f15a5}',
        Nerd::Sliders => '\u{f462}',
        Nerd::SlopeDownhill => '\u{f0dff}',
        Nerd::SlopeUphill => '\u{f0e00}',
        Nerd::SlotMachine => '\u{f1114}',
        Nerd::SlotMachineOutline => '\u{f1115}',
        Nerd::Smaller => '\u{e200}',
        Nerd::SmartCard => '\u{f10bd}',
        Nerd::SmartCardOff => '\u{f18f7}',
        Nerd::SmartCardOffOutline => '\u{f18f8}',
        Nerd::SmartCardOutline => '\u{f10be}',
        Nerd::SmartCardReader => '\u{f10bf}',
        Nerd::SmartCardReaderOutline => '\u{f10c0}',
        Nerd::Smile => '\u{f118}',
        Nerd::Smiley => '\u{eb54}',
        Nerd::SmileyOne => '\u{f4a2}',
        Nerd::Smog => '\u{f0a71}',
        Nerd::Smoke => '\u{f1799}',
        Nerd::SmokeDetector => '\u{f0392}',
        Nerd::SmokeDetectorAlert => '\u{f192e}',
        Nerd::SmokeDetectorAlertOutline => '\u{f192f}',
        Nerd::SmokeDetectorOff => '\u{f1809}',
        Nerd::SmokeDetectorOffOutline => '\u{f180a}',
        Nerd::SmokeDetectorOutline => '\u{f1808}',
        Nerd::SmokeDetectorVariant => '\u{f180b}',
        Nerd::SmokeDetectorVariantAlert => '\u{f1930}',
        Nerd::SmokeDetectorVariantOff => '\u{f180c}',
        Nerd::Smoking => '\u{f04b4}',
        Nerd::SmokingOff => '\u{f04b5}',
        Nerd::SmokingPipe => '\u{f140d}',
        Nerd::SmokingPipeOff => '\u{f1428}',
        Nerd::Snail => '\u{f1677}',
        Nerd::Snake => '\u{f150e}',
        Nerd::Snapchat => '\u{f04b6}',
        Nerd::Snappy => '\u{f32c}',
        Nerd::Snowboard => '\u{f1307}',
        Nerd::Snowflake => '\u{f0717}',
        Nerd::SnowflakeAlert => '\u{f0f29}',
        Nerd::SnowflakeCheck => '\u{f1a70}',
        Nerd::SnowflakeMelt => '\u{f12cb}',
        Nerd::SnowflakeOff => '\u{f14e3}',
        Nerd::SnowflakeThermometer => '\u{f1a71}',
        Nerd::SnowflakeVariant => '\u{f0f2a}',
        Nerd::Snowing => '\u{e201}',
        Nerd::Snowman => '\u{f04b7}',
        Nerd::Snowmobile => '\u{f06dd}',
        Nerd::Snowshoeing => '\u{f1a72}',
        Nerd::Soccer => '\u{f04b8}',
        Nerd::SoccerField => '\u{f0834}',
        Nerd::SocialDistanceSixFeet => '\u{f157a}',
        Nerd::SocialDistanceTwoMeters => '\u{f1579}',
        Nerd::Soda => '\u{e202}',
        Nerd::Sofa => '\u{e203}',
        Nerd::SofaOne => '\u{f04b9}',
        Nerd::SofaOutline => '\u{f156d}',
        Nerd::SofaSingle => '\u{f156e}',
        Nerd::SofaSingleOutline => '\u{f156f}',
        Nerd::SolarPanel => '\u{f0d9b}',
        Nerd::SolarPanelLarge => '\u{f0d9c}',
        Nerd::SolarPower => '\u{f0a72}',
        Nerd::SolarPowerVariant => '\u{f1a73}',
        Nerd::SolarPowerVariantOutline => '\u{f1a74}',
        Nerd::SolderingIron => '\u{f1092}',
        Nerd::Solid => '\u{f068d}',
        Nerd::Solus => '\u{f32d}',
        Nerd::SonyPlaystation => '\u{f0414}',
        Nerd::Sort => '\u{f0dc}',
        Nerd::SortAlphabeticalAscending => '\u{f05bd}',
        Nerd::SortAlphabeticalAscendingVariant => '\u{f1148}',
        Nerd::SortAlphabeticalDescending => '\u{f05bf}',
        Nerd::SortAlphabeticalDescendingVariant => '\u{f1149}',
        Nerd::SortAlphabeticalVariant => '\u{f04bb}',
        Nerd::SortAsc => '\u{f519}',
        Nerd::SortAscending => '\u{f04bc}',
        Nerd::SortBoolAscending => '\u{f1385}',
        Nerd::SortBoolAscendingVariant => '\u{f1386}',
        Nerd::SortBoolDescending => '\u{f1387}',
        Nerd::SortBoolDescendingVariant => '\u{f1388}',
        Nerd::SortByAlphabet => '\u{f15d}',
        Nerd::SortByAttributes => '\u{f160}',
        Nerd::SortByAttributesAlt => '\u{f161}',
        Nerd::SortByOrder => '\u{f162}',
        Nerd::SortByOrderAlt => '\u{f163}',
        Nerd::SortCalendarAscending => '\u{f1547}',
        Nerd::SortCalendarDescending => '\u{f1548}',
        Nerd::SortClockAscending => '\u{f1549}',
        Nerd::SortClockAscendingOutline => '\u{f154a}',
        Nerd::SortClockDescending => '\u{f154b}',
        Nerd::SortClockDescendingOutline => '\u{f154c}',
        Nerd::SortDesc => '\u{f51a}',
        Nerd::SortDescending => '\u{f04bd}',
        Nerd::SortDown => '\u{f0dd}',
        Nerd::SortNumericAscending => '\u{f1389}',
        Nerd::SortNumericAscendingVariant => '\u{f090d}',
        Nerd::SortNumericDescending => '\u{f138a}',
        Nerd::SortNumericDescendingVariant => '\u{f0ad2}',
        Nerd::SortNumericVariant => '\u{f04be}',
        Nerd::SortOne => '\u{f04ba}',
        Nerd::SortPrecedence => '\u{eb55}',
        Nerd::SortReverseVariant => '\u{f033c}',
        Nerd::SortUp => '\u{f0de}',
        Nerd::SortVariant => '\u{f04bf}',
        Nerd::SortVariantLock => '\u{f0ccd}',
        Nerd::SortVariantLockOpen => '\u{f0cce}',
        Nerd::SortVariantOff => '\u{f1abb}',
        Nerd::SortVariantRemove => '\u{f1147}',
        Nerd::Soundbar => '\u{f17db}',
        Nerd::Soundcloud => '\u{f04c0}',
        Nerd::Soup => '\u{e204}',
        Nerd::SourceBranch => '\u{f062c}',
        Nerd::SourceBranchCheck => '\u{f14cf}',
        Nerd::SourceBranchMinus => '\u{f14cb}',
        Nerd::SourceBranchPlus => '\u{f14ca}',
        Nerd::SourceBranchRefresh => '\u{f14cd}',
        Nerd::SourceBranchRemove => '\u{f14cc}',
        Nerd::SourceBranchSync => '\u{f14ce}',
        Nerd::SourceCommit => '\u{f0718}',
        Nerd::SourceCommitEnd => '\u{f0719}',
        Nerd::SourceCommitEndLocal => '\u{f071a}',
        Nerd::SourceCommitLocal => '\u{f071b}',
        Nerd::SourceCommitNextLocal => '\u{f071c}',
        Nerd::SourceCommitStart => '\u{f071d}',
        Nerd::SourceCommitStartNextLocal => '\u{f071e}',
        Nerd::SourceControl => '\u{ea68}',
        Nerd::SourceFork => '\u{f04c1}',
        Nerd::SourceMerge => '\u{f062d}',
        Nerd::SourcePull => '\u{f04c2}',
        Nerd::SourceRepository => '\u{f0ccf}',
        Nerd::SourceRepositoryMultiple => '\u{f0cd0}',
        Nerd::SoySauce => '\u{f07ee}',
        Nerd::SoySauceOff => '\u{f13fc}',
        Nerd::Spa => '\u{f0cd1}',
        Nerd::SpaOutline => '\u{f0cd2}',
        Nerd::SpaceInvaders => '\u{f0bc9}',
        Nerd::SpaceStation => '\u{f1383}',
        Nerd::Spade => '\u{f0e65}',
        Nerd::SparkleFill => '\u{f51b}',
        Nerd::Speaker => '\u{f04c3}',
        Nerd::SpeakerBluetooth => '\u{f09a2}',
        Nerd::SpeakerMultiple => '\u{f0d38}',
        Nerd::SpeakerOff => '\u{f04c4}',
        Nerd::SpeakerWireless => '\u{f071f}',
        Nerd::Spear => '\u{f1845}',
        Nerd::Speedometer => '\u{f04c5}',
        Nerd::SpeedometerMedium => '\u{f0f85}',
        Nerd::SpeedometerSlow => '\u{f0f86}',
        Nerd::Spellcheck => '\u{f04c6}',
        Nerd::Spermatozoon => '\u{e205}',
        Nerd::Sphere => '\u{f1954}',
        Nerd::SphereOff => '\u{f1955}',
        Nerd::Spider => '\u{f11ea}',
        Nerd::SpiderThread => '\u{f11eb}',
        Nerd::SpiderWeb => '\u{f0bca}',
        Nerd::SpinDouble => '\u{e206}',
        Nerd::Spinner => '\u{f110}',
        Nerd::SpiritLevel => '\u{f14f1}',
        Nerd::SplitHorizontal => '\u{eb56}',
        Nerd::SplitVertical => '\u{eb57}',
        Nerd::SponsorTiers => '\u{f51c}',
        Nerd::SpoonSugar => '\u{f1429}',
        Nerd::Spotify => '\u{f04c7}',
        Nerd::Spotlight => '\u{f04c8}',
        Nerd::SpotlightBeam => '\u{f04c9}',
        Nerd::Spray => '\u{f0665}',
        Nerd::SprayBottle => '\u{f0ae0}',
        Nerd::Sprinkler => '\u{f105f}',
        Nerd::SprinklerFire => '\u{f199d}',
        Nerd::SprinklerVariant => '\u{f1060}',
        Nerd::Sprout => '\u{f0e66}',
        Nerd::SproutOutline => '\u{f0e67}',
        Nerd::Square => '\u{f51d}',
        Nerd::SquareCircle => '\u{f1500}',
        Nerd::SquareEditOutline => '\u{f090c}',
        Nerd::SquareFill => '\u{f445}',
        Nerd::SquareMedium => '\u{f0a13}',
        Nerd::SquareMediumOutline => '\u{f0a14}',
        Nerd::SquareOff => '\u{f12ee}',
        Nerd::SquareOffOutline => '\u{f12ef}',
        Nerd::SquareOne => '\u{f0764}',
        Nerd::SquareOpacity => '\u{f1854}',
        Nerd::SquareOutline => '\u{f0763}',
        Nerd::SquareRoot => '\u{f0784}',
        Nerd::SquareRootBox => '\u{f09a3}',
        Nerd::SquareRounded => '\u{f14fb}',
        Nerd::SquareRoundedBadge => '\u{f1a07}',
        Nerd::SquareRoundedBadgeOutline => '\u{f1a08}',
        Nerd::SquareRoundedOutline => '\u{f14fc}',
        Nerd::SquareSmall => '\u{f0a15}',
        Nerd::SquareWave => '\u{f147b}',
        Nerd::Squeegee => '\u{f0ae1}',
        Nerd::Squirrel => '\u{eb58}',
        Nerd::SquirrelOne => '\u{f483}',
        Nerd::Ssh => '\u{f08c0}',
        Nerd::Stack => '\u{f51e}',
        Nerd::StackExchange => '\u{f060b}',
        Nerd::StackOverflow => '\u{f04cc}',
        Nerd::Stackexchange => '\u{f16c}',
        Nerd::Stackpath => '\u{f0359}',
        Nerd::Stadium => '\u{f0ff9}',
        Nerd::StadiumVariant => '\u{f0720}',
        Nerd::Stairs => '\u{f04cd}',
        Nerd::StairsBox => '\u{f139e}',
        Nerd::StairsDown => '\u{f12be}',
        Nerd::StairsUp => '\u{f12bd}',
        Nerd::Stamper => '\u{f0d39}',
        Nerd::StandardDefinition => '\u{f07ef}',
        Nerd::Star => '\u{f005}',
        Nerd::StarBox => '\u{f0a73}',
        Nerd::StarBoxMultiple => '\u{f1286}',
        Nerd::StarBoxMultipleOutline => '\u{f1287}',
        Nerd::StarBoxOutline => '\u{f0a74}',
        Nerd::StarCheck => '\u{f1566}',
        Nerd::StarCheckOutline => '\u{f156a}',
        Nerd::StarCircle => '\u{f04cf}',
        Nerd::StarCircleOutline => '\u{f09a4}',
        Nerd::StarCog => '\u{f1668}',
        Nerd::StarCogOutline => '\u{f1669}',
        Nerd::StarCrescent => '\u{f0979}',
        Nerd::StarDavid => '\u{f097a}',
        Nerd::StarEmpty => '\u{f006}',
        Nerd::StarFace => '\u{f09a5}',
        Nerd::StarFill => '\u{f51f}',
        Nerd::StarFourPoints => '\u{f0ae2}',
        Nerd::StarFourPointsOutline => '\u{f0ae3}',
        Nerd::StarFull => '\u{eb59}',
        Nerd::StarHalf => '\u{f089}',
        Nerd::StarHalfEmpty => '\u{f123}',
        Nerd::StarHalfFull => '\u{f04d0}',
        Nerd::StarHalfOne => '\u{f0246}',
        Nerd::StarMinus => '\u{f1564}',
        Nerd::StarMinusOutline => '\u{f1568}',
        Nerd::StarOff => '\u{f04d1}',
        Nerd::StarOffOutline => '\u{f155b}',
        Nerd::StarOne => '\u{f41e}',
        Nerd::StarOutline => '\u{f04d2}',
        Nerd::StarPlus => '\u{f1563}',
        Nerd::StarPlusOutline => '\u{f1567}',
        Nerd::StarRemove => '\u{f1565}',
        Nerd::StarRemoveOutline => '\u{f1569}',
        Nerd::StarSettings => '\u{f166a}',
        Nerd::StarSettingsOutline => '\u{f166b}',
        Nerd::StarShooting => '\u{f1741}',
        Nerd::StarShootingOutline => '\u{f1742}',
        Nerd::StarThreePoints => '\u{f0ae4}',
        Nerd::StarThreePointsOutline => '\u{f0ae5}',
        Nerd::StarTwo => '\u{f04ce}',
        Nerd::StateMachine => '\u{f11ef}',
        Nerd::Steam => '\u{f04d3}',
        Nerd::Steering => '\u{f04d4}',
        Nerd::SteeringOff => '\u{f090e}',
        Nerd::StepBackward => '\u{f04d5}',
        Nerd::StepBackwardTwo => '\u{f04d6}',
        Nerd::StepForward => '\u{f04d7}',
        Nerd::StepForwardTwo => '\u{f04d8}',
        Nerd::Stethoscope => '\u{f0f1}',
        Nerd::StethoscopeOne => '\u{f04d9}',
        Nerd::Sticker => '\u{f1364}',
        Nerd::StickerAlert => '\u{f1365}',
        Nerd::StickerAlertOutline => '\u{f1366}',
        Nerd::StickerCheck => '\u{f1367}',
        Nerd::StickerCheckOutline => '\u{f1368}',
        Nerd::StickerCircleOutline => '\u{f05d0}',
        Nerd::StickerEmoji => '\u{f0785}',
        Nerd::StickerMinus => '\u{f1369}',
        Nerd::StickerMinusOutline => '\u{f136a}',
        Nerd::StickerOutline => '\u{f136b}',
        Nerd::StickerPlus => '\u{f136c}',
        Nerd::StickerPlusOutline => '\u{f136d}',
        Nerd::StickerRemove => '\u{f136e}',
        Nerd::StickerRemoveOutline => '\u{f136f}',
        Nerd::StickerText => '\u{f178e}',
        Nerd::StickerTextOutline => '\u{f178f}',
        Nerd::Stocking => '\u{f04da}',
        Nerd::Stomach => '\u{e207}',
        Nerd::StomachOne => '\u{f1093}',
        Nerd::Stool => '\u{f195d}',
        Nerd::StoolOutline => '\u{f195e}',
        Nerd::Stop => '\u{f04d}',
        Nerd::StopCircle => '\u{eba5}',
        Nerd::StopCircleOne => '\u{f0666}',
        Nerd::StopCircleOutline => '\u{f0667}',
        Nerd::StopOne => '\u{f46e}',
        Nerd::StopTwo => '\u{f04db}',
        Nerd::Stopwatch => '\u{f520}',
        Nerd::StorageTank => '\u{f1a75}',
        Nerd::StorageTankOutline => '\u{f1a76}',
        Nerd::Store => '\u{f04dc}',
        Nerd::StoreAlert => '\u{f18c1}',
        Nerd::StoreAlertOutline => '\u{f18c2}',
        Nerd::StoreCheck => '\u{f18c3}',
        Nerd::StoreCheckOutline => '\u{f18c4}',
        Nerd::StoreClock => '\u{f18c5}',
        Nerd::StoreClockOutline => '\u{f18c6}',
        Nerd::StoreCog => '\u{f18c7}',
        Nerd::StoreCogOutline => '\u{f18c8}',
        Nerd::StoreEdit => '\u{f18c9}',
        Nerd::StoreEditOutline => '\u{f18ca}',
        Nerd::StoreMarker => '\u{f18cb}',
        Nerd::StoreMarkerOutline => '\u{f18cc}',
        Nerd::StoreMinus => '\u{f165e}',
        Nerd::StoreMinusOutline => '\u{f18cd}',
        Nerd::StoreOff => '\u{f18ce}',
        Nerd::StoreOffOutline => '\u{f18cf}',
        Nerd::StoreOutline => '\u{f1361}',
        Nerd::StorePlus => '\u{f165f}',
        Nerd::StorePlusOutline => '\u{f18d0}',
        Nerd::StoreRemove => '\u{f1660}',
        Nerd::StoreRemoveOutline => '\u{f18d1}',
        Nerd::StoreSearch => '\u{f18d2}',
        Nerd::StoreSearchOutline => '\u{f18d3}',
        Nerd::StoreSettings => '\u{f18d4}',
        Nerd::StoreSettingsOutline => '\u{f18d5}',
        Nerd::StoreTwofourHour => '\u{f04dd}',
        Nerd::Storefront => '\u{f07c7}',
        Nerd::StorefrontOutline => '\u{f10c1}',
        Nerd::Storm => '\u{e208}',
        Nerd::Stove => '\u{f04de}',
        Nerd::Strategy => '\u{f11d6}',
        Nerd::StretchToPage => '\u{f0f2b}',
        Nerd::StretchToPageOutline => '\u{f0f2c}',
        Nerd::Strikethrough => '\u{f0cc}',
        Nerd::StrikethroughOne => '\u{f521}',
        Nerd::StringLights => '\u{f12ba}',
        Nerd::StringLightsOff => '\u{f12bb}',
        Nerd::SubdirectoryArrowLeft => '\u{f060c}',
        Nerd::SubdirectoryArrowRight => '\u{f060d}',
        Nerd::Submarine => '\u{f156c}',
        Nerd::Subscript => '\u{f12c}',
        Nerd::Subtitles => '\u{f0a16}',
        Nerd::SubtitlesOutline => '\u{f0a17}',
        Nerd::Subway => '\u{f06ac}',
        Nerd::SubwayAlertVariant => '\u{f0d9d}',
        Nerd::SubwayVariant => '\u{f04df}',
        Nerd::Suitcase => '\u{f0f2}',
        Nerd::Summit => '\u{f0786}',
        Nerd::Sun => '\u{f185}',
        Nerd::SunClock => '\u{f1a77}',
        Nerd::SunClockOutline => '\u{f1a78}',
        Nerd::SunCloud => '\u{e21d}',
        Nerd::SunCompass => '\u{f19a5}',
        Nerd::SunOne => '\u{f522}',
        Nerd::SunSnowflake => '\u{f1796}',
        Nerd::SunSnowflakeVariant => '\u{f1a79}',
        Nerd::SunThermometer => '\u{f18d6}',
        Nerd::SunThermometerOutline => '\u{f18d7}',
        Nerd::SunWireless => '\u{f17fe}',
        Nerd::SunWirelessOutline => '\u{f17ff}',
        Nerd::Sunglasses => '\u{f04e0}',
        Nerd::Superscript => '\u{f12b}',
        Nerd::Surfing => '\u{f1746}',
        Nerd::SurroundSound => '\u{f05c5}',
        Nerd::SurroundSoundFiveOne => '\u{f07f2}',
        Nerd::SurroundSoundFiveOneTwo => '\u{f172a}',
        Nerd::SurroundSoundSevenOne => '\u{f07f3}',
        Nerd::SurroundSoundThreeOne => '\u{f07f1}',
        Nerd::SurroundSoundTwoOne => '\u{f1729}',
        Nerd::SurroundSoundTwoZero => '\u{f07f0}',
        Nerd::Sushi => '\u{e21a}',
        Nerd::Svg => '\u{f0721}',
        Nerd::SwapHorizontal => '\u{f04e1}',
        Nerd::SwapHorizontalBold => '\u{f0bcd}',
        Nerd::SwapHorizontalCircle => '\u{f0fe1}',
        Nerd::SwapHorizontalCircleOutline => '\u{f0fe2}',
        Nerd::SwapHorizontalVariant => '\u{f08c1}',
        Nerd::SwapVertical => '\u{f04e2}',
        Nerd::SwapVerticalBold => '\u{f0bce}',
        Nerd::SwapVerticalCircle => '\u{f0fe3}',
        Nerd::SwapVerticalCircleOutline => '\u{f0fe4}',
        Nerd::SwapVerticalVariant => '\u{f08c2}',
        Nerd::Sway => '\u{f35d}',
        Nerd::Swim => '\u{f04e3}',
        Nerd::Switch => '\u{f04e4}',
        Nerd::Sword => '\u{f04e5}',
        Nerd::SwordCross => '\u{f0787}',
        Nerd::SyllabaryHangul => '\u{f1333}',
        Nerd::SyllabaryHiragana => '\u{f1334}',
        Nerd::SyllabaryKatakana => '\u{f1335}',
        Nerd::SyllabaryKatakanaHalfwidth => '\u{f1336}',
        Nerd::Symbol => '\u{f1501}',
        Nerd::SymbolArray => '\u{ea8a}',
        Nerd::SymbolBoolean => '\u{ea8f}',
        Nerd::SymbolClass => '\u{eb5b}',
        Nerd::SymbolColor => '\u{eb5c}',
        Nerd::SymbolConstant => '\u{eb5d}',
        Nerd::SymbolEnum => '\u{ea95}',
        Nerd::SymbolEnumMember => '\u{eb5e}',
        Nerd::SymbolEvent => '\u{ea86}',
        Nerd::SymbolField => '\u{eb5f}',
        Nerd::SymbolFile => '\u{eb60}',
        Nerd::SymbolInterface => '\u{eb61}',
        Nerd::SymbolKey => '\u{ea93}',
        Nerd::SymbolKeyword => '\u{eb62}',
        Nerd::SymbolMethod => '\u{ea8c}',
        Nerd::SymbolMisc => '\u{eb63}',
        Nerd::SymbolNamespace => '\u{ea8b}',
        Nerd::SymbolNumeric => '\u{ea90}',
        Nerd::SymbolOperator => '\u{eb64}',
        Nerd::SymbolParameter => '\u{ea92}',
        Nerd::SymbolProperty => '\u{eb65}',
        Nerd::SymbolRuler => '\u{ea96}',
        Nerd::SymbolSnippet => '\u{eb66}',
        Nerd::SymbolString => '\u{eb8d}',
        Nerd::SymbolStructure => '\u{ea91}',
        Nerd::SymbolVariable => '\u{ea88}',
        Nerd::Symfony => '\u{f0ae6}',
        Nerd::Sync => '\u{ea77}',
        Nerd::SyncAlert => '\u{f04e7}',
        Nerd::SyncCircle => '\u{f1378}',
        Nerd::SyncIgnored => '\u{eb9f}',
        Nerd::SyncOff => '\u{f04e8}',
        Nerd::SyncOne => '\u{f46a}',
        Nerd::SyncTwo => '\u{f04e6}',
        Nerd::Tab => '\u{f523}',
        Nerd::TabExternal => '\u{f524}',
        Nerd::TabMinus => '\u{f0b4b}',
        Nerd::TabOne => '\u{f04e9}',
        Nerd::TabPlus => '\u{f075c}',
        Nerd::TabRemove => '\u{f0b4c}',
        Nerd::TabSearch => '\u{f199e}',
        Nerd::TabUnselected => '\u{f04ea}',
        Nerd::Table => '\u{ebb7}',
        Nerd::TableAccount => '\u{f13b9}',
        Nerd::TableAlert => '\u{f13ba}',
        Nerd::TableArrowDown => '\u{f13bb}',
        Nerd::TableArrowLeft => '\u{f13bc}',
        Nerd::TableArrowRight => '\u{f13bd}',
        Nerd::TableArrowUp => '\u{f13be}',
        Nerd::TableBorder => '\u{f0a18}',
        Nerd::TableCancel => '\u{f13bf}',
        Nerd::TableChair => '\u{f1061}',
        Nerd::TableCheck => '\u{f13c0}',
        Nerd::TableClock => '\u{f13c1}',
        Nerd::TableCog => '\u{f13c2}',
        Nerd::TableColumn => '\u{f0835}',
        Nerd::TableColumnPlusAfter => '\u{f04ec}',
        Nerd::TableColumnPlusBefore => '\u{f04ed}',
        Nerd::TableColumnRemove => '\u{f04ee}',
        Nerd::TableColumnWidth => '\u{f04ef}',
        Nerd::TableEdit => '\u{f04f0}',
        Nerd::TableEye => '\u{f1094}',
        Nerd::TableEyeOff => '\u{f13c3}',
        Nerd::TableFurniture => '\u{f05bc}',
        Nerd::TableHeadersEye => '\u{f121d}',
        Nerd::TableHeadersEyeOff => '\u{f121e}',
        Nerd::TableHeart => '\u{f13c4}',
        Nerd::TableKey => '\u{f13c5}',
        Nerd::TableLarge => '\u{f04f1}',
        Nerd::TableLargePlus => '\u{f0f87}',
        Nerd::TableLargeRemove => '\u{f0f88}',
        Nerd::TableLock => '\u{f13c6}',
        Nerd::TableMergeCells => '\u{f09a6}',
        Nerd::TableMinus => '\u{f13c7}',
        Nerd::TableMultiple => '\u{f13c8}',
        Nerd::TableNetwork => '\u{f13c9}',
        Nerd::TableOfContents => '\u{f0836}',
        Nerd::TableOff => '\u{f13ca}',
        Nerd::TableOne => '\u{f0ce}',
        Nerd::TablePicnic => '\u{f1743}',
        Nerd::TablePivot => '\u{f183c}',
        Nerd::TablePlus => '\u{f0a75}',
        Nerd::TableRefresh => '\u{f13a0}',
        Nerd::TableRemove => '\u{f0a76}',
        Nerd::TableRow => '\u{f0837}',
        Nerd::TableRowHeight => '\u{f04f2}',
        Nerd::TableRowPlusAfter => '\u{f04f3}',
        Nerd::TableRowPlusBefore => '\u{f04f4}',
        Nerd::TableRowRemove => '\u{f04f5}',
        Nerd::TableSearch => '\u{f090f}',
        Nerd::TableSettings => '\u{f0838}',
        Nerd::TableSplitCell => '\u{f142a}',
        Nerd::TableStar => '\u{f13cb}',
        Nerd::TableSync => '\u{f13a1}',
        Nerd::TableTennis => '\u{f0e68}',
        Nerd::TableThree => '\u{f04eb}',
        Nerd::TableTwo => '\u{f525}',
        Nerd::Tablet => '\u{f10a}',
        Nerd::TabletAndroid => '\u{f04f7}',
        Nerd::TabletCellphone => '\u{f09a7}',
        Nerd::TabletDashboard => '\u{f0ece}',
        Nerd::TabletOne => '\u{f04f6}',
        Nerd::Taco => '\u{f0762}',
        Nerd::Tacos => '\u{e219}',
        Nerd::Tag => '\u{ea66}',
        Nerd::TagArrowDown => '\u{f172b}',
        Nerd::TagArrowDownOutline => '\u{f172c}',
        Nerd::TagArrowLeft => '\u{f172d}',
        Nerd::TagArrowLeftOutline => '\u{f172e}',
        Nerd::TagArrowRight => '\u{f172f}',
        Nerd::TagArrowRightOutline => '\u{f1730}',
        Nerd::TagArrowUp => '\u{f1731}',
        Nerd::TagArrowUpOutline => '\u{f1732}',
        Nerd::TagCheck => '\u{f1a7a}',
        Nerd::TagCheckOutline => '\u{f1a7b}',
        Nerd::TagFaces => '\u{f04fa}',
        Nerd::TagHeart => '\u{f068b}',
        Nerd::TagHeartOutline => '\u{f0bcf}',
        Nerd::TagMinus => '\u{f0910}',
        Nerd::TagMinusOutline => '\u{f121f}',
        Nerd::TagMultiple => '\u{f04fb}',
        Nerd::TagMultipleOutline => '\u{f12f7}',
        Nerd::TagOff => '\u{f1220}',
        Nerd::TagOffOutline => '\u{f1221}',
        Nerd::TagOne => '\u{f02b}',
        Nerd::TagOutline => '\u{f04fc}',
        Nerd::TagPlus => '\u{f0722}',
        Nerd::TagPlusOutline => '\u{f1222}',
        Nerd::TagRemove => '\u{f0723}',
        Nerd::TagRemoveOutline => '\u{f1223}',
        Nerd::TagSearch => '\u{f1907}',
        Nerd::TagSearchOutline => '\u{f1908}',
        Nerd::TagText => '\u{f1224}',
        Nerd::TagTextOutline => '\u{f04fd}',
        Nerd::TagThree => '\u{f04f9}',
        Nerd::TagTwo => '\u{f412}',
        Nerd::Tags => '\u{f02c}',
        Nerd::Tails => '\u{f343}',
        Nerd::Tailwind => '\u{f13ff}',
        Nerd::TallyMarkFive => '\u{f1ac0}',
        Nerd::TallyMarkFour => '\u{f1abf}',
        Nerd::TallyMarkOne => '\u{f1abc}',
        Nerd::TallyMarkThree => '\u{f1abe}',
        Nerd::TallyMarkTwo => '\u{f1abd}',
        Nerd::Tangram => '\u{f04f8}',
        Nerd::Tank => '\u{f0d3a}',
        Nerd::TankerTruck => '\u{f0fe5}',
        Nerd::TapeDrive => '\u{f16df}',
        Nerd::TapeMeasure => '\u{f0b4d}',
        Nerd::Target => '\u{f04fe}',
        Nerd::TargetAccount => '\u{f0bd0}',
        Nerd::TargetVariant => '\u{f0a77}',
        Nerd::Tasklist => '\u{eb67}',
        Nerd::TasklistOne => '\u{f4a0}',
        Nerd::Tasks => '\u{f0ae}',
        Nerd::Taxi => '\u{f04ff}',
        Nerd::Tea => '\u{f0d9e}',
        Nerd::TeaOutline => '\u{f0d9f}',
        Nerd::Teamviewer => '\u{f0500}',
        Nerd::TeddyBear => '\u{f18fb}',
        Nerd::Telegram => '\u{e217}',
        Nerd::TelegramCircle => '\u{e215}',
        Nerd::Telescope => '\u{e209}',
        Nerd::TelescopeFill => '\u{f526}',
        Nerd::TelescopeOne => '\u{eb68}',
        Nerd::TelescopeThree => '\u{f0b4e}',
        Nerd::TelescopeTwo => '\u{f46b}',
        Nerd::Television => '\u{f0502}',
        Nerd::TelevisionAmbientLight => '\u{f1356}',
        Nerd::TelevisionBox => '\u{f0839}',
        Nerd::TelevisionClassic => '\u{f07f4}',
        Nerd::TelevisionClassicOff => '\u{f083a}',
        Nerd::TelevisionGuide => '\u{f0503}',
        Nerd::TelevisionOff => '\u{f083b}',
        Nerd::TelevisionPause => '\u{f0f89}',
        Nerd::TelevisionPlay => '\u{f0ecf}',
        Nerd::TelevisionShimmer => '\u{f1110}',
        Nerd::TelevisionStop => '\u{f0f8a}',
        Nerd::TemperatureCelsius => '\u{f0504}',
        Nerd::TemperatureFahrenheit => '\u{f0505}',
        Nerd::TemperatureKelvin => '\u{f0506}',
        Nerd::Tennis => '\u{f0da0}',
        Nerd::TennisBall => '\u{f0507}',
        Nerd::Tent => '\u{f0508}',
        Nerd::Terminal => '\u{ea85}',
        Nerd::TerminalBash => '\u{ebca}',
        Nerd::TerminalCmd => '\u{ebc4}',
        Nerd::TerminalDebian => '\u{ebc5}',
        Nerd::TerminalLinux => '\u{ebc6}',
        Nerd::TerminalOne => '\u{f120}',
        Nerd::TerminalPowershell => '\u{ebc7}',
        Nerd::TerminalTmux => '\u{ebc8}',
        Nerd::TerminalTwo => '\u{f489}',
        Nerd::TerminalUbuntu => '\u{ebc9}',
        Nerd::Terraform => '\u{f1062}',
        Nerd::TestTube => '\u{f0668}',
        Nerd::TestTubeEmpty => '\u{f0911}',
        Nerd::TestTubeOff => '\u{f0912}',
        Nerd::Text => '\u{f09a8}',
        Nerd::TextAccount => '\u{f1570}',
        Nerd::TextBox => '\u{f021a}',
        Nerd::TextBoxCheck => '\u{f0ea6}',
        Nerd::TextBoxCheckOutline => '\u{f0ea7}',
        Nerd::TextBoxEdit => '\u{f1a7c}',
        Nerd::TextBoxEditOutline => '\u{f1a7d}',
        Nerd::TextBoxMinus => '\u{f0ea8}',
        Nerd::TextBoxMinusOutline => '\u{f0ea9}',
        Nerd::TextBoxMultiple => '\u{f0ab7}',
        Nerd::TextBoxMultipleOutline => '\u{f0ab8}',
        Nerd::TextBoxOutline => '\u{f09ed}',
        Nerd::TextBoxPlus => '\u{f0eaa}',
        Nerd::TextBoxPlusOutline => '\u{f0eab}',
        Nerd::TextBoxRemove => '\u{f0eac}',
        Nerd::TextBoxRemoveOutline => '\u{f0ead}',
        Nerd::TextBoxSearch => '\u{f0eae}',
        Nerd::TextBoxSearchOutline => '\u{f0eaf}',
        Nerd::TextHeight => '\u{f034}',
        Nerd::TextLong => '\u{f09aa}',
        Nerd::TextRecognition => '\u{f113d}',
        Nerd::TextSearch => '\u{f13b8}',
        Nerd::TextSearchVariant => '\u{f1a7e}',
        Nerd::TextShadow => '\u{f0669}',
        Nerd::TextShort => '\u{f09a9}',
        Nerd::TextSize => '\u{eb69}',
        Nerd::TextToSpeech => '\u{f050a}',
        Nerd::TextToSpeechOff => '\u{f050b}',
        Nerd::TextWidth => '\u{f035}',
        Nerd::Texture => '\u{f050c}',
        Nerd::TextureBox => '\u{f0fe6}',
        Nerd::Th => '\u{f00a}',
        Nerd::ThLarge => '\u{f009}',
        Nerd::ThList => '\u{f00b}',
        Nerd::Theater => '\u{f050d}',
        Nerd::ThemeLightDark => '\u{f050e}',
        Nerd::Thermometer => '\u{e20a}',
        Nerd::ThermometerAlert => '\u{f0e01}',
        Nerd::ThermometerBluetooth => '\u{f1895}',
        Nerd::ThermometerCheck => '\u{f1a7f}',
        Nerd::ThermometerChevronDown => '\u{f0e02}',
        Nerd::ThermometerChevronUp => '\u{f0e03}',
        Nerd::ThermometerHigh => '\u{e20b}',
        Nerd::ThermometerHighOne => '\u{f10c2}',
        Nerd::ThermometerLines => '\u{f0510}',
        Nerd::ThermometerLow => '\u{e20c}',
        Nerd::ThermometerLowOne => '\u{f10c3}',
        Nerd::ThermometerMinus => '\u{f0e04}',
        Nerd::ThermometerOff => '\u{f1531}',
        Nerd::ThermometerOne => '\u{f050f}',
        Nerd::ThermometerPlus => '\u{f0e05}',
        Nerd::ThermometerWater => '\u{f1a80}',
        Nerd::Thermostat => '\u{f0393}',
        Nerd::ThermostatBox => '\u{f0891}',
        Nerd::ThinClose => '\u{e20d}',
        Nerd::ThoughtBubble => '\u{f07f6}',
        Nerd::ThoughtBubbleOutline => '\u{f07f7}',
        Nerd::ThreeBars => '\u{eb6a}',
        Nerd::ThreeBarsOne => '\u{f44e}',
        Nerd::Threeeighteight => '\u{f19d}',
        Nerd::Threeeightfive => '\u{f19a}',
        Nerd::Threeeightfour => '\u{f199}',
        Nerd::Threeeightnine => '\u{f19e}',
        Nerd::Threeeightseven => '\u{f19c}',
        Nerd::Threeeightsix => '\u{f19b}',
        Nerd::Threeeightthree => '\u{f198}',
        Nerd::Threeeighttwo => '\u{f197}',
        Nerd::Threeeightzero => '\u{f195}',
        Nerd::Threenineeight => '\u{f1a8}',
        Nerd::Threeninefive => '\u{f1a5}',
        Nerd::Threeninenine => '\u{f1a9}',
        Nerd::Threenineseven => '\u{f1a7}',
        Nerd::Threeninesix => '\u{f1a6}',
        Nerd::Threeninethree => '\u{f1a3}',
        Nerd::Threeninetwo => '\u{f1a2}',
        Nerd::Threeoneseven => '\u{f152}',
        Nerd::Threeonetwo => '\u{f14c}',
        Nerd::Threeseveneight => '\u{f193}',
        Nerd::Threesevenfour => '\u{f18e}',
        Nerd::Threesevensix => '\u{f191}',
        Nerd::Threeseventwo => '\u{f18c}',
        Nerd::Threesixsix => '\u{f186}',
        Nerd::Threethreefive => '\u{f165}',
        Nerd::Threethreefour => '\u{f164}',
        Nerd::Threetwonine => '\u{f15e}',
        Nerd::Threezerothree => '\u{f143}',
        Nerd::ThumbDown => '\u{f0511}',
        Nerd::ThumbDownOutline => '\u{f0512}',
        Nerd::ThumbUp => '\u{f0513}',
        Nerd::ThumbUpOutline => '\u{f0514}',
        Nerd::ThumbsDownAlt => '\u{f088}',
        Nerd::ThumbsUpAlt => '\u{f087}',
        Nerd::ThumbsUpDown => '\u{f0515}',
        Nerd::ThumbsUpDownOutline => '\u{f1914}',
        Nerd::Thumbsdown => '\u{eb6b}',
        Nerd::ThumbsdownOne => '\u{f497}',
        Nerd::Thumbsup => '\u{eb6c}',
        Nerd::ThumbsupOne => '\u{f496}',
        Nerd::Thunderbird => '\u{f370}',
        Nerd::Ticket => '\u{f145}',
        Nerd::TicketAccount => '\u{f0517}',
        Nerd::TicketConfirmation => '\u{f0518}',
        Nerd::TicketConfirmationOutline => '\u{f13aa}',
        Nerd::TicketOne => '\u{f0516}',
        Nerd::TicketOutline => '\u{f0913}',
        Nerd::TicketPercent => '\u{f0724}',
        Nerd::TicketPercentOutline => '\u{f142b}',
        Nerd::Tie => '\u{f0519}',
        Nerd::Tilde => '\u{f0725}',
        Nerd::TildeOff => '\u{f18f3}',
        Nerd::Time => '\u{f017}',
        Nerd::Timelapse => '\u{f051a}',
        Nerd::Timeline => '\u{f0bd1}',
        Nerd::TimelineAlert => '\u{f0f95}',
        Nerd::TimelineAlertOutline => '\u{f0f98}',
        Nerd::TimelineCheck => '\u{f1532}',
        Nerd::TimelineCheckOutline => '\u{f1533}',
        Nerd::TimelineClock => '\u{f11fb}',
        Nerd::TimelineClockOutline => '\u{f11fc}',
        Nerd::TimelineHelp => '\u{f0f99}',
        Nerd::TimelineHelpOutline => '\u{f0f9a}',
        Nerd::TimelineMinus => '\u{f1534}',
        Nerd::TimelineMinusOutline => '\u{f1535}',
        Nerd::TimelineOutline => '\u{f0bd2}',
        Nerd::TimelinePlus => '\u{f0f96}',
        Nerd::TimelinePlusOutline => '\u{f0f97}',
        Nerd::TimelineRemove => '\u{f1536}',
        Nerd::TimelineRemoveOutline => '\u{f1537}',
        Nerd::TimelineText => '\u{f0bd3}',
        Nerd::TimelineTextOutline => '\u{f0bd4}',
        Nerd::Timer => '\u{f13ab}',
        Nerd::TimerAlert => '\u{f1acc}',
        Nerd::TimerAlertOutline => '\u{f1acd}',
        Nerd::TimerCancel => '\u{f1ace}',
        Nerd::TimerCancelOutline => '\u{f1acf}',
        Nerd::TimerCheck => '\u{f1ad0}',
        Nerd::TimerCheckOutline => '\u{f1ad1}',
        Nerd::TimerCog => '\u{f1925}',
        Nerd::TimerCogOutline => '\u{f1926}',
        Nerd::TimerEdit => '\u{f1ad2}',
        Nerd::TimerEditOutline => '\u{f1ad3}',
        Nerd::TimerLock => '\u{f1ad4}',
        Nerd::TimerLockOpen => '\u{f1ad5}',
        Nerd::TimerLockOpenOutline => '\u{f1ad6}',
        Nerd::TimerLockOutline => '\u{f1ad7}',
        Nerd::TimerMarker => '\u{f1ad8}',
        Nerd::TimerMarkerOutline => '\u{f1ad9}',
        Nerd::TimerMinus => '\u{f1ada}',
        Nerd::TimerMinusOutline => '\u{f1adb}',
        Nerd::TimerMusic => '\u{f1adc}',
        Nerd::TimerMusicOutline => '\u{f1add}',
        Nerd::TimerOff => '\u{f13ac}',
        Nerd::TimerOffOutline => '\u{f051e}',
        Nerd::TimerOnezero => '\u{f051c}',
        Nerd::TimerOutline => '\u{f051b}',
        Nerd::TimerPause => '\u{f1ade}',
        Nerd::TimerPauseOutline => '\u{f1adf}',
        Nerd::TimerPlay => '\u{f1ae0}',
        Nerd::TimerPlayOutline => '\u{f1ae1}',
        Nerd::TimerPlus => '\u{f1ae2}',
        Nerd::TimerPlusOutline => '\u{f1ae3}',
        Nerd::TimerRefresh => '\u{f1ae4}',
        Nerd::TimerRefreshOutline => '\u{f1ae5}',
        Nerd::TimerRemove => '\u{f1ae6}',
        Nerd::TimerRemoveOutline => '\u{f1ae7}',
        Nerd::TimerSand => '\u{f051f}',
        Nerd::TimerSandComplete => '\u{f199f}',
        Nerd::TimerSandEmpty => '\u{f06ad}',
        Nerd::TimerSandFull => '\u{f078c}',
        Nerd::TimerSandPaused => '\u{f19a0}',
        Nerd::TimerSettings => '\u{f1923}',
        Nerd::TimerSettingsOutline => '\u{f1924}',
        Nerd::TimerStar => '\u{f1ae8}',
        Nerd::TimerStarOutline => '\u{f1ae9}',
        Nerd::TimerStop => '\u{f1aea}',
        Nerd::TimerStopOutline => '\u{f1aeb}',
        Nerd::TimerSync => '\u{f1aec}',
        Nerd::TimerSyncOutline => '\u{f1aed}',
        Nerd::TimerThree => '\u{f051d}',
        Nerd::Timetable => '\u{f0520}',
        Nerd::Tint => '\u{f043}',
        Nerd::Tire => '\u{f1896}',
        Nerd::Toaster => '\u{f1063}',
        Nerd::ToasterOff => '\u{f11b7}',
        Nerd::ToasterOven => '\u{f0cd3}',
        Nerd::ToggleSwitch => '\u{f0521}',
        Nerd::ToggleSwitchOff => '\u{f0522}',
        Nerd::ToggleSwitchOffOutline => '\u{f0a19}',
        Nerd::ToggleSwitchOutline => '\u{f0a1a}',
        Nerd::ToggleSwitchVariant => '\u{f1a25}',
        Nerd::ToggleSwitchVariantOff => '\u{f1a26}',
        Nerd::Toilet => '\u{e20e}',
        Nerd::ToiletOne => '\u{f09ab}',
        Nerd::Toolbox => '\u{f09ac}',
        Nerd::ToolboxOutline => '\u{f09ad}',
        Nerd::Tools => '\u{e20f}',
        Nerd::ToolsOne => '\u{eb6d}',
        Nerd::ToolsThree => '\u{f1064}',
        Nerd::ToolsTwo => '\u{f425}',
        Nerd::Tooltip => '\u{f0523}',
        Nerd::TooltipAccount => '\u{f000c}',
        Nerd::TooltipCellphone => '\u{f183b}',
        Nerd::TooltipCheck => '\u{f155c}',
        Nerd::TooltipCheckOutline => '\u{f155d}',
        Nerd::TooltipEdit => '\u{f0524}',
        Nerd::TooltipEditOutline => '\u{f12c5}',
        Nerd::TooltipImage => '\u{f0525}',
        Nerd::TooltipImageOutline => '\u{f0bd5}',
        Nerd::TooltipMinus => '\u{f155e}',
        Nerd::TooltipMinusOutline => '\u{f155f}',
        Nerd::TooltipOutline => '\u{f0526}',
        Nerd::TooltipPlus => '\u{f0bd6}',
        Nerd::TooltipPlusOutline => '\u{f0527}',
        Nerd::TooltipRemove => '\u{f1560}',
        Nerd::TooltipRemoveOutline => '\u{f1561}',
        Nerd::TooltipText => '\u{f0528}',
        Nerd::TooltipTextOutline => '\u{f0bd7}',
        Nerd::Tooth => '\u{e210}',
        Nerd::ToothOne => '\u{f08c3}',
        Nerd::ToothOutline => '\u{f0529}',
        Nerd::Toothbrush => '\u{f1129}',
        Nerd::ToothbrushElectric => '\u{f112c}',
        Nerd::ToothbrushPaste => '\u{f112a}',
        Nerd::TorBrowser => '\u{f371}',
        Nerd::Torch => '\u{f1606}',
        Nerd::Tortoise => '\u{f0d3b}',
        Nerd::Toslink => '\u{f12b8}',
        Nerd::Tournament => '\u{f09ae}',
        Nerd::TowTruck => '\u{f083c}',
        Nerd::TowerBeach => '\u{f0681}',
        Nerd::TowerFire => '\u{f0682}',
        Nerd::TownHall => '\u{f1875}',
        Nerd::ToyBrick => '\u{f1288}',
        Nerd::ToyBrickMarker => '\u{f1289}',
        Nerd::ToyBrickMarkerOutline => '\u{f128a}',
        Nerd::ToyBrickMinus => '\u{f128b}',
        Nerd::ToyBrickMinusOutline => '\u{f128c}',
        Nerd::ToyBrickOutline => '\u{f128d}',
        Nerd::ToyBrickPlus => '\u{f128e}',
        Nerd::ToyBrickPlusOutline => '\u{f128f}',
        Nerd::ToyBrickRemove => '\u{f1290}',
        Nerd::ToyBrickRemoveOutline => '\u{f1291}',
        Nerd::ToyBrickSearch => '\u{f1292}',
        Nerd::ToyBrickSearchOutline => '\u{f1293}',
        Nerd::TrackLight => '\u{f0914}',
        Nerd::Trackpad => '\u{f07f8}',
        Nerd::TrackpadLock => '\u{f0933}',
        Nerd::Tractor => '\u{f0892}',
        Nerd::TractorVariant => '\u{f14c4}',
        Nerd::Trademark => '\u{f0a78}',
        Nerd::TrafficCone => '\u{f137c}',
        Nerd::TrafficLight => '\u{f052b}',
        Nerd::TrafficLightOutline => '\u{f182a}',
        Nerd::Train => '\u{f052c}',
        Nerd::TrainCar => '\u{f0bd8}',
        Nerd::TrainCarPassenger => '\u{f1733}',
        Nerd::TrainCarPassengerDoor => '\u{f1734}',
        Nerd::TrainCarPassengerDoorOpen => '\u{f1735}',
        Nerd::TrainCarPassengerVariant => '\u{f1736}',
        Nerd::TrainVariant => '\u{f08c4}',
        Nerd::Tram => '\u{f052d}',
        Nerd::TramSide => '\u{f0fe7}',
        Nerd::Transcribe => '\u{f052e}',
        Nerd::TranscribeClose => '\u{f052f}',
        Nerd::Transfer => '\u{f1065}',
        Nerd::TransferDown => '\u{f0da1}',
        Nerd::TransferLeft => '\u{f0da2}',
        Nerd::TransferRight => '\u{f0530}',
        Nerd::TransferUp => '\u{f0da3}',
        Nerd::TransitConnection => '\u{f0d3c}',
        Nerd::TransitConnectionHorizontal => '\u{f1546}',
        Nerd::TransitConnectionVariant => '\u{f0d3d}',
        Nerd::TransitDetour => '\u{f0f8b}',
        Nerd::TransitSkip => '\u{f1515}',
        Nerd::TransitTransfer => '\u{f06ae}',
        Nerd::Transition => '\u{f0915}',
        Nerd::TransitionMasked => '\u{f0916}',
        Nerd::Translate => '\u{f05ca}',
        Nerd::TranslateOff => '\u{f0e06}',
        Nerd::TransmissionTower => '\u{f0d3e}',
        Nerd::TransmissionTowerExport => '\u{f192c}',
        Nerd::TransmissionTowerImport => '\u{f192d}',
        Nerd::TransmissionTowerOff => '\u{f19dd}',
        Nerd::Trash => '\u{ea81}',
        Nerd::TrashCan => '\u{f0a79}',
        Nerd::TrashCanOutline => '\u{f0a7a}',
        Nerd::TrashOne => '\u{f014}',
        Nerd::TrashTwo => '\u{f48e}',
        Nerd::Tray => '\u{f1294}',
        Nerd::TrayAlert => '\u{f1295}',
        Nerd::TrayArrowDown => '\u{f0120}',
        Nerd::TrayArrowUp => '\u{f011d}',
        Nerd::TrayFull => '\u{f1296}',
        Nerd::TrayMinus => '\u{f1297}',
        Nerd::TrayPlus => '\u{f1298}',
        Nerd::TrayRemove => '\u{f1299}',
        Nerd::TreasureChest => '\u{f0726}',
        Nerd::Tree => '\u{e21c}',
        Nerd::TreeOne => '\u{f0531}',
        Nerd::TreeOutline => '\u{f0e69}',
        Nerd::Trello => '\u{f181}',
        Nerd::TrelloOne => '\u{f0532}',
        Nerd::TrendingDown => '\u{f0533}',
        Nerd::TrendingNeutral => '\u{f0534}',
        Nerd::TrendingUp => '\u{f0535}',
        Nerd::Triangle => '\u{f0536}',
        Nerd::TriangleDown => '\u{eb6e}',
        Nerd::TriangleDownOne => '\u{f44b}',
        Nerd::TriangleLeft => '\u{eb6f}',
        Nerd::TriangleLeftOne => '\u{f438}',
        Nerd::TriangleOutline => '\u{f0537}',
        Nerd::TriangleRight => '\u{eb70}',
        Nerd::TriangleRightOne => '\u{f44a}',
        Nerd::TriangleRuler => '\u{e21b}',
        Nerd::TriangleSmallDown => '\u{f1a09}',
        Nerd::TriangleSmallUp => '\u{f1a0a}',
        Nerd::TriangleUp => '\u{eb71}',
        Nerd::TriangleUpOne => '\u{f47e}',
        Nerd::TriangleWave => '\u{f147c}',
        Nerd::Triforce => '\u{f0bd9}',
        Nerd::TrisquelGnuLinux => '\u{f344}',
        Nerd::Trophy => '\u{f091}',
        Nerd::TrophyAward => '\u{f0539}',
        Nerd::TrophyBroken => '\u{f0da4}',
        Nerd::TrophyOne => '\u{f527}',
        Nerd::TrophyOutline => '\u{f053a}',
        Nerd::TrophyTwo => '\u{f0538}',
        Nerd::TrophyVariant => '\u{f053b}',
        Nerd::TrophyVariantOutline => '\u{f053c}',
        Nerd::Truck => '\u{f0d1}',
        Nerd::TruckAlert => '\u{f19de}',
        Nerd::TruckAlertOutline => '\u{f19df}',
        Nerd::TruckCargoContainer => '\u{f18d8}',
        Nerd::TruckCheck => '\u{f0cd4}',
        Nerd::TruckCheckOutline => '\u{f129a}',
        Nerd::TruckDelivery => '\u{f053e}',
        Nerd::TruckDeliveryOutline => '\u{f129b}',
        Nerd::TruckFast => '\u{f0788}',
        Nerd::TruckFastOutline => '\u{f129c}',
        Nerd::TruckFlatbed => '\u{f1891}',
        Nerd::TruckMinus => '\u{f19ae}',
        Nerd::TruckMinusOutline => '\u{f19bd}',
        Nerd::TruckOne => '\u{f053d}',
        Nerd::TruckOutline => '\u{f129d}',
        Nerd::TruckPlus => '\u{f19ad}',
        Nerd::TruckPlusOutline => '\u{f19bc}',
        Nerd::TruckRemove => '\u{f19af}',
        Nerd::TruckRemoveOutline => '\u{f19be}',
        Nerd::TruckSnowflake => '\u{f19a6}',
        Nerd::TruckTrailer => '\u{f0727}',
        Nerd::Trumpet => '\u{f1096}',
        Nerd::TshirtCrew => '\u{f0a7b}',
        Nerd::TshirtCrewOutline => '\u{f053f}',
        Nerd::TshirtV => '\u{f0a7c}',
        Nerd::TshirtVOutline => '\u{f0540}',
        Nerd::Tsunami => '\u{f1a81}',
        Nerd::TumbleDryer => '\u{f0917}',
        Nerd::TumbleDryerAlert => '\u{f11ba}',
        Nerd::TumbleDryerOff => '\u{f11bb}',
        Nerd::Tumblr => '\u{f173}',
        Nerd::TumblrSign => '\u{f174}',
        Nerd::Tune => '\u{f062e}',
        Nerd::TuneVariant => '\u{f1542}',
        Nerd::TuneVertical => '\u{f066a}',
        Nerd::TuneVerticalVariant => '\u{f1543}',
        Nerd::Tunnel => '\u{f183d}',
        Nerd::TunnelOutline => '\u{f183e}',
        Nerd::Turbine => '\u{f1a82}',
        Nerd::Turkey => '\u{f171b}',
        Nerd::Turnstile => '\u{f0cd5}',
        Nerd::TurnstileOutline => '\u{f0cd6}',
        Nerd::Turtle => '\u{f0cd7}',
        Nerd::Tux => '\u{f31a}',
        Nerd::Twitch => '\u{f0543}',
        Nerd::Twitter => '\u{eb72}',
        Nerd::TwitterOne => '\u{f099}',
        Nerd::TwitterSign => '\u{f081}',
        Nerd::TwitterTwo => '\u{f0544}',
        Nerd::TwoFactorAuthentication => '\u{f09af}',
        Nerd::Twoeightthree => '\u{f12d}',
        Nerd::Twosevennine => '\u{f129}',
        Nerd::TypeHierarchy => '\u{ebb9}',
        Nerd::TypeHierarchySub => '\u{ebba}',
        Nerd::TypeHierarchySuper => '\u{ebbb}',
        Nerd::Typewriter => '\u{f0f2d}',
        Nerd::Typography => '\u{f528}',
        Nerd::Ubisoft => '\u{f0bda}',
        Nerd::Ubuntu => '\u{f0548}',
        Nerd::UbuntuInverse => '\u{f31c}',
        Nerd::Ufo => '\u{f10c4}',
        Nerd::UfoOutline => '\u{f10c5}',
        Nerd::Ul => '\u{f0ca}',
        Nerd::UltraHighDefinition => '\u{f07f9}',
        Nerd::Umbraco => '\u{f0549}',
        Nerd::Umbrella => '\u{e220}',
        Nerd::UmbrellaBeach => '\u{f188a}',
        Nerd::UmbrellaBeachOutline => '\u{f188b}',
        Nerd::UmbrellaClosed => '\u{f09b0}',
        Nerd::UmbrellaClosedOutline => '\u{f13e2}',
        Nerd::UmbrellaClosedVariant => '\u{f13e1}',
        Nerd::UmbrellaOne => '\u{f0e9}',
        Nerd::UmbrellaOutline => '\u{f054b}',
        Nerd::UmbrellaTwo => '\u{f054a}',
        Nerd::Underline => '\u{f0cd}',
        Nerd::Undo => '\u{f0e2}',
        Nerd::UndoOne => '\u{f054c}',
        Nerd::UndoVariant => '\u{f054d}',
        Nerd::Unfold => '\u{eb73}',
        Nerd::UnfoldLessHorizontal => '\u{f054e}',
        Nerd::UnfoldLessVertical => '\u{f0760}',
        Nerd::UnfoldMoreHorizontal => '\u{f054f}',
        Nerd::UnfoldMoreVertical => '\u{f0761}',
        Nerd::UnfoldOne => '\u{f42d}',
        Nerd::Ungroup => '\u{f0550}',
        Nerd::UngroupByRefType => '\u{eb98}',
        Nerd::Unicode => '\u{f0ed0}',
        Nerd::Unicorn => '\u{f15c2}',
        Nerd::UnicornVariant => '\u{f15c3}',
        Nerd::Unicycle => '\u{f15e5}',
        Nerd::Unity => '\u{f06af}',
        Nerd::Unlink => '\u{f127}',
        Nerd::UnlinkOne => '\u{f529}',
        Nerd::Unlock => '\u{eb74}',
        Nerd::UnlockAlt => '\u{f13e}',
        Nerd::UnlockOne => '\u{f09c}',
        Nerd::UnlockTwo => '\u{f52a}',
        Nerd::Unmute => '\u{eb75}',
        Nerd::UnmuteOne => '\u{f485}',
        Nerd::Unread => '\u{f52b}',
        Nerd::Unreal => '\u{f09b1}',
        Nerd::Unverified => '\u{eb76}',
        Nerd::UnverifiedOne => '\u{f4a3}',
        Nerd::Update => '\u{f06b0}',
        Nerd::Upload => '\u{f01b}',
        Nerd::UploadAlt => '\u{f093}',
        Nerd::UploadLock => '\u{f1373}',
        Nerd::UploadLockOutline => '\u{f1374}',
        Nerd::UploadMultiple => '\u{f083d}',
        Nerd::UploadNetwork => '\u{f06f6}',
        Nerd::UploadNetworkOutline => '\u{f0cd8}',
        Nerd::UploadOff => '\u{f10c6}',
        Nerd::UploadOffOutline => '\u{f10c7}',
        Nerd::UploadOne => '\u{f40a}',
        Nerd::UploadOutline => '\u{f0e07}',
        Nerd::UploadTwo => '\u{f0552}',
        Nerd::Usb => '\u{f0553}',
        Nerd::UsbFlashDrive => '\u{f129e}',
        Nerd::UsbFlashDriveOutline => '\u{f129f}',
        Nerd::UsbPort => '\u{f11f0}',
        Nerd::Usd => '\u{f155}',
        Nerd::User => '\u{f007}',
        Nerd::UserMd => '\u{f0f0}',
        Nerd::Uterus => '\u{e211}',
        Nerd::Vacuum => '\u{f19a1}',
        Nerd::VacuumOutline => '\u{f19a2}',
        Nerd::Valve => '\u{f1066}',
        Nerd::ValveClosed => '\u{f1067}',
        Nerd::ValveOpen => '\u{f1068}',
        Nerd::VanPassenger => '\u{f07fa}',
        Nerd::VanUtility => '\u{f07fb}',
        Nerd::VanillaOs => '\u{f366}',
        Nerd::Vanish => '\u{f07fc}',
        Nerd::VanishQuarter => '\u{f1554}',
        Nerd::VanityLight => '\u{f11e1}',
        Nerd::Variable => '\u{f0ae7}',
        Nerd::VariableBox => '\u{f1111}',
        Nerd::VariableGroup => '\u{ebb8}',
        Nerd::VectorArrangeAbove => '\u{f0554}',
        Nerd::VectorArrangeBelow => '\u{f0555}',
        Nerd::VectorBezier => '\u{f0ae8}',
        Nerd::VectorCircle => '\u{f0556}',
        Nerd::VectorCircleVariant => '\u{f0557}',
        Nerd::VectorCombine => '\u{f0558}',
        Nerd::VectorCurve => '\u{f0559}',
        Nerd::VectorDifference => '\u{f055a}',
        Nerd::VectorDifferenceAb => '\u{f055b}',
        Nerd::VectorDifferenceBa => '\u{f055c}',
        Nerd::VectorEllipse => '\u{f0893}',
        Nerd::VectorIntersection => '\u{f055d}',
        Nerd::VectorLine => '\u{f055e}',
        Nerd::VectorLink => '\u{f0fe8}',
        Nerd::VectorPoint => '\u{f055f}',
        Nerd::VectorPolygon => '\u{f0560}',
        Nerd::VectorPolygonVariant => '\u{f1856}',
        Nerd::VectorPolyline => '\u{f0561}',
        Nerd::VectorPolylineEdit => '\u{f1225}',
        Nerd::VectorPolylineMinus => '\u{f1226}',
        Nerd::VectorPolylinePlus => '\u{f1227}',
        Nerd::VectorPolylineRemove => '\u{f1228}',
        Nerd::VectorRadius => '\u{f074a}',
        Nerd::VectorRectangle => '\u{f05c6}',
        Nerd::VectorSelection => '\u{f0562}',
        Nerd::VectorSquare => '\u{f0001}',
        Nerd::VectorSquareClose => '\u{f1857}',
        Nerd::VectorSquareEdit => '\u{f18d9}',
        Nerd::VectorSquareMinus => '\u{f18da}',
        Nerd::VectorSquareOpen => '\u{f1858}',
        Nerd::VectorSquarePlus => '\u{f18db}',
        Nerd::VectorSquareRemove => '\u{f18dc}',
        Nerd::VectorTriangle => '\u{f0563}',
        Nerd::VectorUnion => '\u{f0564}',
        Nerd::Venus => '\u{f221}',
        Nerd::Verified => '\u{eb77}',
        Nerd::VerifiedFilled => '\u{ebe9}',
        Nerd::VerifiedOne => '\u{f4a1}',
        Nerd::Versions => '\u{eb78}',
        Nerd::VersionsOne => '\u{f454}',
        Nerd::Vhs => '\u{f0a1b}',
        Nerd::Vibrate => '\u{f0566}',
        Nerd::VibrateOff => '\u{f0cd9}',
        Nerd::Video => '\u{f52c}',
        Nerd::VideoAccount => '\u{f0919}',
        Nerd::VideoBox => '\u{f00fd}',
        Nerd::VideoBoxOff => '\u{f00fe}',
        Nerd::VideoCheck => '\u{f1069}',
        Nerd::VideoCheckOutline => '\u{f106a}',
        Nerd::VideoFourkBox => '\u{f083e}',
        Nerd::VideoHighDefinition => '\u{f152e}',
        Nerd::VideoImage => '\u{f091a}',
        Nerd::VideoInputAntenna => '\u{f083f}',
        Nerd::VideoInputComponent => '\u{f0840}',
        Nerd::VideoInputHdmi => '\u{f0841}',
        Nerd::VideoInputScart => '\u{f0f8c}',
        Nerd::VideoInputSvideo => '\u{f0842}',
        Nerd::VideoMarker => '\u{f19a9}',
        Nerd::VideoMarkerOutline => '\u{f19aa}',
        Nerd::VideoMinus => '\u{f09b2}',
        Nerd::VideoMinusOutline => '\u{f02ba}',
        Nerd::VideoOff => '\u{f0568}',
        Nerd::VideoOffOutline => '\u{f0bdb}',
        Nerd::VideoOne => '\u{f0567}',
        Nerd::VideoOutline => '\u{f0bdc}',
        Nerd::VideoPlus => '\u{f09b3}',
        Nerd::VideoPlusOutline => '\u{f01d3}',
        Nerd::VideoStabilization => '\u{f091b}',
        Nerd::VideoSwitch => '\u{f0569}',
        Nerd::VideoSwitchOutline => '\u{f0790}',
        Nerd::VideoThreed => '\u{f07fd}',
        Nerd::VideoThreedOff => '\u{f13d9}',
        Nerd::VideoThreedVariant => '\u{f0ed1}',
        Nerd::VideoTwod => '\u{f1a1c}',
        Nerd::VideoVintage => '\u{f0a1c}',
        Nerd::VideoWireless => '\u{f0ed2}',
        Nerd::VideoWirelessOutline => '\u{f0ed3}',
        Nerd::ViewAgenda => '\u{f056a}',
        Nerd::ViewAgendaOutline => '\u{f11d8}',
        Nerd::ViewArray => '\u{f056b}',
        Nerd::ViewArrayOutline => '\u{f1485}',
        Nerd::ViewCarousel => '\u{f056c}',
        Nerd::ViewCarouselOutline => '\u{f1486}',
        Nerd::ViewColumn => '\u{f056d}',
        Nerd::ViewColumnOutline => '\u{f1487}',
        Nerd::ViewComfy => '\u{f0e6a}',
        Nerd::ViewComfyOutline => '\u{f1488}',
        Nerd::ViewCompact => '\u{f0e6b}',
        Nerd::ViewCompactOutline => '\u{f0e6c}',
        Nerd::ViewDashboard => '\u{f056e}',
        Nerd::ViewDashboardEdit => '\u{f1947}',
        Nerd::ViewDashboardEditOutline => '\u{f1948}',
        Nerd::ViewDashboardOutline => '\u{f0a1d}',
        Nerd::ViewDashboardVariant => '\u{f0843}',
        Nerd::ViewDashboardVariantOutline => '\u{f1489}',
        Nerd::ViewDay => '\u{f056f}',
        Nerd::ViewDayOutline => '\u{f148a}',
        Nerd::ViewGallery => '\u{f1888}',
        Nerd::ViewGalleryOutline => '\u{f1889}',
        Nerd::ViewGrid => '\u{f0570}',
        Nerd::ViewGridOutline => '\u{f11d9}',
        Nerd::ViewGridPlus => '\u{f0f8d}',
        Nerd::ViewGridPlusOutline => '\u{f11da}',
        Nerd::ViewHeadline => '\u{f0571}',
        Nerd::ViewList => '\u{f0572}',
        Nerd::ViewListOutline => '\u{f148b}',
        Nerd::ViewModule => '\u{f0573}',
        Nerd::ViewModuleOutline => '\u{f148c}',
        Nerd::ViewParallel => '\u{f0728}',
        Nerd::ViewParallelOutline => '\u{f148d}',
        Nerd::ViewQuilt => '\u{f0574}',
        Nerd::ViewQuiltOutline => '\u{f148e}',
        Nerd::ViewSequential => '\u{f0729}',
        Nerd::ViewSequentialOutline => '\u{f148f}',
        Nerd::ViewSplitHorizontal => '\u{f0bcb}',
        Nerd::ViewSplitVertical => '\u{f0bcc}',
        Nerd::ViewStream => '\u{f0575}',
        Nerd::ViewStreamOutline => '\u{f1490}',
        Nerd::ViewWeek => '\u{f0576}',
        Nerd::ViewWeekOutline => '\u{f1491}',
        Nerd::Vimeo => '\u{f0577}',
        Nerd::VimeoSquare => '\u{f194}',
        Nerd::Violin => '\u{f060f}',
        Nerd::VirtualReality => '\u{f0894}',
        Nerd::Virus => '\u{e214}',
        Nerd::VirusOff => '\u{f18e1}',
        Nerd::VirusOffOutline => '\u{f18e2}',
        Nerd::VirusOne => '\u{f13b6}',
        Nerd::VirusOutline => '\u{f13b7}',
        Nerd::Vk => '\u{f189}',
        Nerd::Vlc => '\u{f057c}',
        Nerd::Vm => '\u{ea7a}',
        Nerd::VmActive => '\u{eb79}',
        Nerd::VmConnect => '\u{eba9}',
        Nerd::VmOutline => '\u{eb7a}',
        Nerd::VmRunning => '\u{eb7b}',
        Nerd::Voicemail => '\u{f057d}',
        Nerd::Void => '\u{f32e}',
        Nerd::Volcano => '\u{f1a83}',
        Nerd::VolcanoOutline => '\u{f1a84}',
        Nerd::Volleyball => '\u{f09b4}',
        Nerd::VolumeDown => '\u{f027}',
        Nerd::VolumeHigh => '\u{f057e}',
        Nerd::VolumeLow => '\u{f057f}',
        Nerd::VolumeMedium => '\u{f0580}',
        Nerd::VolumeMinus => '\u{f075e}',
        Nerd::VolumeMute => '\u{f075f}',
        Nerd::VolumeOff => '\u{f0581}',
        Nerd::VolumePlus => '\u{f075d}',
        Nerd::VolumeSource => '\u{f1120}',
        Nerd::VolumeUp => '\u{f028}',
        Nerd::VolumeVariantOff => '\u{f0e08}',
        Nerd::VolumeVibrate => '\u{f1121}',
        Nerd::Vote => '\u{f0a1f}',
        Nerd::VoteOutline => '\u{f0a20}',
        Nerd::Vpn => '\u{f0582}',
        Nerd::VsCodium => '\u{f372}',
        Nerd::Vuejs => '\u{f0844}',
        Nerd::Vuetify => '\u{f0e6d}',
        Nerd::Walk => '\u{f0583}',
        Nerd::Walking => '\u{e213}',
        Nerd::Wall => '\u{f07fe}',
        Nerd::WallFire => '\u{f1a11}',
        Nerd::WallSconce => '\u{f091c}',
        Nerd::WallSconceFlat => '\u{f091d}',
        Nerd::WallSconceFlatOutline => '\u{f17c9}',
        Nerd::WallSconceFlatVariant => '\u{f041c}',
        Nerd::WallSconceFlatVariantOutline => '\u{f17ca}',
        Nerd::WallSconceOutline => '\u{f17cb}',
        Nerd::WallSconceRound => '\u{f0748}',
        Nerd::WallSconceRoundOutline => '\u{f17cc}',
        Nerd::WallSconceRoundVariant => '\u{f091e}',
        Nerd::WallSconceRoundVariantOutline => '\u{f17cd}',
        Nerd::Wallet => '\u{e25e}',
        Nerd::WalletGiftcard => '\u{f0585}',
        Nerd::WalletMembership => '\u{f0586}',
        Nerd::WalletOne => '\u{f0584}',
        Nerd::WalletOutline => '\u{f0bdd}',
        Nerd::WalletPlus => '\u{f0f8e}',
        Nerd::WalletPlusOutline => '\u{f0f8f}',
        Nerd::WalletTravel => '\u{f0587}',
        Nerd::Wallpaper => '\u{f0e09}',
        Nerd::Wan => '\u{f0588}',
        Nerd::Wand => '\u{ebcf}',
        Nerd::Wardrobe => '\u{f0f90}',
        Nerd::WardrobeOutline => '\u{f0f91}',
        Nerd::Warehouse => '\u{f0f81}',
        Nerd::Warning => '\u{ea6c}',
        Nerd::WarningSign => '\u{f071}',
        Nerd::WashingMachine => '\u{f072a}',
        Nerd::WashingMachineAlert => '\u{f11bc}',
        Nerd::WashingMachineOff => '\u{f11bd}',
        Nerd::Watch => '\u{eb7c}',
        Nerd::WatchExport => '\u{f058a}',
        Nerd::WatchExportVariant => '\u{f0895}',
        Nerd::WatchImport => '\u{f058b}',
        Nerd::WatchImportVariant => '\u{f0896}',
        Nerd::WatchOne => '\u{f0589}',
        Nerd::WatchVariant => '\u{f0897}',
        Nerd::WatchVibrate => '\u{f06b1}',
        Nerd::WatchVibrateOff => '\u{f0cda}',
        Nerd::Water => '\u{f058c}',
        Nerd::WaterAlert => '\u{f1502}',
        Nerd::WaterAlertOutline => '\u{f1503}',
        Nerd::WaterBoiler => '\u{f0f92}',
        Nerd::WaterBoilerAlert => '\u{f11b3}',
        Nerd::WaterBoilerOff => '\u{f11b4}',
        Nerd::WaterCheck => '\u{f1504}',
        Nerd::WaterCheckOutline => '\u{f1505}',
        Nerd::WaterCircle => '\u{f1806}',
        Nerd::WaterMinus => '\u{f1506}',
        Nerd::WaterMinusOutline => '\u{f1507}',
        Nerd::WaterOff => '\u{f058d}',
        Nerd::WaterOffOutline => '\u{f1508}',
        Nerd::WaterOpacity => '\u{f1855}',
        Nerd::WaterOutline => '\u{f0e0a}',
        Nerd::WaterPercent => '\u{f058e}',
        Nerd::WaterPercentAlert => '\u{f1509}',
        Nerd::WaterPlus => '\u{f150a}',
        Nerd::WaterPlusOutline => '\u{f150b}',
        Nerd::WaterPolo => '\u{f12a0}',
        Nerd::WaterPump => '\u{f058f}',
        Nerd::WaterPumpOff => '\u{f0f93}',
        Nerd::WaterRemove => '\u{f150c}',
        Nerd::WaterRemoveOutline => '\u{f150d}',
        Nerd::WaterSync => '\u{f17c6}',
        Nerd::WaterThermometer => '\u{f1a85}',
        Nerd::WaterThermometerOutline => '\u{f1a86}',
        Nerd::WaterWell => '\u{f106b}',
        Nerd::WaterWellOutline => '\u{f106c}',
        Nerd::Waterfall => '\u{f1849}',
        Nerd::WateringCan => '\u{f1481}',
        Nerd::WateringCanOutline => '\u{f1482}',
        Nerd::Watermark => '\u{f0612}',
        Nerd::Wave => '\u{f0f2e}',
        Nerd::Waveform => '\u{f147d}',
        Nerd::Waves => '\u{f078d}',
        Nerd::WavesArrowLeft => '\u{f1859}',
        Nerd::WavesArrowRight => '\u{f185a}',
        Nerd::WavesArrowUp => '\u{f185b}',
        Nerd::Wayland => '\u{f367}',
        Nerd::Waze => '\u{f0bde}',
        Nerd::WeatherCloudy => '\u{f0590}',
        Nerd::WeatherCloudyAlert => '\u{f0f2f}',
        Nerd::WeatherCloudyArrowRight => '\u{f0e6e}',
        Nerd::WeatherCloudyClock => '\u{f18f6}',
        Nerd::WeatherFog => '\u{f0591}',
        Nerd::WeatherHail => '\u{f0592}',
        Nerd::WeatherHazy => '\u{f0f30}',
        Nerd::WeatherHurricane => '\u{f0898}',
        Nerd::WeatherLightning => '\u{f0593}',
        Nerd::WeatherLightningRainy => '\u{f067e}',
        Nerd::WeatherNight => '\u{f0594}',
        Nerd::WeatherNightPartlyCloudy => '\u{f0f31}',
        Nerd::WeatherPartlyCloudy => '\u{f0595}',
        Nerd::WeatherPartlyLightning => '\u{f0f32}',
        Nerd::WeatherPartlyRainy => '\u{f0f33}',
        Nerd::WeatherPartlySnowy => '\u{f0f34}',
        Nerd::WeatherPartlySnowyRainy => '\u{f0f35}',
        Nerd::WeatherPouring => '\u{f0596}',
        Nerd::WeatherRainy => '\u{f0597}',
        Nerd::WeatherSnowy => '\u{f0598}',
        Nerd::WeatherSnowyHeavy => '\u{f0f36}',
        Nerd::WeatherSnowyRainy => '\u{f067f}',
        Nerd::WeatherSunny => '\u{f0599}',
        Nerd::WeatherSunnyAlert => '\u{f0f37}',
        Nerd::WeatherSunnyOff => '\u{f14e4}',
        Nerd::WeatherSunset => '\u{f059a}',
        Nerd::WeatherSunsetDown => '\u{f059b}',
        Nerd::WeatherSunsetUp => '\u{f059c}',
        Nerd::WeatherTornado => '\u{f0f38}',
        Nerd::WeatherWindy => '\u{f059d}',
        Nerd::WeatherWindyVariant => '\u{f059e}',
        Nerd::Web => '\u{f059f}',
        Nerd::WebBox => '\u{f0f94}',
        Nerd::WebCancel => '\u{f1790}',
        Nerd::WebCheck => '\u{f0789}',
        Nerd::WebClock => '\u{f124a}',
        Nerd::WebMinus => '\u{f10a0}',
        Nerd::WebOff => '\u{f0a8e}',
        Nerd::WebPlus => '\u{f0033}',
        Nerd::WebRefresh => '\u{f1791}',
        Nerd::WebRemove => '\u{f0551}',
        Nerd::WebSync => '\u{f1792}',
        Nerd::Webcam => '\u{f05a0}',
        Nerd::WebcamOff => '\u{f1737}',
        Nerd::Webhook => '\u{f52d}',
        Nerd::WebhookOne => '\u{f062f}',
        Nerd::Webpack => '\u{f072b}',
        Nerd::Webrtc => '\u{f1248}',
        Nerd::Wechat => '\u{f0611}',
        Nerd::Weibo => '\u{f18a}',
        Nerd::Weight => '\u{f05a1}',
        Nerd::WeightGram => '\u{f0d3f}',
        Nerd::WeightKilogram => '\u{f05a2}',
        Nerd::WeightLifter => '\u{f115d}',
        Nerd::WeightPound => '\u{f09b5}',
        Nerd::Whatsapp => '\u{f05a3}',
        Nerd::WheelBarrow => '\u{f14f2}',
        Nerd::Wheelchair => '\u{f1a87}',
        Nerd::WheelchairAccessibility => '\u{f05a4}',
        Nerd::Whistle => '\u{f09b6}',
        Nerd::WhistleOutline => '\u{f12bc}',
        Nerd::WhiteBalanceAuto => '\u{f05a5}',
        Nerd::WhiteBalanceIncandescent => '\u{f05a6}',
        Nerd::WhiteBalanceIridescent => '\u{f05a7}',
        Nerd::WhiteBalanceSunny => '\u{f05a8}',
        Nerd::Whitespace => '\u{eb7d}',
        Nerd::WholeWord => '\u{eb7e}',
        Nerd::Widgets => '\u{f072c}',
        Nerd::WidgetsOutline => '\u{f1355}',
        Nerd::Wifi => '\u{f05a9}',
        Nerd::WifiAlert => '\u{f16b5}',
        Nerd::WifiArrowDown => '\u{f16b6}',
        Nerd::WifiArrowLeft => '\u{f16b7}',
        Nerd::WifiArrowLeftRight => '\u{f16b8}',
        Nerd::WifiArrowRight => '\u{f16b9}',
        Nerd::WifiArrowUp => '\u{f16ba}',
        Nerd::WifiArrowUpDown => '\u{f16bb}',
        Nerd::WifiCancel => '\u{f16bc}',
        Nerd::WifiCheck => '\u{f16bd}',
        Nerd::WifiCog => '\u{f16be}',
        Nerd::WifiLock => '\u{f16bf}',
        Nerd::WifiLockOpen => '\u{f16c0}',
        Nerd::WifiMarker => '\u{f16c1}',
        Nerd::WifiMinus => '\u{f16c2}',
        Nerd::WifiOff => '\u{f05aa}',
        Nerd::WifiPlus => '\u{f16c3}',
        Nerd::WifiRefresh => '\u{f16c4}',
        Nerd::WifiRemove => '\u{f16c5}',
        Nerd::WifiSettings => '\u{f16c6}',
        Nerd::WifiStar => '\u{f0e0b}',
        Nerd::WifiStrengthAlertOutline => '\u{f092b}',
        Nerd::WifiStrengthFour => '\u{f0928}',
        Nerd::WifiStrengthFourAlert => '\u{f0929}',
        Nerd::WifiStrengthFourLock => '\u{f092a}',
        Nerd::WifiStrengthFourLockOpen => '\u{f16ce}',
        Nerd::WifiStrengthLockOpenOutline => '\u{f16cf}',
        Nerd::WifiStrengthLockOutline => '\u{f092c}',
        Nerd::WifiStrengthOff => '\u{f092d}',
        Nerd::WifiStrengthOffOutline => '\u{f092e}',
        Nerd::WifiStrengthOne => '\u{f091f}',
        Nerd::WifiStrengthOneAlert => '\u{f0920}',
        Nerd::WifiStrengthOneLock => '\u{f0921}',
        Nerd::WifiStrengthOneLockOpen => '\u{f16cb}',
        Nerd::WifiStrengthOutline => '\u{f092f}',
        Nerd::WifiStrengthThree => '\u{f0925}',
        Nerd::WifiStrengthThreeAlert => '\u{f0926}',
        Nerd::WifiStrengthThreeLock => '\u{f0927}',
        Nerd::WifiStrengthThreeLockOpen => '\u{f16cd}',
        Nerd::WifiStrengthTwo => '\u{f0922}',
        Nerd::WifiStrengthTwoAlert => '\u{f0923}',
        Nerd::WifiStrengthTwoLock => '\u{f0924}',
        Nerd::WifiStrengthTwoLockOpen => '\u{f16cc}',
        Nerd::WifiSync => '\u{f16c7}',
        Nerd::Wikimedia => '\u{f36d}',
        Nerd::Wikipedia => '\u{f05ac}',
        Nerd::Wind => '\u{e27e}',
        Nerd::WindPower => '\u{f1a88}',
        Nerd::WindPowerOutline => '\u{f1a89}',
        Nerd::WindTurbine => '\u{f0da5}',
        Nerd::WindTurbineAlert => '\u{f19ab}',
        Nerd::WindTurbineCheck => '\u{f19ac}',
        Nerd::Window => '\u{eb7f}',
        Nerd::WindowClose => '\u{f05ad}',
        Nerd::WindowClosed => '\u{f05ae}',
        Nerd::WindowClosedVariant => '\u{f11db}',
        Nerd::WindowMaximize => '\u{f05af}',
        Nerd::WindowMinimize => '\u{f05b0}',
        Nerd::WindowOpen => '\u{f05b1}',
        Nerd::WindowOpenVariant => '\u{f11dc}',
        Nerd::WindowRestore => '\u{f05b2}',
        Nerd::WindowShutter => '\u{f111c}',
        Nerd::WindowShutterAlert => '\u{f111d}',
        Nerd::WindowShutterCog => '\u{f1a8a}',
        Nerd::WindowShutterOpen => '\u{f111e}',
        Nerd::WindowShutterSettings => '\u{f1a8b}',
        Nerd::Windows => '\u{f17a}',
        Nerd::Windsock => '\u{f15fa}',
        Nerd::Wiper => '\u{f0ae9}',
        Nerd::WiperWash => '\u{f0da6}',
        Nerd::WiperWashAlert => '\u{f18df}',
        Nerd::WizardHat => '\u{f1477}',
        Nerd::WordWrap => '\u{eb80}',
        Nerd::Wordpress => '\u{f05b4}',
        Nerd::Workflow => '\u{f52e}',
        Nerd::WorkspaceTrusted => '\u{ebc1}',
        Nerd::WorkspaceUnknown => '\u{ebc3}',
        Nerd::WorkspaceUntrusted => '\u{ebc2}',
        Nerd::Wrap => '\u{f05b6}',
        Nerd::WrapDisabled => '\u{f0bdf}',
        Nerd::Wrench => '\u{f0ad}',
        Nerd::WrenchClock => '\u{f19a3}',
        Nerd::WrenchOne => '\u{f05b7}',
        Nerd::WrenchOutline => '\u{f0be0}',
        Nerd::Wthreec => '\u{e212}',
        Nerd::X => '\u{f467}',
        Nerd::XCircle => '\u{f52f}',
        Nerd::XCircleFill => '\u{f530}',
        Nerd::Xamarin => '\u{f0845}',
        Nerd::Xbox => '\u{e29d}',
        Nerd::Xerolinux => '\u{f34a}',
        Nerd::Xfce => '\u{f368}',
        Nerd::Xing => '\u{f168}',
        Nerd::XingSign => '\u{f169}',
        Nerd::Xml => '\u{f05c0}',
        Nerd::Xmonad => '\u{f35e}',
        Nerd::Xmpp => '\u{f07ff}',
        Nerd::Xorg => '\u{f369}',
        Nerd::Yahoo => '\u{f0b4f}',
        Nerd::Yeast => '\u{f05c1}',
        Nerd::YinYang => '\u{f0680}',
        Nerd::Yoga => '\u{f117c}',
        Nerd::Youtube => '\u{f167}',
        Nerd::YoutubeGaming => '\u{f0848}',
        Nerd::YoutubeOne => '\u{f05c3}',
        Nerd::YoutubePlay => '\u{f16a}',
        Nerd::YoutubeSign => '\u{f166}',
        Nerd::YoutubeStudio => '\u{f0847}',
        Nerd::YoutubeSubscription => '\u{f0d40}',
        Nerd::YoutubeTv => '\u{f0448}',
        Nerd::Yurt => '\u{f1516}',
        Nerd::ZWave => '\u{f0aea}',
        Nerd::Zap => '\u{26a1}',
        Nerd::Zend => '\u{f0aeb}',
        Nerd::Zigbee => '\u{f0d41}',
        Nerd::ZipBox => '\u{f05c4}',
        Nerd::ZipBoxOutline => '\u{f0ffa}',
        Nerd::ZipDisk => '\u{f0a23}',
        Nerd::ZodiacAquarius => '\u{f0a7d}',
        Nerd::ZodiacAries => '\u{f0a7e}',
        Nerd::ZodiacCancer => '\u{f0a7f}',
        Nerd::ZodiacCapricorn => '\u{f0a80}',
        Nerd::ZodiacGemini => '\u{f0a81}',
        Nerd::ZodiacLeo => '\u{f0a82}',
        Nerd::ZodiacLibra => '\u{f0a83}',
        Nerd::ZodiacPisces => '\u{f0a84}',
        Nerd::ZodiacSagittarius => '\u{f0a85}',
        Nerd::ZodiacScorpio => '\u{f0a86}',
        Nerd::ZodiacTaurus => '\u{f0a87}',
        Nerd::ZodiacVirgo => '\u{f0a88}',
        Nerd::ZoomIn => '\u{f00e}',
        Nerd::ZoomInOne => '\u{f531}',
        Nerd::ZoomOut => '\u{f010}',
        Nerd::ZoomOutOne => '\u{f532}',
        Nerd::ZorinOs => '\u{f32f}',
    }
}

/// Converts an [`Nerd`] into a [`char`]
#[must_use]
pub fn icon_to_string(icon: Nerd) -> String {
    icon_to_char(icon).to_string()
}

impl Display for Nerd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", icon_to_char(*self))
    }
}

impl From<Nerd> for char {
    fn from(icon: Nerd) -> Self {
        icon_to_char(icon)
    }
}

impl From<Nerd> for String {
    fn from(icon: Nerd) -> Self {
        format!("{}", icon_to_char(icon))
    }
}

/// Converts an [`Nerd`] into a [`Text`] with the font.
#[must_use]
pub fn icon_to_text(icon: Nerd) -> Text<'static> {
    text(icon_to_char(icon)).font(NERD_FONT)
}
