use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//use std::f64::consts::PI;


#[derive(Clone, Debug, Copy)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Complex {
        Complex{re, im}
    }
    
    // Fonction pour calculer la magnitude de la fréquence obtenu à l'aide de FFT
    fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    fn zero() -> Complex {
        Complex::new(0.0, 0.0)
    }

    fn exp(k: usize, n: usize) -> Complex {
        let angle = -2.0 * std::f64::consts::PI * (k as f64) / (n as f64);
        Complex::new(angle.cos(), angle.sin())
    }

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }

    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}



pub struct WavReader { // Struct pour stocker le résultat de la lecture du fichier wav
    channels : u16,
    sample_width : u16,
    frame_rate : u32,
    pub samples : Vec<i16>,
}


impl WavReader {
    
    pub fn new_reader(file_path : &str) -> Result<WavReader, std::io::Error>{
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



    //Fonction pour convertir notre sample en liste de nombres complexe
    fn samples_to_complex(samples: &[i16]) -> Vec<Complex> {
        samples.iter().map(|&x| Complex::new(x as f64, 0.0)).collect()
    }

    fn complex_to_samples(output: &[Complex]) -> Vec<i16> {
        output.iter().map(|c| c.re.round() as i16).collect()
    }

   
    pub fn fft(&self) -> Vec<Complex> {
        let complex = Self::samples_to_complex(&self.samples);
        Self::fft_rec(&complex)
    }


    fn fft_rec(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n <= 1 {
        return input.to_vec();
    }

    let mut even: Vec<Complex> = Vec::with_capacity(n / 2);
    let mut odd: Vec<Complex> = Vec::with_capacity(n / 2);

    for i in 0..(n / 2) {
        even.push(input[2 * i]);
        odd.push(input[2 * i + 1]);
    }

    let even_result = Self::fft_rec(&even);
    let odd_result = Self::fft_rec(&odd);

    let mut result: Vec<Complex> = Vec::with_capacity(n);
    for k in 0..n {
        let exp_term = Complex::exp(k, n);
        let odd_term = exp_term.mul(odd_result[k % (n / 2)]);
        result.push(even_result[k % (n / 2)].add(odd_term));
    }

    result
    }


    pub fn ifft(&self) -> Vec<i16> {
        let complex = Self::samples_to_complex(&self.samples);
        let n = complex.len();
        let mut ifft = Self::fft_rec(&complex);
    
        // Take complex conjugate of output and scale by 1/n
        for c in &mut ifft {
            c.im = -c.im / (n as f64);
            c.re = c.re / (n as f64);
        }
    
        let ifft_output = Self::complex_to_samples(&ifft);

        ifft_output
    }

    
}