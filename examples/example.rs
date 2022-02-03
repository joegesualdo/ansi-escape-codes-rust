use ansi_escape_codes::{ASCIIControlCode, ControlSequencePart, ControlSequenceInducerParameter, SelectGraphicRenditionParameter, ForegroundColor, Color, EscapeSequence, BackgroundColor, BrightForegroundColor };

fn main() {
    let params1: Vec<SelectGraphicRenditionParameter> = vec![
        //SelectGraphicRenditionParameter::SetForegroundColorParameter(ForegroundColor::BlueForeground),
        SelectGraphicRenditionParameter::SetForegroundColorParameter(
            ForegroundColor::ForegroundColor(
                Color::Color256(44)
            )
        ),
        SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
            BrightForegroundColor::BrightBlueForeground
        ),
        SelectGraphicRenditionParameter::SetBackgroundColorParameter(
            BackgroundColor::BackgroundColor(
                Color::RGB(255, 0, 127)
            )
        ),
        SelectGraphicRenditionParameter::BoldParameter,
        SelectGraphicRenditionParameter::ItalicParameter,
    ];
    let params2: Vec<SelectGraphicRenditionParameter> = vec![
        //SelectGraphicRenditionParameter::SetForegroundColorParameter(ForegroundColor::BlueForeground),
        SelectGraphicRenditionParameter::SetForegroundColorParameter(
            ForegroundColor::ForegroundColor(
                Color::Color256(44)
            )
        ),
        SelectGraphicRenditionParameter::SetBrightForegroundColorParameter(
            BrightForegroundColor::BrightBlueForeground
        ),
        SelectGraphicRenditionParameter::SetBackgroundColorParameter(
            BackgroundColor::BackgroundColor(
                Color::RGB(255, 0, 127)
            )
        ),
        SelectGraphicRenditionParameter::BoldParameter,
        SelectGraphicRenditionParameter::ItalicParameter,
    ];
    println!(
        "{}test{}test2!{}test3{}",
        EscapeSequence::SelectGraphicRenditionSequence(params1).to_string(),
        EscapeSequence::ResetColorsSequence.to_string(),
        EscapeSequence::SelectGraphicRenditionSequence(params2).to_string(),
        ASCIIControlCode::CarriageReturnCode.to_string()
    );
    //println!("\x1b[55;2mwoo");
}
