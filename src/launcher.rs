mod WavReader;

fn main (){
   
   let file_path = "";
   let wavReader = WavReader::WavReader::new(file_path);
    match wavReader {
        Ok(reader) => {
            let spectrum = reader.fft();
            println!("Spectre de fréquences: {:?}", spectrum);
        }
        Err(e) => println!("Erreur: {}", e),
    }

}