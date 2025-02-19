mod GUI;
mod wav_reader;
use crate::wav_reader::wav_reader::WavReader;

fn main() {

    /* TEST DU GUI */
    GUI::gui::launch();







    /* TEST DU WAV_READER */
    let file_path = "../resource/3s.wav";
    match WavReader::new_reader(file_path) {
        Ok(reader) => {
            let spectrum = reader.fft();
            println!("Spectre de fréquences: {:?}", spectrum);
            let base_samples = reader.samples;
            println!("Samples de bases : {:?}",base_samples);
            //let inverse_samples = reader.ifft();
            //println!("Inverse samples : {:?}",&inverse_samples);
        }
        Err(e) => println!("Erreur: {}", e),
    }
    
}
