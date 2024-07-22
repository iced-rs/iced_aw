//! This file was automatically generated
//! by [Mamba Bronze](https://github.com/Redhawk18/mamba-bronze)

use crate::NERD_FONT;
use std::{
    fmt::{Display, Formatter, Result},
    string::String,
};

use iced::widget::{text, Text};

/// Holds all glyphs of the Lucide font
#[derive(Debug, Clone, Copy)]
pub enum Lucide {
    /// AArrowDown 
    AArrowDown,
    /// AArrowUp 
    AArrowUp,
    /// ALargeSmall 
    ALargeSmall,
    /// Accessibility 
    Accessibility,
    /// Activity 
    Activity,
    /// AirVent 
    AirVent,
    /// Airplay 
    Airplay,
    /// AlarmClock 
    AlarmClock,
    /// AlarmClockCheck 
    AlarmClockCheck,
    /// AlarmClockMinus 
    AlarmClockMinus,
    /// AlarmClockOff 
    AlarmClockOff,
    /// AlarmClockPlus 
    AlarmClockPlus,
    /// AlarmSmoke 
    AlarmSmoke,
    /// Album 
    Album,
    /// AlignCenter 
    AlignCenter,
    /// AlignCenterHorizontal 
    AlignCenterHorizontal,
    /// AlignCenterVertical 
    AlignCenterVertical,
    /// AlignEndHorizontal 
    AlignEndHorizontal,
    /// AlignEndVertical 
    AlignEndVertical,
    /// AlignHorizontalDistributeCenter 
    AlignHorizontalDistributeCenter,
    /// AlignHorizontalDistributeEnd 
    AlignHorizontalDistributeEnd,
    /// AlignHorizontalDistributeStart 
    AlignHorizontalDistributeStart,
    /// AlignHorizontalJustifyCenter 
    AlignHorizontalJustifyCenter,
    /// AlignHorizontalJustifyEnd 
    AlignHorizontalJustifyEnd,
    /// AlignHorizontalJustifyStart 
    AlignHorizontalJustifyStart,
    /// AlignHorizontalSpaceAround 
    AlignHorizontalSpaceAround,
    /// AlignHorizontalSpaceBetween 
    AlignHorizontalSpaceBetween,
    /// AlignJustify 
    AlignJustify,
    /// AlignLeft 
    AlignLeft,
    /// AlignRight 
    AlignRight,
    /// AlignStartHorizontal 
    AlignStartHorizontal,
    /// AlignStartVertical 
    AlignStartVertical,
    /// AlignVerticalDistributeCenter 
    AlignVerticalDistributeCenter,
    /// AlignVerticalDistributeEnd 
    AlignVerticalDistributeEnd,
    /// AlignVerticalDistributeStart 
    AlignVerticalDistributeStart,
    /// AlignVerticalJustifyCenter 
    AlignVerticalJustifyCenter,
    /// AlignVerticalJustifyEnd 
    AlignVerticalJustifyEnd,
    /// AlignVerticalJustifyStart 
    AlignVerticalJustifyStart,
    /// AlignVerticalSpaceAround 
    AlignVerticalSpaceAround,
    /// AlignVerticalSpaceBetween 
    AlignVerticalSpaceBetween,
    /// Ambulance 
    Ambulance,
    /// Ampersand 
    Ampersand,
    /// Ampersands 
    Ampersands,
    /// Anchor 
    Anchor,
    /// Angry 
    Angry,
    /// Annoyed 
    Annoyed,
    /// Antenna 
    Antenna,
    /// Anvil 
    Anvil,
    /// Aperture 
    Aperture,
    /// AppWindow 
    AppWindow,
    /// AppWindowMac 
    AppWindowMac,
    /// Apple 
    Apple,
    /// Archive 
    Archive,
    /// ArchiveRestore 
    ArchiveRestore,
    /// ArchiveX 
    ArchiveX,
    /// Armchair 
    Armchair,
    /// ArrowBigDown 
    ArrowBigDown,
    /// ArrowBigDownDash 
    ArrowBigDownDash,
    /// ArrowBigLeft 
    ArrowBigLeft,
    /// ArrowBigLeftDash 
    ArrowBigLeftDash,
    /// ArrowBigRight 
    ArrowBigRight,
    /// ArrowBigRightDash 
    ArrowBigRightDash,
    /// ArrowBigUp 
    ArrowBigUp,
    /// ArrowBigUpDash 
    ArrowBigUpDash,
    /// ArrowDown 
    ArrowDown,
    /// ArrowDownAZ 
    ArrowDownAZ,
    /// ArrowDownFromLine 
    ArrowDownFromLine,
    /// ArrowDownLeft 
    ArrowDownLeft,
    /// ArrowDownNarrowWide 
    ArrowDownNarrowWide,
    /// ArrowDownOneZero 
    ArrowDownOneZero,
    /// ArrowDownRight 
    ArrowDownRight,
    /// ArrowDownToDot 
    ArrowDownToDot,
    /// ArrowDownToLine 
    ArrowDownToLine,
    /// ArrowDownUp 
    ArrowDownUp,
    /// ArrowDownWideNarrow 
    ArrowDownWideNarrow,
    /// ArrowDownZA 
    ArrowDownZA,
    /// ArrowDownZeroOne 
    ArrowDownZeroOne,
    /// ArrowLeft 
    ArrowLeft,
    /// ArrowLeftFromLine 
    ArrowLeftFromLine,
    /// ArrowLeftRight 
    ArrowLeftRight,
    /// ArrowLeftToLine 
    ArrowLeftToLine,
    /// ArrowRight 
    ArrowRight,
    /// ArrowRightFromLine 
    ArrowRightFromLine,
    /// ArrowRightLeft 
    ArrowRightLeft,
    /// ArrowRightToLine 
    ArrowRightToLine,
    /// ArrowUp 
    ArrowUp,
    /// ArrowUpAZ 
    ArrowUpAZ,
    /// ArrowUpDown 
    ArrowUpDown,
    /// ArrowUpFromDot 
    ArrowUpFromDot,
    /// ArrowUpFromLine 
    ArrowUpFromLine,
    /// ArrowUpLeft 
    ArrowUpLeft,
    /// ArrowUpNarrowWide 
    ArrowUpNarrowWide,
    /// ArrowUpOneZero 
    ArrowUpOneZero,
    /// ArrowUpRight 
    ArrowUpRight,
    /// ArrowUpToLine 
    ArrowUpToLine,
    /// ArrowUpWideNarrow 
    ArrowUpWideNarrow,
    /// ArrowUpZA 
    ArrowUpZA,
    /// ArrowUpZeroOne 
    ArrowUpZeroOne,
    /// ArrowsUpFromLine 
    ArrowsUpFromLine,
    /// Asterisk 
    Asterisk,
    /// AtSign 
    AtSign,
    /// Atom 
    Atom,
    /// AudioLines 
    AudioLines,
    /// AudioWaveform 
    AudioWaveform,
    /// Award 
    Award,
    /// Axe 
    Axe,
    /// AxisThreed 
    AxisThreed,
    /// Baby 
    Baby,
    /// Backpack 
    Backpack,
    /// Badge 
    Badge,
    /// BadgeAlert 
    BadgeAlert,
    /// BadgeCent 
    BadgeCent,
    /// BadgeCheck 
    BadgeCheck,
    /// BadgeDollarSign 
    BadgeDollarSign,
    /// BadgeEuro 
    BadgeEuro,
    /// BadgeHelp 
    BadgeHelp,
    /// BadgeIndianRupee 
    BadgeIndianRupee,
    /// BadgeInfo 
    BadgeInfo,
    /// BadgeJapaneseYen 
    BadgeJapaneseYen,
    /// BadgeMinus 
    BadgeMinus,
    /// BadgePercent 
    BadgePercent,
    /// BadgePlus 
    BadgePlus,
    /// BadgePoundSterling 
    BadgePoundSterling,
    /// BadgeRussianRuble 
    BadgeRussianRuble,
    /// BadgeSwissFranc 
    BadgeSwissFranc,
    /// BadgeX 
    BadgeX,
    /// BaggageClaim 
    BaggageClaim,
    /// Ban 
    Ban,
    /// Banana 
    Banana,
    /// Banknote 
    Banknote,
    /// Barcode 
    Barcode,
    /// Baseline 
    Baseline,
    /// Bath 
    Bath,
    /// Battery 
    Battery,
    /// BatteryCharging 
    BatteryCharging,
    /// BatteryFull 
    BatteryFull,
    /// BatteryLow 
    BatteryLow,
    /// BatteryMedium 
    BatteryMedium,
    /// BatteryWarning 
    BatteryWarning,
    /// Beaker 
    Beaker,
    /// Bean 
    Bean,
    /// BeanOff 
    BeanOff,
    /// Bed 
    Bed,
    /// BedDouble 
    BedDouble,
    /// BedSingle 
    BedSingle,
    /// Beef 
    Beef,
    /// Beer 
    Beer,
    /// BeerOff 
    BeerOff,
    /// Bell 
    Bell,
    /// BellDot 
    BellDot,
    /// BellElectric 
    BellElectric,
    /// BellMinus 
    BellMinus,
    /// BellOff 
    BellOff,
    /// BellPlus 
    BellPlus,
    /// BellRing 
    BellRing,
    /// BetweenHorizontalEnd 
    BetweenHorizontalEnd,
    /// BetweenHorizontalStart 
    BetweenHorizontalStart,
    /// BetweenVerticalEnd 
    BetweenVerticalEnd,
    /// BetweenVerticalStart 
    BetweenVerticalStart,
    /// BicepsFlexed 
    BicepsFlexed,
    /// Bike 
    Bike,
    /// Binary 
    Binary,
    /// Biohazard 
    Biohazard,
    /// Bird 
    Bird,
    /// Bitcoin 
    Bitcoin,
    /// Blend 
    Blend,
    /// Blinds 
    Blinds,
    /// Blocks 
    Blocks,
    /// Bluetooth 
    Bluetooth,
    /// BluetoothConnected 
    BluetoothConnected,
    /// BluetoothOff 
    BluetoothOff,
    /// BluetoothSearching 
    BluetoothSearching,
    /// Bold 
    Bold,
    /// Bolt 
    Bolt,
    /// Bomb 
    Bomb,
    /// Bone 
    Bone,
    /// Book 
    Book,
    /// BookA 
    BookA,
    /// BookAudio 
    BookAudio,
    /// BookCheck 
    BookCheck,
    /// BookCopy 
    BookCopy,
    /// BookDashed 
    BookDashed,
    /// BookDown 
    BookDown,
    /// BookHeadphones 
    BookHeadphones,
    /// BookHeart 
    BookHeart,
    /// BookImage 
    BookImage,
    /// BookKey 
    BookKey,
    /// BookLock 
    BookLock,
    /// BookMarked 
    BookMarked,
    /// BookMinus 
    BookMinus,
    /// BookOpen 
    BookOpen,
    /// BookOpenCheck 
    BookOpenCheck,
    /// BookOpenText 
    BookOpenText,
    /// BookPlus 
    BookPlus,
    /// BookText 
    BookText,
    /// BookType 
    BookType,
    /// BookUp 
    BookUp,
    /// BookUpTwo 
    BookUpTwo,
    /// BookUser 
    BookUser,
    /// BookX 
    BookX,
    /// Bookmark 
    Bookmark,
    /// BookmarkCheck 
    BookmarkCheck,
    /// BookmarkMinus 
    BookmarkMinus,
    /// BookmarkPlus 
    BookmarkPlus,
    /// BookmarkX 
    BookmarkX,
    /// BoomBox 
    BoomBox,
    /// Bot 
    Bot,
    /// BotMessageSquare 
    BotMessageSquare,
    /// BotOff 
    BotOff,
    /// Box 
    Box,
    /// BoxSelect 
    BoxSelect,
    /// Boxes 
    Boxes,
    /// Braces 
    Braces,
    /// Brackets 
    Brackets,
    /// Brain 
    Brain,
    /// BrainCircuit 
    BrainCircuit,
    /// BrainCog 
    BrainCog,
    /// BrickWall 
    BrickWall,
    /// Briefcase 
    Briefcase,
    /// BriefcaseBusiness 
    BriefcaseBusiness,
    /// BriefcaseMedical 
    BriefcaseMedical,
    /// BringToFront 
    BringToFront,
    /// Brush 
    Brush,
    /// Bug 
    Bug,
    /// BugOff 
    BugOff,
    /// BugPlay 
    BugPlay,
    /// Building 
    Building,
    /// BuildingTwo 
    BuildingTwo,
    /// Bus 
    Bus,
    /// BusFront 
    BusFront,
    /// Cable 
    Cable,
    /// CableCar 
    CableCar,
    /// Cake 
    Cake,
    /// CakeSlice 
    CakeSlice,
    /// Calculator 
    Calculator,
    /// Calendar 
    Calendar,
    /// CalendarArrowDown 
    CalendarArrowDown,
    /// CalendarArrowUp 
    CalendarArrowUp,
    /// CalendarCheck 
    CalendarCheck,
    /// CalendarCheckTwo 
    CalendarCheckTwo,
    /// CalendarClock 
    CalendarClock,
    /// CalendarCog 
    CalendarCog,
    /// CalendarDays 
    CalendarDays,
    /// CalendarFold 
    CalendarFold,
    /// CalendarHeart 
    CalendarHeart,
    /// CalendarMinus 
    CalendarMinus,
    /// CalendarMinusTwo 
    CalendarMinusTwo,
    /// CalendarOff 
    CalendarOff,
    /// CalendarPlus 
    CalendarPlus,
    /// CalendarPlusTwo 
    CalendarPlusTwo,
    /// CalendarRange 
    CalendarRange,
    /// CalendarSearch 
    CalendarSearch,
    /// CalendarX 
    CalendarX,
    /// CalendarXTwo 
    CalendarXTwo,
    /// Camera 
    Camera,
    /// CameraOff 
    CameraOff,
    /// Candy 
    Candy,
    /// CandyCane 
    CandyCane,
    /// CandyOff 
    CandyOff,
    /// Cannabis 
    Cannabis,
    /// Captions 
    Captions,
    /// CaptionsOff 
    CaptionsOff,
    /// Car 
    Car,
    /// CarFront 
    CarFront,
    /// CarTaxiFront 
    CarTaxiFront,
    /// Caravan 
    Caravan,
    /// Carrot 
    Carrot,
    /// CaseLower 
    CaseLower,
    /// CaseSensitive 
    CaseSensitive,
    /// CaseUpper 
    CaseUpper,
    /// CassetteTape 
    CassetteTape,
    /// Cast 
    Cast,
    /// Castle 
    Castle,
    /// Cat 
    Cat,
    /// Cctv 
    Cctv,
    /// ChartArea 
    ChartArea,
    /// ChartBar 
    ChartBar,
    /// ChartBarBig 
    ChartBarBig,
    /// ChartBarDecreasing 
    ChartBarDecreasing,
    /// ChartBarIncreasing 
    ChartBarIncreasing,
    /// ChartBarStacked 
    ChartBarStacked,
    /// ChartCandlestick 
    ChartCandlestick,
    /// ChartColumn 
    ChartColumn,
    /// ChartColumnBig 
    ChartColumnBig,
    /// ChartColumnDecreasing 
    ChartColumnDecreasing,
    /// ChartColumnStacked 
    ChartColumnStacked,
    /// ChartLine 
    ChartLine,
    /// ChartNetwork 
    ChartNetwork,
    /// ChartNoAxesColumn 
    ChartNoAxesColumn,
    /// ChartNoAxesColumnDecreasing 
    ChartNoAxesColumnDecreasing,
    /// ChartNoAxesColumnIncreasing 
    ChartNoAxesColumnIncreasing,
    /// ChartNoAxesCombined 
    ChartNoAxesCombined,
    /// ChartNoAxesGantt 
    ChartNoAxesGantt,
    /// ChartPie 
    ChartPie,
    /// ChartScatter 
    ChartScatter,
    /// ChartSpline 
    ChartSpline,
    /// Check 
    Check,
    /// CheckCheck 
    CheckCheck,
    /// ChefHat 
    ChefHat,
    /// Cherry 
    Cherry,
    /// ChevronDown 
    ChevronDown,
    /// ChevronFirst 
    ChevronFirst,
    /// ChevronLast 
    ChevronLast,
    /// ChevronLeft 
    ChevronLeft,
    /// ChevronRight 
    ChevronRight,
    /// ChevronUp 
    ChevronUp,
    /// ChevronsDown 
    ChevronsDown,
    /// ChevronsDownUp 
    ChevronsDownUp,
    /// ChevronsLeft 
    ChevronsLeft,
    /// ChevronsLeftRight 
    ChevronsLeftRight,
    /// ChevronsRight 
    ChevronsRight,
    /// ChevronsRightLeft 
    ChevronsRightLeft,
    /// ChevronsUp 
    ChevronsUp,
    /// ChevronsUpDown 
    ChevronsUpDown,
    /// Chrome 
    Chrome,
    /// Church 
    Church,
    /// Cigarette 
    Cigarette,
    /// CigaretteOff 
    CigaretteOff,
    /// Circle 
    Circle,
    /// CircleAlert 
    CircleAlert,
    /// CircleArrowDown 
    CircleArrowDown,
    /// CircleArrowLeft 
    CircleArrowLeft,
    /// CircleArrowOutDownLeft 
    CircleArrowOutDownLeft,
    /// CircleArrowOutDownRight 
    CircleArrowOutDownRight,
    /// CircleArrowOutUpLeft 
    CircleArrowOutUpLeft,
    /// CircleArrowOutUpRight 
    CircleArrowOutUpRight,
    /// CircleArrowRight 
    CircleArrowRight,
    /// CircleArrowUp 
    CircleArrowUp,
    /// CircleCheck 
    CircleCheck,
    /// CircleCheckBig 
    CircleCheckBig,
    /// CircleChevronDown 
    CircleChevronDown,
    /// CircleChevronLeft 
    CircleChevronLeft,
    /// CircleChevronRight 
    CircleChevronRight,
    /// CircleChevronUp 
    CircleChevronUp,
    /// CircleDashed 
    CircleDashed,
    /// CircleDivide 
    CircleDivide,
    /// CircleDollarSign 
    CircleDollarSign,
    /// CircleDot 
    CircleDot,
    /// CircleDotDashed 
    CircleDotDashed,
    /// CircleEllipsis 
    CircleEllipsis,
    /// CircleEqual 
    CircleEqual,
    /// CircleFadingPlus 
    CircleFadingPlus,
    /// CircleGauge 
    CircleGauge,
    /// CircleHelp 
    CircleHelp,
    /// CircleMinus 
    CircleMinus,
    /// CircleOff 
    CircleOff,
    /// CircleParking 
    CircleParking,
    /// CircleParkingOff 
    CircleParkingOff,
    /// CirclePause 
    CirclePause,
    /// CirclePercent 
    CirclePercent,
    /// CirclePlay 
    CirclePlay,
    /// CirclePlus 
    CirclePlus,
    /// CirclePower 
    CirclePower,
    /// CircleSlash 
    CircleSlash,
    /// CircleSlashTwo 
    CircleSlashTwo,
    /// CircleStop 
    CircleStop,
    /// CircleUser 
    CircleUser,
    /// CircleUserRound 
    CircleUserRound,
    /// CircleX 
    CircleX,
    /// CircuitBoard 
    CircuitBoard,
    /// Citrus 
    Citrus,
    /// Clapperboard 
    Clapperboard,
    /// Clipboard 
    Clipboard,
    /// ClipboardCheck 
    ClipboardCheck,
    /// ClipboardCopy 
    ClipboardCopy,
    /// ClipboardList 
    ClipboardList,
    /// ClipboardMinus 
    ClipboardMinus,
    /// ClipboardPaste 
    ClipboardPaste,
    /// ClipboardPen 
    ClipboardPen,
    /// ClipboardPenLine 
    ClipboardPenLine,
    /// ClipboardPlus 
    ClipboardPlus,
    /// ClipboardType 
    ClipboardType,
    /// ClipboardX 
    ClipboardX,
    /// ClockArrowDown 
    ClockArrowDown,
    /// ClockArrowUp 
    ClockArrowUp,
    /// ClockEight 
    ClockEight,
    /// ClockFive 
    ClockFive,
    /// ClockFour 
    ClockFour,
    /// ClockNine 
    ClockNine,
    /// ClockOne 
    ClockOne,
    /// ClockOneone 
    ClockOneone,
    /// ClockOnetwo 
    ClockOnetwo,
    /// ClockOnezero 
    ClockOnezero,
    /// ClockSeven 
    ClockSeven,
    /// ClockSix 
    ClockSix,
    /// ClockThree 
    ClockThree,
    /// ClockTwo 
    ClockTwo,
    /// Cloud 
    Cloud,
    /// CloudCog 
    CloudCog,
    /// CloudDownload 
    CloudDownload,
    /// CloudDrizzle 
    CloudDrizzle,
    /// CloudFog 
    CloudFog,
    /// CloudHail 
    CloudHail,
    /// CloudLightning 
    CloudLightning,
    /// CloudMoon 
    CloudMoon,
    /// CloudMoonRain 
    CloudMoonRain,
    /// CloudOff 
    CloudOff,
    /// CloudRain 
    CloudRain,
    /// CloudRainWind 
    CloudRainWind,
    /// CloudSnow 
    CloudSnow,
    /// CloudSun 
    CloudSun,
    /// CloudSunRain 
    CloudSunRain,
    /// CloudUpload 
    CloudUpload,
    /// Cloudy 
    Cloudy,
    /// Clover 
    Clover,
    /// Club 
    Club,
    /// Code 
    Code,
    /// CodeXml 
    CodeXml,
    /// Codepen 
    Codepen,
    /// Codesandbox 
    Codesandbox,
    /// Coffee 
    Coffee,
    /// Cog 
    Cog,
    /// Coins 
    Coins,
    /// ColumnsFour 
    ColumnsFour,
    /// ColumnsThree 
    ColumnsThree,
    /// ColumnsTwo 
    ColumnsTwo,
    /// Combine 
    Combine,
    /// Command 
    Command,
    /// Compass 
    Compass,
    /// Component 
    Component,
    /// Computer 
    Computer,
    /// ConciergeBell 
    ConciergeBell,
    /// Cone 
    Cone,
    /// Construction 
    Construction,
    /// Contact 
    Contact,
    /// ContactRound 
    ContactRound,
    /// Container 
    Container,
    /// Contrast 
    Contrast,
    /// Cookie 
    Cookie,
    /// CookingPot 
    CookingPot,
    /// Copy 
    Copy,
    /// CopyCheck 
    CopyCheck,
    /// CopyMinus 
    CopyMinus,
    /// CopyPlus 
    CopyPlus,
    /// CopySlash 
    CopySlash,
    /// CopyX 
    CopyX,
    /// Copyleft 
    Copyleft,
    /// Copyright 
    Copyright,
    /// CornerDownLeft 
    CornerDownLeft,
    /// CornerDownRight 
    CornerDownRight,
    /// CornerLeftDown 
    CornerLeftDown,
    /// CornerLeftUp 
    CornerLeftUp,
    /// CornerRightDown 
    CornerRightDown,
    /// CornerRightUp 
    CornerRightUp,
    /// CornerUpLeft 
    CornerUpLeft,
    /// CornerUpRight 
    CornerUpRight,
    /// Cpu 
    Cpu,
    /// CreativeCommons 
    CreativeCommons,
    /// CreditCard 
    CreditCard,
    /// Croissant 
    Croissant,
    /// Crop 
    Crop,
    /// Cross 
    Cross,
    /// Crosshair 
    Crosshair,
    /// Crown 
    Crown,
    /// Cuboid 
    Cuboid,
    /// CupSoda 
    CupSoda,
    /// Currency 
    Currency,
    /// Cylinder 
    Cylinder,
    /// Dam 
    Dam,
    /// Database 
    Database,
    /// DatabaseBackup 
    DatabaseBackup,
    /// DatabaseZap 
    DatabaseZap,
    /// Delete 
    Delete,
    /// Dessert 
    Dessert,
    /// Diameter 
    Diameter,
    /// Diamond 
    Diamond,
    /// DiamondMinus 
    DiamondMinus,
    /// DiamondPercent 
    DiamondPercent,
    /// DiamondPlus 
    DiamondPlus,
    /// DiceFive 
    DiceFive,
    /// DiceFour 
    DiceFour,
    /// DiceOne 
    DiceOne,
    /// DiceSix 
    DiceSix,
    /// DiceThree 
    DiceThree,
    /// DiceTwo 
    DiceTwo,
    /// Dices 
    Dices,
    /// Diff 
    Diff,
    /// Disc 
    Disc,
    /// DiscAlbum 
    DiscAlbum,
    /// DiscThree 
    DiscThree,
    /// DiscTwo 
    DiscTwo,
    /// Divide 
    Divide,
    /// Dna 
    Dna,
    /// DnaOff 
    DnaOff,
    /// Dock 
    Dock,
    /// Dog 
    Dog,
    /// DollarSign 
    DollarSign,
    /// Donut 
    Donut,
    /// DoorClosed 
    DoorClosed,
    /// DoorOpen 
    DoorOpen,
    /// Dot 
    Dot,
    /// Download 
    Download,
    /// DraftingCompass 
    DraftingCompass,
    /// Drama 
    Drama,
    /// Dribbble 
    Dribbble,
    /// Drill 
    Drill,
    /// Droplet 
    Droplet,
    /// Droplets 
    Droplets,
    /// Drum 
    Drum,
    /// Drumstick 
    Drumstick,
    /// Dumbbell 
    Dumbbell,
    /// Ear 
    Ear,
    /// EarOff 
    EarOff,
    /// Earth 
    Earth,
    /// EarthLock 
    EarthLock,
    /// Eclipse 
    Eclipse,
    /// Egg 
    Egg,
    /// EggFried 
    EggFried,
    /// EggOff 
    EggOff,
    /// Ellipsis 
    Ellipsis,
    /// EllipsisVertical 
    EllipsisVertical,
    /// Equal 
    Equal,
    /// EqualNot 
    EqualNot,
    /// Eraser 
    Eraser,
    /// Euro 
    Euro,
    /// Expand 
    Expand,
    /// ExternalLink 
    ExternalLink,
    /// Eye 
    Eye,
    /// EyeOff 
    EyeOff,
    /// Facebook 
    Facebook,
    /// Factory 
    Factory,
    /// Fan 
    Fan,
    /// FastForward 
    FastForward,
    /// Feather 
    Feather,
    /// Fence 
    Fence,
    /// FerrisWheel 
    FerrisWheel,
    /// Figma 
    Figma,
    /// File 
    File,
    /// FileArchive 
    FileArchive,
    /// FileAudio 
    FileAudio,
    /// FileAudioTwo 
    FileAudioTwo,
    /// FileAxisThreed 
    FileAxisThreed,
    /// FileBadge 
    FileBadge,
    /// FileBadgeTwo 
    FileBadgeTwo,
    /// FileBox 
    FileBox,
    /// FileChartColumn 
    FileChartColumn,
    /// FileChartColumnIncreasing 
    FileChartColumnIncreasing,
    /// FileChartLine 
    FileChartLine,
    /// FileChartPie 
    FileChartPie,
    /// FileCheck 
    FileCheck,
    /// FileCheckTwo 
    FileCheckTwo,
    /// FileClock 
    FileClock,
    /// FileCode 
    FileCode,
    /// FileCodeTwo 
    FileCodeTwo,
    /// FileCog 
    FileCog,
    /// FileDiff 
    FileDiff,
    /// FileDigit 
    FileDigit,
    /// FileDown 
    FileDown,
    /// FileHeart 
    FileHeart,
    /// FileImage 
    FileImage,
    /// FileInput 
    FileInput,
    /// FileJson 
    FileJson,
    /// FileJsonTwo 
    FileJsonTwo,
    /// FileKey 
    FileKey,
    /// FileKeyTwo 
    FileKeyTwo,
    /// FileLock 
    FileLock,
    /// FileLockTwo 
    FileLockTwo,
    /// FileMinus 
    FileMinus,
    /// FileMinusTwo 
    FileMinusTwo,
    /// FileMusic 
    FileMusic,
    /// FileOutput 
    FileOutput,
    /// FilePen 
    FilePen,
    /// FilePenLine 
    FilePenLine,
    /// FilePlus 
    FilePlus,
    /// FilePlusTwo 
    FilePlusTwo,
    /// FileQuestion 
    FileQuestion,
    /// FileScan 
    FileScan,
    /// FileSearch 
    FileSearch,
    /// FileSearchTwo 
    FileSearchTwo,
    /// FileSliders 
    FileSliders,
    /// FileSpreadsheet 
    FileSpreadsheet,
    /// FileStack 
    FileStack,
    /// FileSymlink 
    FileSymlink,
    /// FileTerminal 
    FileTerminal,
    /// FileText 
    FileText,
    /// FileType 
    FileType,
    /// FileTypeTwo 
    FileTypeTwo,
    /// FileUp 
    FileUp,
    /// FileVideo 
    FileVideo,
    /// FileVideoTwo 
    FileVideoTwo,
    /// FileVolume 
    FileVolume,
    /// FileVolumeTwo 
    FileVolumeTwo,
    /// FileWarning 
    FileWarning,
    /// FileX 
    FileX,
    /// FileXTwo 
    FileXTwo,
    /// Files 
    Files,
    /// Film 
    Film,
    /// Filter 
    Filter,
    /// FilterX 
    FilterX,
    /// Fingerprint 
    Fingerprint,
    /// FireExtinguisher 
    FireExtinguisher,
    /// Fish 
    Fish,
    /// FishOff 
    FishOff,
    /// FishSymbol 
    FishSymbol,
    /// Flag 
    Flag,
    /// FlagOff 
    FlagOff,
    /// FlagTriangleLeft 
    FlagTriangleLeft,
    /// FlagTriangleRight 
    FlagTriangleRight,
    /// Flame 
    Flame,
    /// FlameKindling 
    FlameKindling,
    /// Flashlight 
    Flashlight,
    /// FlashlightOff 
    FlashlightOff,
    /// FlaskConical 
    FlaskConical,
    /// FlaskConicalOff 
    FlaskConicalOff,
    /// FlaskRound 
    FlaskRound,
    /// FlipHorizontal 
    FlipHorizontal,
    /// FlipHorizontalTwo 
    FlipHorizontalTwo,
    /// FlipVertical 
    FlipVertical,
    /// FlipVerticalTwo 
    FlipVerticalTwo,
    /// Flower 
    Flower,
    /// FlowerTwo 
    FlowerTwo,
    /// Focus 
    Focus,
    /// FoldHorizontal 
    FoldHorizontal,
    /// FoldVertical 
    FoldVertical,
    /// Folder 
    Folder,
    /// FolderArchive 
    FolderArchive,
    /// FolderCheck 
    FolderCheck,
    /// FolderClock 
    FolderClock,
    /// FolderClosed 
    FolderClosed,
    /// FolderCode 
    FolderCode,
    /// FolderCog 
    FolderCog,
    /// FolderDot 
    FolderDot,
    /// FolderDown 
    FolderDown,
    /// FolderGit 
    FolderGit,
    /// FolderGitTwo 
    FolderGitTwo,
    /// FolderHeart 
    FolderHeart,
    /// FolderInput 
    FolderInput,
    /// FolderKanban 
    FolderKanban,
    /// FolderKey 
    FolderKey,
    /// FolderLock 
    FolderLock,
    /// FolderMinus 
    FolderMinus,
    /// FolderOpen 
    FolderOpen,
    /// FolderOpenDot 
    FolderOpenDot,
    /// FolderOutput 
    FolderOutput,
    /// FolderPen 
    FolderPen,
    /// FolderPlus 
    FolderPlus,
    /// FolderRoot 
    FolderRoot,
    /// FolderSearch 
    FolderSearch,
    /// FolderSearchTwo 
    FolderSearchTwo,
    /// FolderSymlink 
    FolderSymlink,
    /// FolderSync 
    FolderSync,
    /// FolderTree 
    FolderTree,
    /// FolderUp 
    FolderUp,
    /// FolderX 
    FolderX,
    /// Folders 
    Folders,
    /// Footprints 
    Footprints,
    /// Forklift 
    Forklift,
    /// Forward 
    Forward,
    /// Frame 
    Frame,
    /// Framer 
    Framer,
    /// Frown 
    Frown,
    /// Fuel 
    Fuel,
    /// Fullscreen 
    Fullscreen,
    /// GalleryHorizontal 
    GalleryHorizontal,
    /// GalleryHorizontalEnd 
    GalleryHorizontalEnd,
    /// GalleryThumbnails 
    GalleryThumbnails,
    /// GalleryVertical 
    GalleryVertical,
    /// GalleryVerticalEnd 
    GalleryVerticalEnd,
    /// Gamepad 
    Gamepad,
    /// GamepadTwo 
    GamepadTwo,
    /// Gauge 
    Gauge,
    /// Gavel 
    Gavel,
    /// Gem 
    Gem,
    /// Ghost 
    Ghost,
    /// Gift 
    Gift,
    /// GitBranch 
    GitBranch,
    /// GitBranchPlus 
    GitBranchPlus,
    /// GitCommitHorizontal 
    GitCommitHorizontal,
    /// GitCommitVertical 
    GitCommitVertical,
    /// GitCompare 
    GitCompare,
    /// GitCompareArrows 
    GitCompareArrows,
    /// GitFork 
    GitFork,
    /// GitGraph 
    GitGraph,
    /// GitMerge 
    GitMerge,
    /// GitPullRequest 
    GitPullRequest,
    /// GitPullRequestArrow 
    GitPullRequestArrow,
    /// GitPullRequestClosed 
    GitPullRequestClosed,
    /// GitPullRequestCreate 
    GitPullRequestCreate,
    /// GitPullRequestCreateArrow 
    GitPullRequestCreateArrow,
    /// GitPullRequestDraft 
    GitPullRequestDraft,
    /// Github 
    Github,
    /// Gitlab 
    Gitlab,
    /// GlassWater 
    GlassWater,
    /// Glasses 
    Glasses,
    /// Globe 
    Globe,
    /// GlobeLock 
    GlobeLock,
    /// Goal 
    Goal,
    /// Grab 
    Grab,
    /// GraduationCap 
    GraduationCap,
    /// Grape 
    Grape,
    /// GridThreexthree 
    GridThreexthree,
    /// GridTwoxtwo 
    GridTwoxtwo,
    /// GridTwoxtwoCheck 
    GridTwoxtwoCheck,
    /// GridTwoxtwoX 
    GridTwoxtwoX,
    /// Grip 
    Grip,
    /// GripHorizontal 
    GripHorizontal,
    /// GripVertical 
    GripVertical,
    /// Group 
    Group,
    /// Guitar 
    Guitar,
    /// Ham 
    Ham,
    /// Hammer 
    Hammer,
    /// Hand 
    Hand,
    /// HandCoins 
    HandCoins,
    /// HandHeart 
    HandHeart,
    /// HandHelping 
    HandHelping,
    /// HandMetal 
    HandMetal,
    /// HandPlatter 
    HandPlatter,
    /// Handshake 
    Handshake,
    /// HardDrive 
    HardDrive,
    /// HardDriveDownload 
    HardDriveDownload,
    /// HardDriveUpload 
    HardDriveUpload,
    /// HardHat 
    HardHat,
    /// Hash 
    Hash,
    /// Haze 
    Haze,
    /// HdmiPort 
    HdmiPort,
    /// Heading 
    Heading,
    /// HeadingFive 
    HeadingFive,
    /// HeadingFour 
    HeadingFour,
    /// HeadingOne 
    HeadingOne,
    /// HeadingSix 
    HeadingSix,
    /// HeadingThree 
    HeadingThree,
    /// HeadingTwo 
    HeadingTwo,
    /// Headphones 
    Headphones,
    /// Headset 
    Headset,
    /// Heart 
    Heart,
    /// HeartCrack 
    HeartCrack,
    /// HeartHandshake 
    HeartHandshake,
    /// HeartOff 
    HeartOff,
    /// HeartPulse 
    HeartPulse,
    /// Heater 
    Heater,
    /// Hexagon 
    Hexagon,
    /// Highlighter 
    Highlighter,
    /// History 
    History,
    /// Hop 
    Hop,
    /// HopOff 
    HopOff,
    /// Hospital 
    Hospital,
    /// Hotel 
    Hotel,
    /// Hourglass 
    Hourglass,
    /// House 
    House,
    /// HousePlug 
    HousePlug,
    /// HousePlus 
    HousePlus,
    /// IceCreamBowl 
    IceCreamBowl,
    /// IceCreamCone 
    IceCreamCone,
    /// Image 
    Image,
    /// ImageDown 
    ImageDown,
    /// ImageMinus 
    ImageMinus,
    /// ImageOff 
    ImageOff,
    /// ImagePlay 
    ImagePlay,
    /// ImagePlus 
    ImagePlus,
    /// ImageUp 
    ImageUp,
    /// Images 
    Images,
    /// Import 
    Import,
    /// Inbox 
    Inbox,
    /// IndentDecrease 
    IndentDecrease,
    /// IndentIncrease 
    IndentIncrease,
    /// IndianRupee 
    IndianRupee,
    /// Infinity 
    Infinity,
    /// Info 
    Info,
    /// InspectionPanel 
    InspectionPanel,
    /// Instagram 
    Instagram,
    /// Italic 
    Italic,
    /// IterationCcw 
    IterationCcw,
    /// IterationCw 
    IterationCw,
    /// JapaneseYen 
    JapaneseYen,
    /// Joystick 
    Joystick,
    /// Kanban 
    Kanban,
    /// Key 
    Key,
    /// KeyRound 
    KeyRound,
    /// KeySquare 
    KeySquare,
    /// Keyboard 
    Keyboard,
    /// KeyboardMusic 
    KeyboardMusic,
    /// KeyboardOff 
    KeyboardOff,
    /// Lamp 
    Lamp,
    /// LampCeiling 
    LampCeiling,
    /// LampDesk 
    LampDesk,
    /// LampFloor 
    LampFloor,
    /// LampWallDown 
    LampWallDown,
    /// LampWallUp 
    LampWallUp,
    /// LandPlot 
    LandPlot,
    /// Landmark 
    Landmark,
    /// Languages 
    Languages,
    /// Laptop 
    Laptop,
    /// LaptopMinimal 
    LaptopMinimal,
    /// Lasso 
    Lasso,
    /// LassoSelect 
    LassoSelect,
    /// Laugh 
    Laugh,
    /// Layers 
    Layers,
    /// LayersThree 
    LayersThree,
    /// LayersTwo 
    LayersTwo,
    /// LayoutDashboard 
    LayoutDashboard,
    /// LayoutGrid 
    LayoutGrid,
    /// LayoutList 
    LayoutList,
    /// LayoutPanelLeft 
    LayoutPanelLeft,
    /// LayoutPanelTop 
    LayoutPanelTop,
    /// LayoutTemplate 
    LayoutTemplate,
    /// Leaf 
    Leaf,
    /// LeafyGreen 
    LeafyGreen,
    /// Lectern 
    Lectern,
    /// LetterText 
    LetterText,
    /// Library 
    Library,
    /// LibraryBig 
    LibraryBig,
    /// LifeBuoy 
    LifeBuoy,
    /// Ligature 
    Ligature,
    /// Lightbulb 
    Lightbulb,
    /// LightbulbOff 
    LightbulbOff,
    /// Link 
    Link,
    /// LinkTwo 
    LinkTwo,
    /// LinkTwoOff 
    LinkTwoOff,
    /// Linkedin 
    Linkedin,
    /// List 
    List,
    /// ListCheck 
    ListCheck,
    /// ListChecks 
    ListChecks,
    /// ListCollapse 
    ListCollapse,
    /// ListEnd 
    ListEnd,
    /// ListFilter 
    ListFilter,
    /// ListMinus 
    ListMinus,
    /// ListMusic 
    ListMusic,
    /// ListOrdered 
    ListOrdered,
    /// ListPlus 
    ListPlus,
    /// ListRestart 
    ListRestart,
    /// ListStart 
    ListStart,
    /// ListTodo 
    ListTodo,
    /// ListTree 
    ListTree,
    /// ListVideo 
    ListVideo,
    /// ListX 
    ListX,
    /// Loader 
    Loader,
    /// LoaderCircle 
    LoaderCircle,
    /// LoaderPinwheel 
    LoaderPinwheel,
    /// Locate 
    Locate,
    /// LocateFixed 
    LocateFixed,
    /// LocateOff 
    LocateOff,
    /// Lock 
    Lock,
    /// LockKeyhole 
    LockKeyhole,
    /// LockKeyholeOpen 
    LockKeyholeOpen,
    /// LockOpen 
    LockOpen,
    /// LogIn 
    LogIn,
    /// LogOut 
    LogOut,
    /// Logs 
    Logs,
    /// Lollipop 
    Lollipop,
    /// Luggage 
    Luggage,
    /// Magnet 
    Magnet,
    /// Mail 
    Mail,
    /// MailCheck 
    MailCheck,
    /// MailMinus 
    MailMinus,
    /// MailOpen 
    MailOpen,
    /// MailPlus 
    MailPlus,
    /// MailQuestion 
    MailQuestion,
    /// MailSearch 
    MailSearch,
    /// MailWarning 
    MailWarning,
    /// MailX 
    MailX,
    /// Mailbox 
    Mailbox,
    /// Mails 
    Mails,
    /// Map 
    Map,
    /// MapPin 
    MapPin,
    /// MapPinOff 
    MapPinOff,
    /// MapPinned 
    MapPinned,
    /// Martini 
    Martini,
    /// Maximize 
    Maximize,
    /// MaximizeTwo 
    MaximizeTwo,
    /// Medal 
    Medal,
    /// Megaphone 
    Megaphone,
    /// MegaphoneOff 
    MegaphoneOff,
    /// Meh 
    Meh,
    /// MemoryStick 
    MemoryStick,
    /// Menu 
    Menu,
    /// Merge 
    Merge,
    /// MessageCircle 
    MessageCircle,
    /// MessageCircleCode 
    MessageCircleCode,
    /// MessageCircleDashed 
    MessageCircleDashed,
    /// MessageCircleHeart 
    MessageCircleHeart,
    /// MessageCircleMore 
    MessageCircleMore,
    /// MessageCircleOff 
    MessageCircleOff,
    /// MessageCirclePlus 
    MessageCirclePlus,
    /// MessageCircleQuestion 
    MessageCircleQuestion,
    /// MessageCircleReply 
    MessageCircleReply,
    /// MessageCircleWarning 
    MessageCircleWarning,
    /// MessageCircleX 
    MessageCircleX,
    /// MessageSquare 
    MessageSquare,
    /// MessageSquareCode 
    MessageSquareCode,
    /// MessageSquareDashed 
    MessageSquareDashed,
    /// MessageSquareDiff 
    MessageSquareDiff,
    /// MessageSquareDot 
    MessageSquareDot,
    /// MessageSquareHeart 
    MessageSquareHeart,
    /// MessageSquareMore 
    MessageSquareMore,
    /// MessageSquareOff 
    MessageSquareOff,
    /// MessageSquarePlus 
    MessageSquarePlus,
    /// MessageSquareQuote 
    MessageSquareQuote,
    /// MessageSquareReply 
    MessageSquareReply,
    /// MessageSquareShare 
    MessageSquareShare,
    /// MessageSquareText 
    MessageSquareText,
    /// MessageSquareWarning 
    MessageSquareWarning,
    /// MessageSquareX 
    MessageSquareX,
    /// MessagesSquare 
    MessagesSquare,
    /// Mic 
    Mic,
    /// MicOff 
    MicOff,
    /// MicVocal 
    MicVocal,
    /// Microscope 
    Microscope,
    /// Microwave 
    Microwave,
    /// Milestone 
    Milestone,
    /// Milk 
    Milk,
    /// MilkOff 
    MilkOff,
    /// Minimize 
    Minimize,
    /// MinimizeTwo 
    MinimizeTwo,
    /// Minus 
    Minus,
    /// Monitor 
    Monitor,
    /// MonitorCheck 
    MonitorCheck,
    /// MonitorCog 
    MonitorCog,
    /// MonitorDot 
    MonitorDot,
    /// MonitorDown 
    MonitorDown,
    /// MonitorOff 
    MonitorOff,
    /// MonitorPause 
    MonitorPause,
    /// MonitorPlay 
    MonitorPlay,
    /// MonitorSmartphone 
    MonitorSmartphone,
    /// MonitorSpeaker 
    MonitorSpeaker,
    /// MonitorStop 
    MonitorStop,
    /// MonitorUp 
    MonitorUp,
    /// MonitorX 
    MonitorX,
    /// Moon 
    Moon,
    /// MoonStar 
    MoonStar,
    /// Mountain 
    Mountain,
    /// MountainSnow 
    MountainSnow,
    /// Mouse 
    Mouse,
    /// MouseOff 
    MouseOff,
    /// MousePointer 
    MousePointer,
    /// MousePointerBan 
    MousePointerBan,
    /// MousePointerClick 
    MousePointerClick,
    /// MousePointerTwo 
    MousePointerTwo,
    /// Move 
    Move,
    /// MoveDiagonal 
    MoveDiagonal,
    /// MoveDiagonalTwo 
    MoveDiagonalTwo,
    /// MoveDown 
    MoveDown,
    /// MoveDownLeft 
    MoveDownLeft,
    /// MoveDownRight 
    MoveDownRight,
    /// MoveHorizontal 
    MoveHorizontal,
    /// MoveLeft 
    MoveLeft,
    /// MoveRight 
    MoveRight,
    /// MoveThreed 
    MoveThreed,
    /// MoveUp 
    MoveUp,
    /// MoveUpLeft 
    MoveUpLeft,
    /// MoveUpRight 
    MoveUpRight,
    /// MoveVertical 
    MoveVertical,
    /// Music 
    Music,
    /// MusicFour 
    MusicFour,
    /// MusicThree 
    MusicThree,
    /// MusicTwo 
    MusicTwo,
    /// Navigation 
    Navigation,
    /// NavigationOff 
    NavigationOff,
    /// NavigationTwo 
    NavigationTwo,
    /// NavigationTwoOff 
    NavigationTwoOff,
    /// Network 
    Network,
    /// Newspaper 
    Newspaper,
    /// Nfc 
    Nfc,
    /// Notebook 
    Notebook,
    /// NotebookPen 
    NotebookPen,
    /// NotebookTabs 
    NotebookTabs,
    /// NotebookText 
    NotebookText,
    /// NotepadText 
    NotepadText,
    /// NotepadTextDashed 
    NotepadTextDashed,
    /// Nut 
    Nut,
    /// NutOff 
    NutOff,
    /// Octagon 
    Octagon,
    /// OctagonAlert 
    OctagonAlert,
    /// OctagonPause 
    OctagonPause,
    /// OctagonX 
    OctagonX,
    /// Option 
    Option,
    /// Orbit 
    Orbit,
    /// Origami 
    Origami,
    /// Package 
    Package,
    /// PackageCheck 
    PackageCheck,
    /// PackageMinus 
    PackageMinus,
    /// PackageOpen 
    PackageOpen,
    /// PackagePlus 
    PackagePlus,
    /// PackageSearch 
    PackageSearch,
    /// PackageTwo 
    PackageTwo,
    /// PackageX 
    PackageX,
    /// PaintBucket 
    PaintBucket,
    /// PaintRoller 
    PaintRoller,
    /// Paintbrush 
    Paintbrush,
    /// PaintbrushVertical 
    PaintbrushVertical,
    /// Palette 
    Palette,
    /// PanelBottom 
    PanelBottom,
    /// PanelBottomClose 
    PanelBottomClose,
    /// PanelBottomDashed 
    PanelBottomDashed,
    /// PanelBottomOpen 
    PanelBottomOpen,
    /// PanelLeft 
    PanelLeft,
    /// PanelLeftClose 
    PanelLeftClose,
    /// PanelLeftDashed 
    PanelLeftDashed,
    /// PanelLeftOpen 
    PanelLeftOpen,
    /// PanelRight 
    PanelRight,
    /// PanelRightClose 
    PanelRightClose,
    /// PanelRightDashed 
    PanelRightDashed,
    /// PanelRightOpen 
    PanelRightOpen,
    /// PanelTop 
    PanelTop,
    /// PanelTopClose 
    PanelTopClose,
    /// PanelTopDashed 
    PanelTopDashed,
    /// PanelTopOpen 
    PanelTopOpen,
    /// PanelsLeftBottom 
    PanelsLeftBottom,
    /// PanelsRightBottom 
    PanelsRightBottom,
    /// PanelsTopLeft 
    PanelsTopLeft,
    /// Paperclip 
    Paperclip,
    /// Parentheses 
    Parentheses,
    /// ParkingMeter 
    ParkingMeter,
    /// PartyPopper 
    PartyPopper,
    /// Pause 
    Pause,
    /// PawPrint 
    PawPrint,
    /// PcCase 
    PcCase,
    /// Pen 
    Pen,
    /// PenLine 
    PenLine,
    /// PenOff 
    PenOff,
    /// PenTool 
    PenTool,
    /// Pencil 
    Pencil,
    /// PencilLine 
    PencilLine,
    /// PencilOff 
    PencilOff,
    /// PencilRuler 
    PencilRuler,
    /// Pentagon 
    Pentagon,
    /// Percent 
    Percent,
    /// PersonStanding 
    PersonStanding,
    /// PhilippinePeso 
    PhilippinePeso,
    /// Phone 
    Phone,
    /// PhoneCall 
    PhoneCall,
    /// PhoneForwarded 
    PhoneForwarded,
    /// PhoneIncoming 
    PhoneIncoming,
    /// PhoneMissed 
    PhoneMissed,
    /// PhoneOff 
    PhoneOff,
    /// PhoneOutgoing 
    PhoneOutgoing,
    /// Pi 
    Pi,
    /// Piano 
    Piano,
    /// Pickaxe 
    Pickaxe,
    /// PictureInPicture 
    PictureInPicture,
    /// PictureInPictureTwo 
    PictureInPictureTwo,
    /// PiggyBank 
    PiggyBank,
    /// Pilcrow 
    Pilcrow,
    /// PilcrowLeft 
    PilcrowLeft,
    /// PilcrowRight 
    PilcrowRight,
    /// Pill 
    Pill,
    /// PillBottle 
    PillBottle,
    /// Pin 
    Pin,
    /// PinOff 
    PinOff,
    /// Pipette 
    Pipette,
    /// Pizza 
    Pizza,
    /// Plane 
    Plane,
    /// PlaneLanding 
    PlaneLanding,
    /// PlaneTakeoff 
    PlaneTakeoff,
    /// Play 
    Play,
    /// Plug 
    Plug,
    /// PlugTwo 
    PlugTwo,
    /// PlugZap 
    PlugZap,
    /// Plus 
    Plus,
    /// Pocket 
    Pocket,
    /// PocketKnife 
    PocketKnife,
    /// Podcast 
    Podcast,
    /// Pointer 
    Pointer,
    /// PointerOff 
    PointerOff,
    /// Popcorn 
    Popcorn,
    /// Popsicle 
    Popsicle,
    /// PoundSterling 
    PoundSterling,
    /// Power 
    Power,
    /// PowerOff 
    PowerOff,
    /// Presentation 
    Presentation,
    /// Printer 
    Printer,
    /// PrinterCheck 
    PrinterCheck,
    /// Projector 
    Projector,
    /// Proportions 
    Proportions,
    /// Puzzle 
    Puzzle,
    /// Pyramid 
    Pyramid,
    /// QrCode 
    QrCode,
    /// Quote 
    Quote,
    /// Rabbit 
    Rabbit,
    /// Radar 
    Radar,
    /// Radiation 
    Radiation,
    /// Radical 
    Radical,
    /// Radio 
    Radio,
    /// RadioReceiver 
    RadioReceiver,
    /// RadioTower 
    RadioTower,
    /// Radius 
    Radius,
    /// RailSymbol 
    RailSymbol,
    /// Rainbow 
    Rainbow,
    /// Rat 
    Rat,
    /// Ratio 
    Ratio,
    /// Receipt 
    Receipt,
    /// ReceiptCent 
    ReceiptCent,
    /// ReceiptEuro 
    ReceiptEuro,
    /// ReceiptIndianRupee 
    ReceiptIndianRupee,
    /// ReceiptJapaneseYen 
    ReceiptJapaneseYen,
    /// ReceiptPoundSterling 
    ReceiptPoundSterling,
    /// ReceiptRussianRuble 
    ReceiptRussianRuble,
    /// ReceiptSwissFranc 
    ReceiptSwissFranc,
    /// ReceiptText 
    ReceiptText,
    /// RectangleEllipsis 
    RectangleEllipsis,
    /// RectangleHorizontal 
    RectangleHorizontal,
    /// RectangleVertical 
    RectangleVertical,
    /// Recycle 
    Recycle,
    /// Redo 
    Redo,
    /// RedoDot 
    RedoDot,
    /// RedoTwo 
    RedoTwo,
    /// RefreshCcw 
    RefreshCcw,
    /// RefreshCcwDot 
    RefreshCcwDot,
    /// RefreshCw 
    RefreshCw,
    /// RefreshCwOff 
    RefreshCwOff,
    /// Refrigerator 
    Refrigerator,
    /// Regex 
    Regex,
    /// RemoveFormatting 
    RemoveFormatting,
    /// Repeat 
    Repeat,
    /// RepeatOne 
    RepeatOne,
    /// RepeatTwo 
    RepeatTwo,
    /// Replace 
    Replace,
    /// ReplaceAll 
    ReplaceAll,
    /// Reply 
    Reply,
    /// ReplyAll 
    ReplyAll,
    /// Rewind 
    Rewind,
    /// Ribbon 
    Ribbon,
    /// Rocket 
    Rocket,
    /// RockingChair 
    RockingChair,
    /// RollerCoaster 
    RollerCoaster,
    /// RotateCcw 
    RotateCcw,
    /// RotateCcwSquare 
    RotateCcwSquare,
    /// RotateCw 
    RotateCw,
    /// RotateCwSquare 
    RotateCwSquare,
    /// RotateThreed 
    RotateThreed,
    /// Route 
    Route,
    /// RouteOff 
    RouteOff,
    /// Router 
    Router,
    /// RowsFour 
    RowsFour,
    /// RowsThree 
    RowsThree,
    /// RowsTwo 
    RowsTwo,
    /// Rss 
    Rss,
    /// Ruler 
    Ruler,
    /// RussianRuble 
    RussianRuble,
    /// Sailboat 
    Sailboat,
    /// Salad 
    Salad,
    /// Sandwich 
    Sandwich,
    /// Satellite 
    Satellite,
    /// SatelliteDish 
    SatelliteDish,
    /// Save 
    Save,
    /// SaveAll 
    SaveAll,
    /// SaveOff 
    SaveOff,
    /// Scale 
    Scale,
    /// ScaleThreed 
    ScaleThreed,
    /// Scaling 
    Scaling,
    /// Scan 
    Scan,
    /// ScanBarcode 
    ScanBarcode,
    /// ScanEye 
    ScanEye,
    /// ScanFace 
    ScanFace,
    /// ScanLine 
    ScanLine,
    /// ScanQrCode 
    ScanQrCode,
    /// ScanSearch 
    ScanSearch,
    /// ScanText 
    ScanText,
    /// School 
    School,
    /// Scissors 
    Scissors,
    /// ScissorsLineDashed 
    ScissorsLineDashed,
    /// ScreenShare 
    ScreenShare,
    /// ScreenShareOff 
    ScreenShareOff,
    /// Scroll 
    Scroll,
    /// ScrollText 
    ScrollText,
    /// Search 
    Search,
    /// SearchCheck 
    SearchCheck,
    /// SearchCode 
    SearchCode,
    /// SearchSlash 
    SearchSlash,
    /// SearchX 
    SearchX,
    /// Section 
    Section,
    /// Send 
    Send,
    /// SendHorizontal 
    SendHorizontal,
    /// SendToBack 
    SendToBack,
    /// SeparatorHorizontal 
    SeparatorHorizontal,
    /// SeparatorVertical 
    SeparatorVertical,
    /// Server 
    Server,
    /// ServerCog 
    ServerCog,
    /// ServerCrash 
    ServerCrash,
    /// ServerOff 
    ServerOff,
    /// Settings 
    Settings,
    /// SettingsTwo 
    SettingsTwo,
    /// Shapes 
    Shapes,
    /// Share 
    Share,
    /// ShareTwo 
    ShareTwo,
    /// Sheet 
    Sheet,
    /// Shell 
    Shell,
    /// Shield 
    Shield,
    /// ShieldAlert 
    ShieldAlert,
    /// ShieldBan 
    ShieldBan,
    /// ShieldCheck 
    ShieldCheck,
    /// ShieldEllipsis 
    ShieldEllipsis,
    /// ShieldHalf 
    ShieldHalf,
    /// ShieldMinus 
    ShieldMinus,
    /// ShieldOff 
    ShieldOff,
    /// ShieldPlus 
    ShieldPlus,
    /// ShieldQuestion 
    ShieldQuestion,
    /// ShieldX 
    ShieldX,
    /// Ship 
    Ship,
    /// ShipWheel 
    ShipWheel,
    /// Shirt 
    Shirt,
    /// ShoppingBag 
    ShoppingBag,
    /// ShoppingBasket 
    ShoppingBasket,
    /// ShoppingCart 
    ShoppingCart,
    /// Shovel 
    Shovel,
    /// ShowerHead 
    ShowerHead,
    /// Shrink 
    Shrink,
    /// Shrub 
    Shrub,
    /// Shuffle 
    Shuffle,
    /// Sigma 
    Sigma,
    /// Signal 
    Signal,
    /// SignalHigh 
    SignalHigh,
    /// SignalLow 
    SignalLow,
    /// SignalMedium 
    SignalMedium,
    /// SignalZero 
    SignalZero,
    /// Signature 
    Signature,
    /// Signpost 
    Signpost,
    /// SignpostBig 
    SignpostBig,
    /// Siren 
    Siren,
    /// SkipBack 
    SkipBack,
    /// SkipForward 
    SkipForward,
    /// Skull 
    Skull,
    /// Slack 
    Slack,
    /// Slash 
    Slash,
    /// Slice 
    Slice,
    /// SlidersHorizontal 
    SlidersHorizontal,
    /// SlidersVertical 
    SlidersVertical,
    /// Smartphone 
    Smartphone,
    /// SmartphoneCharging 
    SmartphoneCharging,
    /// SmartphoneNfc 
    SmartphoneNfc,
    /// Smile 
    Smile,
    /// SmilePlus 
    SmilePlus,
    /// Snail 
    Snail,
    /// Snowflake 
    Snowflake,
    /// Sofa 
    Sofa,
    /// Soup 
    Soup,
    /// Space 
    Space,
    /// Spade 
    Spade,
    /// Sparkle 
    Sparkle,
    /// Sparkles 
    Sparkles,
    /// Speaker 
    Speaker,
    /// Speech 
    Speech,
    /// SpellCheck 
    SpellCheck,
    /// SpellCheckTwo 
    SpellCheckTwo,
    /// Spline 
    Spline,
    /// Split 
    Split,
    /// SprayCan 
    SprayCan,
    /// Sprout 
    Sprout,
    /// Square 
    Square,
    /// SquareActivity 
    SquareActivity,
    /// SquareArrowDown 
    SquareArrowDown,
    /// SquareArrowDownLeft 
    SquareArrowDownLeft,
    /// SquareArrowDownRight 
    SquareArrowDownRight,
    /// SquareArrowLeft 
    SquareArrowLeft,
    /// SquareArrowOutDownLeft 
    SquareArrowOutDownLeft,
    /// SquareArrowOutDownRight 
    SquareArrowOutDownRight,
    /// SquareArrowOutUpLeft 
    SquareArrowOutUpLeft,
    /// SquareArrowOutUpRight 
    SquareArrowOutUpRight,
    /// SquareArrowRight 
    SquareArrowRight,
    /// SquareArrowUp 
    SquareArrowUp,
    /// SquareArrowUpLeft 
    SquareArrowUpLeft,
    /// SquareArrowUpRight 
    SquareArrowUpRight,
    /// SquareAsterisk 
    SquareAsterisk,
    /// SquareBottomDashedScissors 
    SquareBottomDashedScissors,
    /// SquareChartGantt 
    SquareChartGantt,
    /// SquareCheck 
    SquareCheck,
    /// SquareCheckBig 
    SquareCheckBig,
    /// SquareChevronDown 
    SquareChevronDown,
    /// SquareChevronLeft 
    SquareChevronLeft,
    /// SquareChevronRight 
    SquareChevronRight,
    /// SquareChevronUp 
    SquareChevronUp,
    /// SquareCode 
    SquareCode,
    /// SquareDashedBottom 
    SquareDashedBottom,
    /// SquareDashedBottomCode 
    SquareDashedBottomCode,
    /// SquareDashedKanban 
    SquareDashedKanban,
    /// SquareDashedMousePointer 
    SquareDashedMousePointer,
    /// SquareDivide 
    SquareDivide,
    /// SquareDot 
    SquareDot,
    /// SquareEqual 
    SquareEqual,
    /// SquareFunction 
    SquareFunction,
    /// SquareKanban 
    SquareKanban,
    /// SquareLibrary 
    SquareLibrary,
    /// SquareM 
    SquareM,
    /// SquareMenu 
    SquareMenu,
    /// SquareMinus 
    SquareMinus,
    /// SquareMousePointer 
    SquareMousePointer,
    /// SquareParking 
    SquareParking,
    /// SquareParkingOff 
    SquareParkingOff,
    /// SquarePen 
    SquarePen,
    /// SquarePercent 
    SquarePercent,
    /// SquarePi 
    SquarePi,
    /// SquarePilcrow 
    SquarePilcrow,
    /// SquarePlay 
    SquarePlay,
    /// SquarePlus 
    SquarePlus,
    /// SquarePower 
    SquarePower,
    /// SquareRadical 
    SquareRadical,
    /// SquareScissors 
    SquareScissors,
    /// SquareSigma 
    SquareSigma,
    /// SquareSlash 
    SquareSlash,
    /// SquareSplitHorizontal 
    SquareSplitHorizontal,
    /// SquareSplitVertical 
    SquareSplitVertical,
    /// SquareStack 
    SquareStack,
    /// SquareTerminal 
    SquareTerminal,
    /// SquareUser 
    SquareUser,
    /// SquareUserRound 
    SquareUserRound,
    /// SquareX 
    SquareX,
    /// Squircle 
    Squircle,
    /// Squirrel 
    Squirrel,
    /// Stamp 
    Stamp,
    /// Star 
    Star,
    /// StarHalf 
    StarHalf,
    /// StarOff 
    StarOff,
    /// StepBack 
    StepBack,
    /// StepForward 
    StepForward,
    /// Stethoscope 
    Stethoscope,
    /// Sticker 
    Sticker,
    /// StickyNote 
    StickyNote,
    /// Store 
    Store,
    /// StretchHorizontal 
    StretchHorizontal,
    /// StretchVertical 
    StretchVertical,
    /// Strikethrough 
    Strikethrough,
    /// Subscript 
    Subscript,
    /// Sun 
    Sun,
    /// SunDim 
    SunDim,
    /// SunMedium 
    SunMedium,
    /// SunMoon 
    SunMoon,
    /// SunSnow 
    SunSnow,
    /// Sunrise 
    Sunrise,
    /// Sunset 
    Sunset,
    /// Superscript 
    Superscript,
    /// SwatchBook 
    SwatchBook,
    /// SwissFranc 
    SwissFranc,
    /// SwitchCamera 
    SwitchCamera,
    /// Sword 
    Sword,
    /// Swords 
    Swords,
    /// Syringe 
    Syringe,
    /// Table 
    Table,
    /// TableCellsMerge 
    TableCellsMerge,
    /// TableCellsSplit 
    TableCellsSplit,
    /// TableColumnsSplit 
    TableColumnsSplit,
    /// TableProperties 
    TableProperties,
    /// TableRowsSplit 
    TableRowsSplit,
    /// TableTwo 
    TableTwo,
    /// Tablet 
    Tablet,
    /// TabletSmartphone 
    TabletSmartphone,
    /// Tablets 
    Tablets,
    /// Tag 
    Tag,
    /// Tags 
    Tags,
    /// TallyFive 
    TallyFive,
    /// TallyFour 
    TallyFour,
    /// TallyOne 
    TallyOne,
    /// TallyThree 
    TallyThree,
    /// TallyTwo 
    TallyTwo,
    /// Tangent 
    Tangent,
    /// Target 
    Target,
    /// Telescope 
    Telescope,
    /// Tent 
    Tent,
    /// TentTree 
    TentTree,
    /// Terminal 
    Terminal,
    /// TestTube 
    TestTube,
    /// TestTubeDiagonal 
    TestTubeDiagonal,
    /// TestTubes 
    TestTubes,
    /// Text 
    Text,
    /// TextCursor 
    TextCursor,
    /// TextCursorInput 
    TextCursorInput,
    /// TextQuote 
    TextQuote,
    /// TextSearch 
    TextSearch,
    /// TextSelect 
    TextSelect,
    /// Theater 
    Theater,
    /// Thermometer 
    Thermometer,
    /// ThermometerSnowflake 
    ThermometerSnowflake,
    /// ThermometerSun 
    ThermometerSun,
    /// ThumbsDown 
    ThumbsDown,
    /// ThumbsUp 
    ThumbsUp,
    /// Ticket 
    Ticket,
    /// TicketCheck 
    TicketCheck,
    /// TicketMinus 
    TicketMinus,
    /// TicketPercent 
    TicketPercent,
    /// TicketPlus 
    TicketPlus,
    /// TicketSlash 
    TicketSlash,
    /// TicketX 
    TicketX,
    /// Timer 
    Timer,
    /// TimerOff 
    TimerOff,
    /// TimerReset 
    TimerReset,
    /// ToggleLeft 
    ToggleLeft,
    /// ToggleRight 
    ToggleRight,
    /// Tornado 
    Tornado,
    /// Torus 
    Torus,
    /// Touchpad 
    Touchpad,
    /// TouchpadOff 
    TouchpadOff,
    /// TowerControl 
    TowerControl,
    /// ToyBrick 
    ToyBrick,
    /// Tractor 
    Tractor,
    /// TrafficCone 
    TrafficCone,
    /// TrainFront 
    TrainFront,
    /// TrainFrontTunnel 
    TrainFrontTunnel,
    /// TrainTrack 
    TrainTrack,
    /// TramFront 
    TramFront,
    /// Trash 
    Trash,
    /// TrashTwo 
    TrashTwo,
    /// TreeDeciduous 
    TreeDeciduous,
    /// TreePalm 
    TreePalm,
    /// TreePine 
    TreePine,
    /// Trees 
    Trees,
    /// Trello 
    Trello,
    /// TrendingDown 
    TrendingDown,
    /// TrendingUp 
    TrendingUp,
    /// Triangle 
    Triangle,
    /// TriangleAlert 
    TriangleAlert,
    /// TriangleRight 
    TriangleRight,
    /// Trophy 
    Trophy,
    /// Truck 
    Truck,
    /// Turtle 
    Turtle,
    /// Tv 
    Tv,
    /// TvMinimal 
    TvMinimal,
    /// TvMinimalPlay 
    TvMinimalPlay,
    /// Twitch 
    Twitch,
    /// Twitter 
    Twitter,
    /// Type 
    Type,
    /// TypeOutline 
    TypeOutline,
    /// Umbrella 
    Umbrella,
    /// UmbrellaOff 
    UmbrellaOff,
    /// Underline 
    Underline,
    /// Undo 
    Undo,
    /// UndoDot 
    UndoDot,
    /// UndoTwo 
    UndoTwo,
    /// UnfoldHorizontal 
    UnfoldHorizontal,
    /// UnfoldVertical 
    UnfoldVertical,
    /// Ungroup 
    Ungroup,
    /// University 
    University,
    /// Unlink 
    Unlink,
    /// UnlinkTwo 
    UnlinkTwo,
    /// Unplug 
    Unplug,
    /// Upload 
    Upload,
    /// Usb 
    Usb,
    /// User 
    User,
    /// UserCheck 
    UserCheck,
    /// UserCog 
    UserCog,
    /// UserMinus 
    UserMinus,
    /// UserPen 
    UserPen,
    /// UserPlus 
    UserPlus,
    /// UserRound 
    UserRound,
    /// UserRoundCheck 
    UserRoundCheck,
    /// UserRoundCog 
    UserRoundCog,
    /// UserRoundMinus 
    UserRoundMinus,
    /// UserRoundPen 
    UserRoundPen,
    /// UserRoundPlus 
    UserRoundPlus,
    /// UserRoundSearch 
    UserRoundSearch,
    /// UserRoundX 
    UserRoundX,
    /// UserSearch 
    UserSearch,
    /// UserX 
    UserX,
    /// Users 
    Users,
    /// UsersRound 
    UsersRound,
    /// Utensils 
    Utensils,
    /// UtensilsCrossed 
    UtensilsCrossed,
    /// UtilityPole 
    UtilityPole,
    /// Variable 
    Variable,
    /// Vault 
    Vault,
    /// Vegan 
    Vegan,
    /// VenetianMask 
    VenetianMask,
    /// Vibrate 
    Vibrate,
    /// VibrateOff 
    VibrateOff,
    /// Video 
    Video,
    /// VideoOff 
    VideoOff,
    /// Videotape 
    Videotape,
    /// View 
    View,
    /// Voicemail 
    Voicemail,
    /// Volume 
    Volume,
    /// VolumeOne 
    VolumeOne,
    /// VolumeTwo 
    VolumeTwo,
    /// VolumeX 
    VolumeX,
    /// Vote 
    Vote,
    /// Wallet 
    Wallet,
    /// WalletCards 
    WalletCards,
    /// WalletMinimal 
    WalletMinimal,
    /// Wallpaper 
    Wallpaper,
    /// Wand 
    Wand,
    /// WandSparkles 
    WandSparkles,
    /// Warehouse 
    Warehouse,
    /// WashingMachine 
    WashingMachine,
    /// Watch 
    Watch,
    /// Waves 
    Waves,
    /// Waypoints 
    Waypoints,
    /// Webcam 
    Webcam,
    /// Webhook 
    Webhook,
    /// WebhookOff 
    WebhookOff,
    /// Weight 
    Weight,
    /// Wheat 
    Wheat,
    /// WheatOff 
    WheatOff,
    /// WholeWord 
    WholeWord,
    /// Wifi 
    Wifi,
    /// WifiHigh 
    WifiHigh,
    /// WifiLow 
    WifiLow,
    /// WifiOff 
    WifiOff,
    /// WifiZero 
    WifiZero,
    /// Wind 
    Wind,
    /// Wine 
    Wine,
    /// WineOff 
    WineOff,
    /// Workflow 
    Workflow,
    /// Worm 
    Worm,
    /// WrapText 
    WrapText,
    /// Wrench 
    Wrench,
    /// X 
    X,
    /// Youtube 
    Youtube,
    /// Zap 
    Zap,
    /// ZapOff 
    ZapOff,
    /// ZoomIn 
    ZoomIn,
    /// ZoomOut 
    ZoomOut,
}

/// Converts an [`Lucide`] into a [`char`]
#[must_use]
pub const fn icon_to_char(icon: Lucide) -> char {
    match icon {
        Lucide::AArrowDown => '\u{e58a}',
        Lucide::AArrowUp => '\u{e58b}',
        Lucide::ALargeSmall => '\u{e58c}',
        Lucide::Accessibility => '\u{e297}',
        Lucide::Activity => '\u{e038}',
        Lucide::AirVent => '\u{e351}',
        Lucide::Airplay => '\u{e039}',
        Lucide::AlarmClock => '\u{e03a}',
        Lucide::AlarmClockCheck => '\u{e1ec}',
        Lucide::AlarmClockMinus => '\u{e1ed}',
        Lucide::AlarmClockOff => '\u{e23b}',
        Lucide::AlarmClockPlus => '\u{e1ee}',
        Lucide::AlarmSmoke => '\u{e580}',
        Lucide::Album => '\u{e03b}',
        Lucide::AlignCenter => '\u{e03c}',
        Lucide::AlignCenterHorizontal => '\u{e26c}',
        Lucide::AlignCenterVertical => '\u{e26d}',
        Lucide::AlignEndHorizontal => '\u{e26e}',
        Lucide::AlignEndVertical => '\u{e26f}',
        Lucide::AlignHorizontalDistributeCenter => '\u{e03d}',
        Lucide::AlignHorizontalDistributeEnd => '\u{e03e}',
        Lucide::AlignHorizontalDistributeStart => '\u{e03f}',
        Lucide::AlignHorizontalJustifyCenter => '\u{e272}',
        Lucide::AlignHorizontalJustifyEnd => '\u{e273}',
        Lucide::AlignHorizontalJustifyStart => '\u{e274}',
        Lucide::AlignHorizontalSpaceAround => '\u{e275}',
        Lucide::AlignHorizontalSpaceBetween => '\u{e276}',
        Lucide::AlignJustify => '\u{e040}',
        Lucide::AlignLeft => '\u{e041}',
        Lucide::AlignRight => '\u{e042}',
        Lucide::AlignStartHorizontal => '\u{e270}',
        Lucide::AlignStartVertical => '\u{e271}',
        Lucide::AlignVerticalDistributeCenter => '\u{e27e}',
        Lucide::AlignVerticalDistributeEnd => '\u{e27f}',
        Lucide::AlignVerticalDistributeStart => '\u{e280}',
        Lucide::AlignVerticalJustifyCenter => '\u{e277}',
        Lucide::AlignVerticalJustifyEnd => '\u{e278}',
        Lucide::AlignVerticalJustifyStart => '\u{e279}',
        Lucide::AlignVerticalSpaceAround => '\u{e27a}',
        Lucide::AlignVerticalSpaceBetween => '\u{e27b}',
        Lucide::Ambulance => '\u{e5c0}',
        Lucide::Ampersand => '\u{e4a1}',
        Lucide::Ampersands => '\u{e4a2}',
        Lucide::Anchor => '\u{e043}',
        Lucide::Angry => '\u{e2fc}',
        Lucide::Annoyed => '\u{e2fd}',
        Lucide::Antenna => '\u{e4e7}',
        Lucide::Anvil => '\u{e585}',
        Lucide::Aperture => '\u{e044}',
        Lucide::AppWindow => '\u{e42b}',
        Lucide::AppWindowMac => '\u{e5d7}',
        Lucide::Apple => '\u{e352}',
        Lucide::Archive => '\u{e045}',
        Lucide::ArchiveRestore => '\u{e2cd}',
        Lucide::ArchiveX => '\u{e511}',
        Lucide::Armchair => '\u{e2c0}',
        Lucide::ArrowBigDown => '\u{e1e1}',
        Lucide::ArrowBigDownDash => '\u{e422}',
        Lucide::ArrowBigLeft => '\u{e1e2}',
        Lucide::ArrowBigLeftDash => '\u{e423}',
        Lucide::ArrowBigRight => '\u{e1e3}',
        Lucide::ArrowBigRightDash => '\u{e424}',
        Lucide::ArrowBigUp => '\u{e1e4}',
        Lucide::ArrowBigUpDash => '\u{e425}',
        Lucide::ArrowDown => '\u{e046}',
        Lucide::ArrowDownAZ => '\u{e41a}',
        Lucide::ArrowDownFromLine => '\u{e459}',
        Lucide::ArrowDownLeft => '\u{e047}',
        Lucide::ArrowDownNarrowWide => '\u{e048}',
        Lucide::ArrowDownOneZero => '\u{e419}',
        Lucide::ArrowDownRight => '\u{e049}',
        Lucide::ArrowDownToDot => '\u{e452}',
        Lucide::ArrowDownToLine => '\u{e45a}',
        Lucide::ArrowDownUp => '\u{e04a}',
        Lucide::ArrowDownWideNarrow => '\u{e04b}',
        Lucide::ArrowDownZA => '\u{e41b}',
        Lucide::ArrowDownZeroOne => '\u{e418}',
        Lucide::ArrowLeft => '\u{e04c}',
        Lucide::ArrowLeftFromLine => '\u{e45b}',
        Lucide::ArrowLeftRight => '\u{e24a}',
        Lucide::ArrowLeftToLine => '\u{e45c}',
        Lucide::ArrowRight => '\u{e04d}',
        Lucide::ArrowRightFromLine => '\u{e45d}',
        Lucide::ArrowRightLeft => '\u{e41c}',
        Lucide::ArrowRightToLine => '\u{e45e}',
        Lucide::ArrowUp => '\u{e04e}',
        Lucide::ArrowUpAZ => '\u{e41f}',
        Lucide::ArrowUpDown => '\u{e381}',
        Lucide::ArrowUpFromDot => '\u{e453}',
        Lucide::ArrowUpFromLine => '\u{e45f}',
        Lucide::ArrowUpLeft => '\u{e04f}',
        Lucide::ArrowUpNarrowWide => '\u{e050}',
        Lucide::ArrowUpOneZero => '\u{e41e}',
        Lucide::ArrowUpRight => '\u{e051}',
        Lucide::ArrowUpToLine => '\u{e460}',
        Lucide::ArrowUpWideNarrow => '\u{e420}',
        Lucide::ArrowUpZA => '\u{e421}',
        Lucide::ArrowUpZeroOne => '\u{e41d}',
        Lucide::ArrowsUpFromLine => '\u{e4d9}',
        Lucide::Asterisk => '\u{e1ef}',
        Lucide::AtSign => '\u{e052}',
        Lucide::Atom => '\u{e3db}',
        Lucide::AudioLines => '\u{e55f}',
        Lucide::AudioWaveform => '\u{e560}',
        Lucide::Award => '\u{e053}',
        Lucide::Axe => '\u{e054}',
        Lucide::AxisThreed => '\u{e2fe}',
        Lucide::Baby => '\u{e2ce}',
        Lucide::Backpack => '\u{e2c8}',
        Lucide::Badge => '\u{e479}',
        Lucide::BadgeAlert => '\u{e47a}',
        Lucide::BadgeCent => '\u{e514}',
        Lucide::BadgeCheck => '\u{e241}',
        Lucide::BadgeDollarSign => '\u{e47b}',
        Lucide::BadgeEuro => '\u{e515}',
        Lucide::BadgeHelp => '\u{e47c}',
        Lucide::BadgeIndianRupee => '\u{e516}',
        Lucide::BadgeInfo => '\u{e47d}',
        Lucide::BadgeJapaneseYen => '\u{e517}',
        Lucide::BadgeMinus => '\u{e47e}',
        Lucide::BadgePercent => '\u{e47f}',
        Lucide::BadgePlus => '\u{e480}',
        Lucide::BadgePoundSterling => '\u{e518}',
        Lucide::BadgeRussianRuble => '\u{e519}',
        Lucide::BadgeSwissFranc => '\u{e51a}',
        Lucide::BadgeX => '\u{e481}',
        Lucide::BaggageClaim => '\u{e2c9}',
        Lucide::Ban => '\u{e055}',
        Lucide::Banana => '\u{e353}',
        Lucide::Banknote => '\u{e056}',
        Lucide::Barcode => '\u{e538}',
        Lucide::Baseline => '\u{e285}',
        Lucide::Bath => '\u{e2ab}',
        Lucide::Battery => '\u{e057}',
        Lucide::BatteryCharging => '\u{e058}',
        Lucide::BatteryFull => '\u{e059}',
        Lucide::BatteryLow => '\u{e05a}',
        Lucide::BatteryMedium => '\u{e05b}',
        Lucide::BatteryWarning => '\u{e3b0}',
        Lucide::Beaker => '\u{e05c}',
        Lucide::Bean => '\u{e393}',
        Lucide::BeanOff => '\u{e394}',
        Lucide::Bed => '\u{e2c1}',
        Lucide::BedDouble => '\u{e2c2}',
        Lucide::BedSingle => '\u{e2c3}',
        Lucide::Beef => '\u{e3a9}',
        Lucide::Beer => '\u{e2cf}',
        Lucide::BeerOff => '\u{e5de}',
        Lucide::Bell => '\u{e05d}',
        Lucide::BellDot => '\u{e430}',
        Lucide::BellElectric => '\u{e581}',
        Lucide::BellMinus => '\u{e1f0}',
        Lucide::BellOff => '\u{e05e}',
        Lucide::BellPlus => '\u{e1f1}',
        Lucide::BellRing => '\u{e224}',
        Lucide::BetweenHorizontalEnd => '\u{e596}',
        Lucide::BetweenHorizontalStart => '\u{e597}',
        Lucide::BetweenVerticalEnd => '\u{e598}',
        Lucide::BetweenVerticalStart => '\u{e599}',
        Lucide::BicepsFlexed => '\u{e5f0}',
        Lucide::Bike => '\u{e1d2}',
        Lucide::Binary => '\u{e1f2}',
        Lucide::Biohazard => '\u{e446}',
        Lucide::Bird => '\u{e3c9}',
        Lucide::Bitcoin => '\u{e05f}',
        Lucide::Blend => '\u{e5a1}',
        Lucide::Blinds => '\u{e3c4}',
        Lucide::Blocks => '\u{e4ff}',
        Lucide::Bluetooth => '\u{e060}',
        Lucide::BluetoothConnected => '\u{e1b8}',
        Lucide::BluetoothOff => '\u{e1b9}',
        Lucide::BluetoothSearching => '\u{e1ba}',
        Lucide::Bold => '\u{e061}',
        Lucide::Bolt => '\u{e591}',
        Lucide::Bomb => '\u{e2ff}',
        Lucide::Bone => '\u{e35c}',
        Lucide::Book => '\u{e062}',
        Lucide::BookA => '\u{e549}',
        Lucide::BookAudio => '\u{e54a}',
        Lucide::BookCheck => '\u{e54b}',
        Lucide::BookCopy => '\u{e3f1}',
        Lucide::BookDashed => '\u{e3f2}',
        Lucide::BookDown => '\u{e3f3}',
        Lucide::BookHeadphones => '\u{e54c}',
        Lucide::BookHeart => '\u{e54d}',
        Lucide::BookImage => '\u{e54e}',
        Lucide::BookKey => '\u{e3f4}',
        Lucide::BookLock => '\u{e3f5}',
        Lucide::BookMarked => '\u{e3f6}',
        Lucide::BookMinus => '\u{e3f7}',
        Lucide::BookOpen => '\u{e063}',
        Lucide::BookOpenCheck => '\u{e385}',
        Lucide::BookOpenText => '\u{e54f}',
        Lucide::BookPlus => '\u{e3f8}',
        Lucide::BookText => '\u{e550}',
        Lucide::BookType => '\u{e551}',
        Lucide::BookUp => '\u{e3f9}',
        Lucide::BookUpTwo => '\u{e4ab}',
        Lucide::BookUser => '\u{e552}',
        Lucide::BookX => '\u{e3fa}',
        Lucide::Bookmark => '\u{e064}',
        Lucide::BookmarkCheck => '\u{e524}',
        Lucide::BookmarkMinus => '\u{e23c}',
        Lucide::BookmarkPlus => '\u{e23d}',
        Lucide::BookmarkX => '\u{e525}',
        Lucide::BoomBox => '\u{e4f3}',
        Lucide::Bot => '\u{e1bb}',
        Lucide::BotMessageSquare => '\u{e5d3}',
        Lucide::BotOff => '\u{e5e5}',
        Lucide::Box => '\u{e065}',
        Lucide::BoxSelect => '\u{e1bc}',
        Lucide::Boxes => '\u{e2d0}',
        Lucide::Braces => '\u{e36e}',
        Lucide::Brackets => '\u{e448}',
        Lucide::Brain => '\u{e3ca}',
        Lucide::BrainCircuit => '\u{e3cb}',
        Lucide::BrainCog => '\u{e3cc}',
        Lucide::BrickWall => '\u{e586}',
        Lucide::Briefcase => '\u{e066}',
        Lucide::BriefcaseBusiness => '\u{e5da}',
        Lucide::BriefcaseMedical => '\u{e5db}',
        Lucide::BringToFront => '\u{e4f4}',
        Lucide::Brush => '\u{e1d3}',
        Lucide::Bug => '\u{e20c}',
        Lucide::BugOff => '\u{e512}',
        Lucide::BugPlay => '\u{e513}',
        Lucide::Building => '\u{e1cc}',
        Lucide::BuildingTwo => '\u{e290}',
        Lucide::Bus => '\u{e1d4}',
        Lucide::BusFront => '\u{e500}',
        Lucide::Cable => '\u{e4e8}',
        Lucide::CableCar => '\u{e501}',
        Lucide::Cake => '\u{e348}',
        Lucide::CakeSlice => '\u{e4be}',
        Lucide::Calculator => '\u{e1bd}',
        Lucide::Calendar => '\u{e067}',
        Lucide::CalendarArrowDown => '\u{e603}',
        Lucide::CalendarArrowUp => '\u{e604}',
        Lucide::CalendarCheck => '\u{e2b7}',
        Lucide::CalendarCheckTwo => '\u{e2b8}',
        Lucide::CalendarClock => '\u{e304}',
        Lucide::CalendarCog => '\u{e5f2}',
        Lucide::CalendarDays => '\u{e2b9}',
        Lucide::CalendarFold => '\u{e5b9}',
        Lucide::CalendarHeart => '\u{e305}',
        Lucide::CalendarMinus => '\u{e2ba}',
        Lucide::CalendarMinusTwo => '\u{e5ba}',
        Lucide::CalendarOff => '\u{e2bb}',
        Lucide::CalendarPlus => '\u{e2bc}',
        Lucide::CalendarPlusTwo => '\u{e5bb}',
        Lucide::CalendarRange => '\u{e2bd}',
        Lucide::CalendarSearch => '\u{e306}',
        Lucide::CalendarX => '\u{e2be}',
        Lucide::CalendarXTwo => '\u{e2bf}',
        Lucide::Camera => '\u{e068}',
        Lucide::CameraOff => '\u{e069}',
        Lucide::Candy => '\u{e395}',
        Lucide::CandyCane => '\u{e4bf}',
        Lucide::CandyOff => '\u{e396}',
        Lucide::Cannabis => '\u{e5d9}',
        Lucide::Captions => '\u{e3a8}',
        Lucide::CaptionsOff => '\u{e5c6}',
        Lucide::Car => '\u{e1d5}',
        Lucide::CarFront => '\u{e502}',
        Lucide::CarTaxiFront => '\u{e503}',
        Lucide::Caravan => '\u{e53e}',
        Lucide::Carrot => '\u{e25a}',
        Lucide::CaseLower => '\u{e3dc}',
        Lucide::CaseSensitive => '\u{e3dd}',
        Lucide::CaseUpper => '\u{e3de}',
        Lucide::CassetteTape => '\u{e4cf}',
        Lucide::Cast => '\u{e06a}',
        Lucide::Castle => '\u{e3e4}',
        Lucide::Cat => '\u{e390}',
        Lucide::Cctv => '\u{e582}',
        Lucide::ChartArea => '\u{e4d8}',
        Lucide::ChartBar => '\u{e2a2}',
        Lucide::ChartBarBig => '\u{e4ac}',
        Lucide::ChartBarDecreasing => '\u{e60c}',
        Lucide::ChartBarIncreasing => '\u{e60d}',
        Lucide::ChartBarStacked => '\u{e60e}',
        Lucide::ChartCandlestick => '\u{e4ad}',
        Lucide::ChartColumn => '\u{e2a3}',
        Lucide::ChartColumnBig => '\u{e4ae}',
        Lucide::ChartColumnDecreasing => '\u{e2a4}',
        Lucide::ChartColumnStacked => '\u{e60f}',
        Lucide::ChartLine => '\u{e2a5}',
        Lucide::ChartNetwork => '\u{e610}',
        Lucide::ChartNoAxesColumn => '\u{e06c}',
        Lucide::ChartNoAxesColumnDecreasing => '\u{e06d}',
        Lucide::ChartNoAxesColumnIncreasing => '\u{e06e}',
        Lucide::ChartNoAxesCombined => '\u{e611}',
        Lucide::ChartNoAxesGantt => '\u{e4c9}',
        Lucide::ChartPie => '\u{e06f}',
        Lucide::ChartScatter => '\u{e48f}',
        Lucide::ChartSpline => '\u{e612}',
        Lucide::Check => '\u{e070}',
        Lucide::CheckCheck => '\u{e392}',
        Lucide::ChefHat => '\u{e2ac}',
        Lucide::Cherry => '\u{e354}',
        Lucide::ChevronDown => '\u{e071}',
        Lucide::ChevronFirst => '\u{e243}',
        Lucide::ChevronLast => '\u{e244}',
        Lucide::ChevronLeft => '\u{e072}',
        Lucide::ChevronRight => '\u{e073}',
        Lucide::ChevronUp => '\u{e074}',
        Lucide::ChevronsDown => '\u{e075}',
        Lucide::ChevronsDownUp => '\u{e228}',
        Lucide::ChevronsLeft => '\u{e076}',
        Lucide::ChevronsLeftRight => '\u{e293}',
        Lucide::ChevronsRight => '\u{e077}',
        Lucide::ChevronsRightLeft => '\u{e294}',
        Lucide::ChevronsUp => '\u{e078}',
        Lucide::ChevronsUpDown => '\u{e211}',
        Lucide::Chrome => '\u{e079}',
        Lucide::Church => '\u{e3e5}',
        Lucide::Cigarette => '\u{e2c6}',
        Lucide::CigaretteOff => '\u{e2c7}',
        Lucide::Circle => '\u{e07a}',
        Lucide::CircleAlert => '\u{e07b}',
        Lucide::CircleArrowDown => '\u{e07c}',
        Lucide::CircleArrowLeft => '\u{e07d}',
        Lucide::CircleArrowOutDownLeft => '\u{e3fc}',
        Lucide::CircleArrowOutDownRight => '\u{e3fd}',
        Lucide::CircleArrowOutUpLeft => '\u{e3fe}',
        Lucide::CircleArrowOutUpRight => '\u{e3ff}',
        Lucide::CircleArrowRight => '\u{e07e}',
        Lucide::CircleArrowUp => '\u{e07f}',
        Lucide::CircleCheck => '\u{e226}',
        Lucide::CircleCheckBig => '\u{e080}',
        Lucide::CircleChevronDown => '\u{e4e2}',
        Lucide::CircleChevronLeft => '\u{e4e3}',
        Lucide::CircleChevronRight => '\u{e4e4}',
        Lucide::CircleChevronUp => '\u{e4e5}',
        Lucide::CircleDashed => '\u{e4b5}',
        Lucide::CircleDivide => '\u{e081}',
        Lucide::CircleDollarSign => '\u{e482}',
        Lucide::CircleDot => '\u{e349}',
        Lucide::CircleDotDashed => '\u{e4b6}',
        Lucide::CircleEllipsis => '\u{e34a}',
        Lucide::CircleEqual => '\u{e405}',
        Lucide::CircleFadingPlus => '\u{e5c1}',
        Lucide::CircleGauge => '\u{e4e6}',
        Lucide::CircleHelp => '\u{e082}',
        Lucide::CircleMinus => '\u{e083}',
        Lucide::CircleOff => '\u{e406}',
        Lucide::CircleParking => '\u{e3cd}',
        Lucide::CircleParkingOff => '\u{e3ce}',
        Lucide::CirclePause => '\u{e084}',
        Lucide::CirclePercent => '\u{e51f}',
        Lucide::CirclePlay => '\u{e085}',
        Lucide::CirclePlus => '\u{e086}',
        Lucide::CirclePower => '\u{e555}',
        Lucide::CircleSlash => '\u{e407}',
        Lucide::CircleSlashTwo => '\u{e213}',
        Lucide::CircleStop => '\u{e087}',
        Lucide::CircleUser => '\u{e466}',
        Lucide::CircleUserRound => '\u{e467}',
        Lucide::CircleX => '\u{e088}',
        Lucide::CircuitBoard => '\u{e408}',
        Lucide::Citrus => '\u{e379}',
        Lucide::Clapperboard => '\u{e29b}',
        Lucide::Clipboard => '\u{e089}',
        Lucide::ClipboardCheck => '\u{e219}',
        Lucide::ClipboardCopy => '\u{e225}',
        Lucide::ClipboardList => '\u{e08a}',
        Lucide::ClipboardMinus => '\u{e5c3}',
        Lucide::ClipboardPaste => '\u{e3ec}',
        Lucide::ClipboardPen => '\u{e307}',
        Lucide::ClipboardPenLine => '\u{e308}',
        Lucide::ClipboardPlus => '\u{e5c4}',
        Lucide::ClipboardType => '\u{e309}',
        Lucide::ClipboardX => '\u{e222}',
        Lucide::ClockArrowDown => '\u{e605}',
        Lucide::ClockArrowUp => '\u{e606}',
        Lucide::ClockEight => '\u{e255}',
        Lucide::ClockFive => '\u{e252}',
        Lucide::ClockFour => '\u{e251}',
        Lucide::ClockNine => '\u{e256}',
        Lucide::ClockOne => '\u{e24b}',
        Lucide::ClockOneone => '\u{e24d}',
        Lucide::ClockOnetwo => '\u{e24e}',
        Lucide::ClockOnezero => '\u{e24c}',
        Lucide::ClockSeven => '\u{e254}',
        Lucide::ClockSix => '\u{e253}',
        Lucide::ClockThree => '\u{e250}',
        Lucide::ClockTwo => '\u{e24f}',
        Lucide::Cloud => '\u{e08c}',
        Lucide::CloudCog => '\u{e30a}',
        Lucide::CloudDownload => '\u{e08d}',
        Lucide::CloudDrizzle => '\u{e08e}',
        Lucide::CloudFog => '\u{e214}',
        Lucide::CloudHail => '\u{e08f}',
        Lucide::CloudLightning => '\u{e090}',
        Lucide::CloudMoon => '\u{e215}',
        Lucide::CloudMoonRain => '\u{e2fa}',
        Lucide::CloudOff => '\u{e091}',
        Lucide::CloudRain => '\u{e092}',
        Lucide::CloudRainWind => '\u{e093}',
        Lucide::CloudSnow => '\u{e094}',
        Lucide::CloudSun => '\u{e216}',
        Lucide::CloudSunRain => '\u{e2fb}',
        Lucide::CloudUpload => '\u{e095}',
        Lucide::Cloudy => '\u{e217}',
        Lucide::Clover => '\u{e096}',
        Lucide::Club => '\u{e49b}',
        Lucide::Code => '\u{e097}',
        Lucide::CodeXml => '\u{e206}',
        Lucide::Codepen => '\u{e098}',
        Lucide::Codesandbox => '\u{e099}',
        Lucide::Coffee => '\u{e09a}',
        Lucide::Cog => '\u{e30b}',
        Lucide::Coins => '\u{e09b}',
        Lucide::ColumnsFour => '\u{e58e}',
        Lucide::ColumnsThree => '\u{e09d}',
        Lucide::ColumnsTwo => '\u{e09c}',
        Lucide::Combine => '\u{e451}',
        Lucide::Command => '\u{e09e}',
        Lucide::Compass => '\u{e09f}',
        Lucide::Component => '\u{e2ad}',
        Lucide::Computer => '\u{e4e9}',
        Lucide::ConciergeBell => '\u{e37c}',
        Lucide::Cone => '\u{e528}',
        Lucide::Construction => '\u{e3b8}',
        Lucide::Contact => '\u{e0a0}',
        Lucide::ContactRound => '\u{e468}',
        Lucide::Container => '\u{e4da}',
        Lucide::Contrast => '\u{e0a1}',
        Lucide::Cookie => '\u{e26b}',
        Lucide::CookingPot => '\u{e589}',
        Lucide::Copy => '\u{e0a2}',
        Lucide::CopyCheck => '\u{e400}',
        Lucide::CopyMinus => '\u{e401}',
        Lucide::CopyPlus => '\u{e402}',
        Lucide::CopySlash => '\u{e403}',
        Lucide::CopyX => '\u{e404}',
        Lucide::Copyleft => '\u{e0a3}',
        Lucide::Copyright => '\u{e0a4}',
        Lucide::CornerDownLeft => '\u{e0a5}',
        Lucide::CornerDownRight => '\u{e0a6}',
        Lucide::CornerLeftDown => '\u{e0a7}',
        Lucide::CornerLeftUp => '\u{e0a8}',
        Lucide::CornerRightDown => '\u{e0a9}',
        Lucide::CornerRightUp => '\u{e0aa}',
        Lucide::CornerUpLeft => '\u{e0ab}',
        Lucide::CornerUpRight => '\u{e0ac}',
        Lucide::Cpu => '\u{e0ad}',
        Lucide::CreativeCommons => '\u{e3b6}',
        Lucide::CreditCard => '\u{e0ae}',
        Lucide::Croissant => '\u{e2ae}',
        Lucide::Crop => '\u{e0af}',
        Lucide::Cross => '\u{e1e5}',
        Lucide::Crosshair => '\u{e0b0}',
        Lucide::Crown => '\u{e1d6}',
        Lucide::Cuboid => '\u{e529}',
        Lucide::CupSoda => '\u{e2d1}',
        Lucide::Currency => '\u{e230}',
        Lucide::Cylinder => '\u{e52a}',
        Lucide::Dam => '\u{e60b}',
        Lucide::Database => '\u{e0b1}',
        Lucide::DatabaseBackup => '\u{e3af}',
        Lucide::DatabaseZap => '\u{e510}',
        Lucide::Delete => '\u{e0b2}',
        Lucide::Dessert => '\u{e4c0}',
        Lucide::Diameter => '\u{e52b}',
        Lucide::Diamond => '\u{e2d2}',
        Lucide::DiamondMinus => '\u{e5e6}',
        Lucide::DiamondPercent => '\u{e520}',
        Lucide::DiamondPlus => '\u{e5e7}',
        Lucide::DiceFive => '\u{e28b}',
        Lucide::DiceFour => '\u{e28a}',
        Lucide::DiceOne => '\u{e287}',
        Lucide::DiceSix => '\u{e28c}',
        Lucide::DiceThree => '\u{e289}',
        Lucide::DiceTwo => '\u{e288}',
        Lucide::Dices => '\u{e2c5}',
        Lucide::Diff => '\u{e30c}',
        Lucide::Disc => '\u{e0b3}',
        Lucide::DiscAlbum => '\u{e561}',
        Lucide::DiscThree => '\u{e499}',
        Lucide::DiscTwo => '\u{e3fb}',
        Lucide::Divide => '\u{e0b4}',
        Lucide::Dna => '\u{e397}',
        Lucide::DnaOff => '\u{e398}',
        Lucide::Dock => '\u{e5d8}',
        Lucide::Dog => '\u{e391}',
        Lucide::DollarSign => '\u{e0b5}',
        Lucide::Donut => '\u{e4c1}',
        Lucide::DoorClosed => '\u{e3d9}',
        Lucide::DoorOpen => '\u{e3da}',
        Lucide::Dot => '\u{e454}',
        Lucide::Download => '\u{e0b6}',
        Lucide::DraftingCompass => '\u{e52c}',
        Lucide::Drama => '\u{e526}',
        Lucide::Dribbble => '\u{e0b7}',
        Lucide::Drill => '\u{e592}',
        Lucide::Droplet => '\u{e0b8}',
        Lucide::Droplets => '\u{e0b9}',
        Lucide::Drum => '\u{e562}',
        Lucide::Drumstick => '\u{e25b}',
        Lucide::Dumbbell => '\u{e3a5}',
        Lucide::Ear => '\u{e386}',
        Lucide::EarOff => '\u{e387}',
        Lucide::Earth => '\u{e1f3}',
        Lucide::EarthLock => '\u{e5d1}',
        Lucide::Eclipse => '\u{e5a2}',
        Lucide::Egg => '\u{e25d}',
        Lucide::EggFried => '\u{e355}',
        Lucide::EggOff => '\u{e399}',
        Lucide::Ellipsis => '\u{e0ba}',
        Lucide::EllipsisVertical => '\u{e0bb}',
        Lucide::Equal => '\u{e1be}',
        Lucide::EqualNot => '\u{e1bf}',
        Lucide::Eraser => '\u{e28f}',
        Lucide::Euro => '\u{e0bc}',
        Lucide::Expand => '\u{e21a}',
        Lucide::ExternalLink => '\u{e0bd}',
        Lucide::Eye => '\u{e0be}',
        Lucide::EyeOff => '\u{e0bf}',
        Lucide::Facebook => '\u{e0c0}',
        Lucide::Factory => '\u{e29f}',
        Lucide::Fan => '\u{e37d}',
        Lucide::FastForward => '\u{e0c1}',
        Lucide::Feather => '\u{e0c2}',
        Lucide::Fence => '\u{e587}',
        Lucide::FerrisWheel => '\u{e484}',
        Lucide::Figma => '\u{e0c3}',
        Lucide::File => '\u{e0c4}',
        Lucide::FileArchive => '\u{e30d}',
        Lucide::FileAudio => '\u{e30e}',
        Lucide::FileAudioTwo => '\u{e30f}',
        Lucide::FileAxisThreed => '\u{e310}',
        Lucide::FileBadge => '\u{e311}',
        Lucide::FileBadgeTwo => '\u{e312}',
        Lucide::FileBox => '\u{e313}',
        Lucide::FileChartColumn => '\u{e314}',
        Lucide::FileChartColumnIncreasing => '\u{e315}',
        Lucide::FileChartLine => '\u{e316}',
        Lucide::FileChartPie => '\u{e317}',
        Lucide::FileCheck => '\u{e0c5}',
        Lucide::FileCheckTwo => '\u{e0c6}',
        Lucide::FileClock => '\u{e318}',
        Lucide::FileCode => '\u{e0c7}',
        Lucide::FileCodeTwo => '\u{e463}',
        Lucide::FileCog => '\u{e319}',
        Lucide::FileDiff => '\u{e31a}',
        Lucide::FileDigit => '\u{e0c8}',
        Lucide::FileDown => '\u{e31b}',
        Lucide::FileHeart => '\u{e31c}',
        Lucide::FileImage => '\u{e31d}',
        Lucide::FileInput => '\u{e0c9}',
        Lucide::FileJson => '\u{e36f}',
        Lucide::FileJsonTwo => '\u{e370}',
        Lucide::FileKey => '\u{e31e}',
        Lucide::FileKeyTwo => '\u{e31f}',
        Lucide::FileLock => '\u{e320}',
        Lucide::FileLockTwo => '\u{e321}',
        Lucide::FileMinus => '\u{e0ca}',
        Lucide::FileMinusTwo => '\u{e0cb}',
        Lucide::FileMusic => '\u{e563}',
        Lucide::FileOutput => '\u{e0cc}',
        Lucide::FilePen => '\u{e322}',
        Lucide::FilePenLine => '\u{e323}',
        Lucide::FilePlus => '\u{e0cd}',
        Lucide::FilePlusTwo => '\u{e0ce}',
        Lucide::FileQuestion => '\u{e324}',
        Lucide::FileScan => '\u{e325}',
        Lucide::FileSearch => '\u{e0cf}',
        Lucide::FileSearchTwo => '\u{e326}',
        Lucide::FileSliders => '\u{e5a5}',
        Lucide::FileSpreadsheet => '\u{e327}',
        Lucide::FileStack => '\u{e4a6}',
        Lucide::FileSymlink => '\u{e328}',
        Lucide::FileTerminal => '\u{e329}',
        Lucide::FileText => '\u{e0d0}',
        Lucide::FileType => '\u{e32a}',
        Lucide::FileTypeTwo => '\u{e371}',
        Lucide::FileUp => '\u{e32b}',
        Lucide::FileVideo => '\u{e32c}',
        Lucide::FileVideoTwo => '\u{e32d}',
        Lucide::FileVolume => '\u{e32e}',
        Lucide::FileVolumeTwo => '\u{e32f}',
        Lucide::FileWarning => '\u{e330}',
        Lucide::FileX => '\u{e0d1}',
        Lucide::FileXTwo => '\u{e0d2}',
        Lucide::Files => '\u{e0d3}',
        Lucide::Film => '\u{e0d4}',
        Lucide::Filter => '\u{e0d5}',
        Lucide::FilterX => '\u{e3b9}',
        Lucide::Fingerprint => '\u{e2cb}',
        Lucide::FireExtinguisher => '\u{e583}',
        Lucide::Fish => '\u{e3aa}',
        Lucide::FishOff => '\u{e3b4}',
        Lucide::FishSymbol => '\u{e4f9}',
        Lucide::Flag => '\u{e0d6}',
        Lucide::FlagOff => '\u{e292}',
        Lucide::FlagTriangleLeft => '\u{e237}',
        Lucide::FlagTriangleRight => '\u{e238}',
        Lucide::Flame => '\u{e0d7}',
        Lucide::FlameKindling => '\u{e53f}',
        Lucide::Flashlight => '\u{e0d8}',
        Lucide::FlashlightOff => '\u{e0d9}',
        Lucide::FlaskConical => '\u{e0da}',
        Lucide::FlaskConicalOff => '\u{e39a}',
        Lucide::FlaskRound => '\u{e0db}',
        Lucide::FlipHorizontal => '\u{e361}',
        Lucide::FlipHorizontalTwo => '\u{e362}',
        Lucide::FlipVertical => '\u{e363}',
        Lucide::FlipVerticalTwo => '\u{e364}',
        Lucide::Flower => '\u{e2d3}',
        Lucide::FlowerTwo => '\u{e2d4}',
        Lucide::Focus => '\u{e29e}',
        Lucide::FoldHorizontal => '\u{e440}',
        Lucide::FoldVertical => '\u{e441}',
        Lucide::Folder => '\u{e0dc}',
        Lucide::FolderArchive => '\u{e331}',
        Lucide::FolderCheck => '\u{e332}',
        Lucide::FolderClock => '\u{e333}',
        Lucide::FolderClosed => '\u{e334}',
        Lucide::FolderCode => '\u{e600}',
        Lucide::FolderCog => '\u{e335}',
        Lucide::FolderDot => '\u{e4ca}',
        Lucide::FolderDown => '\u{e336}',
        Lucide::FolderGit => '\u{e40e}',
        Lucide::FolderGitTwo => '\u{e40f}',
        Lucide::FolderHeart => '\u{e337}',
        Lucide::FolderInput => '\u{e338}',
        Lucide::FolderKanban => '\u{e4cb}',
        Lucide::FolderKey => '\u{e339}',
        Lucide::FolderLock => '\u{e33a}',
        Lucide::FolderMinus => '\u{e0dd}',
        Lucide::FolderOpen => '\u{e247}',
        Lucide::FolderOpenDot => '\u{e4cc}',
        Lucide::FolderOutput => '\u{e33b}',
        Lucide::FolderPen => '\u{e33c}',
        Lucide::FolderPlus => '\u{e0de}',
        Lucide::FolderRoot => '\u{e4cd}',
        Lucide::FolderSearch => '\u{e33d}',
        Lucide::FolderSearchTwo => '\u{e33e}',
        Lucide::FolderSymlink => '\u{e33f}',
        Lucide::FolderSync => '\u{e4ce}',
        Lucide::FolderTree => '\u{e340}',
        Lucide::FolderUp => '\u{e341}',
        Lucide::FolderX => '\u{e342}',
        Lucide::Folders => '\u{e343}',
        Lucide::Footprints => '\u{e3bd}',
        Lucide::Forklift => '\u{e3c5}',
        Lucide::Forward => '\u{e229}',
        Lucide::Frame => '\u{e291}',
        Lucide::Framer => '\u{e0df}',
        Lucide::Frown => '\u{e0e0}',
        Lucide::Fuel => '\u{e2af}',
        Lucide::Fullscreen => '\u{e539}',
        Lucide::GalleryHorizontal => '\u{e4d3}',
        Lucide::GalleryHorizontalEnd => '\u{e4d4}',
        Lucide::GalleryThumbnails => '\u{e4d5}',
        Lucide::GalleryVertical => '\u{e4d6}',
        Lucide::GalleryVerticalEnd => '\u{e4d7}',
        Lucide::Gamepad => '\u{e0e1}',
        Lucide::GamepadTwo => '\u{e0e2}',
        Lucide::Gauge => '\u{e1c0}',
        Lucide::Gavel => '\u{e0e3}',
        Lucide::Gem => '\u{e242}',
        Lucide::Ghost => '\u{e20e}',
        Lucide::Gift => '\u{e0e4}',
        Lucide::GitBranch => '\u{e0e5}',
        Lucide::GitBranchPlus => '\u{e1f4}',
        Lucide::GitCommitHorizontal => '\u{e0e6}',
        Lucide::GitCommitVertical => '\u{e557}',
        Lucide::GitCompare => '\u{e35d}',
        Lucide::GitCompareArrows => '\u{e558}',
        Lucide::GitFork => '\u{e28d}',
        Lucide::GitGraph => '\u{e559}',
        Lucide::GitMerge => '\u{e0e7}',
        Lucide::GitPullRequest => '\u{e0e8}',
        Lucide::GitPullRequestArrow => '\u{e55a}',
        Lucide::GitPullRequestClosed => '\u{e35e}',
        Lucide::GitPullRequestCreate => '\u{e55b}',
        Lucide::GitPullRequestCreateArrow => '\u{e55c}',
        Lucide::GitPullRequestDraft => '\u{e35f}',
        Lucide::Github => '\u{e0e9}',
        Lucide::Gitlab => '\u{e0ea}',
        Lucide::GlassWater => '\u{e2d5}',
        Lucide::Glasses => '\u{e20d}',
        Lucide::Globe => '\u{e0eb}',
        Lucide::GlobeLock => '\u{e5d2}',
        Lucide::Goal => '\u{e4aa}',
        Lucide::Grab => '\u{e1e6}',
        Lucide::GraduationCap => '\u{e234}',
        Lucide::Grape => '\u{e356}',
        Lucide::GridThreexthree => '\u{e0ec}',
        Lucide::GridTwoxtwo => '\u{e504}',
        Lucide::GridTwoxtwoCheck => '\u{e5e9}',
        Lucide::GridTwoxtwoX => '\u{e5ea}',
        Lucide::Grip => '\u{e3b5}',
        Lucide::GripHorizontal => '\u{e0ed}',
        Lucide::GripVertical => '\u{e0ee}',
        Lucide::Group => '\u{e469}',
        Lucide::Guitar => '\u{e564}',
        Lucide::Ham => '\u{e5dc}',
        Lucide::Hammer => '\u{e0ef}',
        Lucide::Hand => '\u{e1d7}',
        Lucide::HandCoins => '\u{e5bd}',
        Lucide::HandHeart => '\u{e5be}',
        Lucide::HandHelping => '\u{e3bc}',
        Lucide::HandMetal => '\u{e22c}',
        Lucide::HandPlatter => '\u{e5bf}',
        Lucide::Handshake => '\u{e5c5}',
        Lucide::HardDrive => '\u{e0f0}',
        Lucide::HardDriveDownload => '\u{e4ea}',
        Lucide::HardDriveUpload => '\u{e4eb}',
        Lucide::HardHat => '\u{e0f1}',
        Lucide::Hash => '\u{e0f2}',
        Lucide::Haze => '\u{e0f3}',
        Lucide::HdmiPort => '\u{e4ec}',
        Lucide::Heading => '\u{e388}',
        Lucide::HeadingFive => '\u{e38d}',
        Lucide::HeadingFour => '\u{e38c}',
        Lucide::HeadingOne => '\u{e389}',
        Lucide::HeadingSix => '\u{e38e}',
        Lucide::HeadingThree => '\u{e38b}',
        Lucide::HeadingTwo => '\u{e38a}',
        Lucide::Headphones => '\u{e0f4}',
        Lucide::Headset => '\u{e5c2}',
        Lucide::Heart => '\u{e0f5}',
        Lucide::HeartCrack => '\u{e2d6}',
        Lucide::HeartHandshake => '\u{e2d7}',
        Lucide::HeartOff => '\u{e295}',
        Lucide::HeartPulse => '\u{e372}',
        Lucide::Heater => '\u{e593}',
        Lucide::Hexagon => '\u{e0f6}',
        Lucide::Highlighter => '\u{e0f7}',
        Lucide::History => '\u{e1f5}',
        Lucide::Hop => '\u{e39b}',
        Lucide::HopOff => '\u{e39c}',
        Lucide::Hospital => '\u{e5dd}',
        Lucide::Hotel => '\u{e3e6}',
        Lucide::Hourglass => '\u{e296}',
        Lucide::House => '\u{e0f8}',
        Lucide::HousePlug => '\u{e5f5}',
        Lucide::HousePlus => '\u{e5f6}',
        Lucide::IceCreamBowl => '\u{e3ab}',
        Lucide::IceCreamCone => '\u{e357}',
        Lucide::Image => '\u{e0f9}',
        Lucide::ImageDown => '\u{e541}',
        Lucide::ImageMinus => '\u{e1f6}',
        Lucide::ImageOff => '\u{e1c1}',
        Lucide::ImagePlay => '\u{e5e4}',
        Lucide::ImagePlus => '\u{e1f7}',
        Lucide::ImageUp => '\u{e5d0}',
        Lucide::Images => '\u{e5c9}',
        Lucide::Import => '\u{e22f}',
        Lucide::Inbox => '\u{e0fa}',
        Lucide::IndentDecrease => '\u{e0fb}',
        Lucide::IndentIncrease => '\u{e0fc}',
        Lucide::IndianRupee => '\u{e0fd}',
        Lucide::Infinity => '\u{e1e7}',
        Lucide::Info => '\u{e0fe}',
        Lucide::InspectionPanel => '\u{e588}',
        Lucide::Instagram => '\u{e0ff}',
        Lucide::Italic => '\u{e100}',
        Lucide::IterationCcw => '\u{e428}',
        Lucide::IterationCw => '\u{e429}',
        Lucide::JapaneseYen => '\u{e101}',
        Lucide::Joystick => '\u{e359}',
        Lucide::Kanban => '\u{e4e1}',
        Lucide::Key => '\u{e102}',
        Lucide::KeyRound => '\u{e4a8}',
        Lucide::KeySquare => '\u{e4a9}',
        Lucide::Keyboard => '\u{e284}',
        Lucide::KeyboardMusic => '\u{e565}',
        Lucide::KeyboardOff => '\u{e5e3}',
        Lucide::Lamp => '\u{e2d8}',
        Lucide::LampCeiling => '\u{e2d9}',
        Lucide::LampDesk => '\u{e2da}',
        Lucide::LampFloor => '\u{e2db}',
        Lucide::LampWallDown => '\u{e2dc}',
        Lucide::LampWallUp => '\u{e2dd}',
        Lucide::LandPlot => '\u{e52d}',
        Lucide::Landmark => '\u{e23a}',
        Lucide::Languages => '\u{e103}',
        Lucide::Laptop => '\u{e1cd}',
        Lucide::LaptopMinimal => '\u{e1d8}',
        Lucide::Lasso => '\u{e1ce}',
        Lucide::LassoSelect => '\u{e1cf}',
        Lucide::Laugh => '\u{e300}',
        Lucide::Layers => '\u{e104}',
        Lucide::LayersThree => '\u{e52f}',
        Lucide::LayersTwo => '\u{e52e}',
        Lucide::LayoutDashboard => '\u{e1c2}',
        Lucide::LayoutGrid => '\u{e105}',
        Lucide::LayoutList => '\u{e1d9}',
        Lucide::LayoutPanelLeft => '\u{e475}',
        Lucide::LayoutPanelTop => '\u{e476}',
        Lucide::LayoutTemplate => '\u{e207}',
        Lucide::Leaf => '\u{e2de}',
        Lucide::LeafyGreen => '\u{e474}',
        Lucide::Lectern => '\u{e5ee}',
        Lucide::LetterText => '\u{e60a}',
        Lucide::Library => '\u{e106}',
        Lucide::LibraryBig => '\u{e553}',
        Lucide::LifeBuoy => '\u{e107}',
        Lucide::Ligature => '\u{e43f}',
        Lucide::Lightbulb => '\u{e1c3}',
        Lucide::LightbulbOff => '\u{e208}',
        Lucide::Link => '\u{e108}',
        Lucide::LinkTwo => '\u{e109}',
        Lucide::LinkTwoOff => '\u{e10a}',
        Lucide::Linkedin => '\u{e10b}',
        Lucide::List => '\u{e10c}',
        Lucide::ListCheck => '\u{e5ff}',
        Lucide::ListChecks => '\u{e1d0}',
        Lucide::ListCollapse => '\u{e5a0}',
        Lucide::ListEnd => '\u{e2df}',
        Lucide::ListFilter => '\u{e465}',
        Lucide::ListMinus => '\u{e23e}',
        Lucide::ListMusic => '\u{e2e0}',
        Lucide::ListOrdered => '\u{e1d1}',
        Lucide::ListPlus => '\u{e23f}',
        Lucide::ListRestart => '\u{e457}',
        Lucide::ListStart => '\u{e2e1}',
        Lucide::ListTodo => '\u{e4c8}',
        Lucide::ListTree => '\u{e40d}',
        Lucide::ListVideo => '\u{e2e2}',
        Lucide::ListX => '\u{e240}',
        Lucide::Loader => '\u{e10d}',
        Lucide::LoaderCircle => '\u{e10e}',
        Lucide::LoaderPinwheel => '\u{e5eb}',
        Lucide::Locate => '\u{e1da}',
        Lucide::LocateFixed => '\u{e1db}',
        Lucide::LocateOff => '\u{e282}',
        Lucide::Lock => '\u{e10f}',
        Lucide::LockKeyhole => '\u{e536}',
        Lucide::LockKeyholeOpen => '\u{e537}',
        Lucide::LockOpen => '\u{e110}',
        Lucide::LogIn => '\u{e111}',
        Lucide::LogOut => '\u{e112}',
        Lucide::Logs => '\u{e5f9}',
        Lucide::Lollipop => '\u{e4c2}',
        Lucide::Luggage => '\u{e2ca}',
        Lucide::Magnet => '\u{e2b5}',
        Lucide::Mail => '\u{e113}',
        Lucide::MailCheck => '\u{e365}',
        Lucide::MailMinus => '\u{e366}',
        Lucide::MailOpen => '\u{e367}',
        Lucide::MailPlus => '\u{e368}',
        Lucide::MailQuestion => '\u{e369}',
        Lucide::MailSearch => '\u{e36a}',
        Lucide::MailWarning => '\u{e36b}',
        Lucide::MailX => '\u{e36c}',
        Lucide::Mailbox => '\u{e3d8}',
        Lucide::Mails => '\u{e36d}',
        Lucide::Map => '\u{e114}',
        Lucide::MapPin => '\u{e115}',
        Lucide::MapPinOff => '\u{e2a6}',
        Lucide::MapPinned => '\u{e542}',
        Lucide::Martini => '\u{e2e3}',
        Lucide::Maximize => '\u{e116}',
        Lucide::MaximizeTwo => '\u{e117}',
        Lucide::Medal => '\u{e373}',
        Lucide::Megaphone => '\u{e235}',
        Lucide::MegaphoneOff => '\u{e374}',
        Lucide::Meh => '\u{e118}',
        Lucide::MemoryStick => '\u{e44a}',
        Lucide::Menu => '\u{e119}',
        Lucide::Merge => '\u{e444}',
        Lucide::MessageCircle => '\u{e11a}',
        Lucide::MessageCircleCode => '\u{e567}',
        Lucide::MessageCircleDashed => '\u{e568}',
        Lucide::MessageCircleHeart => '\u{e569}',
        Lucide::MessageCircleMore => '\u{e56a}',
        Lucide::MessageCircleOff => '\u{e56b}',
        Lucide::MessageCirclePlus => '\u{e56c}',
        Lucide::MessageCircleQuestion => '\u{e56d}',
        Lucide::MessageCircleReply => '\u{e56e}',
        Lucide::MessageCircleWarning => '\u{e56f}',
        Lucide::MessageCircleX => '\u{e570}',
        Lucide::MessageSquare => '\u{e11b}',
        Lucide::MessageSquareCode => '\u{e571}',
        Lucide::MessageSquareDashed => '\u{e410}',
        Lucide::MessageSquareDiff => '\u{e572}',
        Lucide::MessageSquareDot => '\u{e573}',
        Lucide::MessageSquareHeart => '\u{e574}',
        Lucide::MessageSquareMore => '\u{e575}',
        Lucide::MessageSquareOff => '\u{e576}',
        Lucide::MessageSquarePlus => '\u{e411}',
        Lucide::MessageSquareQuote => '\u{e577}',
        Lucide::MessageSquareReply => '\u{e578}',
        Lucide::MessageSquareShare => '\u{e579}',
        Lucide::MessageSquareText => '\u{e57a}',
        Lucide::MessageSquareWarning => '\u{e57b}',
        Lucide::MessageSquareX => '\u{e57c}',
        Lucide::MessagesSquare => '\u{e412}',
        Lucide::Mic => '\u{e11c}',
        Lucide::MicOff => '\u{e11d}',
        Lucide::MicVocal => '\u{e34d}',
        Lucide::Microscope => '\u{e2e4}',
        Lucide::Microwave => '\u{e37e}',
        Lucide::Milestone => '\u{e298}',
        Lucide::Milk => '\u{e39d}',
        Lucide::MilkOff => '\u{e39e}',
        Lucide::Minimize => '\u{e11e}',
        Lucide::MinimizeTwo => '\u{e11f}',
        Lucide::Minus => '\u{e120}',
        Lucide::Monitor => '\u{e121}',
        Lucide::MonitorCheck => '\u{e487}',
        Lucide::MonitorCog => '\u{e608}',
        Lucide::MonitorDot => '\u{e488}',
        Lucide::MonitorDown => '\u{e426}',
        Lucide::MonitorOff => '\u{e1dc}',
        Lucide::MonitorPause => '\u{e489}',
        Lucide::MonitorPlay => '\u{e48a}',
        Lucide::MonitorSmartphone => '\u{e3a6}',
        Lucide::MonitorSpeaker => '\u{e210}',
        Lucide::MonitorStop => '\u{e48b}',
        Lucide::MonitorUp => '\u{e427}',
        Lucide::MonitorX => '\u{e48c}',
        Lucide::Moon => '\u{e122}',
        Lucide::MoonStar => '\u{e415}',
        Lucide::Mountain => '\u{e231}',
        Lucide::MountainSnow => '\u{e232}',
        Lucide::Mouse => '\u{e28e}',
        Lucide::MouseOff => '\u{e5e0}',
        Lucide::MousePointer => '\u{e123}',
        Lucide::MousePointerBan => '\u{e5ec}',
        Lucide::MousePointerClick => '\u{e124}',
        Lucide::MousePointerTwo => '\u{e1c4}',
        Lucide::Move => '\u{e125}',
        Lucide::MoveDiagonal => '\u{e1c5}',
        Lucide::MoveDiagonalTwo => '\u{e1c6}',
        Lucide::MoveDown => '\u{e491}',
        Lucide::MoveDownLeft => '\u{e492}',
        Lucide::MoveDownRight => '\u{e493}',
        Lucide::MoveHorizontal => '\u{e1c7}',
        Lucide::MoveLeft => '\u{e494}',
        Lucide::MoveRight => '\u{e495}',
        Lucide::MoveThreed => '\u{e2e5}',
        Lucide::MoveUp => '\u{e496}',
        Lucide::MoveUpLeft => '\u{e497}',
        Lucide::MoveUpRight => '\u{e498}',
        Lucide::MoveVertical => '\u{e1c8}',
        Lucide::Music => '\u{e126}',
        Lucide::MusicFour => '\u{e350}',
        Lucide::MusicThree => '\u{e34f}',
        Lucide::MusicTwo => '\u{e34e}',
        Lucide::Navigation => '\u{e127}',
        Lucide::NavigationOff => '\u{e2a8}',
        Lucide::NavigationTwo => '\u{e128}',
        Lucide::NavigationTwoOff => '\u{e2a7}',
        Lucide::Network => '\u{e129}',
        Lucide::Newspaper => '\u{e34c}',
        Lucide::Nfc => '\u{e3c7}',
        Lucide::Notebook => '\u{e59a}',
        Lucide::NotebookPen => '\u{e59b}',
        Lucide::NotebookTabs => '\u{e59c}',
        Lucide::NotebookText => '\u{e59d}',
        Lucide::NotepadText => '\u{e59e}',
        Lucide::NotepadTextDashed => '\u{e59f}',
        Lucide::Nut => '\u{e39f}',
        Lucide::NutOff => '\u{e3a0}',
        Lucide::Octagon => '\u{e12a}',
        Lucide::OctagonAlert => '\u{e12b}',
        Lucide::OctagonPause => '\u{e21b}',
        Lucide::OctagonX => '\u{e12c}',
        Lucide::Option => '\u{e1f8}',
        Lucide::Orbit => '\u{e3eb}',
        Lucide::Origami => '\u{e5e8}',
        Lucide::Package => '\u{e12d}',
        Lucide::PackageCheck => '\u{e266}',
        Lucide::PackageMinus => '\u{e267}',
        Lucide::PackageOpen => '\u{e2cc}',
        Lucide::PackagePlus => '\u{e268}',
        Lucide::PackageSearch => '\u{e269}',
        Lucide::PackageTwo => '\u{e344}',
        Lucide::PackageX => '\u{e26a}',
        Lucide::PaintBucket => '\u{e2e6}',
        Lucide::PaintRoller => '\u{e5a3}',
        Lucide::Paintbrush => '\u{e2e7}',
        Lucide::PaintbrushVertical => '\u{e2e8}',
        Lucide::Palette => '\u{e1dd}',
        Lucide::PanelBottom => '\u{e431}',
        Lucide::PanelBottomClose => '\u{e432}',
        Lucide::PanelBottomDashed => '\u{e433}',
        Lucide::PanelBottomOpen => '\u{e434}',
        Lucide::PanelLeft => '\u{e12e}',
        Lucide::PanelLeftClose => '\u{e21c}',
        Lucide::PanelLeftDashed => '\u{e435}',
        Lucide::PanelLeftOpen => '\u{e21d}',
        Lucide::PanelRight => '\u{e436}',
        Lucide::PanelRightClose => '\u{e437}',
        Lucide::PanelRightDashed => '\u{e438}',
        Lucide::PanelRightOpen => '\u{e439}',
        Lucide::PanelTop => '\u{e43a}',
        Lucide::PanelTopClose => '\u{e43b}',
        Lucide::PanelTopDashed => '\u{e43c}',
        Lucide::PanelTopOpen => '\u{e43d}',
        Lucide::PanelsLeftBottom => '\u{e12f}',
        Lucide::PanelsRightBottom => '\u{e58d}',
        Lucide::PanelsTopLeft => '\u{e130}',
        Lucide::Paperclip => '\u{e131}',
        Lucide::Parentheses => '\u{e449}',
        Lucide::ParkingMeter => '\u{e505}',
        Lucide::PartyPopper => '\u{e347}',
        Lucide::Pause => '\u{e132}',
        Lucide::PawPrint => '\u{e4fa}',
        Lucide::PcCase => '\u{e44b}',
        Lucide::Pen => '\u{e133}',
        Lucide::PenLine => '\u{e134}',
        Lucide::PenOff => '\u{e5f3}',
        Lucide::PenTool => '\u{e135}',
        Lucide::Pencil => '\u{e1f9}',
        Lucide::PencilLine => '\u{e4f5}',
        Lucide::PencilOff => '\u{e5f4}',
        Lucide::PencilRuler => '\u{e4f6}',
        Lucide::Pentagon => '\u{e530}',
        Lucide::Percent => '\u{e136}',
        Lucide::PersonStanding => '\u{e21e}',
        Lucide::PhilippinePeso => '\u{e609}',
        Lucide::Phone => '\u{e137}',
        Lucide::PhoneCall => '\u{e138}',
        Lucide::PhoneForwarded => '\u{e139}',
        Lucide::PhoneIncoming => '\u{e13a}',
        Lucide::PhoneMissed => '\u{e13b}',
        Lucide::PhoneOff => '\u{e13c}',
        Lucide::PhoneOutgoing => '\u{e13d}',
        Lucide::Pi => '\u{e477}',
        Lucide::Piano => '\u{e566}',
        Lucide::Pickaxe => '\u{e5cb}',
        Lucide::PictureInPicture => '\u{e3b2}',
        Lucide::PictureInPictureTwo => '\u{e3b3}',
        Lucide::PiggyBank => '\u{e13e}',
        Lucide::Pilcrow => '\u{e3a7}',
        Lucide::PilcrowLeft => '\u{e5e1}',
        Lucide::PilcrowRight => '\u{e5e2}',
        Lucide::Pill => '\u{e3c1}',
        Lucide::PillBottle => '\u{e5ef}',
        Lucide::Pin => '\u{e259}',
        Lucide::PinOff => '\u{e2b6}',
        Lucide::Pipette => '\u{e13f}',
        Lucide::Pizza => '\u{e358}',
        Lucide::Plane => '\u{e1de}',
        Lucide::PlaneLanding => '\u{e3d1}',
        Lucide::PlaneTakeoff => '\u{e3d2}',
        Lucide::Play => '\u{e140}',
        Lucide::Plug => '\u{e383}',
        Lucide::PlugTwo => '\u{e384}',
        Lucide::PlugZap => '\u{e461}',
        Lucide::Plus => '\u{e141}',
        Lucide::Pocket => '\u{e142}',
        Lucide::PocketKnife => '\u{e4a5}',
        Lucide::Podcast => '\u{e1fa}',
        Lucide::Pointer => '\u{e1e8}',
        Lucide::PointerOff => '\u{e584}',
        Lucide::Popcorn => '\u{e4c3}',
        Lucide::Popsicle => '\u{e4c4}',
        Lucide::PoundSterling => '\u{e143}',
        Lucide::Power => '\u{e144}',
        Lucide::PowerOff => '\u{e209}',
        Lucide::Presentation => '\u{e4b3}',
        Lucide::Printer => '\u{e145}',
        Lucide::PrinterCheck => '\u{e5fa}',
        Lucide::Projector => '\u{e4b4}',
        Lucide::Proportions => '\u{e5d4}',
        Lucide::Puzzle => '\u{e29c}',
        Lucide::Pyramid => '\u{e531}',
        Lucide::QrCode => '\u{e1df}',
        Lucide::Quote => '\u{e239}',
        Lucide::Rabbit => '\u{e4fb}',
        Lucide::Radar => '\u{e49c}',
        Lucide::Radiation => '\u{e447}',
        Lucide::Radical => '\u{e5c7}',
        Lucide::Radio => '\u{e146}',
        Lucide::RadioReceiver => '\u{e1fb}',
        Lucide::RadioTower => '\u{e409}',
        Lucide::Radius => '\u{e532}',
        Lucide::RailSymbol => '\u{e506}',
        Lucide::Rainbow => '\u{e4c7}',
        Lucide::Rat => '\u{e3f0}',
        Lucide::Ratio => '\u{e4ed}',
        Lucide::Receipt => '\u{e3d7}',
        Lucide::ReceiptCent => '\u{e5aa}',
        Lucide::ReceiptEuro => '\u{e5ab}',
        Lucide::ReceiptIndianRupee => '\u{e5ac}',
        Lucide::ReceiptJapaneseYen => '\u{e5ad}',
        Lucide::ReceiptPoundSterling => '\u{e5ae}',
        Lucide::ReceiptRussianRuble => '\u{e5af}',
        Lucide::ReceiptSwissFranc => '\u{e5b0}',
        Lucide::ReceiptText => '\u{e5b1}',
        Lucide::RectangleEllipsis => '\u{e21f}',
        Lucide::RectangleHorizontal => '\u{e37a}',
        Lucide::RectangleVertical => '\u{e37b}',
        Lucide::Recycle => '\u{e2e9}',
        Lucide::Redo => '\u{e147}',
        Lucide::RedoDot => '\u{e455}',
        Lucide::RedoTwo => '\u{e2a0}',
        Lucide::RefreshCcw => '\u{e148}',
        Lucide::RefreshCcwDot => '\u{e4b7}',
        Lucide::RefreshCw => '\u{e149}',
        Lucide::RefreshCwOff => '\u{e49d}',
        Lucide::Refrigerator => '\u{e37f}',
        Lucide::Regex => '\u{e1fc}',
        Lucide::RemoveFormatting => '\u{e3b7}',
        Lucide::Repeat => '\u{e14a}',
        Lucide::RepeatOne => '\u{e1fd}',
        Lucide::RepeatTwo => '\u{e416}',
        Lucide::Replace => '\u{e3df}',
        Lucide::ReplaceAll => '\u{e3e0}',
        Lucide::Reply => '\u{e22a}',
        Lucide::ReplyAll => '\u{e22b}',
        Lucide::Rewind => '\u{e14b}',
        Lucide::Ribbon => '\u{e55d}',
        Lucide::Rocket => '\u{e286}',
        Lucide::RockingChair => '\u{e233}',
        Lucide::RollerCoaster => '\u{e485}',
        Lucide::RotateCcw => '\u{e14c}',
        Lucide::RotateCcwSquare => '\u{e5d5}',
        Lucide::RotateCw => '\u{e14d}',
        Lucide::RotateCwSquare => '\u{e5d6}',
        Lucide::RotateThreed => '\u{e2ea}',
        Lucide::Route => '\u{e543}',
        Lucide::RouteOff => '\u{e544}',
        Lucide::Router => '\u{e3c3}',
        Lucide::RowsFour => '\u{e590}',
        Lucide::RowsThree => '\u{e58f}',
        Lucide::RowsTwo => '\u{e43e}',
        Lucide::Rss => '\u{e14e}',
        Lucide::Ruler => '\u{e14f}',
        Lucide::RussianRuble => '\u{e150}',
        Lucide::Sailboat => '\u{e382}',
        Lucide::Salad => '\u{e3ac}',
        Lucide::Sandwich => '\u{e3ad}',
        Lucide::Satellite => '\u{e44c}',
        Lucide::SatelliteDish => '\u{e44d}',
        Lucide::Save => '\u{e151}',
        Lucide::SaveAll => '\u{e414}',
        Lucide::SaveOff => '\u{e5f8}',
        Lucide::Scale => '\u{e212}',
        Lucide::ScaleThreed => '\u{e2eb}',
        Lucide::Scaling => '\u{e2ec}',
        Lucide::Scan => '\u{e257}',
        Lucide::ScanBarcode => '\u{e53a}',
        Lucide::ScanEye => '\u{e53b}',
        Lucide::ScanFace => '\u{e375}',
        Lucide::ScanLine => '\u{e258}',
        Lucide::ScanQrCode => '\u{e5fb}',
        Lucide::ScanSearch => '\u{e53c}',
        Lucide::ScanText => '\u{e53d}',
        Lucide::School => '\u{e3e7}',
        Lucide::Scissors => '\u{e152}',
        Lucide::ScissorsLineDashed => '\u{e4ee}',
        Lucide::ScreenShare => '\u{e153}',
        Lucide::ScreenShareOff => '\u{e154}',
        Lucide::Scroll => '\u{e2ed}',
        Lucide::ScrollText => '\u{e464}',
        Lucide::Search => '\u{e155}',
        Lucide::SearchCheck => '\u{e4af}',
        Lucide::SearchCode => '\u{e4b0}',
        Lucide::SearchSlash => '\u{e4b1}',
        Lucide::SearchX => '\u{e4b2}',
        Lucide::Section => '\u{e5ed}',
        Lucide::Send => '\u{e156}',
        Lucide::SendHorizontal => '\u{e4f7}',
        Lucide::SendToBack => '\u{e4f8}',
        Lucide::SeparatorHorizontal => '\u{e1c9}',
        Lucide::SeparatorVertical => '\u{e1ca}',
        Lucide::Server => '\u{e157}',
        Lucide::ServerCog => '\u{e345}',
        Lucide::ServerCrash => '\u{e1e9}',
        Lucide::ServerOff => '\u{e1ea}',
        Lucide::Settings => '\u{e158}',
        Lucide::SettingsTwo => '\u{e245}',
        Lucide::Shapes => '\u{e4b8}',
        Lucide::Share => '\u{e159}',
        Lucide::ShareTwo => '\u{e15a}',
        Lucide::Sheet => '\u{e15b}',
        Lucide::Shell => '\u{e4fc}',
        Lucide::Shield => '\u{e15c}',
        Lucide::ShieldAlert => '\u{e1fe}',
        Lucide::ShieldBan => '\u{e15d}',
        Lucide::ShieldCheck => '\u{e1ff}',
        Lucide::ShieldEllipsis => '\u{e51b}',
        Lucide::ShieldHalf => '\u{e51c}',
        Lucide::ShieldMinus => '\u{e51d}',
        Lucide::ShieldOff => '\u{e15e}',
        Lucide::ShieldPlus => '\u{e51e}',
        Lucide::ShieldQuestion => '\u{e413}',
        Lucide::ShieldX => '\u{e200}',
        Lucide::Ship => '\u{e3be}',
        Lucide::ShipWheel => '\u{e507}',
        Lucide::Shirt => '\u{e1cb}',
        Lucide::ShoppingBag => '\u{e15f}',
        Lucide::ShoppingBasket => '\u{e4ef}',
        Lucide::ShoppingCart => '\u{e160}',
        Lucide::Shovel => '\u{e161}',
        Lucide::ShowerHead => '\u{e380}',
        Lucide::Shrink => '\u{e220}',
        Lucide::Shrub => '\u{e2ee}',
        Lucide::Shuffle => '\u{e162}',
        Lucide::Sigma => '\u{e201}',
        Lucide::Signal => '\u{e25f}',
        Lucide::SignalHigh => '\u{e260}',
        Lucide::SignalLow => '\u{e261}',
        Lucide::SignalMedium => '\u{e262}',
        Lucide::SignalZero => '\u{e263}',
        Lucide::Signature => '\u{e5f7}',
        Lucide::Signpost => '\u{e545}',
        Lucide::SignpostBig => '\u{e546}',
        Lucide::Siren => '\u{e2ef}',
        Lucide::SkipBack => '\u{e163}',
        Lucide::SkipForward => '\u{e164}',
        Lucide::Skull => '\u{e221}',
        Lucide::Slack => '\u{e165}',
        Lucide::Slash => '\u{e522}',
        Lucide::Slice => '\u{e2f0}',
        Lucide::SlidersHorizontal => '\u{e29a}',
        Lucide::SlidersVertical => '\u{e166}',
        Lucide::Smartphone => '\u{e167}',
        Lucide::SmartphoneCharging => '\u{e22e}',
        Lucide::SmartphoneNfc => '\u{e3c8}',
        Lucide::Smile => '\u{e168}',
        Lucide::SmilePlus => '\u{e301}',
        Lucide::Snail => '\u{e4fd}',
        Lucide::Snowflake => '\u{e169}',
        Lucide::Sofa => '\u{e2c4}',
        Lucide::Soup => '\u{e3ae}',
        Lucide::Space => '\u{e3e1}',
        Lucide::Spade => '\u{e49e}',
        Lucide::Sparkle => '\u{e483}',
        Lucide::Sparkles => '\u{e417}',
        Lucide::Speaker => '\u{e16a}',
        Lucide::Speech => '\u{e523}',
        Lucide::SpellCheck => '\u{e49f}',
        Lucide::SpellCheckTwo => '\u{e4a0}',
        Lucide::Spline => '\u{e38f}',
        Lucide::Split => '\u{e445}',
        Lucide::SprayCan => '\u{e49a}',
        Lucide::Sprout => '\u{e1eb}',
        Lucide::Square => '\u{e16b}',
        Lucide::SquareActivity => '\u{e4b9}',
        Lucide::SquareArrowDown => '\u{e42c}',
        Lucide::SquareArrowDownLeft => '\u{e4ba}',
        Lucide::SquareArrowDownRight => '\u{e4bb}',
        Lucide::SquareArrowLeft => '\u{e42d}',
        Lucide::SquareArrowOutDownLeft => '\u{e5a6}',
        Lucide::SquareArrowOutDownRight => '\u{e5a7}',
        Lucide::SquareArrowOutUpLeft => '\u{e5a8}',
        Lucide::SquareArrowOutUpRight => '\u{e5a9}',
        Lucide::SquareArrowRight => '\u{e42e}',
        Lucide::SquareArrowUp => '\u{e42f}',
        Lucide::SquareArrowUpLeft => '\u{e4bc}',
        Lucide::SquareArrowUpRight => '\u{e4bd}',
        Lucide::SquareAsterisk => '\u{e16c}',
        Lucide::SquareBottomDashedScissors => '\u{e4f0}',
        Lucide::SquareChartGantt => '\u{e16d}',
        Lucide::SquareCheck => '\u{e55e}',
        Lucide::SquareCheckBig => '\u{e16e}',
        Lucide::SquareChevronDown => '\u{e3d3}',
        Lucide::SquareChevronLeft => '\u{e3d4}',
        Lucide::SquareChevronRight => '\u{e3d5}',
        Lucide::SquareChevronUp => '\u{e3d6}',
        Lucide::SquareCode => '\u{e16f}',
        Lucide::SquareDashedBottom => '\u{e4c5}',
        Lucide::SquareDashedBottomCode => '\u{e4c6}',
        Lucide::SquareDashedKanban => '\u{e170}',
        Lucide::SquareDashedMousePointer => '\u{e50e}',
        Lucide::SquareDivide => '\u{e171}',
        Lucide::SquareDot => '\u{e172}',
        Lucide::SquareEqual => '\u{e173}',
        Lucide::SquareFunction => '\u{e22d}',
        Lucide::SquareKanban => '\u{e174}',
        Lucide::SquareLibrary => '\u{e554}',
        Lucide::SquareM => '\u{e508}',
        Lucide::SquareMenu => '\u{e458}',
        Lucide::SquareMinus => '\u{e175}',
        Lucide::SquareMousePointer => '\u{e202}',
        Lucide::SquareParking => '\u{e3cf}',
        Lucide::SquareParkingOff => '\u{e3d0}',
        Lucide::SquarePen => '\u{e176}',
        Lucide::SquarePercent => '\u{e521}',
        Lucide::SquarePi => '\u{e48d}',
        Lucide::SquarePilcrow => '\u{e490}',
        Lucide::SquarePlay => '\u{e486}',
        Lucide::SquarePlus => '\u{e177}',
        Lucide::SquarePower => '\u{e556}',
        Lucide::SquareRadical => '\u{e5c8}',
        Lucide::SquareScissors => '\u{e4f1}',
        Lucide::SquareSigma => '\u{e48e}',
        Lucide::SquareSlash => '\u{e178}',
        Lucide::SquareSplitHorizontal => '\u{e3ba}',
        Lucide::SquareSplitVertical => '\u{e3bb}',
        Lucide::SquareStack => '\u{e4a7}',
        Lucide::SquareTerminal => '\u{e20a}',
        Lucide::SquareUser => '\u{e46a}',
        Lucide::SquareUserRound => '\u{e46b}',
        Lucide::SquareX => '\u{e179}',
        Lucide::Squircle => '\u{e57f}',
        Lucide::Squirrel => '\u{e4a4}',
        Lucide::Stamp => '\u{e3bf}',
        Lucide::Star => '\u{e17a}',
        Lucide::StarHalf => '\u{e20b}',
        Lucide::StarOff => '\u{e2b0}',
        Lucide::StepBack => '\u{e3ed}',
        Lucide::StepForward => '\u{e3ee}',
        Lucide::Stethoscope => '\u{e2f1}',
        Lucide::Sticker => '\u{e302}',
        Lucide::StickyNote => '\u{e303}',
        Lucide::Store => '\u{e3e8}',
        Lucide::StretchHorizontal => '\u{e27c}',
        Lucide::StretchVertical => '\u{e27d}',
        Lucide::Strikethrough => '\u{e17b}',
        Lucide::Subscript => '\u{e25c}',
        Lucide::Sun => '\u{e17c}',
        Lucide::SunDim => '\u{e299}',
        Lucide::SunMedium => '\u{e2b1}',
        Lucide::SunMoon => '\u{e2b2}',
        Lucide::SunSnow => '\u{e376}',
        Lucide::Sunrise => '\u{e17d}',
        Lucide::Sunset => '\u{e17e}',
        Lucide::Superscript => '\u{e25e}',
        Lucide::SwatchBook => '\u{e5a4}',
        Lucide::SwissFranc => '\u{e17f}',
        Lucide::SwitchCamera => '\u{e180}',
        Lucide::Sword => '\u{e2b3}',
        Lucide::Swords => '\u{e2b4}',
        Lucide::Syringe => '\u{e2f2}',
        Lucide::Table => '\u{e181}',
        Lucide::TableCellsMerge => '\u{e5cc}',
        Lucide::TableCellsSplit => '\u{e5cd}',
        Lucide::TableColumnsSplit => '\u{e5ce}',
        Lucide::TableProperties => '\u{e4e0}',
        Lucide::TableRowsSplit => '\u{e5cf}',
        Lucide::TableTwo => '\u{e2f9}',
        Lucide::Tablet => '\u{e182}',
        Lucide::TabletSmartphone => '\u{e50f}',
        Lucide::Tablets => '\u{e3c2}',
        Lucide::Tag => '\u{e183}',
        Lucide::Tags => '\u{e360}',
        Lucide::TallyFive => '\u{e4df}',
        Lucide::TallyFour => '\u{e4de}',
        Lucide::TallyOne => '\u{e4db}',
        Lucide::TallyThree => '\u{e4dd}',
        Lucide::TallyTwo => '\u{e4dc}',
        Lucide::Tangent => '\u{e533}',
        Lucide::Target => '\u{e184}',
        Lucide::Telescope => '\u{e5ca}',
        Lucide::Tent => '\u{e227}',
        Lucide::TentTree => '\u{e540}',
        Lucide::Terminal => '\u{e185}',
        Lucide::TestTube => '\u{e40a}',
        Lucide::TestTubeDiagonal => '\u{e40b}',
        Lucide::TestTubes => '\u{e40c}',
        Lucide::Text => '\u{e3ef}',
        Lucide::TextCursor => '\u{e264}',
        Lucide::TextCursorInput => '\u{e265}',
        Lucide::TextQuote => '\u{e4a3}',
        Lucide::TextSearch => '\u{e5b2}',
        Lucide::TextSelect => '\u{e3e2}',
        Lucide::Theater => '\u{e527}',
        Lucide::Thermometer => '\u{e186}',
        Lucide::ThermometerSnowflake => '\u{e187}',
        Lucide::ThermometerSun => '\u{e188}',
        Lucide::ThumbsDown => '\u{e189}',
        Lucide::ThumbsUp => '\u{e18a}',
        Lucide::Ticket => '\u{e20f}',
        Lucide::TicketCheck => '\u{e5b3}',
        Lucide::TicketMinus => '\u{e5b4}',
        Lucide::TicketPercent => '\u{e5b5}',
        Lucide::TicketPlus => '\u{e5b6}',
        Lucide::TicketSlash => '\u{e5b7}',
        Lucide::TicketX => '\u{e5b8}',
        Lucide::Timer => '\u{e1e0}',
        Lucide::TimerOff => '\u{e249}',
        Lucide::TimerReset => '\u{e236}',
        Lucide::ToggleLeft => '\u{e18b}',
        Lucide::ToggleRight => '\u{e18c}',
        Lucide::Tornado => '\u{e218}',
        Lucide::Torus => '\u{e534}',
        Lucide::Touchpad => '\u{e44e}',
        Lucide::TouchpadOff => '\u{e44f}',
        Lucide::TowerControl => '\u{e3c0}',
        Lucide::ToyBrick => '\u{e34b}',
        Lucide::Tractor => '\u{e509}',
        Lucide::TrafficCone => '\u{e50a}',
        Lucide::TrainFront => '\u{e50b}',
        Lucide::TrainFrontTunnel => '\u{e50c}',
        Lucide::TrainTrack => '\u{e50d}',
        Lucide::TramFront => '\u{e2a9}',
        Lucide::Trash => '\u{e18d}',
        Lucide::TrashTwo => '\u{e18e}',
        Lucide::TreeDeciduous => '\u{e2f3}',
        Lucide::TreePalm => '\u{e281}',
        Lucide::TreePine => '\u{e2f4}',
        Lucide::Trees => '\u{e2f5}',
        Lucide::Trello => '\u{e18f}',
        Lucide::TrendingDown => '\u{e190}',
        Lucide::TrendingUp => '\u{e191}',
        Lucide::Triangle => '\u{e192}',
        Lucide::TriangleAlert => '\u{e193}',
        Lucide::TriangleRight => '\u{e4f2}',
        Lucide::Trophy => '\u{e377}',
        Lucide::Truck => '\u{e194}',
        Lucide::Turtle => '\u{e4fe}',
        Lucide::Tv => '\u{e195}',
        Lucide::TvMinimal => '\u{e203}',
        Lucide::TvMinimalPlay => '\u{e5f1}',
        Lucide::Twitch => '\u{e196}',
        Lucide::Twitter => '\u{e197}',
        Lucide::Type => '\u{e198}',
        Lucide::TypeOutline => '\u{e607}',
        Lucide::Umbrella => '\u{e199}',
        Lucide::UmbrellaOff => '\u{e548}',
        Lucide::Underline => '\u{e19a}',
        Lucide::Undo => '\u{e19b}',
        Lucide::UndoDot => '\u{e456}',
        Lucide::UndoTwo => '\u{e2a1}',
        Lucide::UnfoldHorizontal => '\u{e442}',
        Lucide::UnfoldVertical => '\u{e443}',
        Lucide::Ungroup => '\u{e46c}',
        Lucide::University => '\u{e3e9}',
        Lucide::Unlink => '\u{e19c}',
        Lucide::UnlinkTwo => '\u{e19d}',
        Lucide::Unplug => '\u{e462}',
        Lucide::Upload => '\u{e19e}',
        Lucide::Usb => '\u{e35a}',
        Lucide::User => '\u{e19f}',
        Lucide::UserCheck => '\u{e1a0}',
        Lucide::UserCog => '\u{e346}',
        Lucide::UserMinus => '\u{e1a1}',
        Lucide::UserPen => '\u{e601}',
        Lucide::UserPlus => '\u{e1a2}',
        Lucide::UserRound => '\u{e46d}',
        Lucide::UserRoundCheck => '\u{e46e}',
        Lucide::UserRoundCog => '\u{e46f}',
        Lucide::UserRoundMinus => '\u{e470}',
        Lucide::UserRoundPen => '\u{e602}',
        Lucide::UserRoundPlus => '\u{e471}',
        Lucide::UserRoundSearch => '\u{e57d}',
        Lucide::UserRoundX => '\u{e472}',
        Lucide::UserSearch => '\u{e57e}',
        Lucide::UserX => '\u{e1a3}',
        Lucide::Users => '\u{e1a4}',
        Lucide::UsersRound => '\u{e473}',
        Lucide::Utensils => '\u{e2f6}',
        Lucide::UtensilsCrossed => '\u{e2f7}',
        Lucide::UtilityPole => '\u{e3c6}',
        Lucide::Variable => '\u{e478}',
        Lucide::Vault => '\u{e594}',
        Lucide::Vegan => '\u{e3a1}',
        Lucide::VenetianMask => '\u{e2aa}',
        Lucide::Vibrate => '\u{e223}',
        Lucide::VibrateOff => '\u{e29d}',
        Lucide::Video => '\u{e1a5}',
        Lucide::VideoOff => '\u{e1a6}',
        Lucide::Videotape => '\u{e4d0}',
        Lucide::View => '\u{e1a7}',
        Lucide::Voicemail => '\u{e1a8}',
        Lucide::Volume => '\u{e1a9}',
        Lucide::VolumeOne => '\u{e1aa}',
        Lucide::VolumeTwo => '\u{e1ab}',
        Lucide::VolumeX => '\u{e1ac}',
        Lucide::Vote => '\u{e3b1}',
        Lucide::Wallet => '\u{e204}',
        Lucide::WalletCards => '\u{e4d1}',
        Lucide::WalletMinimal => '\u{e4d2}',
        Lucide::Wallpaper => '\u{e450}',
        Lucide::Wand => '\u{e246}',
        Lucide::WandSparkles => '\u{e35b}',
        Lucide::Warehouse => '\u{e3ea}',
        Lucide::WashingMachine => '\u{e595}',
        Lucide::Watch => '\u{e1ad}',
        Lucide::Waves => '\u{e283}',
        Lucide::Waypoints => '\u{e547}',
        Lucide::Webcam => '\u{e205}',
        Lucide::Webhook => '\u{e378}',
        Lucide::WebhookOff => '\u{e5bc}',
        Lucide::Weight => '\u{e535}',
        Lucide::Wheat => '\u{e3a2}',
        Lucide::WheatOff => '\u{e3a3}',
        Lucide::WholeWord => '\u{e3e3}',
        Lucide::Wifi => '\u{e1ae}',
        Lucide::WifiHigh => '\u{e5fc}',
        Lucide::WifiLow => '\u{e5fd}',
        Lucide::WifiOff => '\u{e1af}',
        Lucide::WifiZero => '\u{e5fe}',
        Lucide::Wind => '\u{e1b0}',
        Lucide::Wine => '\u{e2f8}',
        Lucide::WineOff => '\u{e3a4}',
        Lucide::Workflow => '\u{e42a}',
        Lucide::Worm => '\u{e5df}',
        Lucide::WrapText => '\u{e248}',
        Lucide::Wrench => '\u{e1b1}',
        Lucide::X => '\u{e1b2}',
        Lucide::Youtube => '\u{e1b3}',
        Lucide::Zap => '\u{e1b4}',
        Lucide::ZapOff => '\u{e1b5}',
        Lucide::ZoomIn => '\u{e1b6}',
        Lucide::ZoomOut => '\u{e1b7}',
    }
}

/// Converts an [`Lucide`] into a [`char`]
#[must_use]
pub fn icon_to_string(icon: Lucide) -> String {
    icon_to_char(icon).to_string()
}

impl Display for Lucide {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", icon_to_char(*self))
    }
}

impl From<Lucide> for char {
    fn from(icon: Lucide) -> Self {
        icon_to_char(icon)
    }
}

impl From<Lucide> for String {
    fn from(icon: Lucide) -> Self {
        format!("{}", icon_to_char(icon))
    }
}

/// Converts an [`Nerd`] into a [`Text`] with the font.
#[must_use]
pub fn icon_to_text(icon: Lucide) -> Text<'static> {
    text(icon_to_char(icon)).font(NERD_FONT)
}
