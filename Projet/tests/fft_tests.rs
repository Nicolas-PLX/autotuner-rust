mod GUI;
mod wav_reader;





fn distance(a: &[Complex], b: &[Complex]) -> f64 {
    assert_eq!(a.len(), b.len());
    let sum = a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).norm()).sum::<f64>();
    (sum / a.len() as f64).sqrt()
}


fn compare_fft(input: &[Complex]) -> f64 {
    let my_fft = WavReader::new(input).fft();
    let ref_fft = fft::FFT::new(input.len()).transform(input);
    distance(&my_fft, &ref_fft)
}


#[test]
fn test_fft() -> boolean {
    let mut max_distance = 0.0;
    for freq in 1..1000 {
        for amp in 1..10 {
            let input = generate_signal(freq, amp);
            let distance = compare_fft(&input);
            max_distance = max_distance.max(distance);
        }
    }
    false
}