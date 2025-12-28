pub fn conversao_analogica(
    faixa_bits: &[f32],   // Ex: [0.0, 27648.0]
    faixa_sensor: &[f32], // Ex: [0.0, 100.0] para 0-100°C
    valor_lido: i32       // O valor que vem da entrada analógica
) -> f32 {
    // Convertemos o valor lido uma única vez para f32
    let x = valor_lido as f32;

    // Numerador: (x - x0) * (y1 - y0)
    let num = (x - faixa_bits[0]) * (faixa_sensor[1] - faixa_sensor[0]);

    // Denominador: (x1 - x0)
    let den = faixa_bits[1] - faixa_bits[0];

    // Retorno (sem ponto e vírgula no final)
    let resultado:f32 = (num / den) + faixa_sensor[0];

    // Printa resultado para debug
    println!("Valor convertido: {}", resultado);

    // Retorna o valor convertido
    resultado
}


pub fn verificar_alarmes(valor: f32, valor_ll: f32, valor_hh: f32) {
    match valor {
        v if v < valor_ll => println!("Alarme Baixo!"),
        v if v > valor_hh => println!("Alarme Alto!"),
        _ => println!("Valor dentro da faixa."),
    }
}