use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new_cmp(re: f64, im: f64) -> Complex {
        Complex{re, im}
    }

    fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}




pub struct WavReader { // Struct pour stocker le résultat de la lecture du fichier wav
    pub channels : u16,
    pub sample_width : u16,
    pub frame_rate : u32,
    pub samples : Vec<i16>,
}

pub fn test(){
        println!("test");
    }

impl WavReader {
    
    

    pub fn new(file_path : &str) -> Result<WavReader, std::io::Error>{
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        // On lis l'en-tête du fichier WAV
        let mut header = [0u8; 44]; //44 correspond à la taille standard d'un entête WAv TODO : a vérifié
        reader.read_exact(&mut header)?;

        // Récupérer les paramètres du fichier WAV à partir de l'en-tête
        let channels = u16::from_le_bytes([header[22], header[23]]);
        let sample_width = u16::from_le_bytes([header[34], header[35]]);
        let frame_rate = u32::from_le_bytes([
            header[24], header[25], header[26], header[27]
        ]);

        // Lire les données audio brutes
        let mut raw_data = Vec::new();
        reader.read_to_end(&mut raw_data)?;

        // Convertir les données audio brutes en échantillons
        let mut samples = Vec::new();
        let sample_size = sample_width as usize / 8;
        for i in (0..raw_data.len()).step_by(sample_size) {
            let mut sample_bytes = [0u8; 4];
            sample_bytes[..sample_size].copy_from_slice(&raw_data[i..i + sample_size]);
            let sample_value = match sample_width {
                8 => sample_bytes[0] as i16,
                16 => i16::from_le_bytes([sample_bytes[0], sample_bytes[1]]),
                24 => i32::from_le_bytes([0, sample_bytes[0], sample_bytes[1], sample_bytes[2]]) as i16,
                32 => i32::from_le_bytes([sample_bytes[0], sample_bytes[1], sample_bytes[2], sample_bytes[3]]) as i16,
                _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid sample width")),
            };
            samples.push(sample_value);
        }

        Ok(WavReader { channels, sample_width, frame_rate, samples })
    }





    // Algo basique de FFT (ALgo de Cooley-Tukey)
    /* TODO : a refaire
    pub fn fft(&self) -> Vec<Complex> {
        let mut spectre = Vec::with_capacity(self.samples.len());

        for k in 0..self.samples.len(){
            let mut sum = Complex::new_cmp(0.0,0.0);
            for n in 0..self.samples.len() {
                let angle = -2.0 * PI * k as f64 * n as f64 / self.samples.len() as f64;
                let c = Complex::new_cmp(angle.cos(), angle.sin());
                let sample_as_complex = Complex::new_cmp(self.samples[n] as f64, 0.0);
                sum.re += sample_as_complex.re * c.re - sample_as_complex.im * c.im;
                sum.im += sample_as_complex.re * c.im + sample_as_complex.im * c.re;
            }   
            spectre.push(sum);
        }
        spectre
    }

    */

}