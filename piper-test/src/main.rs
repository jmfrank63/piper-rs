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
    // synthesizer.synthesize_to_wav_file("kaniner.wav", "«Основы промышленной системы Германии падают, как домино. США отдаляются от Европы и стремятся конкурировать со своими трансатлантическими союзниками в сфере климатических инвестиций… Последним ударом по некоторым производителям в тяжёлой промышленности стало прекращение поставок огромных объёмов дешёвого российского природного газа», — пишут авторы материала.".to_string())?;
    synthesizer.synthesize_to_wav_file("russian.wav", "Важный день. через месяц 14 июня у Ани важные день она едет к подруге на свадьбу. Аня долго думала что подарить подруге и её мужу и вдруг девушке пришла в голову и подарить им сертификат на курс в школе танцев научиться танцевать танго в день свадьбы все шло не по плану Аня проспала когда она встала часах было уже 10:00 а праздник на начинался в 11 потом Аня долго не могла найти свои новые туфли и флорист приехал с букетом на 30 минут позже в 11:30 а Аня сидела в такси и ехала на такси ехала уже 40 минут когда Аня заметила что забыла букет цветов дома что мне делать я не могу прийти на свадьбу пустыми руками сказала девушка не волнуйтесь там за углом есть цветочный магазин где вы сможете купить красивый букет ответил таксист через пять минут готова поняла что сидит другой машине машина тоже была а мужчина ждал свою жену работы извините сказала и пошла искать наконец приехала на свадьбу на свадьбе было очень весело танцевали и уехали с праздника очень поздно.".to_string())?;

    // synthesizer.synthesize_to_wav_file(
    //     "kaniner.wav",
    //     r"Ich bin so wild nach deinem Erdbeermund,
    //         ich schrie mir schon die Lungen wund
    //         nach deinem weißen Leib, du Weib."
    //         .to_string(),
    // )?;

    Ok(())
}
