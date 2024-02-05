mod WavReader;
use std::env;

fn main (){
   //WavReader::wav_reader::test();

   let current_dir = env::current_dir().expect("Unable to get current directory");
    println!("Current directory: {:?}", current_dir);
   
   let file_path = "./resource/3s.wav";
   println!("{}",file_path);
   let wavReader = WavReader::WavReader::WavReader::new(file_path);
    match wavReader {
        Ok(reader) => {
            //let spectrum = reader.fft();
            //println!("Spectre de fréquences: {:?}", spectrum);
            println!("Echantillons audio bruts: {:?}", reader.samples);
            println!("Nombre de canaux: {}", reader.channels);
            println!("Largeur d'échantillon (en octets): {}", reader.sample_width);
            println!("Fréquence d'échantillonnage: {}", reader.frame_rate);
        }
        Err(e) => println!("Erreur: {}", e),
    }

}