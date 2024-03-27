fn create_midi(pitch_tracking: Vec<f32>) {
    
}

//Convertir une fréquence en note (donne la note la plus proche pour l'instant), utile notamment pour l'exportation en MIDI
frequency_to_note(frequency: f32) -> {
    let reference_note = 440.0; // Fréquence de la note A a l'octave 4 couramment utilisée comme note de référence
    let semitone = ((target_frequency / reference_frequency).log2() * 12 as f32).round() as i32; //12 car il y a 12 semi ton par octave
    semitone;
}
