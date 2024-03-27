pub fn pitch_tracking(wav_reader: &WavReader) -> Vec<f32> {
    // Define frame size and hop size
    let frame_size = 1024; // Choose appropriate frame size (e.g., 20-40 ms)
    let hop_size = 256; // Choose appropriate hop size (e.g., 50% overlap)

    // Pitch contour = evolution du pitch au fil du temps
    let mut pitch_contour: Vec<f32> = Vec::new();

    // Process each frame of the audio signal
    for frame_start in (0..wav_reader.samples.len()).step_by(hop_size) {
        let frame_end = (frame_start + frame_size).min(wav_reader.samples.len());
        let frame = &wav_reader.samples[frame_start..frame_end];

        let (fft_peak_frequency, autocorr_peak_frequency) = process_frame(frame, wav_reader.frame_rate);

        // Moyenne avec coeff des résultats FFT et autocorrelation, valeurs a tester
        let weighted_avg_frequency = (0.8 * autocorr_peak_frequency + 0.2 * fft_peak_frequency);

        pitch_contour.push(weighted_avg_frequency);
    }

    // Post processing = smoothing et d'autres techniques pour améliorer la précision du résultat
    let final_pitch_contour = post_processing(&pitch_contour);

    final_pitch_contour
}



// Function to process each frame (compute FFT and autocorrelation)
fn process_frame(frame: &[i16], sample_rate: u32) -> (f32, f32) {

    let fft_spectrum = frame.fft();
    let autocorrelation_function = autocorrelation(frame);

    let fft_peak_frequency = peak_frequency_fft(&fft_spectrum, sample_rate);
    let autocorr_peak_frequency = peak_frequency_autocorr(&autocorrelation_function, sample_rate);

    // Return peak frequencies
    (fft_peak_frequency, autocorr_peak_frequency)
}

// Calculer l'auto correlation
fn compute_autocorrelation(frame: &[i16]) -> Vec<f32> {
    let frame_len = frame.len();
    let mut autocorrelation_function = vec![0.0; frame_len];

    for lag in 0..frame_len {
        for i in lag..frame_len {
            autocorrelation_function[lag] += (frame[i] as f32) * (frame[i - lag] as f32);
        }
    }

    autocorrelation_function
}


// Trouver les peaks avec FFT
fn peak_frequency_fft(sample: Vec<Complex>, sample_rate: u32) -> f32 {
    let (peak_index, _) = sample.iter().enumerate().max_by_key(|x| x.magnitude()).unwrap();
    let bin_frequency = peak_index as f32 * sample_rate / sample.len() as f32;
    bin_frequency
}

// Trouver les peaks avec l'auto correlation
fn find_peak_frequency(autocorrelation_function: &[f32], sample_rate: u32) -> f32 {
    let peak_index = autocorrelation_function.iter().enumerate().skip(1) .max_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)).map(|(index, _)| index)unwrap_or(0);
    let fundamental_period = peak_index as f32;
    let peak_frequency = sample_rate as f32 / fundamental_period;

    peak_frequency
}





// Post Processing du pitch contour
fn post_processing(pitch_contour: &[f32]) -> Vec<f32> {
    pitch_contour;
}


