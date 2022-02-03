use std::fmt;

pub enum ASCIIControlCode {
    NullCode,
    StartOfHeadingCode,
    StartOfTextCode,
    EndofTextCode,
    EndOfTransmissionCode,
    EnquiryCode,
    AcknowledgeCode,
    BellCode,
    BackspaceCode,
    HorizontalTabCode,
    LineFeedCode,
    VerticalTabCode,
    FormFeedCode,
    CarriageReturnCode,
    ShiftOutCode,
    ShiftInCode,
    DataLinkEscapeCode,
    DeviceControlOneCode,
    DeviceControlTwoCode,
    DeviceControlThreeCode,
    DeviceControlFourCode,
    NegativeAcknowledgeCode,
    SynchronousIdleCode,
    EndOfTransmissionBlockCode,
    CancelCode,
    EndOfMediumCode,
    SubsituteCode,
    EscapeCode,
    FileSeperatorCode,
    GroupSeperatorCode,
    RecordSeperatorCode,
    UnitSeperatorCode,
    SpaceCode,
    DeleteCode,
}

impl ASCIIControlCode {
    pub fn to_string(self) -> String {
        match self {
            ASCIIControlCode::EscapeCode => String::from("\x1b"),
            ASCIIControlCode::LineFeedCode=> String::from("\x0A"),
            ASCIIControlCode::CarriageReturnCode => String::from("\x0D"),
            _ => panic!(), // TODO: Implement the rest
        }
    }
}

pub enum ControlSequenceInducerParameter {
    CursorUpParameter(u32),
    CursorDownParameter(u32),
    CursorForwardParameter(u32),
    CursorBackParameter(u32),
    CursorNextLineParameter(u32),
    CursorPreviousLineParameter(u32),
    CursorHorizontalAbsoluteParameter(u32),
    CursorPositionParameter(u32, u32),
    EraseScreenAfterCursorParameter,
    EraseScreenBeforeCursorParameter,
    EraseScreenParameter,
    EraseScreenAndScrollbackParameter,
    EraseLineAfterCursorParameter,
    EraseLineBeforeCursorParameter,
    EraseLineParameter,
    ScrollUpParameter(u32),
    ScrollDownParameter(u32),
    HorizontalVerticalPositionParameter(u32, u32),
    SelectGraphicRenditionSequenceParameter(Vec<SelectGraphicRenditionParameter>),
    AUXPortOnParameter,
    AUXPortOffParameter,
    DeviceStatusReportParameter,
    SaveCursorPositionParameter,
    RestoreCursorPositionParameter,
}

impl ControlSequenceInducerParameter {
    pub fn to_string(self) -> String {
        match self {
            ControlSequenceInducerParameter::CursorUpParameter(move_count) => {
                return format!("{}A", move_count.to_string());
            }
            ControlSequenceInducerParameter::CursorDownParameter(move_count) => {
                return format!("{}B", move_count.to_string());
            }
            ControlSequenceInducerParameter::CursorForwardParameter(move_count) => {
                return format!("{}C", move_count.to_string());
            }
            ControlSequenceInducerParameter::CursorBackParameter(move_count) => {
                return format!("{}D", move_count.to_string());
            }
            ControlSequenceInducerParameter::CursorNextLineParameter(move_count) => {
                return format!("{}E", move_count.to_string());
            }
            ControlSequenceInducerParameter::CursorPreviousLineParameter(move_count) => {
                return format!("{}F", move_count.to_string());
            }
            ControlSequenceInducerParameter::CursorHorizontalAbsoluteParameter(move_count) => {
                return format!("{}F", move_count.to_string());
            }
            ControlSequenceInducerParameter::CursorPositionParameter(
                row_position,
                column_position,
            ) => {
                return format!(
                    "{};{}H",
                    row_position.to_string(),
                    column_position.to_string()
                );
            }
            ControlSequenceInducerParameter::EraseScreenAfterCursorParameter => String::from("0J"),
            ControlSequenceInducerParameter::EraseScreenBeforeCursorParameter => String::from("1J"),
            ControlSequenceInducerParameter::EraseScreenParameter => String::from("2J"),
            ControlSequenceInducerParameter::EraseScreenAndScrollbackParameter => {
                String::from("3J")
            }
            ControlSequenceInducerParameter::EraseLineAfterCursorParameter => String::from("0K"),
            ControlSequenceInducerParameter::EraseLineBeforeCursorParameter => String::from("1K"),
            ControlSequenceInducerParameter::EraseLineParameter => String::from("2K"),
            ControlSequenceInducerParameter::ScrollUpParameter(move_count) => {
                return format!("{}S", move_count.to_string());
            }
            ControlSequenceInducerParameter::ScrollDownParameter(move_count) => {
                return format!("{}T", move_count.to_string());
            }
            ControlSequenceInducerParameter::HorizontalVerticalPositionParameter(
                row_position,
                column_position,
            ) => {
                return format!(
                    "{};{}f",
                    row_position.to_string(),
                    column_position.to_string()
                );
            }
            ControlSequenceInducerParameter::SelectGraphicRenditionSequenceParameter(params) => {
                let s: String = params
                    .into_iter()
                    .map(|param| format!("{};", param.to_string()))
                    .collect();
                return format!("{}m", s);
            }
            ControlSequenceInducerParameter::AUXPortOnParameter => String::from("5i"),
            ControlSequenceInducerParameter::AUXPortOffParameter => String::from("4i"),
            ControlSequenceInducerParameter::DeviceStatusReportParameter => String::from("6n"), // TODO: This is probably wrong
            ControlSequenceInducerParameter::SaveCursorPositionParameter => String::from("s"),
            ControlSequenceInducerParameter::RestoreCursorPositionParameter => String::from("u"),
        }
    }
}

pub enum SelectGraphicRenditionParameter {
    ResetNormalParameter,
    BoldParameter,
    IncreasedSensitivityParameter,
    FaintParameter,
    DecreasedSensitivityParameter,
    ItalicParameter,
    UnderlineParameter,
    SlowBlinkParameter,
    RapidBlinkParameter,
    ReverseVideoParameter,
    ConcealParameter,
    CrossedOutParameter,
    PrimaryFontParameter,
    DefaultFontParameter,
    AlternativeFontParameter(u32), // TODO: x should only be 11-19
    FrakturParameter,
    BoldOffParameter,
    DoubleUnderlineParameter,
    NormalColorParameter,
    NormalIntensityParameter,
    NotItalicParameter,
    NotFrakturParameter,
    UnderlineOffParameter,
    BlinkOffParameter,
    InverseOffParameter,
    RevealParameter,
    NotCrossedOutParameter,
    SetForegroundColorParameter(ForegroundColor),
    DefaultForegroundColorParameter,
    SetBackgroundColorParameter(BackgroundColor),
    DefaultBackgroundColorParameter,
    FramedParameter,
    EncircledParameter,
    OverlinedParameter,
    NotFramedParameter,
    NotEncircledParameter,
    NotOverlinedParameter,
    IdeogramUnderlineParameter,
    RightSideLineParameter,
    // IdeogramDoubleUnderlineParameter,
    // DoubleLIneOnRightSideParameter,
    // IdeogramStressMarkingParameter,
    // IdeogramAttributesOffParameter,
    SetBrightForegroundColorParameter(BrightForegroundColor),
    SetBrightBackgroundColorParameter(BrightBackgroundColor),
}

impl SelectGraphicRenditionParameter {
    pub fn to_string(self) -> String {
        match self {
            SelectGraphicRenditionParameter::ResetNormalParameter => String::from("0"),
            SelectGraphicRenditionParameter::BoldParameter => String::from("1"),
            SelectGraphicRenditionParameter::IncreasedSensitivityParameter => String::from("1"),
            SelectGraphicRenditionParameter::FaintParameter => String::from("2"),
            SelectGraphicRenditionParameter::DecreasedSensitivityParameter => String::from("2"),
            SelectGraphicRenditionParameter::ItalicParameter => String::from("3"),
            SelectGraphicRenditionParameter::UnderlineParameter => String::from("4"),
            SelectGraphicRenditionParameter::SlowBlinkParameter => String::from("5"),
            SelectGraphicRenditionParameter::RapidBlinkParameter => String::from("6"),
            SelectGraphicRenditionParameter::ReverseVideoParameter => String::from("7"),
            SelectGraphicRenditionParameter::ConcealParameter => String::from("8"),
            SelectGraphicRenditionParameter::CrossedOutParameter => String::from("9"),
            SelectGraphicRenditionParameter::PrimaryFontParameter => String::from("10"),
            SelectGraphicRenditionParameter::DefaultFontParameter => String::from("10"),
            SelectGraphicRenditionParameter::AlternativeFontParameter(x) => x.to_string(),
            SelectGraphicRenditionParameter::FrakturParameter => String::from("20"),
            SelectGraphicRenditionParameter::BoldOffParameter => String::from("21"),
            SelectGraphicRenditionParameter::DoubleUnderlineParameter => String::from("21"),
            SelectGraphicRenditionParameter::NormalColorParameter => String::from("22"),
            SelectGraphicRenditionParameter::NormalIntensityParameter => String::from("22"),
            SelectGraphicRenditionParameter::NotItalicParameter => String::from("23"),
            SelectGraphicRenditionParameter::NotFrakturParameter => String::from("23"),
            SelectGraphicRenditionParameter::UnderlineOffParameter => String::from("24"),
            SelectGraphicRenditionParameter::BlinkOffParameter => String::from("25"),
            SelectGraphicRenditionParameter::InverseOffParameter => String::from("27"),
            SelectGraphicRenditionParameter::RevealParameter => String::from("28"),
            SelectGraphicRenditionParameter::NotCrossedOutParameter => String::from("29"),
            SelectGraphicRenditionParameter::SetForegroundColorParameter(foreground_color) => {
                return format!("38;{}", foreground_color.to_string())
            }
            SelectGraphicRenditionParameter::DefaultForegroundColorParameter => String::from("39"),
            SelectGraphicRenditionParameter::SetBackgroundColorParameter(background_color) => {
                return format!("48;{}", background_color.to_string())
            }
            SelectGraphicRenditionParameter::DefaultBackgroundColorParameter => String::from("49"),
            SelectGraphicRenditionParameter::FramedParameter => String::from("51"),
            SelectGraphicRenditionParameter::EncircledParameter => String::from("52"),
            SelectGraphicRenditionParameter::OverlinedParameter => String::from("53"),
            SelectGraphicRenditionParameter::NotFramedParameter => String::from("54"),
            SelectGraphicRenditionParameter::NotEncircledParameter => String::from("54"),
            SelectGraphicRenditionParameter::NotOverlinedParameter => String::from("53"),
            SelectGraphicRenditionParameter::IdeogramUnderlineParameter => String::from("60"),
            SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
                foreground_color,
            ) => foreground_color.to_string(),
            SelectGraphicRenditionParameter::SetBrightBackgroundColorParameter(
                background_color,
            ) => background_color.to_string(),
            _ => panic!(), //TODO: Implement the rest
        }
    }
}

pub enum ForegroundColor {
    BlackForeground,
    RedForeground,
    GreenForeground,
    YellowForeground,
    BlueForeground,
    MagentaForeground,
    CyanForeground,
    WhiteForeground,
    ForegroundColor(Color),
}

impl ForegroundColor {
    pub fn to_string(self) -> String {
        match self {
            ForegroundColor::BlackForeground => String::from("30"),
            ForegroundColor::RedForeground => String::from("31"),
            ForegroundColor::GreenForeground => String::from("32"),
            ForegroundColor::YellowForeground => String::from("33"),
            ForegroundColor::BlueForeground => String::from("34"),
            ForegroundColor::MagentaForeground => String::from("35"),
            ForegroundColor::CyanForeground => String::from("36"),
            ForegroundColor::WhiteForeground => String::from("37"),
            ForegroundColor::ForegroundColor(color) => color.to_string(),
        }
    }
}

pub enum BrightForegroundColor {
    BrightBlackForeground,
    BrightRedForeground,
    BrightGreenForeground,
    BrightYellowForeground,
    BrightBlueForeground,
    BrightMagentaForeground,
    BrightCyanForeground,
    BrightWhiteForeground,
}
impl BrightForegroundColor {
    pub fn to_string(self) -> String {
        match self {
            BrightForegroundColor::BrightBlackForeground => String::from("1;30"),
            BrightForegroundColor::BrightRedForeground => String::from("1;31"),
            BrightForegroundColor::BrightGreenForeground => String::from("1;32"),
            BrightForegroundColor::BrightYellowForeground => String::from("1;33"),
            BrightForegroundColor::BrightBlueForeground => String::from("1;34"),
            BrightForegroundColor::BrightMagentaForeground => String::from("1;35"),
            BrightForegroundColor::BrightCyanForeground => String::from("1;36"),
            BrightForegroundColor::BrightWhiteForeground => String::from("1;37"),
        }
    }
}

pub enum BackgroundColor {
    BlackBackground,
    RedBackground,
    GreenBackground,
    YellowBackground,
    BlueBackground,
    MagentaBackground,
    CyanBackground,
    WhiteBackground,
    BackgroundColor(Color),
}

impl BackgroundColor {
    pub fn to_string(self) -> String {
        match self {
            BackgroundColor::BlackBackground => String::from("40"),
            BackgroundColor::RedBackground => String::from("41"),
            BackgroundColor::GreenBackground => String::from("42"),
            BackgroundColor::YellowBackground => String::from("43"),
            BackgroundColor::BlueBackground => String::from("44"),
            BackgroundColor::MagentaBackground => String::from("45"),
            BackgroundColor::CyanBackground => String::from("46"),
            BackgroundColor::WhiteBackground => String::from("47"),
            BackgroundColor::BackgroundColor(color) => color.to_string(),
        }
    }
}

pub enum BrightBackgroundColor {
    BrightBlackBackground,
    BrightRedBackground,
    BrightGreenBackground,
    BrightYellowBackground,
    BrightBlueBackground,
    BrightMagentaBackground,
    BrightCyanBackground,
    BrightWhiteBackground,
}

impl BrightBackgroundColor {
    pub fn to_string(self) -> String {
        match self {
            BrightBackgroundColor::BrightBlackBackground => String::from("100"),
            BrightBackgroundColor::BrightRedBackground => String::from("101"),
            BrightBackgroundColor::BrightGreenBackground => String::from("102"),
            BrightBackgroundColor::BrightYellowBackground => String::from("103"),
            BrightBackgroundColor::BrightBlueBackground => String::from("104"),
            BrightBackgroundColor::BrightMagentaBackground => String::from("105"),
            BrightBackgroundColor::BrightCyanBackground => String::from("106"),
            BrightBackgroundColor::BrightWhiteBackground => String::from("107"),
        }
    }
}

pub type ColorCode = u32;
pub type RedColorCode = ColorCode;
pub type GreenColorCode = ColorCode;
pub type BlueColorCode = ColorCode;

pub enum Color {
    Color256(ColorCode), // TODO: Validate only integers 1-256
    RGB(RedColorCode, GreenColorCode, BlueColorCode), // TODO: Validate only integers 1-256
}

impl Color {
    pub fn to_string(self) -> String {
        match self {
            Color::Color256(color_code) => format!("5;{}", color_code.to_string()),
            Color::RGB(red_color_code, green_color_code, blue_color_code) => format!(
                "2;{};{};{}",
                red_color_code.to_string(),
                green_color_code.to_string(),
                blue_color_code.to_string()
            ),
        }
    }
}

pub enum EscapeSequence {
    CursorUpSequence(u32),
    CursorDownSequence(u32),
    CursorForwardSequence(u32),
    CursorBackSequence(u32),
    CursorNextLineSequence(u32),
    CursorPreviousLineSequence(u32),
    CursorHorizontalAbsoluteSequence(u32),
    CursorPositionSequence(u32, u32),
    EraseScreenAfterCursorSequence,
    EraseScreenBeforeCursorSequence,
    EraseScreenSequence,
    EraseScreenAndScrollbackSequence,
    EraseLineAfterCursorSequence,
    EraseLineBeforeCursorSequence,
    EraseLineSequence,
    ScrollUpSequence(u32),
    ScrollDownSequence(u32),
    HorizontalVerticalPositionSequence(u32, u32),
    SelectGraphicRenditionSequence(Vec<SelectGraphicRenditionParameter>),
    AUXPortOnSequence,
    AUXPortOffSequence,
    DeviceStatusReportSequence,
    SaveCursorPositionSequence,
    RestoreCursorPositionSequence,
    ResetColorsSequence,
    ResetAttributesSequence,
}

impl EscapeSequence {
    pub fn to_string(self) -> String {
        match self {
            EscapeSequence::CursorUpSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorUpParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::CursorDownSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorDownParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::CursorForwardSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorForwardParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::CursorBackSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorBackParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::CursorNextLineSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorNextLineParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::CursorPreviousLineSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorPreviousLineParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::CursorHorizontalAbsoluteSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorHorizontalAbsoluteParameter(
                            move_count
                        )
                    )
                    .to_string()
                );
            }
            EscapeSequence::CursorPositionSequence(row_position, column_position) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::CursorPositionParameter(
                            row_position,
                            column_position
                        )
                    )
                    .to_string()
                );
            }
            EscapeSequence::EraseScreenAfterCursorSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::EraseScreenAfterCursorParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::EraseScreenBeforeCursorSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::EraseScreenBeforeCursorParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::EraseScreenSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::EraseScreenParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::EraseScreenAndScrollbackSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::EraseScreenAndScrollbackParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::EraseLineAfterCursorSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::EraseLineAfterCursorParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::EraseLineBeforeCursorSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::EraseLineBeforeCursorParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::EraseLineSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::EraseLineParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::ScrollUpSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::ScrollUpParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::ScrollDownSequence(move_count) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::ScrollDownParameter(move_count)
                    )
                    .to_string()
                );
            }
            EscapeSequence::HorizontalVerticalPositionSequence(row_position, column_position) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::HorizontalVerticalPositionParameter(
                            row_position,
                            column_position
                        )
                    )
                    .to_string()
                );
            }
            EscapeSequence::SelectGraphicRenditionSequence(params) => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::SelectGraphicRenditionSequenceParameter(
                            params
                        )
                    )
                    .to_string()
                );
            }
            EscapeSequence::AUXPortOnSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::AUXPortOnParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::AUXPortOffSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::AUXPortOffParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::DeviceStatusReportSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::DeviceStatusReportParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::SaveCursorPositionSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::SaveCursorPositionParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::RestoreCursorPositionSequence => {
                return format!(
                    "{}{}",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                    ControlSequencePart::ControlSequenceInducerParameter(
                        ControlSequenceInducerParameter::RestoreCursorPositionParameter
                    )
                    .to_string()
                );
            }
            EscapeSequence::ResetColorsSequence => {
                return format!(
                    "{}0m",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                );
            }
            EscapeSequence::ResetAttributesSequence => {
                return format!(
                    "{}39;49m",
                    ControlSequencePart::ControlSequenceInducer.to_string(),
                );
            }
        }
    }
}

// defined as an escape character followed immediately by a bracket: ESC[.
pub enum ControlSequencePart {
    ControlSequenceInducer,
    ControlSequenceInducerParameter(ControlSequenceInducerParameter),
}

impl ControlSequencePart {
    pub fn to_string(self) -> String {
        match self {
            ControlSequencePart::ControlSequenceInducer => {
                format!("{}[", ASCIIControlCode::EscapeCode.to_string())
            }
            ControlSequencePart::ControlSequenceInducerParameter(param) => {
                format!("{}", param.to_string())
            }
            _ => String::from("ERROR"),
        }
    }
}

// type ControlSequence = (ASCIIControlCode::EscapeCode, String)

//pub fn get_string(sequence: ASCIIControlCode, CSIParameter, SelectGraphicRenditionParameter) {
//match sequence
//}

pub fn ASCIIControlCode_string(code: ASCIIControlCode) -> String {
    match code {
        ASCIIControlCode::EscapeCode => String::from("\x1b"),
        _ => String::from("ERROR"),
    }
}
