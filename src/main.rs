mod processamento;
use processamento::{conversao_analogica, verificar_alarmes};

fn main() {
    // 0. Valores teste
    let limite_inf = 10.0;
    let limite_sup = 80.0;
    let faixa_bits = [0.0, 27648.0];
    let faixa_sensor = [0.0, 100.0];
    let valor_lido = 27000; // Exemplo de valor lido da entrada analógica

    // 2. Converte o valor lido para a temperatura em °C
    let temp_atual = conversao_analogica(&faixa_bits, &faixa_sensor, valor_lido);
    // 3. Chama a função
    let _resultado = verificar_alarmes(temp_atual, limite_inf, limite_sup);
}
