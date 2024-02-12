use std::{error::Error, sync::Arc};

use once_cell::sync::Lazy;

use piper::{synth::PiperSpeechSynthesizer, vits::VitsModel};

static ENVIRONMENT: Lazy<Arc<ort::Environment>> =
    Lazy::new(|| Arc::new(ort::Environment::default()));

fn main() -> Result<(), Box<dyn Error>> {
    // let speaker = Arc::new(VitsModel::new(
    //     "piper-test/piper-voices/sv/sv_SE/nst/medium/sv_SE-nst-medium.onnx.json".into(),
    //     "piper-test/piper-voices/sv/sv_SE/nst/medium/sv_SE-nst-medium.onnx".into(),
    //     &ENVIRONMENT,
    // )?);
    // let speaker = Arc::new(VitsModel::new(
    //     "piper-test/piper-voices/uk/uk_UA/ukrainian_tts/medium/uk_UA-ukrainian_tts-medium.onnx.json".into(),
    //     "piper-test/piper-voices/uk/uk_UA/ukrainian_tts/medium/uk_UA-ukrainian_tts-medium.onnx".into(),
    //     &ENVIRONMENT,
    // )?);
    let speaker = Arc::new(VitsModel::new(
        "piper-test/piper-voices/ru/ru_RU/irina/medium/ru_RU-irina-medium.onnx.json".into(),
        "piper-test/piper-voices/ru/ru_RU/irina/medium/ru_RU-irina-medium.onnx".into(),
        &ENVIRONMENT,
    )?);
    // let speaker = Arc::new(VitsModel::new(
    //     "piper-test/piper-voices/de/de_DE/thorsten_emotional/medium/de_DE-thorsten_emotional-medium.onnx.json".into(),
    //     "piper-test/piper-voices/de/de_DE/thorsten_emotional/medium/de_DE-thorsten_emotional-medium.onnx".into(),
    //     &ENVIRONMENT,
    // )?);
    for speaker in speaker.speakers()? {
        println!("Speaker {}: {}", speaker.0, speaker.1);
    }
    speaker.set_length_scale(1.)?;
    // speaker.set_speaker("default".to_string())?;
    // speaker.set_speaker("whisper".to_string())?;
    // speaker.set_speaker("drunk".to_string())?;


    let synthesizer = PiperSpeechSynthesizer::new(speaker)?;
    // synthesizer.synthesize_to_wav_file("kaniner.wav", "Hej på dig min lilla kanin!".to_string())?;
    // synthesizer.synthesize_to_wav_file("kaniner.wav", "Весе́лка, також ра́йдуга оптичне явище в атмосфері, що являє собою одну, дві чи декілька різнокольорових дуг ,або кіл, якщо дивитися з повітря, що спостерігаються на тлі хмари, якщо вона розташована проти Сонця.".to_string())?;
    synthesizer.synthesize_to_wav_file("kaniner.wav", "«Основы промышленной системы Германии падают, как домино. США отдаляются от Европы и стремятся конкурировать со своими трансатлантическими союзниками в сфере климатических инвестиций… Последним ударом по некоторым производителям в тяжёлой промышленности стало прекращение поставок огромных объёмов дешёвого российского природного газа», — пишут авторы материала.".to_string())?;
    // synthesizer.synthesize_to_wav_file(
    //     "kaniner.wav",
    //     r"Ich bin so wild nach deinem Erdbeermund,
    //         ich schrie mir schon die Lungen wund
    //         nach deinem weißen Leib, du Weib."
    //         .to_string(),
    // )?;

    Ok(())
}
